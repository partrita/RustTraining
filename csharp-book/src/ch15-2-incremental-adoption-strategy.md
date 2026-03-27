## 단계별 도입 전략 (Incremental Adoption Strategy)

> **학습 내용:** C#/.NET 기반 조직에 Rust를 도입하기 위한 단계별 접근 방식 —
> 학습 예제(1~4주차)부터 성능 최우선 컴포넌트 교체(5~8주차), 
> 그리고 새로운 마이크로서비스 구축(9~12주차)까지 팀의 도입 일정을 구체적으로 살펴봅니다.
>
> **난이도:** 🟡 중급

### 1단계: 학습 및 실험 (Weeks 1-4)
```rust
// 명령줄 도구 및 유틸리티부터 시작하세요
// 예시: 로그 파일 분석기
use std::fs;
use std::collections::HashMap;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    #[arg(short, long)]
    file: String,
    
    #[arg(short, long, default_value = "10")]
    top: usize,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    
    let content = fs::read_to_string(&args.file)?;
    let mut word_count = HashMap::new();
    
    for line in content.lines() {
        for word in line.split_whitespace() {
            let word = word.to_lowercase();
            *word_count.entry(word).or_insert(0) += 1;
        }
    }
    
    let mut sorted: Vec<_> = word_count.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1));
    
    for (word, count) in sorted.into_iter().take(args.top) {
        println!("{}: {}", word, count);
    }
    
    Ok(())
}
```

### 2단계: 성능 최우선 컴포넌트 교체 (Weeks 5-8)
```rust
// CPU 집약적인 데이터 처리 로직을 교체하세요
// 예시: 이미지 처리 마이크로서비스
use image::{DynamicImage, ImageBuffer, Rgb};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use warp::Filter;

#[derive(Serialize, Deserialize)]
struct ProcessingRequest {
    image_data: Vec<u8>,
    operation: String,
    parameters: serde_json::Value,
}

#[derive(Serialize)]
struct ProcessingResponse {
    processed_image: Vec<u8>,
    processing_time_ms: u64,
}

async fn process_image(request: ProcessingRequest) -> Result<ProcessingResponse, Box<dyn std::error::Error + Send + Sync>> {
    let start = std::time::Instant::now();
    
    let img = image::load_from_memory(&request.image_data)?;
    
    let processed = match request.operation.as_str() {
        "blur" => {
            let radius = request.parameters["radius"].as_f64().unwrap_or(2.0) as f32;
            img.blur(radius)
        }
        "grayscale" => img.grayscale(),
        "resize" => {
            let width = request.parameters["width"].as_u64().unwrap_or(100) as u32;
            let height = request.parameters["height"].as_u64().unwrap_or(100) as u32;
            img.resize(width, height, image::imageops::FilterType::Lanczos3)
        }
        _ => return Err("Unknown operation".into()),
    };
    
    let mut buffer = Vec::new();
    processed.write_to(&mut std::io::Cursor::new(&mut buffer), image::ImageOutputFormat::Png)?;
    
    Ok(ProcessingResponse {
        processed_image: buffer,
        processing_time_ms: start.elapsed().as_millis() as u64,
    })
}

#[tokio::main]
async fn main() {
    let process_route = warp::path("process")
        .and(warp::post())
        .and(warp::body::json())
        .and_then(|req: ProcessingRequest| async move {
            match process_image(req).await {
                Ok(response) => Ok(warp::reply::json(&response)),
                Err(e) => Err(warp::reject::custom(ProcessingError(e.to_string()))),
            }
        });

    warp::serve(process_route)
        .run(([127, 0, 0, 1], 3030))
        .await;
}

#[derive(Debug)]
struct ProcessingError(String);
impl warp::reject::Reject for ProcessingError {}
```

