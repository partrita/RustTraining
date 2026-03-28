# 1. 철학 — 왜 타입이 테스트보다 뛰어난가 🟢

> **학습 목표:** 컴파일 타임 올바름의 세 가지 수준(값, 상태, 프로토콜)을 이해하고, 제네릭 함수 시그니처가 어떻게 컴파일러가 확인하는 보증수표 역할을 하는지 배웁니다. 또한 '설계에 의한 올바름(Correct-by-construction)' 패턴이 언제 투자가치가 있고 언제는 아닌지 구분해 봅니다.
>
> **관련 장:** [02장](ch02-typed-command-interfaces-request-determi.md) (타이핑된 명령), [05장](ch05-protocol-state-machines-type-state-for-r.md) (타입 상태), [13장](ch13-reference-card.md) (참조 카드)

---

### 런타임 검사의 비용

흔히 볼 수 있는 진단 코드베이스의 런타임 가드(Guard)를 살펴보겠습니다.

```rust,ignore
fn read_sensor(sensor_type: &str, raw: &[u8]) -> f64 {
    match sensor_type {
        "temperature" => raw[0] as i8 as f64,          // 부호 있는 바이트
        "fan_speed"   => u16::from_le_bytes([raw[0], raw[1]]) as f64,
        "voltage"     => u16::from_le_bytes([raw[0], raw[1]]) as f64 / 1000.0,
        _             => panic!("알 수 없는 센서 타입: {sensor_type}"),
    }
}
```

이 함수에는 컴파일러가 잡아낼 수 없는 **네 가지 실패 모드**가 존재합니다.

1. **오타**: `"temperture"`라고 적으면 런타임에 패닉이 발생합니다.
2. **잘못된 바이트 길이**: `fan_speed`인데 1바이트만 넘기면 런타임에 패닉이 발생합니다.
3. **논리적 버그**: 호출자가 섭씨(°C) 온도를 반환받아 놓고 RPM인 줄 알고 사용해도 아무도 알려주지 않습니다.
4. **누락**: 새로운 센서 타입이 추가되었는데 이 `match` 문에 반영되지 않으면 런타임에 패닉이 발생합니다.

이러한 모든 실패는 **배포 이후**에나 발견됩니다. 테스트가 도움이 될 수는 있겠지만, 그것도 누군가가 그 상황을 예상하고 테스트를 작성했을 때의 이야기입니다. 하지만 타입 시스템은 아무도 예상하지 못한 경우를 포함한 **모든** 경우를 커버합니다.

---

### 올바름의 세 가지 수준

#### 1단계 — 값의 올바름 (Value Correctness)
**잘못된 값을 표현 불가능하게 만드세요.**

```rust,ignore
// ❌ 어떤 u16이든 "포트"가 될 수 있음 — 0은 유효하지 않지만 컴파일은 됨
fn connect(port: u16) { /* ... */ }

// ✅ 유효성이 검증된 포트만 존재할 수 있음
pub struct Port(u16);  // 필드는 비공개(private)

impl TryFrom<u16> for Port {
    type Error = &'static str;
    fn try_from(v: u16) -> Result<Self, Self::Error> {
        if v > 0 { Ok(Port(v)) } else { Err("포트는 0보다 커야 함") }
    }
}

fn connect(port: Port) { /* ... */ }
// Port(0)은 절대 생성될 수 없음 — 불변성이 모든 곳에서 유지됨
```

**하드웨어 예시:** `SensorId(u8)` — 원시 센서 번호를 감싸서 그것이 SDR(Sensor Data Record) 범위 내에 있는지 유효성을 검사합니다.

#### 2단계 — 상태의 올바름 (State Correctness)
**잘못된 상태 전이를 표현 불가능하게 만드세요.**

```rust,ignore
use std::marker::PhantomData;

struct Disconnected;
struct Connected;

struct Socket<State> {
    fd: i32,
    _state: PhantomData<State>,
}

impl Socket<Disconnected> {
    fn connect(self, addr: &str) -> Socket<Connected> {
        // ... 연결 로직 ...
        Socket { fd: self.fd, _state: PhantomData }
    }
}

impl Socket<Connected> {
    fn send(&mut self, data: &[u8]) { /* ... */ }
    fn disconnect(self) -> Socket<Disconnected> {
        Socket { fd: self.fd, _state: PhantomData }
    }
}

// Socket<Disconnected> 타입에는 send() 메서드가 없음 — 호출 시 컴파일 에러
```

**하드웨어 예시:** GPIO 핀 모드 — `Pin<Input>`은 `read()`를 가질 수 있지만 `write()`는 가질 수 없습니다.

