# Rust 매크로: 전처리기에서 메타프로그래밍까지

> **학습 내용:** Rust 매크로의 작동 방식, 함수나 제네릭 대신 매크로를 사용하는 시점, 그리고 매크로가 C/C++ 전처리기를 어떻게 대체하는지 배웁니다. 이 장을 마치면 직접 `macro_rules!` 매크로를 작성할 수 있고 `#[derive(Debug)]`가 내부적으로 어떤 일을 하는지 이해하게 될 것입니다.

매크로는 Rust를 시작하자마자 접하게 되는 기능이지만(`println!("hello")`), 대부분의 강좌에서 가장 나중에 설명하는 기능이기도 합니다. 이 장에서는 그 간격을 메워보겠습니다.

### 매크로가 존재하는 이유

Rust에서 코드 재사용은 대부분 함수와 제네릭이 담당합니다. 매크로는 타입 시스템이 닿지 못하는 틈새를 채워줍니다.

| 요구 사항 | 함수/제네릭으로 가능한가? | 매크로로 가능한가? | 이유 |
|------|-------------------|--------|-----|
| 값 계산 | ✅ `fn max<T: Ord>(a: T, b: T) -> T` | — | 타입 시스템이 처리 가능함 |
| 가변 인자(Variable number of arguments) 수용 | ❌ Rust에는 가변 인자 함수가 없음 | ✅ `println!("{} {}", a, b)` | 매크로는 어떤 수의 토큰도 수용함 |
| 반복적인 `impl` 블록 생성 | ❌ 제네릭만으로는 불가능함 | ✅ `macro_rules!` | 매크로는 컴파일 타임에 코드를 생성함 |
| 컴파일 타임에 코드 실행 | ❌ `const fn`은 제한적임 | ✅ 절차적 매크로 (Procedural macros) | 컴파일 타임에 완전한 Rust 코드 실행 가능 |
| 조건부 코드 포함 | ❌ | ✅ `#[cfg(...)]` | 속성 매크로가 컴파일을 제어함 |

C/C++ 개발자라면 매크로를 *전처리기를 대체할 수 있는 유일하고 올바른 대안*으로 생각하십시오. 다만 Rust 매크로는 가공되지 않은 텍스트가 아닌 구문 트리(syntax tree) 상에서 작동하므로, 위생적(hygienic, 이름 충돌 없음)이고 타입을 인식할 수 있습니다.

---

## `macro_rules!`를 사용한 선언적 매크로 (Declarative Macros)

선언적 매크로(또는 "예시에 의한 매크로")는 Rust에서 가장 흔한 매크로 형태입니다. 값에 대해 `match`를 사용하듯 구문에 대해 패턴 매칭을 수행합니다.

### 기본 문법

```rust
macro_rules! say_hello {
    () => {
        println!("안녕하세요!");
    };
}

fn main() {
    say_hello!();  // 다음으로 확장됨: println!("안녕하세요!");
}
```

이름 뒤의 `!`는 컴파일러와 개발자에게 이것이 매크로 호출임을 알려줍니다.

### 인자를 사용한 패턴 매칭

매크로는 조각 지정자(fragment specifiers)를 사용하여 *토큰 트리*에 대해 매칭을 수행합니다.

```rust
macro_rules! greet {
    // 패턴 1: 인자 없음
    () => {
        println!("안녕하세요, 세상아!");
    };
    // 패턴 2: 하나의 식(expression) 인자
    ($name:expr) => {
        println!("안녕하세요, {}님!", $name);
    };
}

fn main() {
    greet!();           // "안녕하세요, 세상아!"
    greet!("Rust");     // "안녕하세요, Rust님!"
}
```

#### 조각 지정자(Fragment specifiers) 참조

