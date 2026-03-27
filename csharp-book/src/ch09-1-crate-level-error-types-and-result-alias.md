## 크레이트 수준 에러 타입과 Result 별칭

> **학습 내용:** `thiserror`를 사용하여 크레이트별 에러 열거형(enum)을 정의하는 실전 패턴, `Result<T>` 타입 별칭 생성 방법, 그리고 라이브러리 개발(`thiserror`)과 애플리케이션 개발(`anyhow`)에서의 선택 기준.
>
> **난이도:** 🟡 중급

실무 수준의 Rust 개발을 위한 핵심 패턴은 크레이트별 에러 열거형을 정의하고 `Result` 타입 별칭(alias)을 만들어 반복되는 코드를 제거하는 것입니다.

### 권장 패턴
```rust
// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("데이터베이스 에러: {0}")]
    Database(#[from] sqlx::Error),

    #[error("HTTP 에러: {0}")]
    Http(#[from] reqwest::Error),

    #[error("직렬화 에러: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("검증 에러: {message}")]
    Validation { message: String },

    #[error("찾을 수 없음: {entity} (ID: {id})")]
    NotFound { entity: String, id: String },
}

/// 크레이트 전체에서 사용할 Result 별칭 — 모든 함수는 이 타입을 반환합니다.
pub type Result<T> = std::result::Result<T, AppError>;
```

### 크레이트 전체에서의 활용
```rust
use crate::error::{AppError, Result};

pub async fn get_user(id: Uuid) -> Result<User> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_optional(&pool)
        .await?;  // sqlx::Error → #[from]에 의해 AppError::Database로 자동 변환

    user.ok_or_else(|| AppError::NotFound {
        entity: "User".into(),
        id: id.to_string(),
    })
}

pub async fn create_user(req: CreateUserRequest) -> Result<User> {
    if req.name.trim().is_empty() {
        return Err(AppError::Validation {
            message: "이름은 비어 있을 수 없습니다".into(),
        });
    }
    // ...
}
```

### C#과의 비교
```csharp
// C#에서의 유사한 패턴
public class AppException : Exception
{
    public string ErrorCode { get; }
    public AppException(string code, string message) : base(message)
    {
        ErrorCode = code;
    }
}

// 하지만 C#에서는 호출자가 어떤 예외가 발생할지 미리 알기 어렵습니다.
// Rust에서는 에러 타입이 함수의 시그니처에 명시적으로 드러납니다.
```

### 이 패턴이 중요한 이유
- **`thiserror`**는 `Display`와 `Error` 구현을 자동으로 생성해 줍니다.
- **`#[from]`**을 사용하면 `?` 연산자가 라이브러리 에러를 자동으로 변환할 수 있습니다.
- `Result<T>` 별칭 덕분에 모든 함수 시그니처가 깔끔해집니다: `fn foo() -> Result<Bar>`
- **C# 예외와 달리**, 호출자는 타입 시스템을 통해 발생 가능한 모든 에러 변이를 확인할 수 있습니다.


### thiserror vs anyhow: 어떤 것을 언제 사용할까요?

Rust 에러 처리에서 가장 많이 쓰이는 두 크레이트입니다. 이들 중 하나를 선택하는 것이 에러 처리 전략의 첫걸음입니다.

| | `thiserror` | `anyhow` |
|---|---|---|
| **용도** | **라이브러리**를 위한 구조화된 에러 타입 정의 | **애플리케이션**을 위한 빠른 에러 처리 |
| **출력** | 직접 제어하는 커스텀 열거형 | 불투명한 `anyhow::Error` 래퍼 |
| **호출자 시점** | 타입 내의 모든 에러 변이를 확인 가능 | `anyhow::Error`만 보임 (내용 확인 어려움) |
| **적합한 곳** | 라이브러리 크레이트, API, 다른 코드가 호출하는 기능 | 바이너리, 스크립트, 프로토타입, CLI 도구 |
| **다운캐스팅** | 변이(variant)에 직접 `match` 사용 | `error.downcast_ref::<MyError>()` 사용 |

```rust
// thiserror — 라이브러리용 (호출자가 에러 변이에 따른 처리가 필요한 경우)
use thiserror::Error;

#[derive(Error, Debug)]
pub enum StorageError {
    #[error("파일을 찾을 수 없음: {path}")]
    NotFound { path: String },

    #[error("권한 거부됨: {0}")]
    PermissionDenied(String),

    #[error(transparent)]
    Io(#[from] std::io::Error),
}

pub fn read_config(path: &str) -> Result<String, StorageError> {
    std::fs::read_to_string(path).map_err(|e| match e.kind() {
        std::io::ErrorKind::NotFound => StorageError::NotFound { path: path.into() },
        std::io::ErrorKind::PermissionDenied => StorageError::PermissionDenied(path.into()),
        _ => StorageError::Io(e),
    })
}
```

```rust
// anyhow — 애플리케이션용 (에러를 정의하기보다 전달하고 로깅하는 데 집중)
use anyhow::{Context, Result};

fn main() -> Result<()> {
    let config = std::fs::read_to_string("config.toml")
        .context("설정 파일을 읽지 못했습니다")?;

    let port: u16 = config.parse()
        .context("포트 번호 파싱에 실패했습니다")?;

    println!("포트 {port}에서 대기 중");
    Ok(())
}
// anyhow::Result<T> = Result<T, anyhow::Error>와 같습니다.
// .context()는 에러에 사람이 읽기 쉬운 문맥 정보를 추가합니다.
```