### 3단계: 새로운 마이크로서비스 (Weeks 9-12)
```rust
// Rust로 새로운 서비스를 처음부터 구축해 보세요
// 예시: 인증 서비스
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use sqlx::{Pool, Postgres};
use uuid::Uuid;
use bcrypt::{hash, verify, DEFAULT_COST};

#[derive(Clone)]
struct AppState {
    db: Pool<Postgres>,
    jwt_secret: String,
}

#[derive(Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

#[derive(Deserialize)]
struct LoginRequest {
    email: String,
    password: String,
}

#[derive(Serialize)]
struct LoginResponse {
    token: String,
    user_id: Uuid,
}

async fn login(
    State(state): State<AppState>,
    Json(request): Json<LoginRequest>,
) -> Result<Json<LoginResponse>, StatusCode> {
    let user = sqlx::query!(
        "SELECT id, password_hash FROM users WHERE email = $1",
        request.email
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let user = user.ok_or(StatusCode::UNAUTHORIZED)?;

    if !verify(&request.password, &user.password_hash)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
    {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let claims = Claims {
        sub: user.id.to_string(),
        exp: (chrono::Utc::now() + chrono::Duration::hours(24)).timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_ref()),
    )
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(LoginResponse {
        token,
        user_id: user.id,
    }))
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = std::env::var("DATABASE_URL")?;
    let jwt_secret = std::env::var("JWT_SECRET")?;
    
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(20)
        .connect(&database_url)
        .await?;

    let app_state = AppState {
        db: pool,
        jwt_secret,
    };

    let app = Router::new()
        .route("/login", post(login))
        .with_state(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}
```

***

## 팀 도입 일정 (Team Adoption Timeline)

### 첫째 달: 기초 다지기 (Month 1: Foundation)
**1~2주차: 구문 및 소유권**
- C#과의 기본 구문 차이점 학습
- 소유권(ownership), 빌림(borrowing), 수명(lifetimes) 이해
- 소규모 연습: CLI 도구, 파일 처리 프로그램 제작

**3~4주차: 에러 처리 및 타입 시스템**
- 예외(Exceptions) 대신 `Result<T, E>` 사용
- nullable 타입 대신 `Option<T>` 사용
- 패턴 매칭 및 철저한 검사(exhaustive checking) 활용

**권장 연습 문제:**
```rust
// 1~2주차: 파일 처리기
fn process_log_file(path: &str) -> Result<Vec<String>, std::io::Error> {
    let content = std::fs::read_to_string(path)?;
    let errors: Vec<String> = content
        .lines()
        .filter(|line| line.contains("ERROR"))
        .map(|line| line.to_string())
        .collect();
    Ok(errors)
}

// 3~4주차: 에러 처리를 포함한 JSON 처리기
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct LogEntry {
    timestamp: String,
    level: String,
    message: String,
}

fn parse_log_entries(json_str: &str) -> Result<Vec<LogEntry>, Box<dyn std::error::Error>> {
    let entries: Vec<LogEntry> = serde_json::from_str(json_str)?;
    Ok(entries)
}
```

### 둘째 달: 실전 적용 (Month 2: Practical Applications)
**5~6주차: 트레이트 및 제네릭**
- 인터페이스 대신 트레이트(Trait) 시스템 활용
- 제네릭 제약 및 바운드 설정
- 일반적인 관용구 및 패턴 학습

**7~8주차: 비동기 프로그래밍 및 동시성**
- `async`/`await`의 유사점과 차이점
- 통신을 위한 채널(Channels) 사용
- 스레드 안전성 보장 방식 학습

**권장 프로젝트:**
```rust
// 5~6주차: 제네릭 데이터 처리기
trait DataProcessor<T> {
    type Output;
    type Error;
    
    fn process(&self, data: T) -> Result<Self::Output, Self::Error>;
}

struct JsonProcessor;

impl DataProcessor<&str> for JsonProcessor {
    type Output = serde_json::Value;
    type Error = serde_json::Error;
    
    fn process(&self, data: &str) -> Result<Self::Output, Self::Error> {
        serde_json::from_str(data)
    }
}

// 7~8주차: 비동기 웹 클라이언트
async fn fetch_and_process_data(urls: Vec<&str>) -> Result<(), Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    
    let tasks: Vec<_> = urls
        .into_iter()
        .map(|url| {
            let client = client.clone();
            tokio::spawn(async move {
                let response = client.get(url).send().await?;
                let text = response.text().await?;
                println!("{}로부터 {} 바이트를 가져왔습니다", url, text.len());
                Ok::<(), reqwest::Error>(())
            })
        })
        .collect();
    
    for task in tasks {
        task.await??;
    }
    
    Ok(())
}
```

### 셋째 달 이후: 운영 환경 통합 (Month 3+: Production Integration)
**9~12주차: 실제 프로젝트 수행**
- 중요도가 낮거나 독립적인 컴포넌트를 Rust로 재작성
- 포괄적인 에러 처리 구현
- 로깅, 메트릭, 테스트 추가
- 성능 프로파일링 및 최적화

**지속적 활동: 팀 리뷰 및 멘토링**
- Rust 관용구에 집중한 코드 리뷰 진행
- 페어 프로그래밍 세션 운영
- 지식 공유 세션 정례화

***