| 지정자 | 매칭 대상 | 예시 |
|-----------|---------|---------|
| `$x:expr` | 모든 식(expression) | `42`, `a + b`, `foo()` |
| `$x:ty` | 타입 | `i32`, `Vec<String>`, `&str` |
| `$x:ident` | 식별자(identifier) | `foo`, `my_var` |
| `$x:pat` | 패턴 | `Some(x)`, `_`, `(a, b)` |
| `$x:stmt` | 문장(statement) | `let x = 5;` |
| `$x:block` | 블록 | `{ println!("hi"); 42 }` |
| `$x:literal` | 리터럴 | `42`, `"hello"`, `true` |
| `$x:tt` | 단일 토큰 트리 | 무엇이든 가능 — 와일드카드 역할 |
| `$x:item` | 아이템 (fn, struct, impl 등) | `fn foo() {}` |

### 반복(Repetition) — 강력한 핵심 기능

C/C++ 매크로는 루프를 돌 수 없지만, Rust 매크로는 패턴을 반복할 수 있습니다.

```rust
macro_rules! make_vec {
    // 쉼표로 구분된 0개 이상의 식에 매칭
    ( $( $element:expr ),* ) => {
        {
            let mut v = Vec::new();
            $( v.push($element); )*  // 매칭된 각 요소에 대해 반복
            v
        }
    };
}

fn main() {
    let v = make_vec![1, 2, 3, 4, 5];
    println!("{v:?}");  // [1, 2, 3, 4, 5]
}
```

`$( ... ),*` 문법은 "쉼표로 구분된 이 패턴이 0개 이상 일치함"을 의미합니다. 확장부의 `$( ... )*`는 일치하는 항목마다 본문을 한 번씩 반복합니다.

> **이것이 표준 라이브러리의 `vec![]`가 구현된 방식과 정확히 일치합니다.** 실제 소스 코드는 다음과 같습니다:
> ```rust
> macro_rules! vec {
>     () => { Vec::new() };
>     ($elem:expr; $n:expr) => { vec::from_elem($elem, $n) };
>     ($($x:expr),+ $(,)?) => { <[_]>::into_vec(Box::new([$($x),+])) };
> }
> ```
> 마지막의 `$(,)?`는 선택적인 끝 쉼표(trailing comma)를 허용합니다.

#### 반복 연산자

| 연산자 | 의미 | 예시 |
|----------|---------|---------|
| `$( ... )*` | 0개 이상 | `vec![]`, `vec![1]`, `vec![1, 2, 3]` |
| `$( ... )+` | 1개 이상 | 최소 하나의 요소가 필요함 |
| `$( ... )?` | 0개 또는 1개 | 선택적 요소 |

### 실무 예시: `hashmap!` 생성자

표준 라이브러리에는 `vec![]`는 있지만 `hashmap!{}`은 없습니다. 직접 만들어 봅시다.

```rust
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* $(,)? ) => {
        {
            let mut map = std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
}

fn main() {
    let scores = hashmap! {
        "Alice" => 95,
        "Bob" => 87,
        "Carol" => 92,  // $(,)? 덕분에 끝 쉼표 허용
    };
    println!("{scores:?}");
}
```

### 실제 예시: 진단 체크 매크로

임베디드나 진단 코드에서 흔히 쓰이는 패턴 — 조건을 확인하고 에러를 반환합니다.

```rust
use thiserror::Error;

#[derive(Error, Debug)]
enum DiagError {
    #[error("체크 실패: {0}")]
    CheckFailed(String),
}

macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}

fn run_diagnostics(temp: f64, voltage: f64) -> Result<(), DiagError> {
    diag_check!(temp < 85.0, "GPU 과열");
    diag_check!(voltage > 0.8, "레일 전압 낮음");
    diag_check!(voltage < 1.5, "레일 전압 높음");
    println!("모든 체크 통과");
    Ok(())
}
```

