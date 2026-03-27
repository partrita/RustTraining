# Rust Option 및 Result 핵심 요약

> **학습 내용:** 관용적인(idiomatic) 에러 처리 패턴을 배웁니다 — `unwrap()`의 안전한 대안, 전파를 위한 `?` 연산자, 커스텀 에러 타입, 그리고 실제 운영 코드에서 `anyhow`와 `thiserror`를 언제 사용해야 하는지 알아봅니다.

- ```Option```과 ```Result```는 관용적인 Rust의 필수적인 부분입니다.
- **`unwrap()`의 안전한 대안들**:
```rust
// Option<T>의 안전한 대안
let value = opt.unwrap_or(default);              // 대체값 제공
let value = opt.unwrap_or_else(|| compute());    // 대체값을 위한 지연 계산
let value = opt.unwrap_or_default();             // Default 트레이트 구현 사용
let value = opt.expect("설명 메시지");           // 패닉이 허용되는 상황에서만 사용

// Result<T, E>의 안전한 대안
let value = result.unwrap_or(fallback);          // 에러 무시, 대체값 사용
let value = result.unwrap_or_else(|e| handle(e)); // 에러 처리 후 대체값 반환
let value = result.unwrap_or_default();          // Default 트레이트 사용
```
- **명시적 제어를 위한 패턴 매칭**:
```rust
match some_option {
    Some(value) => println!("값 발견: {}", value),
    None => println!("값이 없음"),
}

match some_result {
    Ok(value) => process(value),
    Err(error) => log_error(error),
}
```
- **에러 전파를 위한 `?` 연산자 사용**: 에러 발생 시 즉시 반환하여 상위로 전달
```rust
fn process_file(path: &str) -> Result<String, std::io::Error> {
    let content = std::fs::read_to_string(path)?; // 자동으로 에러 반환
    Ok(content.to_uppercase())
}
```
- **변환 메서드**:
    - `map()`: 성공 값 변환 `Ok(T)` -> `Ok(U)` 또는 `Some(T)` -> `Some(U)`
    - `map_err()`: 에러 타입 변환 `Err(E)` -> `Err(F)`
    - `and_then()`: 실패할 수 있는 작업들을 체인으로 연결
