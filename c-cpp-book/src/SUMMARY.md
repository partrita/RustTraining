# 요약

[소개](ch00-introduction.md)

---

# 제 I 부 — 기초

- [1. 서론 및 동기](ch01-introduction-and-motivation.md)
    - [C/C++ 개발자에게 Rust가 필요한 이유](ch01-1-why-c-cpp-developers-need-rust.md)
- [2. 시작하기](ch02-getting-started.md)
- [3. 내장 타입](ch03-built-in-types.md)
- [4. 제어 흐름](ch04-control-flow.md)
- [5. 데이터 구조](ch05-data-structures.md)
- [6. 열거형(Enums) 및 패턴 매칭](ch06-enums-and-pattern-matching.md)
- [7. 소유권(Ownership) 및 빌림(Borrowing)](ch07-ownership-and-borrowing.md)
    - [수명(Lifetimes) 및 빌림 심층 분석](ch07-1-lifetimes-and-borrowing-deep-dive.md)
    - [스마트 포인터 및 내부 가변성(Interior Mutability)](ch07-2-smart-pointers-and-interior-mutability.md)
- [8. 크레이트(Crates) 및 모듈](ch08-crates-and-modules.md)
    - [테스트 패턴](ch08-1-testing-patterns.md)
- [9. 에러 처리](ch09-error-handling.md)
    - [에러 처리 모범 사례](ch09-1-error-handling-best-practices.md)
- [10. 트레이트(Traits)](ch10-traits.md)
    - [제네릭(Generics)](ch10-1-generics.md)
- [11. From 및 Into 트레이트](ch11-from-and-into-traits.md)
- [12. 클로저(Closures)](ch12-closures.md)
    - [반복자(Iterator) 강력한 도구들](ch12-1-iterator-power-tools.md)
- [13. 동시성(Concurrency)](ch13-concurrency.md)
- [14. Unsafe Rust 및 FFI](ch14-unsafe-rust-and-ffi.md)

---

# 제 II 부 — 심층 분석

- [15. no_std — 표준 라이브러리 없는 Rust](ch15-no_std-rust-without-the-standard-library.md)
    - [임베디드 심층 분석](ch15-1-embedded-deep-dive.md)
- [16. 사례 연구: 실제 C++에서 Rust로의 전환](ch16-case-studies.md)
    - [사례 연구 — 수명 빌림](ch16-1-case-study-lifetime-borrowing.md)

---

# 제 III 부 — 모범 사례 및 참조

- [17. 모범 사례](ch17-best-practices.md)
    - [과도한 clone() 방지하기](ch17-1-avoiding-excessive-clone.md)
    - [검사되지 않은 인덱싱 방지하기](ch17-2-avoiding-unchecked-indexing.md)
    - [할당 피라미드 축소하기](ch17-3-collapsing-assignment-pyramids.md)
    - [로깅 및 트레이싱 에코시스템](ch17-4-logging-and-tracing-ecosystem.md)
- [18. C++ → Rust 의미론적 심층 분석](ch18-cpp-rust-semantic-deep-dives.md)
- [19. Rust 매크로: 전처리기에서 메타프로그래밍까지](ch19-macros.md)