> **C/C++ 비교:**
> ```c
> // C 전처리기 — 단순 텍스트 치환, 타입 안전성 없음, 위생적이지 않음
> #define DIAG_CHECK(cond, msg) \
>     do { if (!(cond)) { log_error(msg); return -1; } } while(0)
> ```
> Rust 버전은 적절한 `Result` 타입을 반환하며, 이중 평가 위험이 없고, 컴파일러가 `$cond`가 실제로 `bool` 식인지 검사합니다.

### 위생(Hygiene): Rust 매크로가 안전한 이유

C/C++ 매크로 버그는 대개 이름 충돌에서 발생합니다.

```c
// C: 위험함 — `x`가 호출자의 `x`를 가릴 수 있음
#define SQUARE(x) ((x) * (x))
int x = 5;
int result = SQUARE(x++);  // 미정의 동작: x가 두 번 증가함!
```

Rust 매크로는 **위생적(hygienic)**입니다. 매크로 내부에서 생성된 변수는 밖으로 새어 나가지 않습니다.

```rust
macro_rules! make_x {
    () => {
        let x = 42;  // 이 `x`는 매크로 확장 스코프 내로 제한됨
    };
}

fn main() {
    let x = 10;
    make_x!();
    println!("{x}");  // 42가 아닌 10 출력 — 위생 덕분에 충돌 방지
}
```

매크로의 `x`와 호출자의 `x`는 이름이 같더라도 컴파일러에 의해 서로 다른 변수로 취급됩니다. **이는 C 전처리기에서는 불가능한 일입니다.**

---

## 자주 쓰이는 표준 라이브러리 매크로

1장에서부터 사용해 온 매크로들입니다. 실제로 어떤 일을 하는지 살펴봅시다.

| 매크로 | 역할 | 확장 결과 (간략화) |
|-------|-------------|------------------------|
| `println!("{}", x)` | 표준 출력에 포맷팅하여 출력 + 줄바꿈 | `std::io::_print(format_args!(...))` |
| `eprintln!("{}", x)` | 표준 에러에 출력 + 줄바꿈 | 위와 같으나 stderr 사용 |
| `format!("{}", x)` | `String`으로 포맷팅 | `String`을 할당하고 반환 |
| `vec![1, 2, 3]` | 요소를 포함한 `Vec` 생성 | 대략 `Vec::from([1, 2, 3])` |
| `todo!()` | 미구현 코드 표시 | `panic!("not yet implemented")` |
| `unimplemented!()` | 의도적인 미구현 코드 표시 | `panic!("not implemented")` |
| `unreachable!()` | 도달 불가능함을 표시 | `panic!("unreachable")` |
| `assert!(cond)` | 조건이 거짓이면 패닉 | `if !cond { panic!(...) }` |
| `assert_eq!(a, b)` | 두 값이 다르면 패닉 | 실패 시 두 값을 모두 보여줌 |
| `dbg!(expr)` | 식과 값을 stderr에 출력하고 값 반환 | `eprintln!("[파일:라인] 식 = {:#?}", &식); 식` |
| `include_str!("file.txt")` | 컴파일 타임에 파일 내용을 `&str`로 포함 | 컴파일 중 파일 읽기 수행 |
| `include_bytes!("data.bin")` | 컴파일 타임에 파일 내용을 `&[u8]`로 포함 | 컴파일 중 파일 읽기 수행 |
| `cfg!(condition)` | 컴파일 타임 조건을 `bool`로 반환 | 타겟에 따라 `true` 또는 `false` |
| `env!("VAR")` | 컴파일 타임에 환경 변수 읽기 | 설정되지 않은 경우 컴파일 실패 |
| `concat!("a", "b")` | 컴파일 타임에 리터럴 결합 | `"ab"` |

### `dbg!` — 매일 사용하게 될 디버깅 매크로

```rust
fn factorial(n: u32) -> u32 {
    if dbg!(n <= 1) {     // 출력: [src/main.rs:2] n <= 1 = false
        dbg!(1)           // 출력: [src/main.rs:3] 1 = 1
    } else {
        dbg!(n * factorial(n - 1))  // 중간 값들을 출력함
    }
}

fn main() {
    dbg!(factorial(4));   // 파일:라인 정보와 함께 모든 재귀 호출을 출력함
}
```

