# C# 개발자를 위한 Rust: 전체 교육 가이드

C# 경험이 있는 개발자를 위한 Rust 학습 종합 가이드입니다. 이 가이드는 기본 문법부터 고급 패턴까지 아우르며, 두 언어 사이의 개념적 변화와 실무적인 차이점에 집중합니다.

## 강의 개요
- **Rust를 선택해야 하는 이유** — C# 개발자에게 Rust가 중요한 이유: 성능, 안전성, 정확성
- **시작하기** — 설치, 도구 활용, 그리고 첫 번째 프로그램 작성
- **기본 구성 요소** — 타입(Type), 변수, 제어 흐름
- **데이터 구조** — 배열, 튜플, 구조체(Struct), 컬렉션
- **패턴 매칭과 열거형(Enum)** — 대수적 데이터 타입(Algebraic Data Types)과 철저한(Exhaustive) 매칭
- **소유권(Ownership)과 빌림(Borrowing)** — Rust의 메모리 관리 모델
- **모듈과 크레이트(Crate)** — 코드 조직화와 의존성 관리
- **에러 핸들링** — Result 기반의 에러 전파
- **트레이트(Trait)와 제네릭(Generic)** — Rust의 타입 시스템
- **클로저(Closure)와 반복자(Iterator)** — 함수형 프로그래밍 패턴
- **동시성(Concurrency)** — 타입 시스템이 보장하는 안전한 동시성, async/await 심층 분석
- **Unsafe Rust와 FFI** — 안전한 Rust의 범위를 넘어서야 할 때와 그 방법
- **마이그레이션 패턴** — 실제 C#에서 Rust로의 전환 패턴 및 점진적 도입 전략
- **베스트 프랙티스** — C# 개발자를 위한 관용적인(Idiomatic) Rust 작성법

---

# 자습 가이드

본 자료는 강사 주도 학습과 자습 모두에 적합하도록 설계되었습니다. 혼자서 학습하신다면, 다음 지침을 통해 학습 효과를 극대화할 수 있습니다.

**권장 학습 진도:**

| 장 | 주제 | 권장 시간 | 체크포인트 |
|----------|-------|---------------|------------|
| 1~4장 | 설정, 타입, 제어 흐름 | 1일 | Rust로 CLI 온도 변환기를 작성할 수 있음 |
| 5~6장 | 데이터 구조, 열거형, 패턴 매칭 | 1~2일 | 데이터를 포함하는 열거형을 정의하고 `match`로 철저하게 처리할 수 있음 |
| 7장 | 소유권과 빌림 | 1~2일 | 왜 `let s2 = s1`이 `s1`을 무효화하는지 설명할 수 있음 |
| 8~9장 | 모듈, 에러 핸들링 | 1일 | `?` 연산자로 에러를 전파하는 다중 파일 프로젝트를 생성할 수 있음 |
| 10~12장 | 트레이트, 제네릭, 클로저, 반복자 | 1~2일 | LINQ 체인을 Rust 반복자로 변환할 수 있음 |
| 13장 | 동시성과 비동기(Async) | 1일 | `Arc<Mutex<T>>`를 사용하여 스레드 안전한 카운터를 작성할 수 있음 |
| 14장 | Unsafe Rust, FFI, 테스트 | 1일 | P/Invoke를 통해 C#에서 Rust 함수를 호출할 수 있음 |
| 15~16장 | 마이그레이션, 베스트 프랙티스, 도구 | 자율 학습 | 참고 자료 — 실제 코드를 작성할 때 수시로 확인 |
| 17장 | 캡스톤 프로젝트 | 1~2일 | 날씨 데이터를 가져오는 작동하는 CLI 도구를 완성함 |