- **자체 API에서의 활용**: 예외나 에러 코드 대신 `Result<T, E>`를 선호하세요.
- **참고 문헌**: [Option 문서](https://doc.rust-lang.org/std/option/enum.Option.html) | [Result 문서](https://doc.rust-lang.org/std/result/enum.Result.html)

# Rust의 일반적인 함정과 디버깅 팁
- **빌림(Borrowing) 문제**: 초보자가 가장 흔히 하는 실수
    - "cannot borrow as mutable" -> 한 번에 하나의 가변 참조자만 허용됨
    - "borrowed value does not live long enough" -> 참조자가 가리키는 데이터보다 더 오래 생존함
    - **해결책**: 범위 `{}`를 사용하여 참조자의 수명을 제한하거나, 필요한 경우 데이터를 클론(clone)하세요.
- **트레이트 구현 누락**: "method not found" 에러
    - **해결책**: 일반적인 트레이트들을 위해 `#[derive(Debug, Clone, PartialEq)]`를 추가하세요.
    - `cargo run`보다 더 나은 에러 메시지를 얻으려면 `cargo check`를 사용하세요.
- **디버그 모드에서의 정수 오버플로**: Rust는 오버플로 발생 시 패닉을 일으킵니다.
    - **해결책**: 명시적인 동작을 위해 `wrapping_add()`, `saturating_add()`, 또는 `checked_add()`를 사용하세요.
- **String vs &str 혼동**: 용도에 따른 서로 다른 타입
    - 문자열 슬라이스(빌려온 것)에는 `&str`을, 소유권이 있는 문자열에는 `String`을 사용하세요.
    - **해결책**: `&str`을 `String`으로 변환하려면 `.to_string()`이나 `String::from()`을 사용하세요.
- **빌림 검사기(Borrow Checker)와 싸우지 마세요**: 억지로 이기려 하지 마세요.
    - **해결책**: 빌림 규칙에 맞게 코드를 재구조화하세요.
    - 복잡한 공유 시나리오에서는 `Rc<RefCell<T>>` 사용을 고려해 보세요 (절제해서 사용).

## 에러 처리 예시: 좋은 예 vs 나쁜 예
```rust
// [에러] 나쁜 예: 예상치 못하게 패닉이 발생할 수 있음
fn bad_config_reader() -> String {
    let config = std::env::var("CONFIG_FILE").unwrap(); // 설정되지 않았다면 패닉!
    std::fs::read_to_string(config).unwrap()           // 파일이 없다면 패닉!
}

// [OK] 좋은 예: 에러를 우아하게 처리함
fn good_config_reader() -> Result<String, ConfigError> {
    let config_path = std::env::var("CONFIG_FILE")
        .unwrap_or_else(|_| "default.conf".to_string()); // 기본값으로 대체
    
    let content = std::fs::read_to_string(config_path)
        .map_err(ConfigError::FileRead)?;                // 에러 변환 및 전파
    
    Ok(content)
}

// [OK] 더 좋은 예: 적절한 에러 타입 정의
use thiserror::Error;

#[derive(Error, Debug)]
enum ConfigError {
    #[error("설정 파일을 읽지 못했습니다: {0}")]
    FileRead(#[from] std::io::Error),
    
    #[error("유효하지 않은 설정: {message}")]
    Invalid { message: String },
}
```

여기서 일어나는 일을 분석해 보겠습니다. `ConfigError`는 I/O 에러와 유효성 검사 에러라는 **두 가지 변형(variant)**만 가지고 있습니다. 이는 대부분의 모듈에서 적절한 시작점입니다.

| `ConfigError` 변형 | 데이터 | 생성 방법 |
|----------------------|-------|-----------|
| `FileRead(io::Error)` | 원본 I/O 에러 | `#[from]`이 `?`를 통한 자동 변환 생성 |
| `Invalid { message }` | 사람이 읽을 수 있는 설명 | 사용자의 유효성 검사 코드 |

이제 `Result<T, ConfigError>`를 반환하는 함수를 작성할 수 있습니다:

```rust
fn read_config(path: &str) -> Result<String, ConfigError> {
    let content = std::fs::read_to_string(path)?;  // io::Error → ConfigError::FileRead
    if content.is_empty() {
        return Err(ConfigError::Invalid {
            message: "설정 파일이 비어 있습니다".to_string(),
        });
    }
    Ok(content)
}
```

> **🟢 자기 주도 학습 체크포인트:** 다음 질문에 답할 수 있는지 확인하세요:
> 1. 왜 `read_to_string` 호출 시 `?`가 작동하나요? (`#[from]`이 `impl From<io::Error> for ConfigError`를 생성하기 때문입니다)
> 2. 세 번째 변형인 `MissingKey(String)`를 추가하면 어떤 코드를 변경해야 하나요? (변형만 추가하면 됩니다. 기존 코드는 그대로 컴파일됩니다)

## 크레이트 레벨 에러 타입 및 Result 별칭(Alias)

프로젝트가 단일 파일을 넘어 확장됨에 따라, 여러 모듈 레벨의 에러들을 하나의 **크레이트 레벨 에러 타입**으로 결합하게 됩니다. 이는 실제 Rust 운영 환경에서의 표준 패턴입니다. 위의 `ConfigError`에서 확장해 보겠습니다.

실제 Rust 프로젝트에서 모든 크레이트(또는 중요한 모듈)는 자신만의 `Error` 열거형과 `Result` 타입 별칭을 정의합니다. 이는 관용적인 패턴으로, C++에서 라이브러리별 예외 계층 구조를 정의하고 `using Result = std::expected<T, Error>`를 사용하는 것과 유사합니다.

### 패턴 예시

```rust
// src/error.rs (또는 lib.rs 상단)
use thiserror::Error;

/// 이 크레이트에서 발생할 수 있는 모든 에러
#[derive(Error, Debug)]
pub enum Error {
    #[error("I/O 에러: {0}")]
    Io(#[from] std::io::Error),          // From을 통한 자동 변환

    #[error("JSON 파싱 에러: {0}")]
    Json(#[from] serde_json::Error),     // From을 통한 자동 변환

    #[error("유효하지 않은 센서 ID: {0}")]
    InvalidSensor(u32),                  // 도메인 특화 변형

    #[error("{ms}ms 후 타임아웃 발생")]
    Timeout { ms: u64 },
}

/// 크레이트 전체에서 사용할 Result 별칭 — 타이핑 수고를 덜어줍니다.
pub type Result<T> = core::result::Result<T, Error>;
```

### 함수를 단순화하는 방법

별칭이 없다면 다음과 같이 작성해야 합니다:

```rust
// 번거로움 — 에러 타입이 도처에 반복됨
fn read_sensor(id: u32) -> Result<f64, crate::Error> { ... }
fn parse_config(path: &str) -> Result<Config, crate::Error> { ... }
```

별칭을 사용하면:

```rust
// 깔끔함 — 단순히 `Result<T>`만 사용
use crate::{Error, Result};

fn read_sensor(id: u32) -> Result<f64> {
    if id > 128 {
        return Err(Error::InvalidSensor(id));
    }
    let raw = std::fs::read_to_string(format!("/dev/sensor/{id}"))?; // io::Error → Error::Io
    let value: f64 = raw.trim().parse()
        .map_err(|_| Error::InvalidSensor(id))?;
    Ok(value)
}
```

`Io`에 붙은 `#[from]` 속성은 다음 `impl`을 자동으로 생성합니다:

```rust
// thiserror의 #[from]에 의해 자동 생성됨
impl From<std::io::Error> for Error {
    fn from(source: std::io::Error) -> Self {
        Error::Io(source)
    }
}
```

이것이 `?`가 작동하는 원리입니다. 어떤 함수가 `std::io::Error`를 반환하고 여러분의 함수가 `Result<T>`(여러분이 만든 별칭)를 반환할 때, 컴파일러는 자동으로 `From::from()`을 호출하여 에러를 변환합니다.

### 모듈 레벨 에러 조합하기

규모가 큰 크레이트는 에러를 모듈별로 나누고 크레이트 루트에서 이들을 조합합니다:

```rust
// src/config/error.rs
#[derive(thiserror::Error, Debug)]
pub enum ConfigError {
    #[error("누락된 키: {0}")]
    MissingKey(String),
    #[error("'{key}'에 대한 유효하지 않은 값: {reason}")]
    InvalidValue { key: String, reason: String },
}

// src/error.rs (크레이트 레벨)
#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error(transparent)]               // Display 구현을 내부 에러에 위임
    Config(#[from] crate::config::ConfigError),

    #[error("I/O 에러: {0}")]
    Io(#[from] std::io::Error),
}
pub type Result<T> = core::result::Result<T, Error>;
```

호출자는 여전히 특정 설정 에러에 대해 매칭할 수 있습니다:

```rust
match result {
    Err(Error::Config(ConfigError::MissingKey(k))) => eprintln!("설정에 '{k}'를 추가하세요"),
    Err(e) => eprintln!("기타 에러: {e}"),
    Ok(v) => use_value(v),
}
```

### C++ 비교

| 개념 | C++ | Rust |
|---------|-----|------|
| 에러 계층 구조 | `class AppError : public std::runtime_error` | `#[derive(thiserror::Error)] enum Error { ... }` |
| 에러 반환 | `std::expected<T, Error>` 또는 `throw` | `fn foo() -> Result<T>` |
| 에러 변환 | 수동 `try/catch` + 재발생(rethrow) | `#[from]` + `?` — 상용구 없음 |
| Result 별칭 | `template<class T> using Result = std::expected<T, Error>;` | `pub type Result<T> = core::result::Result<T, Error>;` |
| 에러 메시지 | `what()` 재정의 | `#[error("...")]` — `Display` 구현으로 컴파일됨 |
