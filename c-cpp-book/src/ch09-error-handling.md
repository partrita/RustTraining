# Rust의 에러 처리: Option과 Result

> **학습 목표:** Rust가 null 포인터를 `Option<T>`로, 예외(Exceptions)를 `Result<T, E>`로 어떻게 대체하는지 배웁니다. 에러를 숨겨진 제어 흐름이 아닌 하나의 **'값(Value)'**으로 취급하는 Rust만의 철학을 이해하고, `?` 연산자를 활용해 에러 전파를 우아하게 처리하는 방법을 익힙니다.

---

### 에러 처리의 두 기둥: Option과 Result
Rust의 에러 처리는 표준 라이브러리에 정의된 단순한 `enum` 타입 두 가지를 기반으로 합니다.

```rust
// 1. 값이 있을 수도, 없을 수도 있는 상황 (Null 대체)
enum Option<T> {
    Some(T),  // 유효한 값이 있음
    None,     // 값이 없음
}

// 2. 작업이 성공하거나 실패할 수 있는 상황 (예외 대체)
enum Result<T, E> {
    Ok(T),    // 성공 및 결과값
    Err(E),   // 실패 및 에러 정보
}
```

### C++ 개발자를 위한 에러 처리 매핑

| **C++ 패턴** | **Rust 대응 개념** | **결정적 차이점** |
| :--- | :--- | :--- |
| `throw runtime_error(msg)` | `Err(Error::Msg(msg))` | 반환 타입에 에러가 명시되어 처리를 강제함 |
| `try { ... } catch (...)` | `match result { ... }` | 숨겨진 제어 흐름 없이 로직이 명시적임 |
| `std::optional<T>` | `Option<T>` | 컴파일러가 `None` 케이스 처리를 엄격히 검사함 |
| `noexcept` 주석 | (기본 동작) | 모든 Rust 함수는 예외를 던지지 않는 것이 기본임 |
| `errno` 또는 반환 코드 | `Result<T, E>` | 타입 안전하며, 결과를 무시할 경우 경고 발생 |

---

# 1. 값이 없는 경우의 처리: Option

Rust에는 **Null 포인터가 없습니다.** 대신 `Option<T>`를 사용하여 값이 없을 가능성을 명시적으로 표현합니다.

```rust
fn main() {
    let text = "Hello Rust";
    
    // find는 찾으면 Some(index), 못 찾으면 None을 반환합니다.
    let index = text.find('R'); 
    
    match index {
        Some(i) => println!("'R'의 위치: {i}"),
        None => println!("찾을 수 없습니다."),
    }
}
```

### Option 활용 팁
- **`unwrap()`**: 값이 있으면 꺼내고, 없으면 **즉시 패닉**을 일으킵니다. 테스트용 외에는 실무에서 지양해야 합니다.
- **`unwrap_or(default)`**: 값이 없으면 지정한 기본값을 대신 사용합니다. (안전함)
- **`if let`**: 특정 케이스(`Some`)에만 관심이 있을 때 코드를 간결하게 만들어 줍니다.

---

# 2. 실패할 수 있는 작업의 처리: Result

작업이 실패했을 때 그 이유(에러 내용)가 중요하다면 `Result`를 사용합니다.

```rust
use std::num::ParseIntError;

fn main() {
    let number_str = "12345";
    let parse_result: Result<i32, ParseIntError> = number_str.parse();
    
    match parse_result {
        Ok(n) => println!("숫자 변환 성공: {n}"),
        Err(e) => println!("변환 실패 사유: {e}"),
    }
}
```

### `?` 연산자: 에러 전파의 마법
C++에서 예외가 자동으로 상위로 전파되듯, Rust에서는 `?` 기호 하나로 에러를 상위 함수로 넘길 수 있습니다.

```rust
fn get_data_from_file() -> Result<String, std::io::Error> {
    // File::open이 에러를 내면 즉시 함수를 종료하고 에러를 반환합니다.
    let mut file = std::fs::File::open("config.txt")?; 
    let mut contents = String::new();
    
    // read_to_string 역시 에러 발생 시 즉시 전파합니다.
    file.read_to_string(&mut contents)?;
    
    Ok(contents)
}
```

---

# 💡 심층 분석: C++ 예외 vs Rust Result

### C++의 고질적 문제: 숨겨진 제어 흐름
C++ 예외는 함수 시그니처만 봐서는 이 함수가 어떤 에러를 던질지 알기 어렵습니다. 또한 `try-catch`를 잊어도 컴파일러는 아무 말도 해주지 않으며, 이는 런타임 충돌로 이어집니다.

```mermaid
graph TD
    subgraph "C++: 불투명한 예외"
        C_FUNC["함수 호출"] --> C_THROW["예외 발생!"]
        C_THROW --> C_MISS["[위험] Catch 누락?"]
        C_MISS --> C_CRASH["프로그램 비정상 종료"]
    end
    
    subgraph "Rust: 명시적인 Result"
        R_FUNC["함수 호출"] --> R_RESULT["Result 타입 반환"]
        R_RESULT --> R_MUST["[필수] 패턴 매칭 강제"]
        R_MUST --> R_SAFE["모든 경우 처리됨 (안전)"]
    end

    style C_CRASH fill:#ff6b6b,color:#000
    style R_SAFE fill:#51cf66,color:#000
```

---

# 📝 실습 연습: 에러 전파와 로깅

🟡 **중급 과정** — 아래의 로직을 완성하여 에러 처리 흐름을 익혀보세요.

1.  `log(x: u32) -> Result<(), ()>`: 입력된 `x`가 42이면 성공(`Ok`), 아니면 에러(`Err`)를 반환합니다.
2.  `run_task(x: u32) -> Result<(), ()>`: `log(x)`를 호출하되, `?` 연산자를 사용하여 에러 발생 시 즉시 종료되도록 하세요.

```rust
fn log(x: u32) -> Result<(), ()> {
    if x == 42 {
        println!("로그: 정확한 값 42가 입력되었습니다.");
        Ok(())
    } else {
        Err(())
    }
}

fn run_task(x: u32) -> Result<(), ()> {
    // '?' 연산자를 사용하여 에러 발생 시 이 지점에서 함수를 조기 종료(return)시키세요.
    log(x)?;
    
    println!("축하합니다! 작업을 무사히 마쳤습니다.");
    Ok(())
}

fn main() {
    println!("--- 42를 입력했을 때 ---");
    let _ = run_task(42);
    
    println!("\n--- 43을 입력했을 때 ---");
    let _ = run_task(43);
}
```
> **성공 출력 예시**: 43을 입력했을 때는 "축하합니다!" 문구가 출력되지 않아야 합니다.

---

### 패닉(Panic): 복구 불가능한 에러
모든 에러를 `Result`로 처리할 필요는 없습니다. 아래와 같은 치명적인 버그 상황에서는 `panic!`을 발생시켜 프로그램을 안전하게 멈추는 것이 낫습니다.

- **인덱스 범위를 벗어난 접근**: `arr[100]` (범위 밖일 때)
- **논리적 모순**: 절대 일어날 수 없는 조건에 도달했을 때 (`unreachable!()`)
- **강제 중단**: 무결성 검사 실패 시 (`assert!`)

> **권장 사항**: 라이브러리 개발자라면 최대한 `Result`를 반환하여 호출자가 결정하게 하세요. `panic`은 주로 애플리케이션의 최상단이나 명백한 버그 상황에서만 사용하는 것이 좋습니다.