`dbg!`는 감싸고 있는 값을 반환하므로, 프로그램 동작을 바꾸지 않고 어디든 삽입할 수 있습니다. 표준 출력이 아닌 표준 에러(stderr)에 출력되므로 프로그램의 실제 출력과 섞이지 않습니다. **코드를 커밋하기 전에는 모든 `dbg!` 호출을 제거하십시오.**

### 포맷 문자열 문법

`println!`, `format!`, `eprintln!`, `write!` 등은 모두 동일한 포맷 메커니즘을 사용합니다. 빠른 참조 가이드를 확인하십시오.

```rust
let name = "sensor";
let value = 3.14159;
let count = 42;

println!("{name}");                    // 이름으로 변수 지정 (Rust 1.58+)
println!("{}", name);                  // 순서대로 지정
println!("{value:.2}");                // 소수점 2자리: "3.14"
println!("{count:>10}");               // 우측 정렬, 너비 10: "        42"
println!("{count:0>10}");              // 0으로 채우기: "0000000042"
println!("{count:#06x}");              // 접두사 포함 16진수: "0x002a"
println!("{count:#010b}");             // 접두사 포함 2진수: "0b00101010"
println!("{value:?}");                 // Debug 포맷
println!("{value:#?}");                // 보기 좋게 출력된(Pretty-printed) Debug 포맷
```

> **C 개발자:** 이것을 타입 안전한 `printf`라고 생각하십시오. 컴파일러는 `{:.2}`가 문자열이 아닌 부동 소수점에 적용되었는지 확인합니다. `%s`/`%d` 포맷 불일치 버그가 발생하지 않습니다.
>
> **C++ 개발자:** 이것은 `std::cout << std::fixed << std::setprecision(2) << value`를 하나의 읽기 쉬운 포맷 문자열로 대체합니다.

---

## Derive 매크로

이 책의 거의 모든 구조체에서 `#[derive(...)]`를 보셨을 것입니다.

```rust
#[derive(Debug, Clone, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}
```

`#[derive(Debug)]`는 **derive 매크로**로, 트레이트 구현을 자동으로 생성하는 특별한 종류의 절차적 매크로입니다. 다음은 이것이 생성하는 코드의 간략한 모습입니다.

```rust
// Point에 대해 #[derive(Debug)]가 생성하는 코드:
impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Point")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
```

`#[derive(Debug)]`가 없다면 모든 구조체마다 저런 `impl` 블록을 직접 작성해야 했을 것입니다.

### 자주 쓰이는 derive 트레이트들

| Derive | 생성되는 기능 | 사용 시점 |
|--------|-------------------|-------------|
| `Debug` | `{:?}` 포맷팅 | 거의 항상 — 디버깅을 위한 출력 기능 제공 |
| `Clone` | `.clone()` 메서드 | 값을 복제해야 할 때 |
| `Copy` | 대입 시 암시적 복사 | 정수, `[f64; 3]` 등 스택 전용 소형 타입 |
| `PartialEq` / `Eq` | `==` 및 `!=` 연산자 | 동등 비교가 필요할 때 |
| `PartialOrd` / `Ord` | `<`, `>`, `<=`, `>=` 연산자 | 순서 비교가 필요할 때 |
| `Hash` | `HashMap`/`HashSet` 용 해싱 | 맵의 키로 사용될 타입 |
| `Default` | `Type::default()` 생성자 | 합리적인 0 또는 빈 값이 있는 타입 |
| `serde::Serialize` / `Deserialize` | JSON/TOML 등 직렬화 | API 경계를 넘나드는 데이터 타입 |

### Derive 결정 트리

