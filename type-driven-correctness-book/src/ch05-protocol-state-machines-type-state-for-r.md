# 5. 프로토콜 상태 머신 — 실제 하드웨어를 위한 타입 상태 🔴

> **학습 목표:** 타입 상태(Type-state) 인코딩을 통해 프로토콜 위반(잘못된 순서의 명령, 종료 후 재사용 등)을 어떻게 컴파일 에러로 전환하는지 배웁니다. IPMI 세션 생명주기와 PCIe 링크 훈련(Link training)에 이 패턴을 적용해 봅니다.
>
> **관련 장:** [01장](ch01-the-philosophy-why-types-beat-tests.md) (상태 올바름), [04장](ch04-capability-tokens-zero-cost-proof-of-aut.md) (토큰), [09장](ch09-phantom-types-for-resource-tracking.md) (팬텀 타입), [11장](ch11-fourteen-tricks-from-the-trenches.md) (타입 상태 빌더와 비동기 타입 상태)

---

### 문제 요망: 프로토콜 위반

하드웨어 프로토콜은 **엄격한 상태 머신**을 가집니다. IPMI 세션은 `비인증 → 인증됨 → 활성 → 종료됨`의 상태를 거칩니다. PCIe 링크 훈련은 `감지 → 폴링 → 설정 → L0` 단계를 따릅니다. 잘못된 상태에서 명령을 보내면 세션이 깨지거나 버스가 멈출 수 있습니다.

C/C++에서는 열거형(enum)과 런타임 검사로 상태를 추적합니다.

```c
typedef enum { IDLE, AUTHENTICATED, ACTIVE, CLOSED } session_state_t;

int ipmi_send_command(ipmi_session_t *s, uint8_t cmd, uint8_t *data, int len) {
    if (s->state != ACTIVE) {        // 런타임 검사 — 잊어버리기 쉽습니다
        return -EINVAL;
    }
    // ... 명령 전송 ...
    return 0;
}
```

---

### 타입 상태(Type-State) 패턴

타입 상태 패턴에서는 각 프로토콜 상태를 **별개의 타입**으로 정의합니다. 상태 전이는 이전 상태를 소비하고 새로운 상태를 반환하는 메서드로 구현됩니다. 잘못된 상태에서는 메서드 자체가 **존재하지 않기 때문에** 컴파일러가 오류를 잡아냅니다.

#### IPMI 세션 생명주기 예시

```rust,ignore
use std::marker::PhantomData;

// 상태를 나타내는 제로 크기 마커 타입들
pub struct Idle;
pub struct Authenticated;
pub struct Active;
pub struct Closed;

/// 현재 상태를 타입 매개변수로 갖는 IPMI 세션.
pub struct IpmiSession<State> {
    transport: String,
    session_id: Option<u32>,
    _state: PhantomData<State>,
}

// 전이: Idle → Authenticated
impl IpmiSession<Idle> {
    pub fn new(host: &str) -> Self {
        IpmiSession { transport: host.to_string(), session_id: None, _state: PhantomData }
    }

    pub fn authenticate(self, user: &str, pass: &str) -> Result<IpmiSession<Authenticated>, String> {
        // self를 소비(consume)하여 Idle 상태를 무효화함
        Ok(IpmiSession { transport: self.transport, session_id: Some(42), _state: PhantomData })
    }
}

// 전이: Authenticated → Active
impl IpmiSession<Authenticated> {
    pub fn activate(self) -> Result<IpmiSession<Active>, String> {
        Ok(IpmiSession { transport: self.transport, session_id: self.session_id, _state: PhantomData })
    }
}

// Active 상태에서만 사용 가능한 작업들
impl IpmiSession<Active> {
    pub fn send_command(&mut self, netfn: u8, cmd: u8, data: &[u8]) -> Vec<u8> {
        // 전송 로직...
        vec![0x00]
    }

    pub fn close(self) -> IpmiSession<Closed> {
        IpmiSession { transport: self.transport, session_id: None, _state: PhantomData }
    }
}
```

사용 예시:

```rust,ignore
fn ipmi_workflow() -> Result<(), String> {
    let session = IpmiSession::new("192.168.1.100");

    // session.send_command(...); 
    // ❌ 에러: IpmiSession<Idle>에는 send_command 메서드가 없음

    let session = session.authenticate("admin", "password")?;
    let mut session = session.activate()?;

    // ✅ 이제야 send_command를 호출할 수 있음
    let response = session.send_command(0x04, 0x2D, &[1]);

    let _closed = session.close();
    // _closed.send_command(...); // ❌ 에러: 종료된 세션에서는 명령 불가
    Ok(())
}
```

컴파일러는 다음을 보장합니다:
- 활성화 전 인증 필수
- 명령 전송 전 활성화 필수
- 종료 후 명령 전송 불가

---

### 타입 상태와 역량 토큰의 결합

진단 프로그램이 **활성화된 세션**과 **관리자 권한**을 모두 요구한다고 가정해 봅시다.

```rust,ignore
/// 펌웨어 업데이트는 활성 세션(타입 상태)과 관리자 토큰(역량 토큰)을 모두 요구함
pub fn firmware_update(
    session: &mut IpmiSession<Active>,   // 세션이 활성 상태임을 증명
    _admin: &AdminToken,                 // 호출자가 관리자임을 증명
    image: &[u8],
) -> Result<(), String> {
    // 런타임 검사 불필요 — 시그니처가 곧 검증임
    session.send_command(0x2C, 0x01, image);
    Ok(())
}
```

---

### 타입 상태 적용 가이드

| 프로토콜 / 상황 | 타입 상태 권장 여부 |
|----------|:------:|
| IPMI/Redfish 세션 생명주기 | ✅ **강력 권장** — 인증 전 명령 전송 방지 |
| PCIe 링크 훈련 (LTSSM) | ✅ **강력 권장** — 링크가 준비되기 전 TLP 전송 방지 |
| TLS 핸드쉐이크 | ✅ **권장** — 핸드쉐이크 순서 강제 |
| 단순한 요청/응답 | ⚠️ **불필요** — 상태가 2개인 경우는 과함 |
| 비상태형 메시지 전송 | ❌ **불필요** — 추적할 상태가 없음 |

---

### 핵심 요약

1. **잘못된 호출의 원천 차단** — 메서드가 유효한 상태의 타입에만 정의되므로 순서 위반이 물리적으로 불가능합니다.
2. **소유권을 통한 전이** — 각 전이 메서드는 `self`를 소비하므로, 전이 후에도 이전 상태를 붙잡고 있을 수 없습니다.
3. **복합적 안전성** — 타입 상태와 역량 토큰(04장), 단회용 증명(03장)을 결합하여 복잡한 로직을 하나의 무결한 상태 머신으로 관리할 수 있습니다.
4. **제로 비용** — 모든 검사는 컴파일 타임에 수행되며, 실제 바이너리에는 상태 추적을 위한 오버헤드가 남지 않습니다.

