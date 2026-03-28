# 13. 실습 가이드 — 타입 안전한 Redfish 클라이언트 🟡

> **학습 목표:** 타입 상태 세션, 역량 토큰, 팬텀 타입 리소스 탐색, 차원 분석, 유효성 검증 경계, 빌더 타입 상태, 그리고 단회용 타입을 결합하여 완전하고 오버헤드 없는 Redfish 클라이언트를 구축합니다. 모든 프로토콜 위반은 런타임이 아닌 컴파일 타임 에러가 됩니다.
>
> **관련 장:** [02장](ch02-typed-command-interfaces-request-determi.md) (타입 지정 명령), [03장](ch03-single-use-types-cryptographic-guarantee.md) (단회용 타입), [04장](ch04-capability-tokens-zero-cost-proof-of-aut.md) (역량 토큰), [05장](ch05-protocol-state-machines-type-state-for-r.md) (타입 상태), [06장](ch06-dimensional-analysis-making-the-compiler.md) (차원 분석), [07장](ch07-validated-boundaries-parse-dont-validate.md) (유효성 검증 경계), [09장](ch09-phantom-types-for-resource-tracking.md) (팬텀 타입), [10장](ch15-const-fn-compile-time-correctness-proofs.md) (Const Fn), [11장](ch11-fourteen-tricks-from-the-trenches.md) (팁 4 — 빌더 타입 상태)

---

### Redfish와 타입 안전성

IPMI가 바이트 수준의 프로토콜이라면, Redfish는 REST API 기반의 복잡한 리소스 트리를 다룹니다. Redfish에서는 다음과 같은 위험 요소들이 존재하지만, 타입 기반 설계로 이를 원천 차단할 수 있습니다.

| 위험 요소 | 예시 | 결과(기존 환경) | 해결 방법(타입 기반) |
|--------|---------|-------------|-------------|
| 잘못된 URI | `/Chassis/1/Processors` (부모가 틀림) | 404 또는 데이터 유실 | 팬텀 기반 리소스 탐색 (09장) |
| 권한 누락 | 일반 사용자가 `Manager.Reset` 호출 | 403 에러 또는 보안 사고 | 역량 토큰 (04장) |
| 불완전한 PATCH | 필수 설정 항목 누락 | 설정 오염 또는 무시됨 | 빌더 타입 상태 (11장) |
| 단위 혼동 | 온도를 팬 속도와 비교 | 잘못된 임계치 판단 | 차원 분석 (06장) |

---

### 실습 1 — 세션 생명주기 (타입 상태)

Redfish 세션은 `Disconnected → Connected → Authenticated → Closed`와 같은 엄격한 상태 변화를 따릅니다.

```rust,ignore
pub struct Disconnected;
pub struct Connected;
pub struct Authenticated;

pub struct RedfishSession<S> { /* ... */ }

impl RedfishSession<Connected> {
    pub fn login(self, user: &str) -> RedfishSession<Authenticated> { /* ... */ }
}

impl RedfishSession<Authenticated> {
    pub fn get(&self, path: &str) -> Json { /* API 호출 가능 */ }
}
// Connected 상태에서는 get() 메서드 자체가 없어 호출 불가능함
```

---

### 실습 2 — 권한 토큰 (역량 토큰)

Redfish의 4가지 권한 수준을 제로 크기 타입(ZST) 토큰으로 인코딩합니다.

```rust,ignore
pub struct ConfigureManagerToken; // 관리자 권한 증명

fn reset_to_defaults(
    session: &RedfishSession<Authenticated>,
    _proof: &ConfigureManagerToken, // 이 토큰이 있어야만 호출 가능
) { /* ... */ }

// 관리자로 로그인하지 않으면 해당 토큰을 얻을 수 없어 컴파일 타임에 차단됨
```

---

### 실습 3 — 계층 구조 탐색 (팬텀 타입)

잘못된 URI를 생성하는 것은 불가능해집니다.

```rust,ignore
let root = RedfishPath::root();
let thermal = root.chassis().instance("1").thermal(); // ✅ 정상적인 경로
// let bad = root.thermal(); // ❌ 컴파일 에러: root 아래에 thermal은 존재하지 않음
```

---

### 실습 4 — 차원 텔레메트리 (차원 분석)

서버에서 응답을 파싱할 때 즉시 단위를 부여합니다.

```rust,ignore
let thermal = session.get_resource(&thermal_path)?;
// thermal.reading은 Celsius 타입이므로 Rpm 타입과 비교하려 하면 컴파일 에러 발생
```

---

### 핵심 요약

1. **복합적인 안전망** — 이 장에서는 지금까지 배운 모든 패턴을 결합하여, 리소스 경로, 권한, 상태 변화, 데이터 단위까지 모든 층위에서 안전한 클라이언트를 만드는 법을 보여줍니다.
2. **런타임 에러를 컴파일 에러로** — 404(잘못된 경로), 403(권한 없음), 400(잘못된 요청 형식) 등의 REST 오류 상당수를 코드를 실행하기도 전에 잡아낼 수 있습니다.
3. **오버헤드 없는 추상화** — 모든 타입 체크와 토큰 전달은 컴파일 시점에 사라지며, 실제 실행 시에는 효율적인 네트워크 호출만 남습니다.
4. **규모에 따른 확장성** — 이 패턴들은 소규모 스크립트부터 수천 대의 서버를 관리하는 대규모 인프라 도구까지 동일하게 적용될 수 있습니다.

