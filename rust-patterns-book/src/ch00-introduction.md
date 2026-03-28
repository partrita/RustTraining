# Rust 디자인 패턴 및 엔지니어링 가이드 🟢

## 강사 소개
- **Microsoft SCHIE** (Silicon and Cloud Hardware Infrastructure Engineering) 팀의 수석 펌웨어 아키텍트입니다.
- 보안, 시스템 프로그래밍(펌웨어, 운영 체제, 하이퍼바이저), CPU 및 플랫폼 아키텍처, C++ 시스템 분야의 업계 베테랑입니다.
- 2017년(AWS EC2 재직 당시)부터 Rust 프로그래밍을 시작했으며, 지금까지 이 언어의 매력에 깊이 빠져 있습니다.

---

이 책은 실제 코드베이스에서 발생하는 **중급 이상의 Rust 패턴**들에 대한 실전 가이드입니다. 단순한 언어 튜토리얼이 아니라, 기초적인 Rust를 작성할 줄 아는 개발자가 한 단계 더 도약할 수 있도록 돕는 것을 목표로 합니다. 각 장은 하나의 핵심 개념을 분리하여 설명하고, 언제 왜 해당 패턴을 사용해야 하는지 명확히 하며, 즉시 실행 가능한 코드 예제와 연습 문제를 제공합니다.

## 대상 독자
- *The Rust Programming Language* (공식 입문서)를 마쳤지만 "실제로 어떻게 설계해야 할까?"라는 고민이 있는 개발자
- 실무 시스템을 Rust로 전환하려는 C++ 또는 C# 엔지니어
- 제네릭, 트레이트 경계, 수명(Lifetime) 오류로 인해 벽에 부딪혀 체계적인 툴킷이 필요한 분

## 선수 지식
시작하기 전에 다음 개념들에 익숙해야 합니다:
- 소유권, 빌림, 수명(기초 수준)
- 열거형(Enum), 패턴 매칭, `Option`/`Result`
- 구조체, 메서드, 기본 트레이트(`Display`, `Debug`, `Clone`)
- Cargo 기초: `cargo build`, `cargo test`, `cargo run`

## 책의 구성 및 활용법

### 난이도 범례
각 장은 난이도 수준에 따라 다음과 같이 표시됩니다:

| 기호 | 레벨 | 의미 |
| :--- | :--- | :--- |
| 🟢 | **기초 (Fundamentals)** | 모든 Rust 개발자가 알아야 할 핵심 개념 |
| 🟡 | **중급 (Intermediate)** | 실무 코드베이스에서 널리 사용되는 패턴 |
| 🔴 | **고급 (Advanced)** | 깊이 있는 언어 메커니즘 (필요할 때마다 다시 학습 권장) |

### 학습 로드맵 및 체크포인트

| 파트 | 주제 및 핵심 키워드 | 권장 시간 | 체크포인트 |
| :--- | :--- | :--- | :--- |
| **제 I 부: 타입 수준 패턴** | 제네릭, 트레이트, 뉴타입, PhantomData | 약 10~12시간 | 제네릭과 동적 디스패치의 성능 차이를 설명할 수 있는가? |
| **제 II 부: 동시성 및 런타임** | 채널, 스레드, 클로저, 스마트 포인터 | 약 10~12시간 | 상황에 맞는 동기화 프리미티브를 선택할 수 있는가? |
| **제 III 부: 시스템 및 운영** | 에러 처리, 직렬화, Unsafe, 매크로, API 설계 | 약 15~20시간 | "검증하지 말고 파싱하라(Parse, don't validate)" 패턴을 적용할 수 있는가? |

---

### 연습 문제 활용하기
모든 장의 끝에는 직접 실습할 수 있는 연습 문제가 포함되어 있습니다. 학습 효과를 극대화하려면:
1. **먼저 스스로 풀어보세요**: 정답을 보기 전에 최소 15분은 고민해 보세요.
2. **직접 코드를 타이핑하세요**: 복사-붙여넣기보다 직접 입력하는 것이 근육 기억(Muscle Memory) 형성에 큰 도움이 됩니다.
3. **해결책을 변형해 보세요**: 기능을 추가하거나 제약을 바꿔보며 코드를 의도적으로 망가뜨려 보세요.

부록에 포함된 **캡스톤 프로젝트**는 책 전체에서 배운 패턴들을 하나의 완성된 운영 수준 시스템으로 통합하는 과정입니다.

---

### 요약 메뉴
1.  **[제네릭의 모든 것](ch01-generics-the-full-picture.md)** 🟢: 단형성화, 코드 팽창 트레이드오프, 제네릭 vs 열거형 vs 트레이트 객체.
2.  **[트레이트 심층 분석](ch02-traits-in-depth.md)** 🟡: 연관 타입, GAT, 담요 구현(Blanket impl), vtable, HRTB.
3.  **[뉴타입과 타입 상태 패턴](ch03-the-newtype-and-type-state-patterns.md)** 🟡: 제로 비용 타입 안전성, 컴파일 타임 상태 머신, 빌더 패턴.
4.  **[PhantomData](ch04-phantomdata-types-that-carry-no-data.md)** 🔴: 수명 브랜딩(Lifetime branding), 공변성(Variance), 드롭 체크.
5.  **[채널과 메시지 패싱](ch05-channels-and-message-passing.md)** 🟢: `mpsc`, `select!`, 백프레셔, 액터 패턴.
6.  **[동시성 vs 병렬성 vs 스레드](ch06-concurrency-vs-parallelism-vs-threads.md)** 🟡: Rayon, Mutex/RwLock, 원자(Atomics), 무잠금(Lock-free) 패턴.
7.  **[클로저와 고계 함수](ch07-closures-and-higher-order-functions.md)** 🟢: `Fn` 계열 트레이트, 클로저 캡처, 함수형 콤비네이터.
8.  **[함수형 vs 명령형](ch08-functional-vs-imperative-when-elegance-wins.md)** 🟡: 이터레이터 체인 vs 루프, 상향식 데이터 파이프라인 설계.
9.  **[스마트 포인터와 내부 가변성](ch09-smart-pointers-and-interior-mutability.md)** 🟡: Box, Rc, Arc, RefCell, Cow, Pin.
10. **[에러 처리 패턴](ch10-error-handling-patterns.md)** 🟢: `thiserror` vs `anyhow`, 에러 계층 설계.
11. **[직렬화 및 제로 카피](ch11-serialization-zero-copy-and-binary-data.md)** 🟡: Serde 기초, 열거형 표현식, 제로 카피 역직렬화.
12. **[Unsafe Rust](ch12-unsafe-rust-controlled-danger.md)** 🔴: 5가지 슈퍼파워, FFI, UB 함정 피하기.
13. **[매크로: 코드를 짜는 코드](ch13-macros-code-that-writes-code.md)** 🟡: `macro_rules!`, 절차적 매크로(`syn`/`quote`).
14. **[테스트 및 벤치마킹](ch14-testing-and-benchmarking-patterns.md)** 🟢: 유닛/통합/문서 테스트, 속성 기반 테스트(Proptest).
15. **[크레이트 아키텍처 및 API 설계](ch15-crate-architecture-and-api-design.md)** 🟡: 모듈 레이아웃, 인체공학적 API 설계 가이드.
16. **[비동기/Await 핵심](ch16-asyncawait-essentials.md)** 🔴: Future, Tokio 기초, 비동기 안티 패턴.