#### 3단계 — 프로토콜 올바름 (Protocol Correctness)
**잘못된 상호작용을 표현 불가능하게 만드세요.**

```rust,ignore
use std::io;

trait IpmiCmd {
    type Response;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Self::Response>;
}

// 설명을 위해 단순화함 — 02장에서 net_fn(), cmd_byte(), payload() 
// 등을 포함한 전체 트레이트를 볼 수 있습니다.

struct ReadTemp { sensor_id: u8 }
impl IpmiCmd for ReadTemp {
    type Response = Celsius;
    fn parse_response(&self, raw: &[u8]) -> io::Result<Celsius> {
        Ok(Celsius(raw[0] as i8 as f64))
    }
}

# #[derive(Debug)] struct Celsius(f64);

fn execute<C: IpmiCmd>(cmd: &C, raw: &[u8]) -> io::Result<C::Response> {
    cmd.parse_response(raw)
}
// ReadTemp는 항상 Celsius를 반환함 — 실수로 Rpm을 얻는 일은 절대 없음
```

**하드웨어 예시:** IPMI, Redfish, NVMe 시스템 명령 — 요청(Request) 타입이 응답(Response) 타입을 결정합니다.

---

### 컴파일러가 확인하는 보완 장치로서의 타입

여러분이 다음과 같이 코드를 작성할 때:

```rust,ignore
fn execute<C: IpmiCmd>(cmd: &C) -> io::Result<C::Response>
```

여러분은 단순한 함수를 쓰는 것이 아니라 일종의 **보증**을 하고 있는 것입니다: "`IpmiCmd`를 구현하는 모든 명령 타입 `C`에 대해, 그것을 실행하면 정확히 `C::Response`를 생성한다"는 보증 말입니다. 컴파일러는 빌드할 때마다 이 보증을 **검증**합니다. 만약 타입이 맞지 않으면 프로그램은 컴파일되지 않습니다.

이것이 Rust 타입 시스템이 강력한 이유입니다. 단순히 실수를 잡아내는 데서 그치지 않고, **컴파일 타임에 올바름을 강제**하기 때문입니다.

---

### 언제 이러한 패턴을 사용하지 "않아야" 하는가?

'설계에 의한 올바름'이 항상 최선의 선택은 아닙니다.

| 상황 | 권장 사항 |
|-----------|---------------|
| 보안이 중요한 경계 (전원 시퀀싱, 암호화) | ✅ **무조건** — 여기서의 버그는 하드웨어를 태우거나 비밀을 유출함 |
| 크레이트 간 공개(Public) API | ✅ **대체로** — 오용 자체가 컴파일 에러가 되어야 함 |
| 3개 이상의 상태를 가진 상태 머신 | ✅ **대체로** — 타입 상태가 잘못된 전이를 방지함 |
| 50줄 이내의 단순한 내부 헬퍼 함수 | ❌ **과함** — 간단한 `assert!`로 충분함 |
| 하드웨어 프로토타이핑 / 초기 탐색 시 | ❌ **원시 타입부터** — 동작이 명확해진 후에 타입을 정제함 |
| 사용자용 CLI 파싱 | ⚠️ 경계에서는 `clap` + `TryFrom`을 쓰되, 내부는 원시 타입도 괜찮음 |

핵심 질문은 이것입니다: **"이 버그가 배포 이후에 발생했을 때, 얼마나 심각한가?"**

- 팬(Fan)이 멈춰 GPU가 녹음 → **타입 사용**
- 잘못된 DER 레코드 전송으로 고객이 틀린 데이터 수신 → **타입 사용**
- 단순 디버그 로그 메시지가 약간 틀림 → **`assert!` 사용**

---

### 핵심 요약

1. **올바름의 세 가지 수준** — 값(뉴타입), 상태(타입 상태), 프로토콜(연관 타입) — 각각은 더 넓은 범주의 버그를 제거합니다.
2. **보증으로서의 타입** — 모든 제네릭 함수 시그니처는 빌드할 때마다 컴파일러가 확인하는 계약서입니다.
3. **비용의 문제** — "이 버그가 배포되면 얼마나 치명적인가?"라는 질문이 타입과 테스트 중 무엇이 적절한 도구인지 결정합니다.
4. **타입은 테스트를 보완함** — 타입은 버그의 **범주** 자체를 제거하고, 테스트는 특정 **값**이나 엣지 케이스를 확인합니다.
5. **멈출 때를 알아야 함** — 일회성 프로토타입이나 단순한 내부 헬퍼는 타입 수준의 강력한 제약이 필요하지 않은 경우가 많습니다.