```csharp
// C# 비교:
// thiserror ≈ 특정 속성을 가진 커스텀 예외 클래스 정의
// anyhow ≈ Exception을 잡아서 메시지와 함께 래핑하는 방식:
//   throw new InvalidOperationException("설정 읽기 실패", ex);
```

**가이드라인**: 작성 중인 코드가 **라이브러리**(다른 코드가 호출함)라면 `thiserror`를 사용하십시오. 작성 중인 코드가 **애플리케이션**(최종 실행 파일)이라면 `anyhow`를 사용하십시오. 많은 프로젝트가 라이브러리 크레이트의 공개 API에는 `thiserror`를, `main()` 실행부에는 `anyhow`를 혼합해서 사용하기도 합니다.

### 에러 복구 패턴

C# 개발자들은 특정 예외를 복구하기 위해 `try/catch` 블록을 사용하는 데 익숙합니다. Rust에서는 동일한 목적을 위해 `Result`의 조합기(combinators)를 사용합니다.

```rust
use std::fs;

// 패턴 1: 기본값으로 복구하기
let config = fs::read_to_string("config.toml")
    .unwrap_or_else(|_| String::from("port = 8080"));  // 파일이 없으면 기본값 사용

// 패턴 2: 특정 에러만 복구하고 나머지는 전파하기
fn read_or_create(path: &str) -> Result<String, std::io::Error> {
    match fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
            let default = String::from("# 새 파일");
            fs::write(path, &default)?;
            Ok(default)
        }
        Err(e) => Err(e),  // 권한 에러 등은 상위로 전파
    }
}

// 패턴 3: 전파하기 전에 문맥 추가하기
use anyhow::Context;

fn load_config() -> anyhow::Result<Config> {
    let text = fs::read_to_string("config.toml")
        .context("config.toml 파일을 읽는 중 에러 발생")?;
    let config: Config = toml::from_str(&text)
        .context("config.toml 파싱 중 에러 발생")?;
    Ok(config)
}

// 패턴 4: 에러를 도메인 타입으로 매핑하기
fn parse_port(s: &str) -> Result<u16, AppError> {
    s.parse::<u16>()
        .map_err(|_| AppError::Validation {
            message: format!("유효하지 않은 포트: {s}"),
        })
}
```

```csharp
// C# 대응 코드:
try { config = File.ReadAllText("config.toml"); }
catch (FileNotFoundException) { config = "port = 8080"; }  // 패턴 1

try { /* ... */ }
catch (FileNotFoundException) { /* 파일 생성 */ }         // 패턴 2
catch { throw; }                                          // 나머지 재전파
```

**복구 vs 전파 결정 기준:**
- **복구**: 합리적인 기본값이나 재시도 전략이 있을 때 사용하십시오.
- **전파(`?`)**: *호출자*가 어떻게 처리할지 결정해야 할 때 사용하십시오.
- **문맥 추가(`.context()`)**: 모듈의 경계에서 에러 추적 경로를 남기고 싶을 때 사용하십시오.

---

## 연습 문제

<details>
<summary><strong>🏋️ 연습 문제: 크레이트 에러 타입 설계하기</strong> (클릭하여 확장)</summary>

사용자 등록 서비스를 구축한다고 가정하고, `thiserror`를 사용하여 에러 타입을 설계하십시오.

1. `RegistrationError` 열거형에 다음 변이를 정의하십시오: `DuplicateEmail(String)`, `WeakPassword(String)`, `DatabaseError(#[from] sqlx::Error)`, `RateLimited { retry_after_secs: u64 }`
2. `type Result<T> = std::result::Result<T, RegistrationError>;` 별칭을 만드십시오.
3. `?` 전파와 명시적인 에러 생성을 보여주는 `register_user(email: &str, password: &str) -> Result<()>` 함수를 작성하십시오.

<details>
<summary>🔑 정답</summary>

```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("이미 등록된 이메일: {0}")]
    DuplicateEmail(String),

    #[error("비밀번호가 취약함: {0}")]
    WeakPassword(String),

    #[error("데이터베이스 에러")]
    Database(#[from] sqlx::Error),

    #[error("요청 제한 — {retry_after_secs}초 후 재시도")]
    RateLimited { retry_after_secs: u64 },
}

pub type Result<T> = std::result::Result<T, RegistrationError>;

pub fn register_user(email: &str, password: &str) -> Result<()> {
    if password.len() < 8 {
        return Err(RegistrationError::WeakPassword(
            "최소 8자 이상이어야 합니다".into(),
        ));
    }

    // 이 ?는 sqlx::Error를 RegistrationError::Database로 자동 변환합니다.
    // db.check_email_unique(email).await?;

    // 도메인 로직에 따른 명시적인 에러 생성 예시
    if email.contains("+spam") {
        return Err(RegistrationError::DuplicateEmail(email.to_string()));
    }

    Ok(())
}
```

**핵심 패턴**: 라이브러리 에러에는 `#[from]`을 사용하여 `?`를 활용하고, 도메인 로직에는 명시적으로 `Err(...)`를 사용하십시오. Result 별칭을 쓰면 모든 함수 시그니처가 깔끔하게 유지됩니다.

</details>
</details>

***
