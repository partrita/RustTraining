## C# 개발자를 위한 필수 크레이트 (Essential Crates for C# Developers)

> **학습 내용:** 공통 .NET 라이브러리에 대응하는 Rust 크레이트들 — serde (JSON.NET),
> reqwest (HttpClient), tokio (Task/async), sqlx (Entity Framework)를 알아보고,
> `System.Text.Json`과 비교하여 serde의 속성 시스템을 심층적으로 분석합니다.
>
> **난이도:** 🟡 중급

### 핵심 기능별 대응 크레이트 (Core Functionality Equivalents)

```rust
// C# 개발자를 위한 Cargo.toml 의존성 목록
[dependencies]
# 직렬화 (Newtonsoft.Json 또는 System.Text.Json과 유사)
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# HTTP 클라이언트 (HttpClient와 유사)
reqwest = { version = "0.11", features = ["json"] }

# 비동기 런타임 (Task.Run, async/await와 유사)
tokio = { version = "1.0", features = ["full"] }

# 에러 처리 (커스텀 예외와 유사)
thiserror = "1.0"
anyhow = "1.0"

# 로깅 (ILogger, Serilog와 유사)
log = "0.4"
env_logger = "0.10"

# 날짜/시간 (DateTime과 유사)
chrono = { version = "0.4", features = ["serde"] }

# UUID (System.Guid와 유사)
uuid = { version = "1.0", features = ["v4", "serde"] }

# 컬렉션 (List<T>, Dictionary<K,V>와 유사)
# std에 내장되어 있지만, 고급 컬렉션이 필요한 경우:
indexmap = "2.0"  # 순서가 보장되는 HashMap

# 설정 관리 (IConfiguration과 유사)
config = "0.13"

# 데이터베이스 (Entity Framework와 유사)
sqlx = { version = "0.7", features = ["runtime-tokio-rustls", "postgres", "uuid", "chrono"] }

# 테스트 (xUnit, NUnit과 유사)
# std에 내장되어 있지만, 더 많은 기능이 필요한 경우:
rstest = "0.18"  # 매개변수화된 테스트(Parameterized tests)

# 모킹 (Moq와 유사)
mockall = "0.11"

# 병렬 처리 (Parallel.ForEach와 유사)
rayon = "1.7"
```

### 예시 활용 패턴

```rust
use serde::{Deserialize, Serialize};
use reqwest;
use tokio;
use thiserror::Error;
use chrono::{DateTime, Utc};
use uuid::Uuid;

// 데이터 모델 (속성이 지정된 C# POCO와 유사)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub created_at: DateTime<Utc>,
}

// 커스텀 에러 타입 (커스텀 예외와 유사)
#[derive(Error, Debug)]
pub enum ApiError {
    #[error("HTTP 요청 실패: {0}")]
    Http(#[from] reqwest::Error),
    
    #[error("직렬화 실패: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("사용자를 찾을 수 없음: {id}")]
    UserNotFound { id: Uuid },
    
    #[error("유효성 검사 실패: {message}")]
    Validation { message: String },
}

// 서비스 클래스 대응물
pub struct UserService {
    client: reqwest::Client,
    base_url: String,
}

impl UserService {
    pub fn new(base_url: String) -> Self {
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("HTTP 클라이언트 생성 실패");
            
        UserService { client, base_url }
    }
    
    // 비동기 메서드 (C#의 async Task<User>와 유사)
    pub async fn get_user(&self, id: Uuid) -> Result<User, ApiError> {
        let url = format!("{}/users/{}", self.base_url, id);
        
        let response = self.client
            .get(&url)
            .send()
            .await?;
        
        if response.status() == 404 {
            return Err(ApiError::UserNotFound { id });
        }
        
        let user = response.json::<User>().await?;
        Ok(user)
    }
    
    // 사용자 생성
    pub async fn create_user(&self, name: String, email: String) -> Result<User, ApiError> {
        if name.trim().is_empty() {
            return Err(ApiError::Validation {
                message: "이름은 비어 있을 수 없습니다".to_string(),
            });
        }
        
        let new_user = User {
            id: Uuid::new_v4(),
            name,
            email,
            created_at: Utc::now(),
        };
        
        let response = self.client
            .post(&format!("{}/users", self.base_url))
            .json(&new_user)
            .send()
            .await?;
        
        let created_user = response.json::<User>().await?;
        Ok(created_user)
    }
}

// 실행 예시 (C#의 Main 메서드와 유사)
#[tokio::main]
async fn main() -> Result<(), ApiError> {
    // 로깅 초기화 (ILogger 설정과 유사)
    env_logger::init();
    
    let service = UserService::new("https://api.example.com".to_string());
    
    // 사용자 생성
    let user = service.create_user(
        "John Doe".to_string(),
        "john@example.com".to_string(),
    ).await?;
    
    println!("생성된 사용자: {:?}", user);
    
    // 사용자 조회
    let retrieved_user = service.get_user(user.id).await?;
    println!("조회된 사용자: {:?}", retrieved_user);
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]  // C#의 [Test] 또는 [Fact]와 유사
    async fn test_user_creation() {
        let service = UserService::new("http://localhost:8080".to_string());
        
        let result = service.create_user(
            "Test User".to_string(),
            "test@example.com".to_string(),
        ).await;
        
        assert!(result.is_ok());
        let user = result.unwrap();
        assert_eq!(user.name, "Test User");
        assert_eq!(user.email, "test@example.com");
    }
    
    #[test]
    fn test_validation() {
        // 동기 테스트
        let error = ApiError::Validation {
            message: "부적절한 입력".to_string(),
        };
        
        assert_eq!(error.to_string(), "유효성 검사 실패: 부적절한 입력");
    }
}
```