```text
derive를 써야 할까요?
  │
  ├── 내 타입이 해당 트레이트를 구현한 타입들만 포함하고 있는가?
  │     ├── 예 → #[derive]가 작동함
  │     └── 아니요 → 수동으로 구현하거나 생략하십시오.
  │
  └── 내 타입의 사용자가 이 동작을 기대하는 것이 합리적인가?
        ├── 예 → derive 하십시오 (Debug, Clone, PartialEq는 거의 항상 합리적임)
        └── 아니요 → 하지 마십시오 (예: 파일 핸들이 있는 타입에 Copy를 구현하지 마십시오)
```

> **C++ 비교:** `#[derive(Clone)]`은 올바른 복사 생성자를 자동 생성하는 것과 같습니다. `#[derive(PartialEq)]`는 각 필드를 비교하는 `operator==`를 자동 생성하는 것과 같으며, 이는 C++20의 `= default` 우주선 연산자가 마침내 제공하게 된 기능입니다.

---

## 속성 매크로 (Attribute Macros)

속성 매크로는 연결된 항목을 변형합니다. 이미 여러 번 사용해 보셨을 것입니다.

```rust
#[test]                    // 함수를 테스트로 표시
fn test_addition() {
    assert_eq!(2 + 2, 4);
}

#[cfg(target_os = "linux")] // 리눅스에서만 이 함수를 포함
fn linux_only() { /* ... */ }

#[derive(Debug)]            // Debug 구현 생성
struct MyType { /* ... */ }

#[allow(dead_code)]         // 컴파일러 경고 억제
fn unused_helper() { /* ... */ }

#[must_use]                 // 반환값이 버려지면 경고 발생
fn compute_checksum(data: &[u8]) -> u32 { /* ... */ }
```

자주 쓰이는 내장 속성:

| 속성 | 목적 |
|-----------|---------|
| `#[test]` | 테스트 함수로 표시 |
| `#[cfg(...)]` | 조건부 컴파일 |
| `#[derive(...)]` | 트레이트 구현 자동 생성 |
| `#[allow(...)]` / `#[deny(...)]` / `#[warn(...)]` | 린트(lint) 레벨 제어 |
| `#[must_use]` | 사용되지 않는 반환값에 대해 경고 |
| `#[inline]` / `#[inline(always)]` | 인라인화 힌트 제공 |
| `#[repr(C)]` | C 호환 메모리 레이아웃 사용 (FFI 용) |
| `#[no_mangle]` | 심볼 이름을 바꾸지 않음 (FFI 용) |
| `#[deprecated]` | 선택적 메시지와 함께 사용 중단 표시 |

> **C/C++ 개발자:** 속성은 전처리기 지시문(`#pragma`, `__attribute__((...))`)과 컴파일러별 확장을 하나로 합친 형태입니다. 덧붙여진 확장이 아니라 언어 문법의 정식 일부입니다.

---

## 절차적 매크로 (Procedural Macros) 개념 개요

절차적 매크로("proc macros")는 컴파일 타임에 실행되어 코드를 생성하는 *별도의 Rust 프로그램*으로 작성된 매크로입니다. `macro_rules!`보다 강력하지만 더 복잡합니다.

세 가지 종류가 있습니다.

| 종류 | 문법 | 예시 | 역할 |
|------|--------|---------|-------------|
| **함수형 (Function-like)** | `my_macro!(...)` | `sql!(SELECT * FROM users)` | 커스텀 구문을 파싱하여 Rust 코드 생성 |
| **Derive** | `#[derive(MyTrait)]` | `#[derive(Serialize)]` | 구조체 정의로부터 트레이트 구현 생성 |
| **속성 (Attribute)** | `#[my_attr]` | `#[tokio::main]`, `#[instrument]` | 주석이 달린 항목을 변형함 |

### 이미 절차적 매크로를 사용해 보셨습니다

