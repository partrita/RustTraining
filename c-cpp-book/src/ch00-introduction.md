# C/C++ 프로그래머를 위한 Rust 부트캠프

## 과정 안내
- **학습 내용**
    - Rust 도입 배경 (C 및 C++ 개발자의 관점에서)
    - 로컬 환경 설정 및 설치
    - 기본 문법: 타입, 함수, 제어 흐름, 패턴 매칭
    - 프로젝트 관리: 모듈 시스템과 Cargo 활용
    - 추상화: 트레이트(Traits)와 제네릭(Generics)
    - 데이터 다루기: 컬렉션(Collections)과 에러 처리
    - 심화 개념: 클로저(Closures), 메모리 관리, 수명(Lifetimes), 스마트 포인터
    - 동시성(Concurrency) 프로그래밍
    - 저수준 제어: Unsafe Rust와 FFI(Foreign Function Interface)
    - 임베디드 핵심: 펌웨어 팀을 위한 `no_std` 환경 이해
    - 실전 사례 연구: C++ 프로젝트의 Rust 전환 패턴 분석
- **참고 사항**: 본 과정에서는 비동기(Async) Rust를 깊게 다루지 않습니다. Future, Executor, `Pin`, Tokio 등 운영 환경의 비동기 패턴은 자매 과정인 [비동기 Rust 교육(Async Rust Training)](../async-book/)에서 상세히 확인하실 수 있습니다.

---

# 자기 주도 학습 가이드

본 자료는 강사 주도 교육뿐만 아니라 자기 주도 학습용으로도 정교하게 설계되었습니다. 혼자 공부하시는 분들은 아래 가이드를 참고하여 학습 효과를 극대화해 보시기 바랍니다.

### 📅 권장 학습 일정

| 단계 | 주제 | 권장 시간 | 학습 목표 (체크포인트) |
| :--- | :--- | :--- | :--- |
| **1~4장** | 환경 설정, 타입, 제어 흐름 | 1일 | 기본적인 CLI 온도 변환기를 작성할 수 있습니다. |
| **5~7장** | 데이터 구조와 소유권 | 1~2일 | `let s2 = s1` 실행 시 왜 `s1`을 더 사용할 수 없는지 설명할 수 있습니다. |
| **8~9장** | 모듈화와 에러 처리 | 1일 | `?` 연산자로 에러를 전파하는 멀티 파일 프로젝트를 설계할 수 있습니다. |
| **10~12장** | 트레이트, 제네릭, 클로저 | 1~2일 | 트레이트 경계(Trait Bounds)를 활용한 제네릭 함수를 작성할 수 있습니다. |
| **13~14장** | 동시성과 Unsafe/FFI | 1일 | `Arc<Mutex<T>>`를 사용해 스레드 안전한 카운터를 구현할 수 있습니다. |
| **15~16장** | 심층 분석 섹션 | 자율 학습 | 실무에서 해당 기술이 필요할 때 참조 자료로 활용하세요. |
| **17~19장** | 모범 사례 및 참조 | 자율 학습 | 실제 프로젝트를 구현할 때 기술적인 완성도를 높이기 위해 참고하세요. |

