# C/C++ 프로그래머를 위한 Rust 부트캠프 과정

## 과정 개요
- 과정 개요
    - Rust 도입 배경 (C 및 C++ 관점 모두에서)
    - 로컬 설치
    - 타입, 함수, 제어 흐름, 패턴 매칭
    - 모듈, Cargo
    - 트레이트(Traits), 제네릭(Generics)
    - 컬렉션(Collections), 에러 처리
    - 클로저(Closures), 메모리 관리, 수명(Lifetimes), 스마트 포인터
    - 동시성(Concurrency)
    - FFI(Foreign Function Interface)를 포함한 Unsafe Rust
    - 펌웨어 팀을 위한 `no_std` 및 임베디드 Rust 핵심
    - 사례 연구: 실제 C++에서 Rust로의 전환 패턴
- 이 과정에서는 비동기(Async) Rust를 다루지 않습니다. Future, Executor, `Pin`, Tokio 및 운영 환경의 비동기 패턴에 대해서는 자매 과정인 [비동기 Rust 교육(Async Rust Training)](../async-book/)을 참조하십시오.


---

# 자기 주도 학습 가이드

이 자료는 강사 주도 교육과 자기 주도 학습 모두에 적합합니다. 혼자서 공부하신다면 다음의 방법을 통해 학습 효과를 극대화할 수 있습니다:

**진도 권장 사항:**

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|---------------|------------|
| 1–4 | 설정, 타입, 제어 흐름 | 1일 | CLI 온도 변환기를 작성할 수 있습니다. |
| 5–7 | 데이터 구조, 소유권 | 1–2일 | `let s2 = s1`이 왜 `s1`을 무효화하는지 설명할 수 있습니다. |
| 8–9 | 모듈, 에러 처리 | 1일 | `?`를 사용하여 에러를 전파하는 다중 파일 프로젝트를 만들 수 있습니다. |
| 10–12 | 트레이트, 제네릭, 클로저 | 1–2일 | 트레이트 경계(Trait Bounds)가 있는 제네릭 함수를 작성할 수 있습니다. |
| 13–14 | 동시성, Unsafe/FFI | 1일 | `Arc<Mutex<T>>`를 사용하여 스레드 안전한 카운터를 작성할 수 있습니다. |
| 15–16 | 심층 분석 | 개별 속도에 맞게 | 참조 자료 — 관련이 있을 때 읽어보세요. |
| 17–19 | 모범 사례 및 참조 | 개별 속도에 맞게 | 실제 코드를 작성할 때 참고하세요. |