- `thiserror`의 `#[derive(Error)]` — 에러 열거형에 대해 `Display` 및 `From` 구현 생성
- `serde`의 `#[derive(Serialize, Deserialize)]` — 직렬화 코드 생성
- `#[tokio::main]` — `async fn main()`을 런타임 설정과 `block_on`으로 변환
- `#[test]` — 테스트 하네스에 의해 등록됨 (내장 절차적 매크로)

### 직접 절차적 매크로를 작성해야 할 때

이 과정 중에 직접 작성할 일은 거의 없을 것입니다. 다음의 경우에 유용합니다.
- 컴파일 타임에 구조체 필드나 열거형 변형을 검사해야 할 때 (derive 매크로)
- 도메인 특화 언어(DSL)를 구축할 때 (함수형 매크로)
- 함수 시그니처를 변형해야 할 때 (속성 매크로)

대부분의 코드에서는 `macro_rules!`나 일반 함수로 충분합니다.

> **C++ 비교:** 절차적 매크로는 C++에서 코드 생성기, 템플릿 메타프로그래밍, `protoc` 같은 외부 도구가 하던 역할을 수행합니다. 차이점은 절차적 매크로가 cargo 빌드 파이프라인의 일부라는 것입니다 — 외부 빌드 단계나 CMake 커스텀 명령이 필요 없습니다.

---

## 무엇을 사용할 것인가: 매크로 대 함수 대 제네릭

```text
코드를 생성해야 합니까?
  │
  ├── 아니요 → 함수나 제네릭 함수를 사용하십시오.
  │         (더 단순하고, 에러 메시지가 좋으며, IDE 지원이 원활함)
  │
  └── 예 ─┬── 가변 인자가 필요합니까?
            │     └── 예 → macro_rules! (예: println!, vec!)
            │
            ├── 많은 타입에 대해 반복적인 impl 블록이 필요합니까?
            │     └── 예 → 반복 기능이 있는 macro_rules!
            │
            ├── 구조체 필드를 검사해야 합니까?
            │     └── 예 → Derive 매크로 (절차적 매크로)
            │
            ├── 커스텀 구문(DSL)이 필요합니까?
            │     └── 예 → 함수형 절차적 매크로
            │
            └── 함수나 구조체를 변형해야 합니까?
                  └── 예 → 속성 절차적 매크로
```

**일반 가이드라인:** 함수나 제네릭으로 할 수 있다면 매크로를 쓰지 마십시오. 매크로는 에러 메시지가 더 나쁘고, 매크로 본문 내부에서 IDE 자동 완성이 되지 않으며, 디버깅이 더 어렵습니다.

---

## 연습 문제

### 🟢 연습 문제 1: `min!` 매크로

다음을 수행하는 `min!` 매크로를 작성하십시오.
- `min!(a, b)`는 두 값 중 작은 값을 반환합니다.
- `min!(a, b, c)`는 세 값 중 가장 작은 값을 반환합니다.
- `PartialOrd`를 구현한 모든 타입에서 작동해야 합니다.

**힌트:** `macro_rules!`에 두 개의 매칭 암(arm)이 필요합니다.

<details><summary>해설 (클릭하여 확장)</summary>

```rust
macro_rules! min {
    ($a:expr, $b:expr) => {
        if $a < $b { $a } else { $b }
    };
    ($a:expr, $b:expr, $c:expr) => {
        min!(min!($a, $b), $c)
    };
}

fn main() {
    println!("{}", min!(3, 7));        // 3
    println!("{}", min!(9, 2, 5));     // 2
    println!("{}", min!(1.5, 0.3));    // 0.3
}
```

**참고:** 실제 운영 코드에서는 `std::cmp::min`이나 `a.min(b)`를 선호하십시오. 이 연습 문제는 여러 개의 암을 가진 매크로의 메커니즘을 보여주기 위한 것입니다.

</details>

### 🟡 연습 문제 2: 바닥부터 만드는 `hashmap!`

