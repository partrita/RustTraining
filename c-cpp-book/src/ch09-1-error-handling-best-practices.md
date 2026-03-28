# Rust 에러 처리 베스트 프랙티스

> **학습 목표:** 현업에서 사용하는 관용적인(Idiomatic) 에러 처리 패턴을 마스터합니다. `unwrap()`의 안전한 대안들을 익히고, 커스텀 에러 타입을 정의하는 표준 라이브러리 방식과 `thiserror` 같은 외부 크레이트 활용법을 배웁니다. 또한 대규모 프로젝트에서 에러를 체계적으로 조직화하는 기법을 알아봅니다.

---

### 1. `unwrap()`의 안전한 대안들
코드가 갑자기 중단되는 `unwrap()` 대신, 상황에 맞는 안전한 메서드를 사용하세요.

- **`Option<T>`를 처리할 때**
  - `opt.unwrap_or(default)`: 값이 없으면 지정한 기본값 사용
  - `opt.unwrap_or_else(|| compute())`: 기본값을 계산하는 비용이 클 때 클로저 활용
  - `opt.unwrap_or_default()`: 해당 타입의 기본값(0, 빈 문자열 등) 사용
  - `opt.expect("메시지")`: 패닉이 발생해도 무방한 상황에서 사유를 명시

- **`Result<T, E>`를 처리할 때**
  - `res.unwrap_or(fallback)`: 에러를 무시하고 대체값 사용
  - `res.unwrap_or_else(|e| handle(e))`: 에러 발생 시 로그를 남기거나 복잡한 처리 후 대체값 반환

### 2. 함수형 에러 변환
에러를 단순히 전파하는 것을 넘어, 값을 다른 형태로 가공하거나 타입을 변경할 때 유용한 도구들입니다.

- **`map(f)`**: 성공 시의 결과값을 변환합니다. (`Ok(T)` -> `Ok(U)`)
- **`map_err(f)`**: 에러 타입만 다른 종류로 바꿉니다. (`Err(E)` -> `Err(F)`)
- **`and_then(f)`**: 성공 시 다음 '실패할 수 있는 작업'을 연결합니다. (모나딕 바인딩)

---

# 🚀 실전 에러 관리 패턴: `thiserror` 활용

라이브러리나 규모 있는 프로젝트에서는 에러 사유를 명확히 구분하기 위해 전용 `enum`을 정의합니다. `thiserror` 크레이트는 이 과정을 매우 간결하게 만들어 줍니다.

### 에러 타입 정의 예시

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("설정 파일을 읽을 수 없습니다: {0}")]
    FileRead(#[from] std::io::Error), // io 에러를 자동으로 변환해서 수용함
    
    #[error("유효하지 않은 설정 포맷: {message}")]
    InvalidFormat { message: String },
    
    #[error("필수 키 '{0}'가 누락되었습니다.")]
    MissingKey(String),
}

// Result 별칭(Alias) 정의 - 타이핑 수고를 크게 덜어줍니다.
pub type Result<T> = std::result::Result<T, ConfigError>;
```

### 함수에서의 활용

```rust
fn load_config(path: &str) -> Result<String> {
    // '?' 가 io::Error를 ConfigError::FileRead로 자동 변환합니다 (#[from] 덕분)
    let content = std::fs::read_to_string(path)?; 
    
    if content.is_empty() {
        return Err(ConfigError::InvalidFormat {
            message: "파일 내용이 비어 있습니다.".into(),
        });
    }
    
    Ok(content)
}
```

---

# 💡 흔히 발생하는 함정과 해결책

1.  **빌림 검사기(Borrow Checker)와의 충돌**
    - **통상적인 메시지**: "cannot borrow as mutable...", "does not live long enough"
    - **해결책**: 변수의 스코프 `{}`를 좁혀서 참조자의 수명을 단축하거나, 소유권이 필요한 경우 `.clone()`을 활용하여 독자적인 데이터를 만드세요.
2.  **문자열 타입 혼동 (`String` vs `&str`)**
    - **차이**: `&str`은 데이터의 일부분을 가리키는 포인터(슬라이스)이고, `String`은 메모리를 직접 소유한 동적 버퍼입니다.
    - **해결책**: 필요한 타입에 맞춰 `.to_string()`이나 `String::from()`으로 변환하세요.
3.  **정수 오버플로 (Integer Overflow)**
    - **특징**: Rust는 디버그 모드에서 오버플로 발생 시 패닉을 일으켜 잠재적 버그를 잡아줍니다.
    - **해결책**: 의도된 동작이라면 `wrapping_add()`, `checked_add()`, `saturating_add()` 등을 명시적으로 사용하세요.

---

### 대규모 프로젝트의 에러 조직화: 에러 투명화(Transparent)
여러 하위 모듈의 에러를 상위 에러 타입으로 묶을 때 `#[error(transparent)]`를 쓰면 내부 에러의 메시지를 그대로 노출할 수 있습니다.

```rust
#[derive(Error, Debug)]
pub enum AppError {
    #[error(transparent)]
    Config(#[from] ConfigError), // ConfigError의 메시지를 그대로 전달

    #[error("네트워크 연결 실패: {0}")]
    Network(#[from] reqwest::Error),
}

pub type AppResult<T> = std::result::Result<T, AppError>;
```

---

### C++ 대비 Rust 에러 처리 요약

| **구분** | **C++ 방식** | **Rust 방식** | **비고** |
| :--- | :--- | :--- | :--- |
| **에러 계층** | `class Error : public runtime_error` | `#[derive(Error)] enum Error { ... }` | 상속 대신 조합(Composition) 활용 |
| **에러 반환** | `throw / std::expected<T, E>` | **`Result<T, E>` (반환값)** | 에러가 함수 시그니처의 일부임 |
| **자동 변환** | 수동 `try-catch` 후 재발생(Re-throw) | **`#[from]` + `?` 연산자** | 중복 코드가 거의 없음 |
| **메시지 정의** | `what()` 메서드 재정의 | **`#[error("...")]` 속성** | 가독성 높은 선언적 메시지 관리 |

---

# ✅ 학습 체크리스트
- [ ] 에러를 무시하지 않고 `match`나 `if let`으로 반드시 처리하고 있는가?
- [ ] `unwrap()` 대신 `unwrap_or_else`와 같은 안전한 대안을 우선적으로 고려하는가?
- [ ] 반복되는 `Result<T, MyError>`를 줄이기 위해 `type Result<T> = ...` 별칭을 사용하고 있는가?
- [ ] 외부 라이브러리 에러를 내 에러 타입으로 변환할 때 `#[from]`을 활용하고 있는가?
