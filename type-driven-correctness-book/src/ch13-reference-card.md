# 17. 참조 카드 🟡

> **14가지 이상의 '올바른 구성(Correct-by-construction)' 패턴 요약 가이드.** 패턴 선택 플로우차트, 카탈로그, 결합 규칙, 그리고 '타입을 통한 보장' 치트 시트를 제공합니다.
>
> **관련 장:** 책의 모든 장 — 이 카드는 책 전체의 내용을 한눈에 정리한 요약본입니다.

---

### 패턴 카탈로그 (Pattern Catalogue)

| # | 패턴 | 핵심 개념 | 방지하는 버그 | 장 |
|---|---------|---------------|----------|---------|
| 1 | 타입 지성 명령 | `trait IpmiCmd { type Response; }` | 잘못된 응답 타입 처리 | 02장 |
| 2 | 단회용 타입 | `struct Nonce` (Clone/Copy 불가) | 논스(Nonce)/키 재사용 | 03장 |
| 3 | 역량 토큰 | `struct AdminToken { _private: () }` | 권한 없는 접근 | 04장 |
| 4 | 타입 상태 | `Session<Active>` | 프로토콜 순서 위반 | 05장 |
| 5 | 차원 분석 | `struct Celsius(f64)` | 물리 단위 혼동 | 06장 |
| 6 | 유효성 경계 | `struct ValidFru` (via TryFrom) | 검증되지 않은 데이터 사용 | 07장 |
| 7 | 역량 믹스인 | `trait FanDiagMixin: HasSpi` | 버스 접근 권한 누락 | 08장 |
| 8 | 팬텀 타입 | `Register<Width16>` | 너비/방향 불일치 | 09장 |
| 9 | 파수꾼 → Option | `Option<u8>` (`0xFF` 대신) | 파수꾼 값 오인 버그 | 11장 |
| 10 | 봉인된 트레이트 | `trait Cmd: private::Sealed` | 부적절한 외부 구현 | 11장 |
| 11 | 타입 상태 빌더 | `Builder<Set, Missing>` | 불완전한 객체 생성 | 11장 |
| 12 | FromStr 검증 | `impl FromStr for Level` | 잘못된 문자열 입력 | 11장 |
| 13 | 상수 제네릭 | `Bank<const N: usize>` | 버퍼 크기 불일치 | 11장 |
| 14 | 안전한 래퍼 | `MmioRegion::read_u32()` | 무분별한 MMIO/FFI 사용 | 11장 |

---

### 피해야 할 안티 패턴 (Anti-Patterns)

| 안티 패턴 | 문제점 | 올바른 대안 |
|-------------|---------------|-------------------|
| `fn read() -> f64` | 단위가 모호함 (°C? RPM?) | `fn read() -> Celsius` |
| `fn op(is_admin: bool)` | 호출자가 거짓말할 수 있음 | `fn op(_: &AdminToken)` |
| `fn send(s: &Session)` | 세션 상태를 보장 못 함 | `fn send(s: &Session<Active>)` |
| `fn process(d: &[u8])` | 검증되지 않은 원시 데이터 | `fn process(d: &ValidFru)` |
| `let id: u16 = 0xFFFF` | 파수꾼 값을 내부에 들고 다님 | `let id: Option<u16> = None` |
| `Builder::new().finish()` | 필수 필드 누락 가능 | 타입 상태 빌더 사용 |
| `impl Clone` (비밀키 등) | 단회용 보장을 파괴함 | Clone을 유도하지 않음 |

---

### 타입을 통한 보장 (Types as Guarantees)

| 보장 내용 | Rust 표현식 | 예시 |
|-----------|----------------|---------|
| "이 증거가 존재함" | 타입 그 자체 | `AdminToken` |
| "내가 증거를 가지고 있음" | 해당 타입의 값 | `let tok = authenticate()?;` |
| "A이면 B이다" | 함수 `fn(A) -> B` | `fn activate(AdminToken) -> Session<Active>` |
| "A와 B 모두 만족" | 튜플 `(A, B)` | `fn op(a: &A, b: &B)` |
| "A 또는 B 중 하나" | `enum` 또는 `Result` | `Result<Session, Error>` |
| "결코 일어날 수 없음" | `!` (Never 타입) | 생성 자체가 불가능함 |

