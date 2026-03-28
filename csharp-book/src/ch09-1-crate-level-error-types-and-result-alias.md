# 크레이트 수준 에러 타입과 Result 별칭

> **학습 목표:** 실전 Rust 개발에서 에러를 구조화하는 표준 패턴을 익힙니다. `thiserror`를 사용해 라이브러리용 커스텀 에러를 정의하고, `anyhow`를 사용해 애플리케이션에서 에러를 우아하게 전파하는 법, 그리고 `Result<T>` 별칭(Alias)으로 코드를 간결하게 유지하는 법을 배웁니다.

---

### 1. 실전 에러 구조화 패턴
모든 라이브러리나 크레이트는 자신만의 에러 타입을 가지는 것이 좋습니다. `thiserror` 크레이트를 쓰면 반복되는 코드 없이 깔끔하게 에러 열거형을 정의할 수 있습니다.

```rust
// [error.rs 예시]
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("데이터베이스 오류: {0}")]
    Database(#[from] sqlx::Error), // sqlx 에러를 내 에러로 자동 변환

    #[error("인증 실패: {0}")]
    Auth(String),

    #[error("찾을 수 없음: {item}")]
    NotFound { item: String },
}

// 크레이트 전용 Result 별칭 정의
pub type Result<T> = std::result::Result<T, MyError>;
```

---

### 2. `thiserror` vs `anyhow` (가장 중요한 선택 기준)

| **구분** | **`thiserror`** | **`anyhow`** |
| :--- | :--- | :--- |
| **주 사용처** | **라이브러리** 개발 시 | **애플리케이션(Binary)** 개발 시 |
| **핵심 목표** | 호출자에게 **구조화된** 에러 정보 제공 | 에러를 쉽고 빠르게 **전파 및 로깅** |
| **장점** | 에러 종류마다 다른 처리(`match`) 가능 | 어떤 에러든 하나로 묶어 처리 가능 |
| **예시** | `Result<T, MyError>` | `anyhow::Result<T>` |

---

### 3. 애플리케이션에서의 `anyhow` 활용
`anyhow`는 여러 종류의 라이브러리 에러를 하나로 묶어주며, `.context()`를 통해 에러가 어디서 왜 발생했는지 문맥 정보를 추가할 수 있습니다.

```rust
use anyhow::{Context, Result};

fn run_app() -> Result<()> {
    let config = std::fs::read_to_string("config.json")
        .context("설정 파일을 읽는 데 실패했습니다.")?; // 문맥 추가

    let settings: Settings = serde_json::from_str(&config)
        .context("JSON 파싱 에러가 발생했습니다.")?;
    
    Ok(())
}
```

---

### 💡 실무 팁: 에러 매핑 루틴
라이브러리 에러를 내 크레이트 에러로 바꿀 때, `map_err`을 쓰거나 `#[from]`을 활용하세요.
```rust
// map_err을 이용한 커스텀 매핑
let user = db.find_user(id).await
    .map_err(|_| MyError::NotFound { item: format!("User {}", id) })?;
```
이 패턴을 유지하면 모든 함수의 반환 타입이 `Result<T>`로 통일되어 코드가 매우 간결해집니다.

