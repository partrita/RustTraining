## 학습 경로 및 다음 단계 (Learning Path and Next Steps)

> **학습 내용:** 구조화된 학습 로드맵(1~2주 차, 1~3개월 차 이상), 권장 도서 및 리소스,
> C# 개발자가 자주 겪는 함정(소유권 혼동, 빌림 검사기와의 싸움),
> 그리고 `tracing`과 `ILogger`를 활용한 구조화된 가시성 확보 방법을 알아봅니다.
>
> **난이도:** 🟢 초급

### 즉시 실천할 단계 (1~2주 차)
1. **환경 설정**
   - [rustup.rs](https://rustup.rs/)를 통해 Rust 설치
   - rust-analyzer 확장이 포함된 VS Code 설정
   - 첫 번째 `cargo new hello_world` 프로젝트 생성

2. **기본기 다지기**
   - 간단한 예제를 통해 소유권(ownership) 연습
   - 다양한 매개변수 타입(`&str`, `String`, `&mut`)을 사용하는 함수 작성
   - 기본적인 구조체(struct)와 메서드 구현

3. **에러 처리 연습**
   - C#의 try-catch 코드를 Result 기반 패턴으로 변환
   - `?` 연산자와 `match` 문 연습
   - 커스텀 에러 타입 구현

### 중급 목표 (1~2개월 차)
1. **컬렉션과 반복자**
   - `Vec<T>`, `HashMap<K,V>`, `HashSet<T>` 익히기
   - 반복자 메서드 학습: `map`, `filter`, `collect`, `fold`
   - `for` 루프와 반복자 체인(iterator chains) 비교 연습

2. **트레이트(Trait)와 제네릭(Generic)**
   - 공통 트레이트 구현: `Debug`, `Clone`, `PartialEq`
   - 제네릭 함수 및 구조체 작성
   - 트레이트 바운드(trait bounds)와 `where` 절 이해

3. **프로젝트 구조**
   - 코드를 모듈(module)로 조직화
   - `pub` 가시성 이해
   - crates.io의 외부 크레이트 활용

### 고급 주제 (3개월 차 이상)
1. **동시성(Concurrency)**
   - `Send`와 `Sync` 트레이트 이해
   - 기본적인 병렬 처리를 위한 `std::thread` 사용
   - 비동기 프로그래밍을 위한 `tokio` 탐구

2. **메모리 관리**
   - 공유 소유권을 위한 `Rc<T>`와 `Arc<T>` 이해
   - 힙 할당을 위한 `Box<T>` 사용 시기 학습
   - 복잡한 시나리오를 위한 수명(lifetimes) 마스터

3. **실무 프로젝트**
   - `clap`을 이용한 CLI 도구 빌드
   - `axum` 또는 `warp`을 이용한 웹 API 생성
   - 라이브러리 작성 및 crates.io에 게시

### 권장 학습 리소스

#### 도서
- **"The Rust Programming Language"** (온라인 무료) - 공식 가이드북
- **"Rust by Example"** (온라인 무료) - 실습 위주의 예제집
- **"Programming Rust"** (Jim Blandy 저) - 심층적인 기술적 내용

#### 온라인 리소스
- [Rust Playground](https://play.rust-lang.org/) - 브라우저에서 코드 실행
- [Rustlings](https://github.com/rust-lang/rustlings) - 대화형 실습 문제
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - 실용적인 예제

#### 연습 프로젝트
1. **커맨드 라인 계산기** - 열거형(enum)과 패턴 매칭 연습
2. **파일 정리 도구** - 파일 시스템 작업 및 에러 처리 학습
3. **JSON 프로세서** - serde 및 데이터 변환 학습
4. **HTTP 서버** - 비동기 프로그래밍 및 네트워킹 이해
5. **데이터베이스 라이브러리** - 트레이트, 제네릭 및 에러 처리 마스터

### C# 개발자가 자주 겪는 함정 (Common Pitfalls for C# Developers)

#### 소유권 혼동 (Ownership Confusion)
```rust
// 잘못된 방법: 이동(move)된 값 사용 시도
fn wrong_way() {
    let s = String::from("hello");
    takes_ownership(s);
    // println!("{}", s); // 에러: s는 소유권이 이동(move)되었습니다.
}

// 올바른 방법: 필요한 경우 참조(reference) 또는 클론(clone) 사용
fn right_way() {
    let s = String::from("hello");
    borrows_string(&s);
    println!("{}", s); // 정상: s의 소유권은 여전히 여기에 있습니다.
}

fn takes_ownership(s: String) { /* 여기서 s의 소유권이 이동됨 */ }
fn borrows_string(s: &str) { /* 여기서 s를 빌림(borrow) */ }
```

#### 빌림 검사기(Borrow Checker)와의 싸움
```rust
// 잘못된 방법: 여러 개의 가변 참조(mutable reference) 생성
fn wrong_borrowing() {
    let mut v = vec![1, 2, 3];
    let r1 = &mut v;
    // let r2 = &mut v; // 에러: 가변 참조는 동시에 하나만 가질 수 있습니다.
}

// 올바른 방법: 가변 참조의 범위를 제한
fn right_borrowing() {
    let mut v = vec![1, 2, 3];
    {
        let r1 = &mut v;
        r1.push(4);
    } // r1이 여기서 범위를 벗어남(out of scope)
    
    let r2 = &mut v; // 정상: 다른 가변 참조가 존재하지 않음
    r2.push(5);
}
```

#### Null 값 기대 (Expecting Null Values)
```rust
// 잘못된 방법: null과 같은 동작 기대
fn no_null_in_rust() {
    // let s: String = null; // Rust에는 null이 없습니다!
}

// 올바른 방법: Option<T>를 명시적으로 사용
fn use_option_instead() {
    let maybe_string: Option<String> = None;
    
    match maybe_string {
        Some(s) => println!("문자열을 받았습니다: {}", s),
        None => println!("문자열이 없습니다"),
    }
}
```

### 마지막 팁

1. **컴파일러를 신뢰하세요** - Rust의 컴파일러 에러는 적대적인 것이 아니라 도움이 되는 것입니다.
2. **작게 시작하세요** - 간단한 프로그램부터 시작하여 점진적으로 복잡성을 더해 가세요.
3. **다른 사람의 코드를 읽으세요** - GitHub에서 인기 있는 크레이트들을 공부하세요.
4. **도움을 요청하세요** - Rust 커뮤니티는 매우 친절하고 도움을 아끼지 않습니다.
5. **꾸준히 연습하세요** - Rust의 개념들은 연습을 통해 자연스럽게 익숙해집니다.

기억하세요: Rust는 학습 곡선이 있지만, 메모리 안전성, 성능, 그리고 두려움 없는 동시성이라는 보상을 제공합니다. 처음에는 제한적으로 느껴졌던 소유권 시스템이 결국은 정확하고 효율적인 프로그램을 작성하기 위한 강력한 도구가 될 것입니다.

---

**축하합니다!** 이제 C#에서 Rust로 전환하기 위한 견고한 토대를 마련하셨습니다. 간단한 프로젝트부터 시작하여 학습 과정에 인내심을 갖고, 점차 복잡한 애플리케이션으로 나아가십시오. Rust가 제공하는 안전성과 성능상의 이점은 초기 학습 투자의 가치를 충분히 증명해 줄 것입니다.


<!-- ch16.2a: Structured Observability with tracing -->
## 구조화된 가시성: `tracing` vs ILogger 및 Serilog (Structured Observability)

C# 개발자들은 로그 메시지에 타입이 지정된 키-값 속성을 포함하는 **구조화된 로깅(structured logging)** — `ILogger`, **Serilog**, **NLog** 등 — 에 익숙합니다. Rust의 `log` 크레이트는 기본적인 수준별 로깅을 제공하지만, **`tracing`**은 스팬(span), 비동기 인지(async awareness), 분산 추적 지원을 갖춘 프로덕션 표준 구조화 가시성 도구입니다.

### 왜 `log` 대신 `tracing`인가

| 기능 | `log` 크레이트 | `tracing` 크레이트 | C# 대응 개념 |
|---------|------------|-----------------|----------------|
| 레벨별 메시지 | ✅ `info!()`, `error!()` | ✅ `info!()`, `error!()` | `ILogger.LogInformation()` |
| 구조화된 필드 | ❌ 문자열 보간만 가능 | ✅ 타입이 지정된 키-값 필드 | Serilog `Log.Information("{User}", user)` |
| 스팬 (범위 컨텍스트) | ❌ | ✅ `#[instrument]`, `span!()` | `ILogger.BeginScope()` |
| 비동기 인지 | ❌ `.await` 시 컨텍스트 유실 | ✅ 스팬이 `.await`를 따라 유지됨 | `Activity` / `DiagnosticSource` |
| 분산 추적 | ❌ | ✅ OpenTelemetry 통합 | `System.Diagnostics.Activity` |
| 다양한 출력 형식 | 기본 제공 | JSON, pretty, compact, OTLP | Serilog sinks |

### 시작하기
```toml
# Cargo.toml
[dependencies]
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter", "json"] }
```

### 기본 사용법: 구조화된 로깅
```csharp
// C# Serilog
Log.Information("Processing order {OrderId} for {Customer}, total {Total:C}",
    orderId, customer.Name, order.Total);
// 출력: Processing order 12345 for Alice, total $99.95
// JSON: {"OrderId": 12345, "Customer": "Alice", "Total": 99.95, ...}
```

```rust
use tracing::{info, warn, error, debug, instrument};

// 구조화된 필드 — 문자열 보간이 아닌 타입이 지정된 필드 사용
info!(order_id = 12345, customer = "Alice", total = 99.95,
      "주문 처리 중");
// 출력: INFO 주문 처리 중 order_id=12345 customer="Alice" total=99.95
// JSON: {"order_id": 12345, "customer": "Alice", "total": 99.95, ...}

// 동적 값 사용
let order_id = 12345;
info!(order_id, "주문 수신됨");  // 필드 이름 = 변수 이름 축약형

// 조건부 필드
if let Some(promo) = promo_code {
    info!(order_id, promo_code = %promo, "프로모션 적용됨");
    //                        ^ %는 Display 포맷팅을 사용함을 의미
    //                        ?는 Debug 포맷팅을 사용함을 의미
}
```

### 스팬(Span): 비동기 코드의 핵심 기능

스팬은 함수 호출과 `.await` 지점을 가로질러 필드를 전달하는 범위 컨텍스트(scoped context)입니다. C#의 `ILogger.BeginScope()`와 유사하지만 비동기 환경에서도 안전합니다.

```csharp
// C# — Activity / BeginScope
using var activity = new Activity("ProcessOrder").Start();
activity.SetTag("order_id", orderId);

using (_logger.BeginScope(new Dictionary<string, object> { ["OrderId"] = orderId }))
{
    _logger.LogInformation("처리 시작");
    await ProcessPaymentAsync();
    _logger.LogInformation("결제 완료");  // OrderId가 여전히 스코프 내에 있음
}
```

```rust
use tracing::{info, instrument, Instrument};

// #[instrument]는 함수 인자를 필드로 하는 스팬을 자동으로 생성합니다.
#[instrument(skip(db), fields(customer_name))]
async fn process_order(order_id: u64, db: &Database) -> Result<(), AppError> {
    let order = db.get_order(order_id).await?;
    
    // 현재 스팬에 동적으로 필드 기록
    tracing::Span::current().record("customer_name", &order.customer_name.as_str());
    
    info!("처리 시작");
    process_payment(&order).await?;        // .await를 넘어서도 스팬 컨텍스트가 보존됨!
    info!(items = order.items.len(), "결제 완료");
    Ok(())
}
// 이 함수 내부의 모든 로그 메시지에는 다음이 자동으로 포함됩니다:
// order_id=12345 customer_name="Alice"
// 중첩된 비동기 호출에서도 마찬가지입니다!

// 수동 스팬 생성 (BeginScope와 유사)
async fn batch_process(orders: Vec<u64>, db: &Database) {
    for order_id in orders {
        let span = tracing::info_span!("process_order", order_id);
        
        // .instrument(span)을 사용하여 스팬을 future에 부착합니다.
        process_order(order_id, db)
            .instrument(span)
            .await
            .unwrap_or_else(|e| error!("실패: {e}"));
    }
}
```

### Subscriber 설정 (Serilog Sinks와 유사)

```rust
use tracing_subscriber::{fmt, EnvFilter, layer::SubscriberExt, util::SubscriberInitExt};

fn init_tracing() {
    // 개발 환경: 사람이 읽기 좋은 컬러 출력
    tracing_subscriber::registry()
        .with(EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| "my_app=debug,tower_http=info".into()))
        .with(fmt::layer().pretty())  // 컬러가 적용되고 들여쓰기된 스팬 표시
        .init();
}

fn init_tracing_production() {
    // 운영 환경: 로그 집계 시스템을 위한 JSON 출력 (Serilog JSON sink와 유사)
    tracing_subscriber::registry()
        .with(EnvFilter::new("my_app=info"))
        .with(fmt::layer().json())  // 구조화된 JSON 형식
        .init();
    // 출력: {"timestamp":"...","level":"INFO","fields":{"order_id":123},...}
}
```

```bash
# 환경 변수를 통해 로그 레벨 제어 (Serilog의 MinimumLevel과 유사)
RUST_LOG=my_app=debug,hyper=warn cargo run
RUST_LOG=trace cargo run  # 모든 로그 출력
```

### Serilog → tracing 마이그레이션 요약표 (Cheat Sheet)

| Serilog / ILogger | tracing | 참고 사항 |
|-------------------|---------|-------|
| `Log.Information("{Key}", val)` | `info!(key = val, "메시지")` | 필드는 보간 방식이 아닌 타입 지정 방식임 |
| `Log.ForContext("Key", val)` | `span.record("key", val)` | 현재 스팬에 필드 추가 |
| `using BeginScope(...)` | `#[instrument]` 또는 `info_span!()` | `#[instrument]` 사용 시 자동화됨 |
| `.WriteTo.Console()` | `fmt::layer()` | 사람이 읽기 좋은 형식 |
| `.WriteTo.Seq()` / `.File()` | `fmt::layer().json()` + 파일 리다이렉트 | 또는 `tracing-appender` 사용 |
| `.Enrich.WithProperty()` | `span!(Level::INFO, "name", key = val)` | 스팬 필드 활용 |
| `LogEventLevel.Debug` | `tracing::Level::DEBUG` | 동일한 개념 |
| `{@Object}` 구조 해제 | `field = ?value` (Debug) 또는 `%value` (Display) | `?`는 Debug, `%`는 Display 트레이트 사용 |

### OpenTelemetry 통합
```toml
# 분산 추적용 (System.Diagnostics + OTLP exporter와 유사)
[dependencies]
tracing-opentelemetry = "0.22"
opentelemetry = "0.21"
opentelemetry-otlp = "0.14"
```

```rust
// 콘솔 출력과 함께 OpenTelemetry 레이어 추가
use tracing_opentelemetry::OpenTelemetryLayer;

fn init_otel() {
    let tracer = opentelemetry_otlp::new_pipeline()
        .tracing()
        .with_exporter(opentelemetry_otlp::new_exporter().tonic())
        .install_batch(opentelemetry_sdk::runtime::Tokio)
        .expect("OTLP 추적기(tracer) 생성 실패");

    tracing_subscriber::registry()
        .with(OpenTelemetryLayer::new(tracer))  // Jaeger/Tempo 등으로 스팬 전송
        .with(fmt::layer())                      // 콘솔에도 함께 출력
        .init();
}
// 이제 #[instrument]가 적용된 스팬들은 자동으로 분산 추적 데이터가 됩니다!
```

***
