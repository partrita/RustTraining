# 14. 실습 가이드 — 타입 안전한 Redfish 서버 🟡

> **학습 목표:** 응답 빌더 타입 상태, 소스 가용성 토큰, 차원 직렬화, 헬스 롤업(Health Rollup), 스키마 버전 관리 등을 결합하여 **스키마를 위반하는 응답을 결합할 수 없는** Redfish 서버를 구축합니다. 이는 [13장](ch17-redfish-applied-walkthrough.md)의 클라이언트 실습과 대칭을 이룹니다.
>
> **관련 장:** [02장](ch02-typed-command-interfaces-request-determi.md) (액션 디스패치), [04장](ch04-capability-tokens-zero-cost-proof-of-aut.md) (소스 가용성 토큰), [06장](ch06-dimensional-analysis-making-the-compiler.md) (차원 직렬화), [07장](ch07-validated-boundaries-parse-dont-validate.md) (구성 후 직렬화), [09장](ch09-phantom-types-for-resource-tracking.md) (스키마 버전 관리), [11장](ch11-fourteen-tricks-from-the-trenches.md) (빌더 타입 상태)

---

### 대칭적인 문제: 나쁜 데이터의 배출 방지

13장(클라이언트)에서의 문제는 "나쁜 데이터를 신뢰하는 것"이었다면, 14장(서버)에서의 문제는 **"나쁜 데이터를 배출하는 것"**입니다. 서버가 보내는 단 한 번의 잘못된 응답이 수천 대의 클라이언트에 영향을 미칠 수 있습니다.

C 언어로 작성된 서버는 JSON 객체를 수동으로 조립하며 필수 필드를 누락하거나 단위를 잘못 기입하는 실수를 저지르기 쉽습니다.

---

### 실습 1 — 응답 빌더 타입 상태: "직렬화하지 말고 구성하라"

07장의 "검증하지 말고 파싱하라"는 원칙을 서버 측에서 뒤집은 것입니다. 필수 필드가 모두 채워지기 전에는 `.build()` 메서드가 나타나지 않는 빌더를 사용합니다.

```rust,ignore
pub struct ComputerSystemBuilder<Name, Uuid, PowerState, Status> { /* ... */ }

// 모든 필수 필드가 채워진 상태에서만 build() 가능
impl ComputerSystemBuilder<HasField, HasField, HasField, HasField> {
    pub fn build(self) -> Json { /* 스키마를 준수하는 JSON 생성 */ }
}
```

이제 개발자가 "Name" 필드 설정을 잊어버리면 컴파일 타임에 에러가 발생하여, 스키마를 위반하는 응답이 생성되는 것을 원천 차단합니다.

---

### 실습 2 — 소스 가용성 토큰 (역량 토큰의 변형)

서버 측에서 역량 토큰은 "데이터 소스가 성공적으로 초기화되었음"을 증명합니다. SMBIOS 테이블이 깨졌거나 센서 하위 시스템이 응답하지 않는 경우, 해당 토큰이 없으므로 관련 데이터를 사용하는 함수를 호출할 수 없습니다.

```rust,ignore
pub struct SmbiosReady; // SMBIOS 초기화 성공 증명

fn populate_from_smbios(
    builder: Builder,
    _proof: &SmbiosReady, // 이 토큰이 있어야만 SMBIOS 기반 필드 채우기 가능
    tables: &Tables,
) { /* ... */ }
```

---

### 실습 3 — 직렬화 경계에서의 차원 타입 (차원 분석)

클라이언트가 °C를 RPM으로 읽는 것을 막는 것처럼, 서버는 RPM 값을 Celsius 필드에 적는 실수를 컴파일 타임에 차단합니다.

```rust,ignore
pub struct TemperatureMember {
    pub reading_celsius: Celsius, // 반드시 Celsius 타입이어야 함
}
// 실수로 RPM 값을 넣으려 하면 컴파일러가 잡아냄
```

---

### 핵심 요약

1. **스키마 준수 강제** — 타입 상태 빌더를 통해 Redfish 스키마가 요구하는 필수 필드 누락을 컴파일 시점에 방지합니다.
2. **가용성 기반 설계** — 하드웨어 데이터 소스의 성공적인 초기화 여부를 토큰으로 관리하여, 초기화되지 않은 소스에 접근하는 런타임 오류를 차단합니다.
3. **단위 안전한 직렬화** — 서버에서 클라이언트로 전달되는 모든 수치 데이터에 단위를 부여하여, 데이터 오염 가능성을 최소화합니다.
4. **대규모 시스템의 신뢰성** — 서버 측의 타입 안전성은 인프라 전체의 안정성으로 직결됩니다. "일단 실행해보고 에러를 찾는" 방식에서 "컴파일되면 스키마를 준수함이 보장되는" 방식으로 패러다임을 전환할 수 있습니다.