***

## Serde 심층 분석: C# 개발자를 위한 JSON 직렬화 (Serde Deep Dive: JSON Serialization for C# Developers)

C# 개발자들은 `System.Text.Json` 또는 `Newtonsoft.Json`에 크게 의존합니다. Rust에서는 **serde**(serialize/deserialize의 약자)가 보편적인 프레임워크입니다. serde의 속성(attribute) 시스템을 이해하면 대부분의 데이터 처리 시나리오를 해결할 수 있습니다.

### 기본 Derive: 시작점
```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct User {
    name: String,
    age: u32,
    email: String,
}

let user = User { name: "Alice".into(), age: 30, email: "alice@co.com".into() };
let json = serde_json::to_string_pretty(&user)?;
let parsed: User = serde_json::from_str(&json)?;
```

```csharp
// 대응하는 C# 코드
public class User
{
    public string Name { get; set; }
    public int Age { get; set; }
    public string Email { get; set; }
}
var json = JsonSerializer.Serialize(user, new JsonSerializerOptions { WriteIndented = true });
var parsed = JsonSerializer.Deserialize<User>(json);
```

### 필드 레벨 속성 (`[JsonProperty]`와 유사)

```rust
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct ApiResponse {
    // JSON 출력 시 필드 이름 변경 ([JsonPropertyName("user_id")]와 유사)
    #[serde(rename = "user_id")]
    id: u64,

    // 직렬화와 역직렬화 시 서로 다른 이름 사용
    #[serde(rename(serialize = "userName", deserialize = "user_name"))]
    name: String,

    // 이 필드를 완전히 무시 ([JsonIgnore]와 유사)
    #[serde(skip)]
    internal_cache: Option<String>,

    // 직렬화 시에만 무시
    #[serde(skip_serializing)]
    password_hash: String,

    // JSON에 데이터가 없을 경우 기본값 사용 (기본 생성자 값과 유사)
    #[serde(default)]
    is_active: bool,

    // 커스텀 기본값 지정
    #[serde(default = "default_role")]
    role: String,

    // 중첩된 구조체를 부모 구조체에 펼쳐서 넣기 ([JsonExtensionData]와 유사)
    #[serde(flatten)]
    metadata: Metadata,

    // 값이 None인 경우 무시 (null 필드 제외)
    #[serde(skip_serializing_if = "Option::is_none")]
    nickname: Option<String>,
}

fn default_role() -> String { "viewer".into() }

#[derive(Serialize, Deserialize, Debug)]
struct Metadata {
    created_at: String,
    version: u32,
}
```

```csharp
// 대응하는 C# 속성들
public class ApiResponse
{
    [JsonPropertyName("user_id")]
    public ulong Id { get; set; }

    [JsonIgnore]
    public string? InternalCache { get; set; }

    [JsonExtensionData]
    public Dictionary<string, JsonElement>? Metadata { get; set; }
}
```

### 열거형 표현 방식 (C#과의 결정적 차이점)

Rust의 serde는 열거형(enum)에 대해 **네 가지 다른 JSON 표현 방식**을 지원합니다. C# 열거형은 항상 정수나 문자열로 표현되므로, 이는 C#에 직접적인 대응 개념이 없는 부분입니다.