**연습 문제 활용법:**
- 모든 장에는 난이도가 표시된 실습 연습 문제가 있습니다: 🟢 초급, 🟡 중급, 🔴 도전
- **항상 풀이를 보기 전에 먼저 직접 시도해 보세요.** 빌림 검사기(Borrow Checker)와 씨름하는 것은 학습의 일부입니다. 컴파일러의 에러 메시지가 바로 여러분의 스승입니다.
- 15분 이상 막힌다면 풀이를 펼쳐서 공부한 다음, 다시 덮고 처음부터 직접 시도해 보세요.
- [Rust Playground](https://play.rust-lang.org/)를 사용하면 로컬 설치 없이도 코드를 실행해 볼 수 있습니다.

**한계에 부딪혔을 때:**
- 컴파일러 에러 메시지를 주의 깊게 읽으세요. Rust의 에러 메시지는 매우 유용합니다.
- 관련 섹션을 다시 읽어보세요. 소유권(7장)과 같은 개념은 두 번째 읽을 때 이해되는 경우가 많습니다.
- [Rust 표준 라이브러리 문서](https://doc.rust-lang.org/std/)는 훌륭합니다. 모든 타입이나 메서드를 검색해 보세요.
- 비동기 패턴의 경우, 자매 과정인 [비동기 Rust 교육(Async Rust Training)](../async-book/)을 참조하세요.

---

# 목차

## 제 I 부 — 기초

### 1. 서론 및 동기
- [강사 소개 및 일반적인 접근 방식](ch01-introduction-and-motivation.md#speaker-intro-and-general-approach)
- [Rust 도입 배경](ch01-introduction-and-motivation.md#the-case-for-rust)
- [Rust는 이러한 문제를 어떻게 해결하는가?](ch01-introduction-and-motivation.md#how-does-rust-address-these-issues)
- [Rust의 기타 강점 및 특징](ch01-introduction-and-motivation.md#other-rust-usps-and-features)
- [빠른 참조: Rust vs C/C++](ch01-introduction-and-motivation.md#quick-reference-rust-vs-cc)
- [C/C++ 개발자에게 Rust가 필요한 이유](ch01-1-why-c-cpp-developers-need-rust.md)
  - [Rust가 제거하는 문제들 — 전체 목록](ch01-1-why-c-cpp-developers-need-rust.md#what-rust-eliminates--the-complete-list)
  - [C와 C++의 공통 문제점](ch01-1-why-c-cpp-developers-need-rust.md#the-problems-shared-by-c-and-c)
  - [C++에 추가된 추가 문제점들](ch01-1-why-c-cpp-developers-need-rust.md#c-adds-more-problems-on-top)
  - [Rust는 이 모든 것을 어떻게 해결하는가?](ch01-1-why-c-cpp-developers-need-rust.md#how-rust-addresses-all-of-this)

### 2. 시작하기
- [충분한 설명은 그만: 코드로 보여주세요](ch02-getting-started.md#enough-talk-already-show-me-some-code)
- [Rust 로컬 설치](ch02-getting-started.md#rust-local-installation)
- [Rust 패키지 (Crates)](ch02-getting-started.md#rust-packages-crates)
- [예제: Cargo 및 Crates](ch02-getting-started.md#example-cargo-and-crates)

### 3. 기본 타입 및 변수
- [내장 Rust 타입](ch03-built-in-types.md#built-in-rust-types)
- [Rust 타입 지정 및 할당](ch03-built-in-types.md#rust-type-specification-and-assignment)
- [Rust 타입 지정 및 추론](ch03-built-in-types.md#rust-type-specification-and-inference)
- [Rust 변수 및 가변성(Mutability)](ch03-built-in-types.md#rust-variables-and-mutability)

### 4. 제어 흐름
- [Rust if 키워드](ch04-control-flow.md#rust-if-keyword)
- [while 및 for를 사용한 Rust 루프](ch04-control-flow.md#rust-loops-using-while-and-for)
- [loop를 사용한 Rust 루프](ch04-control-flow.md#rust-loops-using-loop)
- [Rust 표현식 블록(Expression Blocks)](ch04-control-flow.md#rust-expression-blocks)

### 5. 데이터 구조 및 컬렉션
- [Rust 배열(Array) 타입](ch05-data-structures.md#rust-array-type)
- [Rust 튜플(Tuples)](ch05-data-structures.md#rust-tuples)
- [Rust 참조자(References)](ch05-data-structures.md#rust-references)
- [C++ 참조 vs Rust 참조 — 주요 차이점](ch05-data-structures.md#c-references-vs-rust-references--key-differences)
- [Rust 슬라이스(Slices)](ch05-data-structures.md#rust-slices)
- [Rust 상수(Constants) 및 정적 변수(Statics)](ch05-data-structures.md#rust-constants-and-statics)
- [Rust 문자열: String vs &str](ch05-data-structures.md#rust-strings-string-vs-str)
- [Rust 구조체(Structs)](ch05-data-structures.md#rust-structs)
- [Rust Vec\<T\>](ch05-data-structures.md#rust-vec-type)
- [Rust HashMap](ch05-data-structures.md#rust-hashmap-type)
- [연습 문제: Vec 및 HashMap](ch05-data-structures.md#exercise-vec-and-hashmap)

### 6. 패턴 매칭 및 열거형
- [Rust 열거형(Enum) 타입](ch06-enums-and-pattern-matching.md#rust-enum-types)
- [Rust match 문](ch06-enums-and-pattern-matching.md#rust-match-statement)
- [연습 문제: match 및 열거형을 사용하여 덧셈과 뺄셈 구현하기](ch06-enums-and-pattern-matching.md#exercise-implement-add-and-subtract-using-match-and-enum)

### 7. 소유권 및 메모리 관리
- [Rust 메모리 관리](ch07-ownership-and-borrowing.md#rust-memory-management)
- [Rust 소유권, 빌림 및 수명(Lifetimes)](ch07-ownership-and-borrowing.md#rust-ownership-borrowing-and-lifetimes)
- [Rust 이동 의미론(Move Semantics)](ch07-ownership-and-borrowing.md#rust-move-semantics)
- [Rust Clone](ch07-ownership-and-borrowing.md#rust-clone)
- [Rust Copy 트레이트](ch07-ownership-and-borrowing.md#rust-copy-trait)
- [Rust Drop 트레이트](ch07-ownership-and-borrowing.md#rust-drop-trait)
- [연습 문제: 이동(Move), 복사(Copy) 및 드롭(Drop)](ch07-ownership-and-borrowing.md#exercise-move-copy-and-drop)
- [Rust 수명 및 빌림](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-and-borrowing)
- [Rust 수명 주석(Lifetime Annotations)](ch07-1-lifetimes-and-borrowing-deep-dive.md#rust-lifetime-annotations)
- [연습 문제: 수명을 사용한 슬라이스 저장](ch07-1-lifetimes-and-borrowing-deep-dive.md#exercise-slice-storage-with-lifetimes)
- [수명 생략 규칙(Lifetime Elision Rules) 심층 분석](ch07-1-lifetimes-and-borrowing-deep-dive.md#lifetime-elision-rules-deep-dive)
- [Rust Box\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#rust-boxt)
- [내부 가변성: Cell\<T\> 및 RefCell\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#interior-mutability-cellt-and-refcellt)
- [공유 소유권: Rc\<T\>](ch07-2-smart-pointers-and-interior-mutability.md#shared-ownership-rct)
- [연습 문제: 공유 소유권 및 내부 가변성](ch07-2-smart-pointers-and-interior-mutability.md#exercise-shared-ownership-and-interior-mutability)

### 8. 모듈 및 크레이트
- [Rust 크레이트 및 모듈](ch08-crates-and-modules.md#rust-crates-and-modules)
- [연습 문제: 모듈 및 함수](ch08-crates-and-modules.md#exercise-modules-and-functions)
- [워크스페이스(Workspaces) 및 크레이트(패키지)](ch08-crates-and-modules.md#workspaces-and-crates-packages)
- [연습 문제: 워크스페이스 및 패키지 의존성 사용하기](ch08-crates-and-modules.md#exercise-using-workspaces-and-package-dependencies)
- [crates.io의 커뮤니티 크레이트 사용하기](ch08-crates-and-modules.md#using-community-crates-from-cratesio)
- [크레이트 의존성 및 SemVer](ch08-crates-and-modules.md#crates-dependencies-and-semver)
- [연습 문제: rand 크레이트 사용하기](ch08-crates-and-modules.md#exercise-using-the-rand-crate)
- [Cargo.toml 및 Cargo.lock](ch08-crates-and-modules.md#cargotoml-and-cargolock)
- [Cargo 테스트 기능](ch08-crates-and-modules.md#cargo-test-feature)
- [기타 Cargo 기능](ch08-crates-and-modules.md#other-cargo-features)
- [테스트 패턴](ch08-1-testing-patterns.md)

### 9. 에러 처리
- [열거형을 Option 및 Result에 연결하기](ch09-error-handling.md#connecting-enums-to-option-and-result)
- [Rust Option 타입](ch09-error-handling.md#rust-option-type)
- [Rust Result 타입](ch09-error-handling.md#rust-result-type)
- [연습 문제: Option을 사용한 log() 함수 구현](ch09-error-handling.md#exercise-log-function-implementation-with-option)
- [Rust 에러 처리](ch09-error-handling.md#rust-error-handling)
- [연습 문제: 에러 처리](ch09-error-handling.md#exercise-error-handling)
- [에러 처리 모범 사례](ch09-1-error-handling-best-practices.md)

### 10. 트레이트 및 제네릭
- [Rust 트레이트](ch10-traits.md#rust-traits)
- [C++ 연산자 오버로딩 → Rust std::ops 트레이트](ch10-traits.md#c-operator-overloading--rust-stdops-traits)
- [연습 문제: Logger 트레이트 구현](ch10-traits.md#exercise-logger-trait-implementation)
- [열거형 vs dyn Trait 사용 시기](ch10-traits.md#when-to-use-enum-vs-dyn-trait)
- [연습 문제: 번역하기 전에 생각하기](ch10-traits.md#exercise-think-before-you-translate)
- [Rust 제네릭](ch10-1-generics.md#rust-generics)
- [연습 문제: 제네릭](ch10-1-generics.md#exercise-generics)
- [Rust 트레이트와 제네릭 결합](ch10-1-generics.md#combining-rust-traits-and-generics)
- [데이터 타입에서의 Rust 트레이트 제약](ch10-1-generics.md#rust-traits-constraints-in-data-types)
- [연습 문제: 트레이트 제약 및 제네릭](ch10-1-generics.md#exercise-traits-constraints-and-generics)
- [Rust 타입 상태(Type State) 패턴 및 제네릭](ch10-1-generics.md#rust-type-state-pattern-and-generics)
- [Rust 빌더(Builder) 패턴](ch10-1-generics.md#rust-builder-pattern)

### 11. 타입 시스템 고급 기능
- [Rust From 및 Into 트레이트](ch11-from-and-into-traits.md#rust-from-and-into-traits)
- [연습 문제: From 및 Into](ch11-from-and-into-traits.md#exercise-from-and-into)
- [Rust Default 트레이트](ch11-from-and-into-traits.md#rust-default-trait)
- [기타 Rust 타입 변환](ch11-from-and-into-traits.md#other-rust-type-conversions)

### 12. 함수형 프로그래밍
- [Rust 클로저](ch12-closures.md#rust-closures)
- [연습 문제: 클로저 및 캡처링](ch12-closures.md#exercise-closures-and-capturing)
- [Rust 반복자(Iterators)](ch12-closures.md#rust-iterators)
- [연습 문제: Rust 반복자](ch12-closures.md#exercise-rust-iterators)
- [반복자 강력한 도구들 참조(Iterator Power Tools Reference)](ch12-1-iterator-power-tools.md#iterator-power-tools-reference)

### 13. 동시성
- [Rust 동시성](ch13-concurrency.md#rust-concurrency)
- [Rust가 데이터 경합(Data Races)을 방지하는 이유: Send 및 Sync](ch13-concurrency.md#why-rust-prevents-data-races-send-and-sync)
- [연습 문제: 멀티스레드 단어 계산기](ch13-concurrency.md#exercise-multi-threaded-word-count)

### 14. Unsafe Rust 및 FFI
- [Unsafe Rust](ch14-unsafe-rust-and-ffi.md#unsafe-rust)
- [간단한 FFI 예제](ch14-unsafe-rust-and-ffi.md#simple-ffi-example-rust-library-function-consumed-by-c)
- [복잡한 FFI 예제](ch14-unsafe-rust-and-ffi.md#complex-ffi-example)
- [Unsafe 코드의 정확성 보장](ch14-unsafe-rust-and-ffi.md#ensuring-correctness-of-unsafe-code)
- [연습 문제: 안전한 FFI 래퍼(Wrapper) 작성](ch14-unsafe-rust-and-ffi.md#exercise-writing-a-safe-ffi-wrapper)

## 제 II 부 — 심층 분석

### 15. no_std — 베어메탈(Bare Metal)을 위한 Rust
- [no_std란 무엇인가?](ch15-no_std-rust-without-the-standard-library.md#what-is-no_std)
- [no_std vs std 사용 시기](ch15-no_std-rust-without-the-standard-library.md#when-to-use-no_std-vs-std)
- [연습 문제: no_std 링 버퍼(Ring Buffer)](ch15-no_std-rust-without-the-standard-library.md#exercise-no_std-ring-buffer)
- [임베디드 심층 분석](ch15-1-embedded-deep-dive.md)

### 16. 사례 연구: 실제 C++에서 Rust로의 전환
- [사례 연구 1: 상속 계층 구조 → 열거형 디스패치](ch16-case-studies.md#case-study-1-inheritance-hierarchy--enum-dispatch)
- [사례 연구 2: shared_ptr 트리 → 아레나(Arena)/인덱스 패턴](ch16-case-studies.md#case-study-2-shared_ptr-tree--arenaindex-pattern)
- [사례 연구 3: 프레임워크 통신 → 수명 빌림](ch16-1-case-study-lifetime-borrowing.md#case-study-3-framework-communication--lifetime-borrowing)
- [사례 연구 4: 거대 객체(God Object) → 조합 가능한 상태](ch16-1-case-study-lifetime-borrowing.md#case-study-4-god-object--composable-state)
- [사례 연구 5: 트레이트 객체 — 언제 사용하는 것이 옳은가?](ch16-1-case-study-lifetime-borrowing.md#case-study-5-trait-objects--when-they-are-right)

## 제 III 부 — 모범 사례 및 참조

### 17. 모범 사례
- [Rust 모범 사례 요약](ch17-best-practices.md#rust-best-practices-summary)
- [과도한 clone() 방지하기](ch17-1-avoiding-excessive-clone.md#avoiding-excessive-clone)
- [검사되지 않은 인덱싱 방지하기](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)
- [할당 피라미드 축소하기](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids)
- [최종 연습 문제: 진단 이벤트 파이프라인](ch17-3-collapsing-assignment-pyramids.md#capstone-exercise-diagnostic-event-pipeline)
- [로깅 및 트레이싱 에코시스템](ch17-4-logging-and-tracing-ecosystem.md#logging-and-tracing-ecosystem)

### 18. C++ → Rust 의미론적 심층 분석
- [캐스팅, 전처리기, 모듈, volatile, static, constexpr, SFINAE 등](ch18-cpp-rust-semantic-deep-dives.md)

### 19. Rust 매크로
- [선언적 매크로 (`macro_rules!`)](ch19-macros.md#declarative-macros-with-macro_rules)
- [일반적인 표준 라이브러리 매크로](ch19-macros.md#common-standard-library-macros)
- [Derive 매크로](ch19-macros.md#derive-macros)
- [속성(Attribute) 매크로](ch19-macros.md#attribute-macros)
- [절차적(Procedural) 매크로 개념 개요](ch19-macros.md#procedural-macros-conceptual-overview)
- [사용 시기: 매크로 vs 함수 vs 제네릭](ch19-macros.md#when-to-use-what-macros-vs-functions-vs-generics)
- [연습 문제](ch19-macros.md#exercises)