위의 예제를 보지 않고 다음을 수행하는 `hashmap!` 매크로를 작성하십시오.
- `key => value` 쌍으로부터 `HashMap`을 생성합니다.
- 끝 쉼표를 지원합니다.
- 해시 가능한 모든 키 타입에 대해 작동합니다.

다음 코드로 테스트하십시오:
```rust
let m = hashmap! {
    "name" => "Alice",
    "role" => "Engineer",
};
assert_eq!(m["name"], "Alice");
assert_eq!(m.len(), 2);
```

<details><summary>해설 (클릭하여 확장)</summary>

```rust
use std::collections::HashMap;

macro_rules! hashmap {
    ( $( $key:expr => $val:expr ),* $(,)? ) => {{
        let mut map = HashMap::new();
        $( map.insert($key, $val); )*
        map
    }};
}

fn main() {
    let m = hashmap! {
        "name" => "Alice",
        "role" => "Engineer",
    };
    assert_eq!(m["name"], "Alice");
    assert_eq!(m.len(), 2);
    println!("테스트 통과!");
}
```

</details>

### 🟡 연습 문제 3: 부동 소수점 비교를 위한 `assert_approx_eq!`

`|a - b| > epsilon`인 경우 패닉을 일으키는 `assert_approx_eq!(a, b, epsilon)` 매크로를 작성하십시오. 이는 정확한 동등 비교가 실패하는 부동 소수점 계산을 테스트할 때 유용합니다.

다음 코드로 테스트하십시오:
```rust
assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);        // 통과해야 함
assert_approx_eq!(3.14159, std::f64::consts::PI, 1e-4); // 통과해야 함
// assert_approx_eq!(1.0, 2.0, 0.5);              // 패닉 발생해야 함
```

<details><summary>해설 (클릭하여 확장)</summary>

```rust
macro_rules! assert_approx_eq {
    ($a:expr, $b:expr, $eps:expr) => {
        let (a, b, eps) = ($a as f64, $b as f64, $eps as f64);
        let diff = (a - b).abs();
        if diff > eps {
            panic!(
                "단언 실패: |{} - {}| = {} > {} (epsilon)",
                a, b, diff, eps
            );
        }
    };
}

fn main() {
    assert_approx_eq!(0.1 + 0.2, 0.3, 1e-10);
    assert_approx_eq!(3.14159, std::f64::consts::PI, 1e-4);
    println!("모든 실수 비교 통과!");
}
```

</details>

### 🔴 연습 문제 4: `impl_display_for_enum!`

간단한 C 스타일 열거형에 대해 `Display` 구현을 생성하는 매크로를 작성하십시오. 다음과 같이 입력했을 때:

```rust
impl_display_for_enum! {
    enum Color {
        Red => "red",
        Green => "green",
        Blue => "blue",
    }
}
```

`enum Color { Red, Green, Blue }` 정의와 각 변형을 문자열에 매핑하는 `impl Display for Color`를 모두 생성해야 합니다.

**힌트:** `$( ... ),*` 반복 기능과 여러 개의 조각 지정자가 모두 필요합니다.

<details><summary>해설 (클릭하여 확장)</summary>

```rust
use std::fmt;

macro_rules! impl_display_for_enum {
    (enum $name:ident { $( $variant:ident => $display:expr ),* $(,)? }) => {
        #[derive(Debug, Clone, Copy, PartialEq)]
        enum $name {
            $( $variant ),*
        }

        impl fmt::Display for $name {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match self {
                    $( $name::$variant => write!(f, "{}", $display), )*
                }
            }
        }
    };
}

impl_display_for_enum! {
    enum Color {
        Red => "red",
        Green => "green",
        Blue => "blue",
    }
}

fn main() {
    let c = Color::Green;
    println!("색상: {c}");          // "색상: green"
    println!("디버그: {c:?}");        // "디버그: Green"
    assert_eq!(format!("{}", Color::Red), "red");
    println!("모든 테스트 통과!");
}
```

</details>