**연습 문제 활용법:**
- 각 장에는 접을 수 있는 `<details>` 블록 안에 실습 문제와 해답이 포함되어 있습니다.
- **해답을 펼치기 전에 반드시 직접 문제를 풀어보세요.** 빌림 검사기(Borrow Checker)와 씨름하는 과정 자체가 학습의 일부입니다. 컴파일러의 에러 메시지가 여러분의 가장 좋은 스승입니다.
- 15분 이상 진전이 없다면 해답을 공부한 뒤, 다시 해답을 닫고 처음부터 직접 작성해 보세요.
- [Rust Playground](https://play.rust-lang.org/)를 이용하면 로컬 설치 없이도 코드를 실행해 볼 수 있습니다.

**난이도 표시:**
- 🟢 **초급** — C# 개념에서 직접적으로 변환 가능한 수준
- 🟡 **중급** — 소유권이나 트레이트에 대한 이해가 필요한 수준
- 🔴 **고급** — 수명(Lifetime), 비동기 내부 구조, 또는 Unsafe 코드 관련

**난관에 부딪혔을 때:**
- 컴파일러 에러 메시지를 주의 깊게 읽으세요. Rust의 에러 메시지는 매우 친절하고 구체적입니다.
- 관련 섹션을 다시 읽어보세요. 소유권(7장) 같은 개념은 두 번째 읽을 때 비로소 이해되는 경우가 많습니다.
- [Rust 표준 라이브러리 문서](https://doc.rust-lang.org/std/)는 훌륭한 자원입니다. 어떤 타입이나 메서드든 검색해 보세요.
- 더 깊이 있는 비동기 패턴은 자매 가이드인 [Async Rust Training](../async-book/)을 참고하세요.

---

# 차례

## 제1부 — 기초 (Foundations)

### 1. 도입 및 동기 🟢
- [C# 개발자에게 Rust가 필요한 이유](ch01-introduction-and-motivation.md#c-개발자에게-rust가-필요한-이유)
- [Rust가 해결하는 C#의 일반적인 고충들](ch01-introduction-and-motivation.md#rust가-해결하는-c-의-일반적인-고충들)
- [C# 대신 Rust를 선택해야 할 때](ch01-introduction-and-motivation.md#c-대신-rust를-선택해야-할-때)
- [언어 철학 비교](ch01-introduction-and-motivation.md#언어-철학-비교)
- [빠른 참조: Rust vs C#](ch01-introduction-and-motivation.md#빠른-참조-rust-vs-c)

### 2. 시작하기 🟢
- [설치 및 설정](ch02-getting-started.md#설치-및-설정)
- [첫 번째 Rust 프로그램](ch02-getting-started.md#첫-번째-rust-프로그램)
- [Cargo vs NuGet/MSBuild](ch02-getting-started.md#cargo-vs-nugetmsbuild)
- [입력 읽기 및 CLI 인자 처리](ch02-getting-started.md#입력-읽기-및-cli-인자-처리)
- [필수 Rust 키워드 *(선택적 참고용)*](ch02-1-essential-keywords-reference.md#c-개발자를-위한-필수-rust-키워드)

### 3. 내장 타입과 변수 🟢
- [변수와 가변성(Mutability)](ch03-built-in-types-and-variables.md#변수와-가변성)
- [기본 타입 비교](ch03-built-in-types-and-variables.md#기본-타입)
- [문자열 타입: String vs &str](ch03-built-in-types-and-variables.md#문자열-타입-string-vs-str)
- [출력 및 문자열 포맷팅](ch03-built-in-types-and-variables.md#출력-및-문자열-포맷팅)
- [타입 캐스팅 및 변환](ch03-built-in-types-and-variables.md#타입-캐스팅-및-변환)
- [진정한 불변성 vs Record의 환상](ch03-1-true-immutability-vs-record-illusions.md#진정한-불변성-vs-record의-환상)

### 4. 제어 흐름 🟢
- [함수(Function) vs 메서드(Method)](ch04-control-flow.md#함수-vs-메서드)
- [표현식(Expression) vs 문(Statement) (중요!)](ch04-control-flow.md#표현식-vs-문-중요)
- [조건문](ch04-control-flow.md#조건문)
- [루프와 반복](ch04-control-flow.md#루프)

### 5. 데이터 구조와 컬렉션 🟢
- [튜플과 구조 분해(Destructuring)](ch05-data-structures-and-collections.md#튜플과-구조-분해)
- [배열과 슬라이스(Slice)](ch05-data-structures-and-collections.md#배열과-슬라이스)
- [구조체(Struct) vs 클래스(Class)](ch05-data-structures-and-collections.md#구조체-vs-클래스)
- [생성자 패턴](ch05-1-constructor-patterns.md#생성자-패턴)
- [`Vec<T>` vs `List<T>`](ch05-2-collections-vec-hashmap-and-iterators.md#vect-vs-listt)
- [HashMap vs Dictionary](ch05-2-collections-vec-hashmap-and-iterators.md#hashmap-vs-dictionary)

### 6. 열거형과 패턴 매칭 🟡
- [대수적 데이터 타입 vs C# Union](ch06-enums-and-pattern-matching.md#대수적-데이터-타입-vs-c-union)
- [철저한 패턴 매칭](ch06-1-exhaustive-matching-and-null-safety.md#철저한-패턴-매칭-컴파일러-보장-vs-런타임-에러)
- [Null 안전성을 위한 `Option<T>`](ch06-1-exhaustive-matching-and-null-safety.md#null-안전성-nullablet-vs-optiont)
- [가드(Guard) 및 고급 패턴](ch06-enums-and-pattern-matching.md#가드-및-고급-패턴)

### 7. 소유권과 빌림 🟡
- [소유권의 이해](ch07-ownership-and-borrowing.md#소유권의-이해)
- [이동 의미론(Move Semantics) vs 참조 의미론(Reference Semantics)](ch07-ownership-and-borrowing.md#이동-의미론)
- [빌림과 참조](ch07-ownership-and-borrowing.md#빌림-기초)
- [메모리 안전성 심층 분석](ch07-1-memory-safety-deep-dive.md#참조-vs-포인터)
- [수명(Lifetime) 심층 분석](ch07-2-lifetimes-deep-dive.md#수명-참조의-유효-기간을-컴파일러에게-알리기) 🔴
- [스마트 포인터, Drop, Deref](ch07-3-smart-pointers-beyond-single-ownership.md#스마트-포인터-단일-소유권으로-부족할-때) 🔴

### 8. 크레이트와 모듈 🟢
- [Rust 모듈 vs C# 네임스페이스](ch08-crates-and-modules.md#rust-모듈-vs-c-네임스페이스)
- [크레이트 vs .NET 어셈블리](ch08-crates-and-modules.md#크레이트-vs-net-어셈블리)
- [패키지 관리: Cargo vs NuGet](ch08-1-package-management-cargo-vs-nuget.md#패키지-관리-cargo-vs-nuget)

### 9. 에러 핸들링 🟡
- [예외(Exception) vs `Result<T, E>`](ch09-error-handling.md#예외-vs-resultt-e)
- [? 연산자](ch09-error-handling.md#--연산자-간결하게-에러-전파하기)
- [사용자 정의 에러 타입](ch06-1-exhaustive-matching-and-null-safety.md#사용자-정의-에러-타입)
- [크레이트 수준의 에러 타입과 Result 별칭](ch09-1-crate-level-error-types-and-result-alias.md#크레이트-수준의-에러-타입과-result-별칭)
- [에러 복구 패턴](ch09-1-crate-level-error-types-and-result-alias.md#에러-복구-패턴)

### 10. 트레이트와 제네릭 🟡
- [트레이트 vs 인터페이스](ch10-traits-and-generics.md#트레이트---rust의-인터페이스)
- [상속 vs 구성(Composition)](ch10-2-inheritance-vs-composition.md#상속-vs-구성)
- [제네릭 제약: where vs 트레이트 바운드](ch10-1-generic-constraints.md#제네릭-제약-where-vs-트레이트-바운드)
- [주요 표준 라이브러리 트레이트](ch10-traits-and-generics.md#주요-표준-라이브러리-트레이트)

### 11. From 및 Into 트레이트 🟡
- [Rust의 타입 변환](ch11-from-and-into-traits.md#rust의-타입-변환)
- [커스텀 타입에 From 구현하기](ch11-from-and-into-traits.md#rust-from-및-into)

### 12. 클로저와 반복자 🟡
- [Rust 클로저](ch12-closures-and-iterators.md#rust-클로저)
- [LINQ vs Rust 반복자](ch12-closures-and-iterators.md#linq-vs-rust-반복자)
- [매크로 입문](ch12-1-macros-primer.md#매크로-코드를-작성하는-코드)

---

## 제2부 — 동시성 및 시스템

### 13. 동시성 🔴
- [스레드 안전성: 관습 vs 타입 시스템 보장](ch13-concurrency.md#스레드-안전성-관습-vs-타입-시스템-보장)
- [async/await: C# Task vs Rust Future](ch13-1-asyncawait-deep-dive.md#비동기-프로그래밍-c-task-vs-rust-future)
- [취소 패턴](ch13-1-asyncawait-deep-dive.md#취소-cancellationtoken-vs-drop--select)
- [Pin 및 tokio::spawn](ch13-1-asyncawait-deep-dive.md#pin-rust-비동기에만-존재하는-개념)

### 14. Unsafe Rust, FFI, 테스트 🟡
- [Unsafe를 사용해야 하는 시점과 이유](ch14-unsafe-rust-and-ffi.md#unsafe가-필요할-때)
- [FFI를 통한 C#과의 상호 운용](ch14-unsafe-rust-and-ffi.md#ffi를-통한-c-과의-상호-운용)
- [Rust vs C# 테스트 방식 비교](ch14-1-testing.md#rust-vs-c-테스트-비교)
- [속성 기반 테스트와 모킹(Mocking)](ch14-1-testing.md#속성-기반-테스트-대규모-정확성-검증)

---

## 제3부 — 마이그레이션 및 베스트 프랙티스

### 15. 마이그레이션 패턴 및 사례 연구 🟡
- [Rust에서 구현하는 일반적인 C# 패턴](ch15-migration-patterns-and-case-studies.md#rust에서-구현하는-일반적인-c-패턴)
- [C# 개발자를 위한 필수 크레이트](ch15-1-essential-crates-for-c-developers.md#c-개발자를-위한-필수-크레이트)
- [점진적 도입 전략](ch15-2-incremental-adoption-strategy.md#점진적-도입-전략)

### 16. 베스트 프랙티스 및 참고 자료 🟡
- [C# 개발자를 위한 관용적인 Rust](ch16-best-practices.md#c-개발자를-위한-베스트-프랙티스)
- [성능 비교: 매니지드 vs 네이티브](ch16-1-performance-comparison-and-migration.md#성능-비교-매니지드-vs-네이티브)
- [흔한 실수와 해결책](ch16-2-learning-path-and-resources.md#c-개발자가-자주-겪는-실수)
- [학습 경로 및 참고 리소스](ch16-2-learning-path-and-resources.md#학습-경로-및-다음-단계)
- [Rust 도구 생태계](ch16-3-rust-tooling-ecosystem.md#c-개발자를-위한-필수-rust-도구)

---

## 캡스톤 프로젝트

### 17. 캡스톤 프로젝트 🟡
- [CLI 날씨 도구 빌드하기](ch17-capstone-project.md#캡스톤-프로젝트-cli-날씨-도구-빌드하기) — 구조체, 트레이트, 에러 핸들링, 비동기, 모듈, serde, 테스트를 하나의 작동하는 애플리케이션으로 통합합니다.
