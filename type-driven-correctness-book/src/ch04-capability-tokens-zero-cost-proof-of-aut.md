# 4. 역량 토큰 — 비용 없는 권한 증명 🟡

> **학습 목표:** 제로 크기 타입(ZST)이 어떻게 컴파일 타임 증명 토큰 역할을 하는지 배웁니다. 이를 통해 권한 계층 구조, 전원 시퀀싱, 그리고 회수 가능한 권한을 **런타임 비용 없이** 강제하는 방법을 익힙니다.
>
> **관련 장:** [03장](ch03-single-use-types-cryptographic-guarantee.md) (단회용 타입), [05장](ch05-protocol-state-machines-type-state-for-r.md) (타입 상태), [08장](ch08-capability-mixins-compile-time-hardware-.md) (믹스인), [10장](ch05-protocol-state-machines-type-state-for-r.md) (통합)

---

### 문제 요망: 무엇을 할 권한이 있는가?

하드웨어 진단에서 일부 작업은 **위험**합니다:

- BMC 펌웨어 프로그래밍
- PCIe 링크 리셋
- OTP 퓨즈 쓰기
- 고전압 테스트 모드 활성화

C/C++에서는 이러한 작업을 런타임 검사로 보호합니다.

```c
// C — 런타임 권한 검사
int reset_pcie_link(bmc_handle_t bmc, int slot) {
    if (!bmc->is_admin) {        // 런타임 검사
        return -EPERM;
    }
    // ... 위험한 작업 수행 ...
    return 0;
}
```

위험한 작업을 수행하는 모든 함수는 이 검사를 반복해야 합니다. 하나라도 잊어버리면 권한 상승(Privilege escalation) 버그가 됩니다.

---

### 증명 토큰으로서의 제로 크기 타입 (ZST)

**역량 토큰(Capability Token)**은 호출자가 특정 작업을 수행할 권한이 있음을 증명하는 제로 크기 타입(ZST)입니다. 런타임에는 **0바이트**를 차지하며, 오직 타입 시스템에만 존재합니다.

```rust,ignore
/// 관리자 권한이 있음을 증명하는 토큰.
/// 제로 크기 타입 — 컴파일 시 완전히 사라짐.
/// Clone/Copy 불가 — 명시적으로 전달되어야 함.
pub struct AdminToken {
    _private: (),   // 모듈 외부에서 생성을 방지함
}

/// PCIe 링크가 훈련(Trained)되었음을 증명하는 토큰.
pub struct LinkTrainedToken {
    _private: (),
}

impl BmcController {
    /// 관리자로 인증 — 역량 토큰을 반환함.
    /// 이것이 AdminToken을 생성할 수 있는 유일한 방법임.
    pub fn authenticate_admin(
        &mut self,
        credentials: &[u8],
    ) -> Result<AdminToken, &'static str> {
        // ... 자격 증명 검증 ...
        Ok(AdminToken { _private: () })
    }

    /// PCIe 링크 리셋 — 관리자 권한과 링크 훈련 증명이 모두 필요함.
    /// 런타임 검사가 필요 없음 — 토큰 자체가 증명이기 때문임.
    pub fn reset_pcie_link(
        &mut self,
        _admin: &AdminToken,         // 비용 없는 권한 증명
        _trained: &LinkTrainedToken,  // 비용 없는 상태 증명
        slot: u32,
    ) -> Result<(), &'static str> {
        println!("{slot}번 슬롯의 PCIe 링크 리셋 중...");
        Ok(())
    }
}
```

사용 예시:

```rust,ignore
fn maintenance_workflow(bmc: &mut BmcController) -> Result<(), &'static str> {
    let admin = bmc.authenticate_admin(b"secret")?; // 단계 1: 권한 획득
    let trained = bmc.train_link()?;                // 단계 2: 상태 증명 획득

    // 단계 3: 컴파일러가 두 토큰을 모두 요구함
    bmc.reset_pcie_link(&admin, &trained, 0)?;
    Ok(())
}
```

이 토큰들은 컴파일된 바이너리에서 **0바이트**가 됩니다. 함수 시그니처는 "이 함수를 호출하려면 `AdminToken`을 제시해야 하며, 이를 얻는 유일한 방법은 `authenticate_admin()`뿐이다"라는 **증명 의무**를 명시적으로 나타냅니다.

---

### 계층적 역량 (Hierarchical Capabilities)

실제 시스템에는 계층이 존재합니다. 관리자는 운영자가 할 수 있는 모든 일을 할 수 있어야 합니다. 이를 트레이트 계층 구조로 모델링할 수 있습니다.

```rust,ignore
pub trait Authenticated { fn token_id(&self) -> u64; }
pub trait Operator: Authenticated {}
pub trait Admin: Operator {}

// 구체적인 토큰들:
pub struct UserToken { id: u64 }
pub struct AdminCapToken { id: u64 }

// AdminCapToken은 Authenticated, Operator, Admin을 모두 만족함
impl Authenticated for AdminCapToken { fn token_id(&self) -> u64 { self.id } }
impl Operator for AdminCapToken {}
impl Admin for AdminCapToken {}

impl Bmc {
    /// 운영자 이상만 진단을 실행할 수 있음
    pub fn run_diag(&mut self, _who: &impl Operator, test: &str) -> bool { true }

    /// 관리자만 펌웨어를 업데이트할 수 있음
    pub fn flash_firmware(&mut self, _who: &impl Admin, image: &[u8]) -> Result<(), &'static str> { Ok(()) }
}
```

`AdminCapToken`은 모든 함수에 전달될 수 있지만, `UserToken`은 `run_diag()`를 호출할 수 없습니다. 컴파일러가 이 권한 모델 전체를 **런타임 비용 0**으로 강제합니다.

---

### 역량 토큰 사용 시나리오

| 시나리오 | 적용 패턴 |
|----------|---------|
| 권한이 필요한 하드웨어 조작 | ZST 증명 토큰 (AdminToken) |
| 다단계 시퀀싱 강제 | 상태 토큰 체인 (StandbyOn → AuxiliaryOn → ...) |
| 역할 기반 접근 제어(RBAC) | 트레이트 계층 구조 (Authenticated → Operator → Admin) |
| 시간 제한 권한 | 수명(Lifetime)이 제한된 토큰 (`ScopedAdminToken<'a>`) |

---

### 핵심 요약

1. **ZST 토큰은 0바이트를 차지함** — 타입 시스템에만 존재하며 LLVM에 의해 완전히 최적화되어 사라집니다.
2. **비공개 생성자 = 위조 불가능** — 오직 인증된 모듈의 특정 함수만이 토큰을 발행할 수 있습니다.
3. **트레이트 계층으로 권한 수준 모델링** — `Admin: Operator: Authenticated` 구조는 실제 RBAC 모델을 완벽히 반영합니다.
4. **자동 권한 회수** — 수명이 할당된 토큰은 세션이 종료되면 빌림 검사기에 의해 자동으로 무효화됩니다.