```rust
use serde::{Deserialize, Serialize};

// 1. 외부 태그 방식 (Externally tagged, 기본값) — 가장 일반적
#[derive(Serialize, Deserialize)]
enum Message {
    Text(String),
    Image { url: String, width: u32 },
    Ping,
}
// Text 변체:  {"Text": "hello"}
// Image 변체: {"Image": {"url": "...", "width": 100}}
// Ping 변체:  "Ping"

// 2. 내부 태그 방식 (Internally tagged) — 다른 언어의 구별된 공용체(discriminated unions)와 유사
#[derive(Serialize, Deserialize)]
#[serde(tag = "type")]
enum Event {
    Created { id: u64, name: String },
    Deleted { id: u64 },
    Updated { id: u64, fields: Vec<String> },
}
// {"type": "Created", "id": 1, "name": "Alice"}
// {"type": "Deleted", "id": 1}

// 3. 인접 태그 방식 (Adjacently tagged) — 태그와 내용이 별도 필드에 위치
#[derive(Serialize, Deserialize)]
#[serde(tag = "t", content = "c")]
enum ApiResult {
    Success(UserData),
    Error(String),
}
// {"t": "Success", "c": {"name": "Alice"}}
// {"t": "Error", "c": "not found"}

// 4. 태그 없음 방식 (Untagged) — serde가 각 변체를 순서대로 시도
#[derive(Serialize, Deserialize)]
#[serde(untagged)]
enum FlexibleValue {
    Integer(i64),
    Float(f64),
    Text(String),
    Bool(bool),
}
// 42, 3.14, "hello", true — serde가 자동으로 변체를 감지함
```

### 커스텀 직렬화 (`JsonConverter`와 유사)
```rust
use serde::{Deserialize, Deserializer, Serialize, Serializer};

// 특정 필드에 대한 커스텀 직렬화
#[derive(Serialize, Deserialize)]
struct Config {
    #[serde(serialize_with = "serialize_duration", deserialize_with = "deserialize_duration")]
    timeout: std::time::Duration,
}

fn serialize_duration<S: Serializer>(dur: &std::time::Duration, s: S) -> Result<S::Ok, S::Error> {
    s.serialize_u64(dur.as_millis() as u64)
}

fn deserialize_duration<'de, D: Deserializer<'de>>(d: D) -> Result<std::time::Duration, D::Error> {
    let ms = u64::deserialize(d)?;
    Ok(std::time::Duration::from_millis(ms))
}
// JSON: {"timeout": 5000}  ↔  Config { timeout: Duration::from_millis(5000) }
```

### 컨테이너 레벨 속성 (Container-Level Attributes)

```rust
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]  // 모든 필드를 JSON에서 camelCase로 변경
struct UserProfile {
    first_name: String,      // → "firstName"
    last_name: String,       // → "lastName"
    email_address: String,   // → "emailAddress"
}

#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields)]  // 알려지지 않은 필드가 포함된 JSON 거부 (엄격한 파싱)
struct StrictConfig {
    port: u16,
    host: String,
}
// serde_json::from_str::<StrictConfig>(r#"{"port":8080,"host":"localhost","extra":true}"#)
// → 에러: unknown field `extra`
```

### 빠른 참조: Serde 속성 (Quick Reference: Serde Attributes)

| 속성 | 적용 수준 | C# 대응 개념 | 용도 |
|-----------|-------|---------------|---------|
| `#[serde(rename = "...")]` | 필드 | `[JsonPropertyName]` | JSON 내 이름 변경 |
| `#[serde(skip)]` | 필드 | `[JsonIgnore]` | 완전히 생략 |
| `#[serde(default)]` | 필드 | 기본값 | 누락 시 `Default::default()` 사용 |
| `#[serde(flatten)]` | 필드 | `[JsonExtensionData]` | 중첩 구조체를 부모에 펼치기 |
| `#[serde(skip_serializing_if = "...")]` | 필드 | `JsonIgnoreCondition` | 조건부 생략 |
| `#[serde(rename_all = "camelCase")]` | 컨테이너 | `PropertyNamingPolicy` | 명명 규칙 일괄 적용 |
| `#[serde(deny_unknown_fields)]` | 컨테이너 | — | 엄격한 역직렬화 |
| `#[serde(tag = "type")]` | 열거형 | 식별자 패턴 | 내부 태그 방식 |
| `#[serde(untagged)]` | 열거형 | — | 변체 순서대로 시도 |
| `#[serde(with = "...")]` | 필드 | `[JsonConverter]` | 커스텀 직렬화/역직렬화 |

### JSON 그 이상의 활용: Serde는 어디에나 존재합니다 (Beyond JSON: serde Works Everywhere)
```rust
// 동일한 derive가 모든 포맷에 작동합니다 — 크레이트만 바꾸면 됩니다.
let user = User { name: "Alice".into(), age: 30, email: "a@b.com".into() };

let json  = serde_json::to_string(&user)?;        // JSON
let toml  = toml::to_string(&user)?;               // TOML (설정 파일)
let yaml  = serde_yaml::to_string(&user)?;          // YAML
let cbor  = serde_cbor::to_vec(&user)?;             // CBOR (바이너리, 압축)
let msgpk = rmp_serde::to_vec(&user)?;              // MessagePack (바이너리)

// 한 번의 #[derive(Serialize, Deserialize)]로 모든 포맷을 자유롭게 사용하세요.
```

***
