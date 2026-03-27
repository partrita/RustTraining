# C# → Rust 교육: 병합된 챕터 계획 (Merged Chapter Plan)

## 소스 문서 (Source Documents)

| 문서 | 파일명 | 라인 수 |
|-----|------|-------|
| **부트스트랩 (Bootstrap, B)** | `RustBootstrapForCSharp.md` | 5,363 |
| **고급 과정 (Advanced, A)** | `RustTrainingForCSharp.md` | 3,021 |
| **원시 데이터 합계** | | **8,384** |
| **병합 후 예상 수치** | (중복 제거 후) | **~5,800** |

## Mermaid 다이어그램 인벤토리 (총 13개 — 모두 고급 과정 문서에 포함)

| # | 고급 과정 라인 | 주제 | 대상 챕터 |
|---|----------|---------|----------------|
| M1 | L84 | 개발 모델 비교 | ch01 |
| M2 | L173 | 메모리 관리: GC vs RAII | ch01 |
| M3 | L282 | C# 널 처리의 진화 | ch06.1 |
| M4 | L410 | C# 판별 공용체 (우회 방법) | ch06 |
| M5 | L536 | C# 패턴 매칭의 한계 | ch06.1 |
| M6 | L667 | C# 레코드 — 얕은 불변성 | ch03.1 |
| M7 | L829 | 런타임 안전성 vs 컴파일 타임 안전성 | ch07.1 |
| M8 | L998 | C# 상속 계층 구조 | ch10.2 |
| M9 | L1153 | C# 예외 모델 | ch09 |
| M10 | L1290 | C# LINQ의 특징 | ch12 |
| M11 | L1463 | C# 제네릭 제약 조건 | ch10.1 |
| M12 | L2156 | C# 스레드 안전성 도전 과제 | ch13 |
| M13 | L2850 | 마이그레이션 전략 결정 트리 | ch16 |

---

## 챕터 구조 (Chapter Structure)

### 제0장: 서론 (Introduction)
<!-- ch00: Introduction -->