### 📝 연습 문제 활용 팁
- 모든 장에는 난이도별 실습 문제가 포함되어 있습니다: 🟢 초급, 🟡 중급, 🔴 도전
- **반드시 해답을 보기 전에 직접 코드를 작성해 보세요.** 빌림 검사기(Borrow Checker)와 씨름하며 고민하는 과정 자체가 성장의 핵심입니다. 컴파일러가 내뱉는 에러 메시지는 여러분의 실력을 키워줄 최고의 스승입니다.
- 15분 이상 진전이 없다면 해답을 보고 원리를 파악한 뒤, 다시 해답을 덮고 처음부터 직접 구현해 보는 방식을 추천합니다.
- [Rust Playground](https://play.rust-lang.org/)를 이용하면 별도의 설치 없이 브라우저에서 바로 코드를 실행해 볼 수 있습니다.

### 💡 학습 중 난관에 부딪혔을 때
- **에러 메시지를 정독하세요**: Rust의 컴파일러 에러 메시지는 해결 방법까지 제시할 정도로 친절하고 상세합니다.
- **기초를 다시 복습하세요**: 소유권(7장) 같은 핵심 개념은 반복해서 읽을 때 비로소 진정한 의미가 이해되는 경우가 많습니다.
- **공식 문서를 활용하세요**: [Rust 표준 라이브러리 문서](https://doc.rust-lang.org/std/)는 매우 훌륭한 자원입니다. 궁금한 타입이나 메서드는 항상 검색해 보는 습관을 들이세요.
- **비동기 개념이 필요하다면**: 자매 과정인 [비동기 Rust 교육(Async Rust Training)](../async-book/)이 큰 도움이 될 것입니다.

---

# 상세 목차

## 제 I 부 — 기초 다지기

### 1. 서론 및 동기
- [강사 소개와 학습 방법](ch01-introduction-and-motivation.md#speaker-intro-and-general-approach)
- [Rust 도입 배경](ch01-introduction-and-motivation.md#the-case-for-rust)
- [Rust의 문제 해결 방식](ch01-introduction-and-motivation.md#how-does-rust-address-these-issues)
- [Rust만의 독보적인 강점](ch01-introduction-and-motivation.md#other-rust-usps-and-features)
- [한눈에 보는 비교: Rust vs C/C++](ch01-introduction-and-motivation.md#quick-reference-rust-vs-cc)
- [C/C++ 개발자에게 Rust가 필요한 이유](ch01-1-why-c-cpp-developers-need-rust.md)
  - [전체 목록: Rust가 해결하는 고질적인 문제들](ch01-1-why-c-cpp-developers-need-rust.md#what-rust-eliminates--the-complete-list)
  - [C와 C++가 공유하는 구조적 결함](ch01-1-why-c-cpp-developers-need-rust.md#the-problems-shared-by-c-and-c)
  - [C++에서 더욱 심화된 복잡성](ch01-1-why-c-cpp-developers-need-rust.md#c-adds-more-problems-on-top)
  - [해결책: Rust는 어떻게 이 모든 것을 극복했는가?](ch01-1-why-c-cpp-developers-need-rust.md#how-rust-addresses-all-of-this)

### 2. 시작하기
- [백문이 불여일견: 코드로 이해하는 Rust](ch02-getting-started.md#enough-talk-already-show-me-some-code)
- [로컬 환경에 Rust 설치하기](ch02-getting-started.md#rust-local-installation)
- [Rust의 패키지 단위: 크레이트(Crates)](ch02-getting-started.md#rust-packages-crates)
- [실전 예제: Cargo와 크레이트 활용](ch02-getting-started.md#example-cargo-and-crates)

### 3. 기본 타입과 변수
- [Rust의 내장 타입 시스템](ch03-built-in-types.md#built-in-rust-types)
- [타입 명시와 값 할당](ch03-built-in-types.md#rust-type-specification-and-assignment)
- [타입 추론 기능 활용하기](ch03-built-in-types.md#rust-type-specification-and-inference)
- [변수 선언과 가변성(Mutability) 이해](ch03-built-in-types.md#rust-variables-and-mutability)

### 4. 제어 흐름
- [조건문: if 키워드](ch04-control-flow.md#rust-if-keyword)
- [반복문: while과 for 루프](ch04-control-flow.md#rust-loops-using-while-and-for)
- [무한 루프와 제어: loop 키워드](ch04-control-flow.md#rust-loops-using-loop)
- [Rust의 강력한 특징: 표현식 블록(Expression Blocks)](ch04-control-flow.md#rust-expression-blocks)

### 5. 데이터 구조와 컬렉션
- [배열(Array) 타입의 특징](ch05-data-structures.md#rust-array-type)
- [복합 타입: 튜플(Tuples)](ch05-data-structures.md#rust-tuples)
- [참조자(References)의 개념](ch05-data-structures.md#rust-references)
- [C++와 Rust 참조자의 결정적 차이](ch05-data-structures.md#c-references-vs-rust-references--key-differences)
- [슬라이스(Slices)로 데이터 다루기](ch05-data-structures.md#rust-slices)
- [상수(Constants)와 정적 변수(Statics)](ch05-data-structures.md#rust-constants-and-statics)
- [문자열 심층 분석: String vs &str](ch05-data-structures.md#rust-strings-string-vs-str)
- [구조체(Structs) 정의와 활용](ch05-data-structures.md#rust-structs)
- [동적 배열: Vec\<T\>](ch05-data-structures.md#rust-vec-type)
- [키-값 저장소: HashMap](ch05-data-structures.md#rust-hashmap-type)
- [연습 문제: Vec과 HashMap 실습](ch05-data-structures.md#exercise-vec-and-hashmap)

### 6. 패턴 매칭과 열거형
- [강력한 열거형(Enum) 시스템](ch06-enums-and-pattern-matching.md#rust-enum-types)
- [제어 흐름의 정수: match 문](ch06-enums-and-pattern-matching.md#rust-match-statement)
- [연습 문제: match와 열거형으로 계산기 구현하기](ch06-enums-and-pattern-matching.md#exercise-implement-add-and-subtract-using-match-and-enum)

### 7. 소유권과 메모리 관리
- [Rust만의 독특한 메모리 관리 철학](ch07-ownership-and-borrowing.md#rust-memory-management)
- [소유권, 빌림, 그리고 수명(Lifetimes)](ch07-ownership-and-borrowing.md#rust-ownership-borrowing-and-lifetimes)
- [이동 의미론(Move Semantics)의 실체](ch07-ownership-and-borrowing.md#rust-move-semantics)
- [데이터 복제: Clone 트레이트](ch07-ownership-and-borrowing.md#rust-clone)
- [자동 복사: Copy 트레이트](ch07-ownership-and-borrowing.md#rust-copy-trait)
- [리소스 해제: Drop 트레이트](ch07-ownership-and-borrowing.md#rust-drop-trait)
- [연습 문제: Move, Copy, Drop 마스터하기](ch07-ownership-and-borrowing.md#exercise-move-copy-and-drop)
- [수명(Lifetimes)과 빌림의 관계](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-and-borrowing)
- [수명 매개변수(Lifetime Annotations) 명시하기](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-annotations)
- [연습 문제: 수명을 활용한 데이터 저장](ch07-1-lifetimes-and-borrowing-deep-dive.md#exercise-slice-storage-with-lifetimes)
- [심층 분석: 수명 생략 규칙(Lifetime Elision Rules)](ch07-1-lifetimes-and-borrowing-deep-dive.md#lifetime-elision-rules-deep-dive)
- [힙 할당 포인터: Box\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#rust-boxt)
- [내부 가변성 패턴: Cell\<T\>과 RefCell\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#interior-mutability-cellt-and-refcellt)
- [공유 소유권: Rc\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#shared-ownership-rct)
- [연습 문제: 공유 소유권과 내부 가변성 조합](ch07-2-smart-pointers-and-interior-mutability.md#exercise-shared-ownership-and-interior-mutability)

### 8. 모듈과 크레이트
- [프로젝트 구조화: 크레이트와 모듈](ch08-crates-and-modules.md#rust-crates-and-modules)
- [연습 문제: 모듈과 함수 설계](ch08-crates-and-modules.md#exercise-modules-and-functions)
- [멀티 프로젝트 관리: 워크스페이스(Workspaces)](ch08-crates-and-modules.md#workspaces-and-crates-packages)
- [연습 문제: 워크스페이스 의존성 설정](ch08-crates-and-modules.md#exercise-using-workspaces-and-package-dependencies)
- [외부 생태계 활용: crates.io 사용법](ch08-crates-and-modules.md#using-community-crates-from-cratesio)
- [의존성 관리와 SemVer 규칙](ch08-crates-and-modules.md#crates-dependencies-and-semver)
- [연습 문제: rand 크레이트 실습](ch08-crates-and-modules.md#exercise-using-the-rand-crate)
- [설정 파일 이해: Cargo.toml과 Cargo.lock](ch08-crates-and-modules.md#cargotoml-and-cargolock)
- [테스트 자동화 도구: Cargo test](ch08-crates-and-modules.md#cargo-test-feature)
- [Cargo의 부가 기능 활용](ch08-crates-and-modules.md#other-cargo-features)
- [실전 테스트 패턴 분석](ch08-1-testing-patterns.md)

### 9. 에러 처리
- [열거형을 활용한 Option과 Result 프로그래밍](ch09-error-handling.md#connecting-enums-to-option-and-result)
- [값이 없을 때의 처리: Option 타입](ch09-error-handling.md#rust-option-type)
- [실패 가능성 다루기: Result 타입](ch09-error-handling.md#rust-result-type)
- [연습 문제: Option을 활용한 로깅 시스템](ch09-error-handling.md#exercise-log-function-implementation-with-option)
- [Rust 에러 처리의 정석](ch09-error-handling.md#rust-error-handling)
- [연습 문제: 실전 에러 핸들링](ch09-error-handling.md#exercise-error-handling)
- [에러 처리 모범 사례(Best Practices)](ch09-1-error-handling-best-practices.md)

### 10. 트레이트와 제네릭
- [Rust의 인터페이스: 트레이트(Traits)](ch10-traits.md#rust-traits)
- [연산자 오버로딩과 std::ops 트레이트](ch10-traits.md#c-operator-overloading--rust-stdops-traits)
- [연습 문제: Logger 트레이트 설계](ch10-traits.md#exercise-logger-trait-implementation)
- [선택의 기로: 열거형 vs dyn Trait](ch10-traits.md#when-to-use-enum-vs-dyn-trait)
- [연습 문제: 설계 역량 강화 퀴즈](ch10-traits.md#exercise-think-before-you-translate)
- [코드 재사용의 핵심: 제네릭(Generics)](ch10-1-generics.md#rust-generics)
- [연습 문제: 제네릭 프로그래밍 실습](ch10-1-generics.md#exercise-generics)
- [트레이트와 제네릭의 강력한 결합](ch10-1-generics.md#combining-rust-traits-and-generics)
- [타입 안전성 강화: 트레이트 제약(Trait Bounds)](ch10-1-generics.md#rust-traits-constraints-in-data-types)
- [연습 문제: 트레이트 제약 조건 활용](ch10-1-generics.md#exercise-traits-constraints-and-generics)
- [고급 패턴: 타입 상태(Type State)와 제네릭](ch10-1-generics.md#rust-type-state-pattern-and-generics)
- [객체 생성 패턴: Rust 빌더(Builder)](ch10-1-generics.md#rust-builder-pattern)

### 11. 타입 시스템 심화
- [타입 변환의 정석: From과 Into 트레이트](ch11-from-and-into-traits.md#rust-from-and-into-traits)
- [연습 문제: 데이터 타입 변환 실습](ch11-from-and-into-traits.md#exercise-from-and-into)
- [기본값 설정: Default 트레이트](ch11-from-and-into-traits.md#rust-default-trait)
- [기타 유용한 타입 변환 기법](ch11-from-and-into-traits.md#other-rust-type-conversions)

### 12. 함수형 프로그래밍 요소
- [유연한 코드 블록: 클로저(Closures)](ch12-closures.md#rust-closures)
- [연습 문제: 클로저와 환경 캡처 실습](ch12-closures.md#exercise-closures-and-capturing)
- [데이터 스트림 처리: 반복자(Iterators)](ch12-closures.md#rust-iterators)
- [연습 문제: 선언적 반복자 활용하기](ch12-closures.md#exercise-rust-iterators)
- [참조: 반복자 강력한 도구들(Iterator Power Tools)](ch12-1-iterator-power-tools.md#iterator-power-tools-reference)

### 13. 동시성 프로그래밍
- [안전한 멀티스레딩의 원리](ch13-concurrency.md#rust-concurrency)
- [데이터 경합(Data Races) 방지 기제: Send와 Sync](ch13-concurrency.md#why-rust-prevents-data-races-send-and-sync)
- [연습 문제: 멀티스레드 단어 계산기 구현](ch13-concurrency.md#exercise-multi-threaded-word-count)

### 14. Unsafe Rust와 FFI
- [두려움 없는 저수준 제어: Unsafe Rust](ch14-unsafe-rust-and-ffi.md#unsafe-rust)
- [기초 FFI: C에서 호출하는 Rust 라이브러리](ch14-unsafe-rust-and-ffi.md#simple-ffi-example-rust-library-function-consumed-by-c)
- [심화 FFI: 복잡한 데이터 구조 공유](ch14-unsafe-rust-and-ffi.md#complex-ffi-example)
- [Unsafe 코드의 안전성 검증 방법](ch14-unsafe-rust-and-ffi.md#ensuring-correctness-of-unsafe-code)
- [연습 문제: 안전한 FFI 래퍼(Wrapper) 설계](ch14-unsafe-rust-and-ffi.md#exercise-writing-a-safe-ffi-wrapper)

## 제 II 부 — 심층 분석 및 운영

### 15. no_std: 베어메탈 환경을 위한 Rust
- [no_std 환경의 정의와 제약 사항](ch15-no_std-rust-without-the-standard-library.md#what-is-no_std)
- [선택 가이드: no_std vs std 사용 시점](ch15-no_std-rust-without-the-standard-library.md#when-to-use-no_std-vs-std)
- [연습 문제: no_std 링 버퍼(Ring Buffer) 구현](ch15-no_std-rust-without-the-standard-library.md#exercise-no_std-ring-buffer)
- [심층 탐구: 임베디드 Rust 시스템](ch15-1-embedded-deep-dive.md)

### 16. 사례 연구: C++에서 Rust로의 전환 실전
- [전략 1: 상속 구조 → 열거형 디스패치 전환](ch16-case-studies.md#case-study-1-inheritance-hierarchy--enum-dispatch)
- [전략 2: 포인터 트리 → 아레나(Arena) 기반 설계](ch16-case-studies.md#case-study-2-shared_ptr-tree--arenaindex-pattern)
- [전략 3: 시스템 통신 → 수명 기반 빌림 모델](ch16-1-case-study-lifetime-borrowing.md#case-study-3-framework-communication--lifetime-borrowing)
- [전략 4: 거대 객체(God Object) → 조합 가능한 상태 분리](ch16-1-case-study-lifetime-borrowing.md#case-study-4-god-object--composable-state)
- [전략 5: 트레이트 객체의 올바른 사용 시점](ch16-1-case-study-lifetime-borrowing.md#case-study-5-trait-objects--when-they-are-right)

## 제 III 부 — 모범 사례와 참조 자료

### 17. 실전 모범 사례
- [Rust 개발 핵심 수칙 요약](ch17-best-practices.md#rust-best-practices-summary)
- [효율성 극대화: 과도한 clone() 호출 방지](ch17-1-avoiding-excessive-clone.md#avoiding-excessive-clone)
- [안전성 확보: 검사되지 않은 인덱싱 지양](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)
- [코드 클린업: 할당 피라미드 구조 개선](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids)
- [최종 과제: 진단 이벤트 파이프라인 설계](ch17-3-collapsing-assignment-pyramids.md#capstone-exercise-diagnostic-event-pipeline)
- [현대적인 모니터링: 로깅 및 트레이싱 생태계](ch17-4-logging-and-tracing-ecosystem.md#logging-and-tracing-ecosystem)

### 18. C++ 개발자를 위한 의미론적 심층 비교
- [캐스팅, 전처리기, volatile, static, SFINAE 등의 Rust 대응](ch18-cpp-rust-semantic-deep-dives.md)

### 19. Rust 매크로 마스터하기
- [선언적 매크로: `macro_rules!` 활용법](ch19-macros.md#declarative-macros-with-macro_rules)
- [자주 쓰이는 표준 라이브러리 매크로 분석](ch19-macros.md#common-standard-library-macros)
- [자동 구현의 마법: Derive 매크로](ch19-macros.md#derive-macros)
- [메타데이터 제어: 속성(Attribute) 매크로](ch19-macros.md#attribute-macros)
- [절차적 매크로(Procedural Macros)의 원리](ch19-macros.md#procedural-macros-conceptual-overview)
- [상황별 선택지: 매크로 vs 함수 vs 제네릭](ch19-macros.md#when-to-use-what-macros-vs-functions-vs-generics)
- [실전 연습 문제](ch19-macros.md#exercises)
