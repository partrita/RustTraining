# 요약 (Summary)

[들어가며](ch00-introduction.md)

---

# 제1부 — 기초 (Foundations)

- [1. 서론 및 동기](ch01-introduction-and-motivation.md)
- [2. 시작하기](ch02-getting-started.md)
    - [필수 키워드 참조 (선택 사항)](ch02-1-essential-keywords-reference.md)
- [3. 내장 타입 및 변수](ch03-built-in-types-and-variables.md)
    - [진정한 불변성 vs 레코드의 환상](ch03-1-true-immutability-vs-record-illusions.md)
- [4. 제어 흐름](ch04-control-flow.md)
- [5. 데이터 구조 및 컬렉션](ch05-data-structures-and-collections.md)
    - [생성자 패턴](ch05-1-constructor-patterns.md)
    - [컬렉션 — Vec, HashMap, 그리고 반복자(Iterator)](ch05-2-collections-vec-hashmap-and-iterators.md)
- [6. 열거형(Enum) 및 패턴 매칭](ch06-enums-and-pattern-matching.md)
    - [철저한 매칭과 널 안전성](ch06-1-exhaustive-matching-and-null-safety.md)
- [7. 소유권과 빌림(Borrowing)](ch07-ownership-and-borrowing.md)
    - [메모리 안전성 심층 탐구](ch07-1-memory-safety-deep-dive.md)
    - [수명(Lifetimes) 심층 탐구](ch07-2-lifetimes-deep-dive.md)
    - [스마트 포인터 — 단일 소유권을 넘어](ch07-3-smart-pointers-beyond-single-ownership.md)
- [8. 크레이트(Crate)와 모듈](ch08-crates-and-modules.md)
    - [패키지 관리 — Cargo vs NuGet](ch08-1-package-management-cargo-vs-nuget.md)
- [9. 에러 처리](ch09-error-handling.md)
    - [크레이트 수준 에러 타입과 Result 별칭](ch09-1-crate-level-error-types-and-result-alias.md)
- [10. 트레이트(Trait)와 제네릭(Generic)](ch10-traits-and-generics.md)
    - [제네릭 제약 조건](ch10-1-generic-constraints.md)
    - [상속 vs 구성(Composition)](ch10-2-inheritance-vs-composition.md)
- [11. From 및 Into 트레이트](ch11-from-and-into-traits.md)
- [12. 클로저(Closure)와 반복자(Iterator)](ch12-closures-and-iterators.md)
    - [매크로 입문](ch12-1-macros-primer.md)

---

# 제2부 — 동시성 및 시스템 (Concurrency & Systems)

- [13. 동시성](ch13-concurrency.md)
    - [Async/Await 심층 탐구](ch13-1-asyncawait-deep-dive.md)
- [14. Unsafe Rust 및 FFI](ch14-unsafe-rust-and-ffi.md)
    - [테스트](ch14-1-testing.md)

---

# 제3부 — 마이그레이션 및 권장 사례 (Migration & Best Practices)

- [15. 마이그레이션 패턴 및 사례 연구](ch15-migration-patterns-and-case-studies.md)
    - [C# 개발자를 위한 필수 크레이트](ch15-1-essential-crates-for-c-developers.md)
    - [단계적 도입 전략](ch15-2-incremental-adoption-strategy.md)
- [16. 권장 사례](ch16-best-practices.md)
    - [성능 비교 및 마이그레이션](ch16-1-performance-comparison-and-migration.md)
    - [학습 경로 및 리소스](ch16-2-learning-path-and-resources.md)
    - [Rust 도구 생태계](ch16-3-rust-tooling-ecosystem.md)

---

# 캡스톤 프로젝트 (Capstone)

- [17. 캡스톤 프로젝트: CLI 날씨 도구 만들기](ch17-capstone-project.md)