**파일명:** `ch00-introduction.md`
**예상 라인 수:** ~30
**내용:** 책 개요, 가이드 활용 방법, 전제 조건 (C# 경험 필수).
**출처:** 신규 콘텐츠 (C/C++ 도서의 ch00 패턴 모델링).

---

### 제1장: 서론 및 동기 (Introduction and Motivation)
<!-- ch01: Introduction and Motivation -->

**파일명:** `ch01-introduction-and-motivation.md`
**예상 라인 수:** ~380
**Mermaid 다이어그램:** M1, M2

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch01.1: Quick Reference --> | B L93–110 | 18 | 빠른 참조표 — **C# 문서 전용, 그대로 유지** |
| <!-- ch01.2: Language Philosophy --> | A L70–125 | 56 | C# vs Rust 철학 비교. **M1** 포함 |
| <!-- ch01.3: GC vs RAII --> | A L126–214 | 89 | GC vs 소유권 개요. **M2** 포함 |
| <!-- ch01.4: The Case for Rust --> | B L111–221 | 111 | 성능, 메모리 안전성 논거 |
| <!-- ch01.5: C# Pain Points --> | B L222–348 | 80 | 약 80라인으로 축소 (널, 예외, GC 문제점 — 이미 ch01.2~01.3에서 다룬 A의 철학 부분과 겹치는 내용 제거) |
| <!-- ch01.6: When to Choose --> | B L349–400 | 52 | Rust vs C# 선택 시점, 실제 영향력 |

**중복 해결:** 부트스트랩의 "문제점(Pain Points)" §1 (널)과 §3 (GC) 부분은 고급 과정의 '철학' 및 'GC vs RAII'와 부분적으로 겹칩니다. 다이어그램이 포함된 고급 과정 버전을 유지하고, 부트스트랩의 문제점 섹션은 중복을 피하기 위해 축소합니다. 문제점 §2 (숨겨진 예외)는 고유한 내용이므로 전체 유지합니다.

---

### 제2장: 시작하기 (Getting Started)
<!-- ch02: Getting Started -->

**파일명:** `ch02-getting-started.md`
**예상 라인 수:** ~170

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch02.1: Installation --> | B L401–434 | 34 | rustup, 도구 비교표 |
| <!-- ch02.2: First Program --> | B L435–486 | 52 | Hello World 비교 (C# vs Rust) |
| <!-- ch02.3: Cargo vs NuGet --> | B L487–564 | 78 | 프로젝트 설정, 명령어, 워크스페이스 vs 솔루션 |

#### 서브 챕터: ch02.1 — C# 개발자를 위한 필수 Rust 키워드
<!-- ch02.1: Keywords Reference -->

**파일명:** `ch02-1-keywords-reference.md`
**예상 라인 수:** ~400
**출처:** B L842–1244 (403라인)
**참고 사항:** 이 포괄적인 키워드 매핑 테이블은 **C# 문서 전용**입니다. 가시성, 메모리, 제어 흐름, 타입 정의, 함수, 변수, 패턴 매칭, 안전성 키워드를 모두 C# 대응 개념과 매핑하여 다룹니다. 내용을 그대로 유지하며, 약 400라인에 달하는 분량은 독립된 서브 챕터로서 충분한 가치가 있습니다.

---

### 제3장: 내장 타입 (Built-in Types)
<!-- ch03: Built-in Types -->

**파일명:** `ch03-built-in-types.md`
**예상 라인 수:** ~280

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch03.1: Variables and Mutability --> | B L565–641 | 77 | let vs var, mut, const, 섀도잉(shadowing) |
| <!-- ch03.2: Primitive Types --> | B L642–707 | 66 | 타입 비교표, 크기별 타입, 추론 |
| <!-- ch03.3: String Types --> | B L708–782 | 75 | String vs &str, 실용적 예제 |
| <!-- ch03.4: Comments and Docs --> | B L783–841 | 59 | 주석, 문서 주석, rustdoc |

#### 서브 챕터: ch03.1 — 진정한 불변성 심층 탐구
<!-- ch03.1: True Immutability -->

**파일명:** `ch03-1-true-immutability.md`
**예상 라인 수:** ~136
**출처:** A L577–712 (136라인)
**Mermaid 다이어그램:** M6
**참고 사항:** C# 레코드의 "불변성 흉내" vs Rust의 진정한 불변성 비교. **M6** (레코드 — 얕은 불변성 다이어그램) 포함. 이 내용은 **C# 문서 전용**입니다. C# 개발자들에게 왜 `record`가 진정한 불변이 아닌지 이해시키는 데 필수적입니다.

---

### 제4장: 제어 흐름 (Control Flow)
<!-- ch04: Control Flow -->

**파일명:** `ch04-control-flow.md`
**예상 라인 수:** ~280

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch04.1: Functions vs Methods --> | B L1638–1745 | 108 | 선언, 표현식 vs 문(statement), 매개변수/반환값 |
| <!-- ch04.2: Conditionals --> | B L1748–1792 | 45 | if/else, if-let, 삼항 연산자 대응 개념 |
| <!-- ch04.3: Loops --> | B L1793–1886 | 93 | loop, while, for, 루프 제어 (break/continue 라벨) |
| <!-- ch04.4: Pattern Matching Preview --> | B L1887–1978 | 35 | 간단한 소개만 포함 (92라인 중 35라인으로 축소). 상세 내용은 제6장에서 다룸. "포괄적인 내용은 제6장을 참조하세요"라는 안내 문구 추가. |

**참고 사항:** 패턴 매칭의 전체 내용(B L1887~1978, 92라인)은 제6장과 심하게 겹칩니다. 여기서는 기본적인 `match` 구문 미리보기(~35라인)만 추출하고 제6장으로 안내합니다.

---

### 제5장: 데이터 구조 (Data Structures)
<!-- ch05: Data Structures -->

**파일명:** `ch05-data-structures.md`
**예상 라인 수:** ~380

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch05.1: Arrays and Slices --> | B L2445–2548 | 104 | C# 배열 vs Rust 배열, 슬라이스, 문자열 슬라이스 |
| <!-- ch05.2: Structs vs Classes --> | B L2673–2807 | 135 | 구조체 정의, 인스턴스 생성, 초기화 패턴 |
| <!-- ch05.3: Methods and Associated Functions --> | B L2808–2941 | 134 | impl 블록, &self/&mut self/self, 메서드 리시버 타입 |

#### 서브 챕터: ch05.1 — 생성자 패턴
<!-- ch05.1: Constructor Patterns -->

**파일명:** `ch05-1-constructor-patterns.md`
**예상 라인 수:** ~210
**출처:** B L3084–3291 (208라인)
**참고 사항:** C# 생성자 vs Rust의 `new()` 관례, `Default` 트레이트, 빌더(builder) 패턴 구현. 독립적인 서브 챕터로 다룰 만큼 내용이 방대하고 중요합니다.

#### 서브 챕터: ch05.2 — 컬렉션: Vec, HashMap, 그리고 반복(Iteration)
<!-- ch05.2: Collections -->

**파일명:** `ch05-2-collections.md`
**예상 라인 수:** ~390

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch05.2.1: Vec vs List --> | B L2163–2307 | 145 | 생성, 초기화, 일반적인 작업, 안전한 접근 |
| <!-- ch05.2.2: HashMap vs Dictionary --> | B L2308–2444 | 137 | 작업, Entry API, 키/값의 소유권 |
| <!-- ch05.2.3: Working with Collections --> | B L2549–2672 | 110 | 반복 패턴, IntoIterator/Iter, 결과 수집 (축소 — LINQ 스타일의 반복자 내용은 제12장으로 이동) |

**중복 참고:** "컬렉션 작업" 섹션(B L2549~2672) 중 일부 반복자 체인 내용은 제12장(클로저/LINQ)과 겹칩니다. 여기서는 기본적인 반복 패턴만 남기고, 고급 반복자 체인과 LINQ 비교는 제12장에서 다룹니다.

---

### 제6장: 열거형 및 패턴 매칭 (Enums and Pattern Matching)
<!-- ch06: Enums and Pattern Matching -->

**파일명:** `ch06-enums-and-pattern-matching.md`
**예상 라인 수:** ~320
**Mermaid 다이어그램:** M4

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch06.1: C# Enum Limitations --> | B L3296–3342 | 47 | C# 열거형의 한계 |
| <!-- ch06.2: Rust Enum Power --> | B L3343–3378 | 36 | 데이터를 포함하는 열거형 변형(variant) |
| <!-- ch06.3: Algebraic Data Types --> | A L319–451 | 100 | ADT vs C# 공용체. **M4** 포함. 133라인에서 ~100라인으로 축소 (위의 기본 열거형 내용과 겹치는 부분 제거) |
| <!-- ch06.4: Pattern Matching --> | B L3379–3461 | 83 | Match 표현식, 구조 해제(destructuring) |
| <!-- ch06.5: Guards and Advanced --> | B L3462–3502 | 41 | Match 가드, 중첩된 패턴 |

#### 서브 챕터: ch06.1 — 철저한 매칭과 널 안전성
<!-- ch06.1: Exhaustive Matching and Null Safety -->

**파일명:** `ch06-1-exhaustive-matching-and-null-safety.md`
**예상 라인 수:** ~300
**Mermaid 다이어그램:** M3, M5

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch06.1.1: Exhaustive Matching --> | A L452–576 | 125 | 컴파일러 보장 vs 런타임 에러. **M5** 포함 |
| <!-- ch06.1.2: Null Safety: Option --> | A L215–318 | 80 | Nullable<T> vs Option<T>. **M3** 포함. ~80라인으로 축소 (B의 Option 섹션과 겹치는 내용 제거) |
| <!-- ch06.1.3: Option and Result --> | B L3503–3615 | 113 | Option<T>와 Result<T,E>의 실전 활용 |

**중복 해결:** 두 문서 모두 Option<T>를 다룹니다. Mermaid 다이어그램이 있고 "널 처리의 진화"라는 심도 있는 서술을 담은 고급 과정 버전(A L215~318)을 개념적 입문으로 사용합니다. 실용적인 코드 예제가 풍부한 부트스트랩 버전(B L3503~3615)은 실습 부분으로 유지합니다. 중복되는 예제는 제거합니다.

---

### 제7장: 소유권과 빌림 (Ownership and Borrowing)
<!-- ch07: Ownership and Borrowing -->

**파일명:** `ch07-ownership-and-borrowing.md`
**예상 라인 수:** ~330

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch07.1: C# Memory Model --> | B L1249–1267 | 19 | C# 참조 타입, GC 복습 |
| <!-- ch07.2: Ownership Rules --> | B L1268–1316 | 49 | 세 가지 규칙, C# 개발자를 위한 '이동(Move)', 복사 vs 이동 |
| <!-- ch07.3: Practical Examples --> | B L1317–1348 | 32 | 값 스와핑(swapping) 예제 |
| <!-- ch07.4: Borrowing --> | B L1349–1472 | 124 | 공유/가변 참조, 빌림 규칙, 참조 안전성 비교 |
| <!-- ch07.5: Move Semantics --> | B L1540–1637 | 98 | 값/참조 타입 vs 이동 의미론, 이동 피하기 |

#### 서브 챕터: ch07.1 — 참조, 포인터, 그리고 메모리 안전성
<!-- ch07.1: Memory Safety Deep Dive -->

**파일명:** `ch07-1-references-pointers-and-memory-safety.md`
**예상 라인 수:** ~220
**Mermaid 다이어그램:** M7

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch07.1.1: References vs Pointers --> | B L1473–1539 | 67 | C# unsafe 포인터 vs Rust의 안전한 참조, 수명(lifetime) 기초 |
| <!-- ch07.1.2: Memory Safety --> | A L713–870 | 158 | 런타임 체크 vs 컴파일 타임 증명. **M7** 포함. 왜 Rust의 소유권이 버그 카테고리 전체를 예방하는지 가장 깊이 있게 다룹니다. **C# 독자에게 매우 유용한 심층 분석**입니다. |

---

### 제8장: 크레이트와 모듈 (Crates and Modules)
<!-- ch08: Crates and Modules -->

**파일명:** `ch08-crates-and-modules.md`
**예상 라인 수:** ~340

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch08.1: Modules vs Namespaces --> | B L3674–3882 | 209 | C# 네임스페이스 → Rust 모듈 매핑, 계층 구조, 가시성, 파일 구성 |
| <!-- ch08.2: Crates vs Assemblies --> | B L3883–4009 | 127 | 어셈블리 모델 vs 크레이트 모델, 크레이트 타입, 워크스페이스 vs 솔루션 |

#### 서브 챕터: ch08.1 — 패키지 관리 심층 탐구
<!-- ch08.1: Package Management -->

**파일명:** `ch08-1-package-management.md`
**예상 라인 수:** ~235

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch08.1.1: Dependencies --> | B L4010–4055 | 46 | Cargo.toml vs .csproj, 의존성 타입 |
| <!-- ch08.1.2: Version Management --> | B L4056–4089 | 34 | 시맨틱 버저닝(SemVer), Cargo.lock |
| <!-- ch08.1.3: Package Sources --> | B L4090–4132 | 43 | crates.io vs NuGet, 대체 레지스트리 |
| <!-- ch08.1.4: Features --> | B L4133–4182 | 50 | 피처 플래그(Feature flags) vs #if DEBUG 조건부 컴파일 |
| <!-- ch08.1.5: External Crates --> | B L4183–4244 | 62 | 주요 크레이트 목록, HTTP 클라이언트 마이그레이션 예제 |

---

### 제9장: 에러 처리 (Error Handling)
<!-- ch09: Error Handling -->

**파일명:** `ch09-error-handling.md`
**예상 라인 수:** ~350
**Mermaid 다이어그램:** M9

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch09.1: C# Exception Model --> | A L1046–1089 | 44 | 예외 기반 처리의 문제점. **M9** 컨텍스트의 일부 |
| <!-- ch09.2: Exceptions vs Result --> | A L1090–1194 | 105 | Result 기반 에러 처리 (고급 과정 버전 — 더 깊이 있고 **M9** 다이어그램 포함) |
| <!-- ch09.3: The ? Operator --> | B L2057–2084 | 28 | ? 연산자를 "C#의 await와 유사한" 방식으로 설명 |
| <!-- ch09.4: Custom Error Types --> | B L3616–3673 | 58 | `thiserror`를 사용한 커스텀 에러 (열거형 챕터에서 이동) |
| <!-- ch09.5: Error Handling Deep Dive --> | B L4558–4715 | 120 | 포괄적인 에러 처리 패턴 (158라인에서 축소 — 위에서 다룬 A의 Result 내용과 겹치는 부분 제거) |

**중복 해결:** 세 가지 소스가 에러 처리를 다룹니다:
1. **B L1979–2162 "에러 처리 기초"** (184라인) — 입문용
2. **B L4558–4715 "에러 처리 심층 탐구"** (158라인) — 고급 패턴
3. **A L1046–1194 "예외 vs Result"** (149라인) — Mermaid를 활용한 개념 비교

**전략:** 개념적 틀은 A의 버전을 사용합니다 (M9 다이어그램이 있고 C#과 더 깊이 비교함). 실용적인 패턴은 B의 심층 탐구를 사용합니다. B의 기초 섹션은 (A + B 심층 탐구 조합으로 충분하므로) 제거합니다. 단, B 기초 섹션에서 ? 연산자를 독특하게 잘 설명한 부분은 유지합니다.

#### 서브 챕터: ch09.1 — 에러 처리 권장 사례
<!-- ch09.1: Error Handling Best Practices -->

**파일명:** `ch09-1-error-handling-best-practices.md`
**예상 라인 수:** ~80
**출처:** B L4612~4715(메인 ch09에서 다루지 않은 실전 패턴) + A L2916~2938('권장 사례' 섹션의 에러 처리 전략)
**참고 사항:** `anyhow` vs `thiserror` 사용 시점, 에러 변환 패턴, 에러 컨텍스트 체이닝 등을 다룹니다. C/C++ 도서의 ch09 + ch09.1 패턴을 따릅니다.

---

### 제10장: 트레이트와 제네릭 (Traits and Generics)
<!-- ch10: Traits and Generics -->

**파일명:** `ch10-traits.md`
**예상 라인 수:** ~380

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch10.1: Traits vs Interfaces --> | B L4245–4383 | 139 | 정의, 구현, C# 인터페이스와의 비교 |
| <!-- ch10.2: Implementing Behavior --> | B L2942–3083 | 100 | 구조체에 트레이트 구현하기, 다중 구현 (142라인에서 축소 — ch10.1과 겹치는 내용 제거) |
| <!-- ch10.3: Trait Objects --> | B L4385–4443 | 59 | 동적 디스패치(dynamic dispatch), dyn Trait, Box<dyn Trait> |
| <!-- ch10.4: Derived Traits --> | B L4444–4491 | 48 | #[derive], 자주 사용되는 파생 트레이트 |
| <!-- ch10.5: Std Library Traits --> | B L4492–4557 | 40 | Display, Debug, Clone, Iterator (축소 — From/Into는 제11장으로 이동) |

#### 서브 챕터: ch10.1 — 제네릭과 제약 조건
<!-- ch10.1: Generics -->

**파일명:** `ch10-1-generics.md`
**예상 라인 수:** ~170
**출처:** A L1338–1505 (168라인)
**Mermaid 다이어그램:** M11
**참고 사항:** C#의 `where T : class` vs Rust의 트레이트 바운드, 단형성화(monomorphization), 연관 타입. **M11** (제네릭 제약 조건 다이어그램) 포함. 고급 과정 문서의 내용이 부트스트랩보다 훨씬 깊이 있습니다.

#### 서브 챕터: ch10.2 — 상속 vs 구성
<!-- ch10.2: Inheritance vs Composition -->

**파일명:** `ch10-2-inheritance-vs-composition.md`
**예상 라인 수:** ~175
**출처:** A L871–1045 (175라인)
**Mermaid 다이어그램:** M8
**참고 사항:** C# 상속 계층 vs Rust 구성 모델. **M8** (상속 계층 다이어그램) 포함. 클래스 계층 구조를 잊어야 하는 **C# 개발자에게 매우 유용하고 고유한 가치**를 제공합니다. 트레이트 객체를 통한 다형성, 뉴타입(newtype) 패턴, 위임(delegation) 등을 다룹니다.

---

### 제11장: From 및 Into 트레이트 (From and Into Traits)
<!-- ch11: From and Into Traits -->

**파일명:** `ch11-from-and-into-traits.md`
**예상 라인 수:** ~120

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch11.1: From/Into Basics --> | B L4492–4530 | 40 | From<T> 구현, 자동 Into<T> (표준 라이브러리 트레이트 섹션에서 추출) |
| <!-- ch11.2: Conversion Patterns --> | 신규 | 40 | C#의 암시적/명시적 연산자 vs From/Into, TryFrom/TryInto |
| <!-- ch11.3: Error Conversions --> | B L4617–4650 | 30 | 에러 타입 변환을 위한 From<E> (에러 처리 심층 탐구에서 추출) |
| <!-- ch11.4: Practical Examples --> | 신규 | 10 | 문자열 변환, 숫자 타입 변환 |

**참고 사항:** 소스 문서 어느 쪽에도 From/Into에 대한 명시적인 챕터는 없습니다. 부트스트랩의 표준 라이브러리 트레이트 섹션(From/Into 예제)과 에러 처리(에러 변환을 위한 From)에서 내용을 수집합니다. C#의 암시적/명시적 캐스트 연산자 매핑을 위한 새로운 연결 내용이 필요합니다. 규모가 작지만(~120라인) 다른 도서 시리즈와의 일관성을 위해 제11장으로 구성합니다.

---

### 제12장: 클로저와 반복자 (Closures and Iterators)
<!-- ch12: Closures and Iterators -->

**파일명:** `ch12-closures-and-iterators.md`
**예상 라인 수:** ~300
**Mermaid 다이어그램:** M10

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch12.1: Closures --> | 신규 | 60 | C# 람다식 vs Rust 클로저, Fn/FnMut/FnOnce 트레이트, 캡처 의미론 (C# 개발자에게 람다는 익숙하므로 소유권 차이에 집중) |
| <!-- ch12.2: LINQ vs Iterators --> | A L1195–1337 | 143 | 포괄적인 LINQ-반복자 매핑. **M10** 포함. **C# 개발자에게 매우 유용하고 독보적인 콘텐츠** |
| <!-- ch12.3: Advanced Iteration --> | B L2595–2672 | 78 | Iterator/IntoIterator/Iter의 차이, 결과 수집 (제5장 컬렉션 작업에서 이동한 고급 반복자 내용) |

**참고 사항:** C/C++ 도서 시리즈에서 제12장은 "클로저"입니다. C# 개발자에게 클로저 자체는 익숙하지만(매일 람다를 사용하므로), Rust 클로저의 차이점(소유권 캡처)과 **매우 뛰어난 LINQ-반복자 매핑**에 초점을 맞춥니다. 고급 과정 문서의 LINQ 섹션은 매우 훌륭합니다.

---

### 제13장: 동시성 (Concurrency)
<!-- ch13: Concurrency -->

**파일명:** `ch13-concurrency.md`
**예상 라인 수:** ~260
**Mermaid 다이어그램:** M12

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch13.1: Thread Safety --> | A L1947–2155 | 209 | 관례 vs 타입 시스템 보장, Send/Sync, Arc/Mutex, 채널. **M12** 포함 |
| <!-- ch13.2: Async Comparison --> | A L2156–2204 | 49 | Rust async/await vs C# async/await, tokio 런타임 |

**참고 사항:** 전적으로 고급 과정 문서의 내용을 사용합니다. 스레드 안전성 섹션은 매우 포괄적이며, C#의 스레드 안전성 관련 도전 과제를 보여주는 M12 다이어그램을 포함합니다. 비동기 비교가 자연스럽게 그 뒤를 잇습니다. (부트스트랩에는 동시성 관련 내용이 없습니다.)

---

### 제14장: Unsafe Rust 및 FFI
<!-- ch14: Unsafe Rust and FFI -->

**파일명:** `ch14-unsafe-rust-and-ffi.md`
**예상 라인 수:** ~120

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch14.1: Unsafe Blocks --> | 신규 | 50 | C# `unsafe` 키워드 vs Rust `unsafe` 블록, unsafe의 허용 범위, 안전성 불변 조건(invariants) |
| <!-- ch14.2: FFI Basics --> | 신규 | 40 | C# P/Invoke + COM 상호 운용 vs Rust FFI (`extern "C"`), bindgen |
| <!-- ch14.3: When to Use Unsafe --> | 신규 | 30 | 가이드라인, 안전한 API를 통한 unsafe 추상화 |

**참고 사항:** 소스 문서 어느 쪽에도 명시적인 unsafe/FFI 내용은 없습니다 (고급 과정 ToC에는 언급되어 있으나 본문이 작성되지 않음). 이 챕터는 신규 작성이 필요합니다. C# 개발자에게 핵심 매핑은: `unsafe {}` 블록, P/Invoke → `extern "C"`, COM 상호 운용 → FFI 바인딩입니다. C#에서 Rust로 전환 시 아주 흔하게 쓰이는 기능은 아니므로 간결하게 유지합니다.

---

### 제15장: 사례 연구 및 실전 마이그레이션 (Case Studies)
<!-- ch15: Case Studies -->

**파일명:** `ch15-case-studies.md`
**예상 라인 수:** ~400

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch15.1: Config Management --> | B L4720–4854 | 135 | C# IConfiguration → Rust 설정 크레이트 마이그레이션 |
| <!-- ch15.2: Data Processing --> | B L4855–5039 | 185 | LINQ 파이프라인 → Rust 반복자 파이프라인 |
| <!-- ch15.3: HTTP Client --> | B L5040–5218 | 80 | HttpClient → reqwest 마이그레이션 (179라인에서 축소 — ch15.2의 UserService 예제와 겹치는 부분 제거) |

#### 서브 챕터: ch15.1 — 공통 패턴 및 필수 크레이트
<!-- ch15.1: Common Patterns and Essential Crates -->

**파일명:** `ch15-1-common-patterns-and-essential-crates.md`
**예상 라인 수:** ~400

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch15.1.1: Repository Pattern --> | A L1506–1625 | 120 | C# 저장소(Repository) → Rust 트레이트 기반 저장소 |
| <!-- ch15.1.2: Builder Pattern --> | A L1626–1743 | 118 | C# 빌더 → 소유권을 소비하는(consuming self) Rust 빌더 |
| <!-- ch15.1.3: Essential Crates --> | A L1744–1946 | 160 | **C# 문서 고유 가치.** 모든 C# 라이브러리를 Rust 대응 크레이트로 매핑 (serde↔Json, reqwest↔HttpClient, tokio↔Task, thiserror↔Exception, sqlx↔EF 등) + 전체 UserService 예제 포함. 203라인에서 ~160라인으로 축소 (ch15 HTTP 클라이언트와 겹치는 내용 제거) |

#### 서브 챕터: ch15.2 — 도입 전략 및 개념 매핑
<!-- ch15.2: Adoption Strategy -->

**파일명:** `ch15-2-adoption-strategy.md`
**예상 라인 수:** ~390

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch15.2.1: Concept Mapping --> | A L2428–2595 | 168 | **고유하고 가치 있는 콘텐츠.** DI → 트레이트 주입, LINQ → 반복자 체인, EF → SQLx, IConfiguration → 설정 크레이트. 각 항목마다 C#/Rust 코드 병기 |
| <!-- ch15.2.2: Incremental Adoption --> | A L2205–2427 | 120 | 1/2/3단계 도입 전략 (223라인에서 축소 — 필수 크레이트 및 개념 매핑과 겹치는 부분 제거) |
| <!-- ch15.2.3: Team Timeline --> | A L2596–2708 | 100 | 구체적인 마일스톤이 포함된 1/2/3개월 차 타임라인 (113라인에서 축소 — 도입 단계와 겹치는 부분 제거) |

---

### 제16장: 권장 사례 (Best Practices)
<!-- ch16: Best Practices -->

**파일명:** `ch16-best-practices.md`
**예상 라인 수:** ~340
**Mermaid 다이어그램:** M13

| 서브 섹션 마커 | 출처 | 라인 수 | 참고 사항 |
|---|---|---|---|
| <!-- ch16.1: Mindset Shifts --> | A L2886–2891 | 6 | 주요 사고방식의 변화 |
| <!-- ch16.2: Code Organization --> | A L2892–2915 | 24 | 프로젝트 구조 권장 사항 |
| <!-- ch16.3: Testing Patterns --> | A L2939–2974 | 36 | #[test], #[cfg(test)], 통합 테스트 |
| <!-- ch16.4: Common Mistakes --> | A L2975–3021 | 47 | 상속 시도, unwrap 오용, 과도한 clone, RefCell 남용 |
| <!-- ch16.5: Performance Comparison --> | A L2709–2883 | 130 | 관리형 vs 네이티브 성능 특징, 벤치마크, CPU 부하, 결정 기준. **M13** (마이그레이션 전략 결정 트리) 포함. 175라인에서 ~130라인으로 축소 (ch01의 '선택 시점'과 겹치는 내용 제거) |
| <!-- ch16.6: Common Pitfalls --> | B L5288–5363 | 76 | 소유권 혼동, 빌림 검사기와의 싸움, 널 값 기대 |

#### 서브 챕터: ch16.1 — 학습 경로 및 리소스
<!-- ch16.1: Learning Path -->

**파일명:** `ch16-1-learning-path.md`
**예상 라인 수:** ~100
**출처:** B L5219~5287 (69라인) + B L5269~5287의 엄선된 리소스
**참고 사항:** 주 단위 및 월 단위 학습 계획. 도서, 온라인 리소스, 연습 프로젝트. 145라인에서 ~100라인으로 축소 (타임라인 내용은 ch15.2 팀 타임라인과 겹침).

---

## SUMMARY.md (mdbook 형식)

```markdown
# 요약 (Summary)

[들어가며](ch00-introduction.md)

---

- [1. 서론 및 동기](ch01-introduction-and-motivation.md)
- [2. 시작하기](ch02-getting-started.md)
    - [키워드 참조](ch02-1-keywords-reference.md)
- [3. 내장 타입](ch03-built-in-types.md)
    - [진정한 불변성 심층 탐구](ch03-1-true-immutability.md)
- [4. 제어 흐름](ch04-control-flow.md)
- [5. 데이터 구조](ch05-data-structures.md)
    - [생성자 패턴](ch05-1-constructor-patterns.md)
    - [컬렉션: Vec, HashMap, 그리고 반복](ch05-2-collections.md)
- [6. 열거형 및 패턴 매칭](ch06-enums-and-pattern-matching.md)
    - [철저한 매칭과 널 안전성](ch06-1-exhaustive-matching-and-null-safety.md)
- [7. 소유권과 빌림](ch07-ownership-and-borrowing.md)
    - [참조, 포인터, 그리고 메모리 안전성](ch07-1-references-pointers-and-memory-safety.md)
- [8. 크레이트와 모듈](ch08-crates-and-modules.md)
    - [패키지 관리 심층 탐구](ch08-1-package-management.md)
- [9. 에러 처리](ch09-error-handling.md)
    - [에러 처리 권장 사례](ch09-1-error-handling-best-practices.md)
- [10. 트레이트와 제네릭](ch10-traits.md)
    - [제네릭](ch10-1-generics.md)
    - [상속 vs 구성](ch10-2-inheritance-vs-composition.md)
- [11. From 및 Into 트레이트](ch11-from-and-into-traits.md)
- [12. 클로저와 반복자](ch12-closures-and-iterators.md)
- [13. 동시성](ch13-concurrency.md)
- [14. Unsafe Rust 및 FFI](ch14-unsafe-rust-and-ffi.md)
- [15. 사례 연구](ch15-case-studies.md)
    - [공통 패턴 및 필수 크레이트](ch15-1-common-patterns-and-essential-crates.md)
    - [도입 전략 및 개념 매핑](ch15-2-adoption-strategy.md)
- [16. 권장 사례](ch16-best-practices.md)
    - [학습 경로 및 리소스](ch16-1-learning-path.md)
```

---

## 중복 해결 요약 (Overlap Resolution Summary)

| 중복 주제 | 부트스트랩 소스 | 고급 과정 소스 | 해결 방안 |
|---|---|---|---|
| **Option/널 안전성** | B L2085–2133, B L3503–3615 | A L215–318 (M3) | 개념 설명은 A를 사용(다이어그램 포함). 실전 예제는 B L3503~3615 사용. B L2085~2133 제거. → ch06.1 |
| **에러 처리** | B L1979–2162 (기초), B L4558–4715 (심층) | A L1046–1194 (M9) | 개념적 틀은 A를 사용(다이어그램 포함). 패턴은 B 심층 탐구 사용. B 기초 제거. → ch09 |
| **패턴 매칭** | B L1887–1978 (소개), B L3379–3502 (전체) | A L452–576 (M5) | ch04에서 간단히 미리보기(~35라인). ch06에서 B L3379+ 기반 전체 설명. A의 철저한 매칭 추가. → ch04, ch06 |
| **트레이트/인터페이스** | B L4245–4557 (전체), B L2942–3083 (구현) | A L871–1045 (상속, M8) | 트레이트 메커니즘은 B 사용(ch10 메인). 상속 vs 구성 철학은 A 사용(ch10.2). B 구현 섹션은 ch10 메인에 통합. |
| **GC vs 소유권** | B L222–270 (문제점) | A L126–214 (M2) | A 사용(다이어그램 포함). B의 문제점 섹션은 중복 제거를 위해 축소. → ch01 |
| **철학/동기** | B L111–400 (사례+문제점) | A L70–125 (M1) | 심도 있는 철학은 A 사용(다이어그램 포함). 실용적인 동기 부여 논거는 B 사용. → ch01 |
| **컬렉션/반복** | B L2549–2672 (작업) | A L1195–1337 (LINQ, M10) | 기본적인 반복은 ch05.2. LINQ 비교는 A 기반으로 ch12에서 처리. B의 고급 반복은 ch12로 이동. |

---

## 챕터별 예상 라인 수 (Estimated Line Counts by Chapter)

| 챕터 | 메인 | 서브 챕터 | 합계 |
|---------|------|-------------|-------|
| ch00 서론 | 30 | — | 30 |
| ch01 서론 및 동기 | 380 | — | 380 |
| ch02 시작하기 | 170 | ch02.1 키워드 (400) | 570 |
| ch03 내장 타입 | 280 | ch03.1 불변성 (136) | 416 |
| ch04 제어 흐름 | 280 | — | 280 |
| ch05 데이터 구조 | 380 | ch05.1 생성자 (210) + ch05.2 컬렉션 (390) | 980 |
| ch06 열거형 및 매칭 | 320 | ch06.1 철저함/널 (300) | 620 |
| ch07 소유권 | 330 | ch07.1 메모리 안전 (220) | 550 |
| ch08 크레이트 및 모듈 | 340 | ch08.1 패키지 관리 (235) | 575 |
| ch09 에러 처리 | 350 | ch09.1 권장 사례 (80) | 430 |
| ch10 트레이트 및 제네릭 | 380 | ch10.1 제네릭 (170) + ch10.2 상속 (175) | 725 |
| ch11 From/Into | 120 | — | 120 |
| ch12 클로저 및 반복자 | 300 | — | 300 |
| ch13 동시성 | 260 | — | 260 |
| ch14 Unsafe 및 FFI | 120 | — | 120 |
| ch15 사례 연구 | 400 | ch15.1 패턴/크레이트 (400) + ch15.2 도입 (390) | 1,190 |
| ch16 권장 사례 | 340 | ch16.1 학습 경로 (100) | 440 |
| **합계** | | | **~7,986** |

**원시 데이터 대비 축소량:** 8,384 → ~5,800 고유 콘텐츠 (중복 제거 후) + ~120 신규 콘텐츠 (ch11 연결, ch14 신규) ≈ **총 5,920라인의 병합된 결과물**이 16개 메인 챕터와 14개 서브 챕터에 걸쳐 구성됨.

---

## 보존된 주요 C# 전용 콘텐츠 (Unique C#-Specific Content Preserved)

| 콘텐츠 | 출처 | 챕터 | 중요성 |
|---------|--------|---------|----------------|
| 빠른 참조표 | B L93–110 | ch01 | 한눈에 보는 C#→Rust 매핑 |
| 키워드 참조 (400라인) | B L842–1244 | ch02.1 | 포괄적인 C# 키워드 → Rust 매핑 |
| 진정한 불변성 vs 레코드 | A L577–712 | ch03.1 | C# `record`가 진정한 불변이 아닌 이유 설명 |
| 13개 Mermaid 다이어그램 | A 다양함 | 다양함 | 시각적인 개념 비교 |
| LINQ vs 반복자 | A L1195–1337 | ch12 | 모든 LINQ 메서드를 Rust에 매핑 |
| DI → 트레이트 주입 | A L2430–2478 | ch15.2 | IServiceCollection → 제네릭 생성자 매핑 |
| EF → SQLx 매핑 | A L2514–2555 | ch15.2 | DbContext → sqlx::query_as! 매핑 |
| IConfiguration → 설정 | A L2556–2595 | ch15.2 | appsettings.json → config 크레이트 매핑 |
| 필수 크레이트 매핑 | A L1744–1946 | ch15.1 | 모든 C# 라이브러리 → Rust 대응 크레이트 매핑 |
| 저장소(Repository) 패턴 | A L1506–1625 | ch15.1 | IRepository → 트레이트 + async_trait 구현 |
| 빌더(Builder) 패턴 | A L1626–1743 | ch15.1 | C# 빌더 → 소유권 소비형 빌더 매핑 |
| 스레드 안전성 보장 | A L1947–2204 | ch13 | 관례에서 타입 시스템에 의한 강제로의 변화 |
| 마이그레이션 결정 트리 | A L2850–2883 | ch16 | 도입 의사 결정을 위한 Mermaid 플로우차트 |
| 성능 벤치마크 | A L2709–2830 | ch16 | 관리형 vs 네이티브 성능 데이터 |
| 팀 도입 타임라인 | A L2596–2708 | ch15.2 | 월별 단계적 도입 계획 |
