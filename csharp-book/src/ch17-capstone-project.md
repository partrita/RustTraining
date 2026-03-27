## 캡스톤 프로젝트: CLI 날씨 도구 만들기 (Capstone Project: Build a CLI Weather Tool)

> **학습 내용:** 지금까지 배운 모든 것 — 구조체, 트레이트, 에러 처리, 비동기, 모듈,
> serde, 그리고 CLI 인자 파싱 — 을 결합하여 실제 작동하는 Rust 애플리케이션을 만드는 방법을 알아봅니다.
> 이 프로젝트는 C# 개발자가 `HttpClient`, `System.Text.Json`, `System.CommandLine`을 사용하여 만드는 도구와 유사한 형태가 될 것입니다.
>
> **난이도:** 🟡 중급

이 캡스톤 프로젝트는 이 책의 모든 파트에서 다룬 개념들을 하나로 모으는 과정입니다. API에서 날씨 데이터를 가져와 표시하는 커맨드 라인 도구인 `weather-cli`를 만들어 보겠습니다. 이 프로젝트는 적절한 모듈 레이아웃, 에러 타입, 그리고 테스트를 갖춘 미니 크레이트 형태로 구성됩니다.

### 프로젝트 개요 (Project Overview)

```mermaid
graph TD
    CLI["main.rs\nclap CLI parser"] --> Client["client.rs\nreqwest + tokio"]
    Client -->|"HTTP GET"| API["날씨 API"]
    Client -->|"JSON → struct"| Model["weather.rs\nserde Deserialize"]
    Model --> Display["display.rs\nfmt::Display"]
    CLI --> Err["error.rs\nthiserror"]
    Client --> Err

    style CLI fill:#bbdefb,color:#000
    style Err fill:#ffcdd2,color:#000
    style Model fill:#c8e6c9,color:#000
```

**완성될 모습:**
```
$ weather-cli --city "Seattle"
🌧  Seattle: 12°C, Overcast clouds
    Humidity: 82%  Wind: 5.4 m/s
```

**사용되는 개념들:**
| 관련 장 | 사용된 개념 |
|---|---|
| 제5장 (구조체) | `WeatherReport`, `Config` 데이터 타입 |
| 제8장 (모듈) | `src/lib.rs`, `src/client.rs`, `src/display.rs` |
| 제9장 (에러) | `thiserror`를 사용한 커스텀 `WeatherError` |
| 제10장 (트레이트) | 포맷팅된 출력을 위한 `Display` 구현 |
| 제11장 (From/Into) | `serde`를 통한 JSON 역직렬화(Deserialization) |
| 제12장 (반복자) | API 응답 배열 처리 |
| 제13장 (비동기) | HTTP 호출을 위한 `reqwest` + `tokio` |
| 제14-1장 (테스트) | 유닛 테스트 + 통합 테스트 |

---

### 1단계: 프로젝트 설정

```bash
cargo new weather-cli
cd weather-cli
```

`Cargo.toml`에 의존성 추가:
```toml
[package]
name = "weather-cli"
version = "0.1.0"
edition = "2021"

[dependencies]
clap = { version = "4", features = ["derive"] }   # CLI 인자 처리 (System.CommandLine과 유사)
reqwest = { version = "0.12", features = ["json"] } # HTTP 클라이언트 (HttpClient와 유사)
serde = { version = "1", features = ["derive"] }    # 직렬화 (System.Text.Json과 유사)
serde_json = "1"
thiserror = "2"                                      # 에러 타입 정의
tokio = { version = "1", features = ["full"] }       # 비동기 런타임
```

```csharp
// C# 대응 의존성:
// dotnet add package System.CommandLine
// dotnet add package System.Net.Http.Json
// (System.Text.Json과 HttpClient는 기본 내장됨)
```

### 2단계: 데이터 타입 정의

`src/weather.rs` 생성:
```rust
use serde::Deserialize;

/// Raw API 응답 (JSON 구조와 일치)
#[derive(Deserialize, Debug)]
pub struct ApiResponse {
    pub main: MainData,
    pub weather: Vec<WeatherCondition>,
    pub wind: WindData,
    pub name: String,
}

#[derive(Deserialize, Debug)]
pub struct MainData {
    pub temp: f64,
    pub humidity: u32,
}

#[derive(Deserialize, Debug)]
pub struct WeatherCondition {
    pub description: String,
    pub icon: String,
}

#[derive(Deserialize, Debug)]
pub struct WindData {
    pub speed: f64,
}

/// 도메인 타입 (API 구조와 분리된 깔끔한 형태)
#[derive(Debug, Clone)]
pub struct WeatherReport {
    pub city: String,
    pub temp_celsius: f64,
    pub description: String,
    pub humidity: u32,
    pub wind_speed: f64,
}

impl From<ApiResponse> for WeatherReport {
    fn from(api: ApiResponse) -> Self {
        let description = api.weather
            .first()
            .map(|w| w.description.clone())
            .unwrap_or_else(|| "Unknown".to_string());

        WeatherReport {
            city: api.name,
            temp_celsius: api.main.temp,
            description,
            humidity: api.main.humidity,
            wind_speed: api.wind.speed,
        }
    }
}
```

```csharp
// C# 대응 코드:
// public record ApiResponse(MainData Main, List<WeatherCondition> Weather, ...);
// public record WeatherReport(string City, double TempCelsius, ...);
// 수동 매핑 또는 AutoMapper 사용
```

**주요 차이점:** Rust에서는 `#[derive(Deserialize)]`와 `From` 구현이 C#의 `JsonSerializer.Deserialize<T>()`와 AutoMapper 역할을 대신합니다. 두 과정 모두 컴파일 타임에 결정되며 리플렉션(reflection)을 사용하지 않습니다.

### 3단계: 에러 타입 정의

`src/error.rs` 생성:
```rust
use thiserror::Error;

#[derive(Error, Debug)]
pub enum WeatherError {
    #[error("HTTP 요청 실패: {0}")]
    Http(#[from] reqwest::Error),

    #[error("도시를 찾을 수 없음: {0}")]
    CityNotFound(String),

    #[error("API 키가 설정되지 않음 — WEATHER_API_KEY 환경변수를 설정하세요")]
    MissingApiKey,
}

pub type Result<T> = std::result::Result<T, WeatherError>;
```

### 4단계: HTTP 클라이언트 구현

`src/client.rs` 생성:
```rust
use crate::error::{WeatherError, Result};
use crate::weather::{ApiResponse, WeatherReport};

pub struct WeatherClient {
    api_key: String,
    http: reqwest::Client,
}

impl WeatherClient {
    pub fn new(api_key: String) -> Self {
        WeatherClient {
            api_key,
            http: reqwest::Client::new(),
        }
    }

    pub async fn get_weather(&self, city: &str) -> Result<WeatherReport> {
        let url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
            city, self.api_key
        );

        let response = self.http.get(&url).send().await?;

        if response.status() == reqwest::StatusCode::NOT_FOUND {
            return Err(WeatherError::CityNotFound(city.to_string()));
        }

        let api_data: ApiResponse = response.json().await?;
        Ok(WeatherReport::from(api_data))
    }
}
```

```csharp
// C# 대응 코드:
// var response = await _httpClient.GetAsync(url);
// if (response.StatusCode == HttpStatusCode.NotFound)
//     throw new CityNotFoundException(city);
// var data = await response.Content.ReadFromJsonAsync<ApiResponse>();
```

**주요 차이점:**
- `try/catch` 대신 `?` 연산자를 사용합니다 — 에러는 `Result`를 통해 자동으로 전파됩니다.
- AutoMapper 대신 `From` 트레이트를 사용하여 `WeatherReport::from(api_data)`와 같이 변환합니다.
- `IHttpClientFactory`가 필요 없습니다 — `reqwest::Client`가 내부적으로 커넥션 풀링(connection pooling)을 관리합니다.

### 5단계: 화면 출력 포맷팅

`src/display.rs` 생성:
```rust
use std::fmt;
use crate::weather::WeatherReport;

impl fmt::Display for WeatherReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let icon = weather_icon(&self.description);
        writeln!(f, "{}  {}: {:.0}°C, {}",
            icon, self.city, self.temp_celsius, self.description)?;
        write!(f, "    Humidity: {}%  Wind: {:.1} m/s",
            self.humidity, self.wind_speed)
    }
}

fn weather_icon(description: &str) -> &str {
    let desc = description.to_lowercase();
    if desc.contains("clear") { "☀️" }
    else if desc.contains("cloud") { "☁️" }
    else if desc.contains("rain") || desc.contains("drizzle") { "🌧" }
    else if desc.contains("snow") { "❄️" }
    else if desc.contains("thunder") { "⛈" }
    else { "🌡" }
}
```

### 6단계: 모든 기능 연결하기

`src/lib.rs`:
```rust
pub mod client;
pub mod display;
pub mod error;
pub mod weather;
```

`src/main.rs`:
```rust
use clap::Parser;
use weather_cli::{client::WeatherClient, error::WeatherError};

#[derive(Parser)]
#[command(name = "weather-cli", about = "커맨드 라인에서 날씨를 가져옵니다")]
struct Cli {
    /// 조회할 도시 이름
    #[arg(short, long)]
    city: String,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    let api_key = match std::env::var("WEATHER_API_KEY") {
        Ok(key) => key,
        Err(_) => {
            eprintln!("에러: {}", WeatherError::MissingApiKey);
            std::process::exit(1);
        }
    };

    let client = WeatherClient::new(api_key);

    match client.get_weather(&cli.city).await {
        Ok(report) => println!("{report}"),
        Err(WeatherError::CityNotFound(city)) => {
            eprintln!("도시를 찾을 수 없습니다: {city}");
            std::process::exit(1);
        }
        Err(e) => {
            eprintln!("에러: {e}");
            std::process::exit(1);
        }
    }
}
```

### 7단계: 테스트 작성

```rust
// src/weather.rs 또는 tests/weather_test.rs에 작성
#[cfg(test)]
mod tests {
    use super::*;

    fn sample_api_response() -> ApiResponse {
        serde_json::from_str(r#"{
            "main": {"temp": 12.3, "humidity": 82},
            "weather": [{"description": "overcast clouds", "icon": "04d"}],
            "wind": {"speed": 5.4},
            "name": "Seattle"
        }"#).unwrap()
    }

    #[test]
    fn api_response_to_weather_report() {
        let report = WeatherReport::from(sample_api_response());
        assert_eq!(report.city, "Seattle");
        assert!((report.temp_celsius - 12.3).abs() < 0.01);
        assert_eq!(report.description, "overcast clouds");
    }

    #[test]
    fn display_format_includes_icon() {
        let report = WeatherReport {
            city: "Test".into(),
            temp_celsius: 20.0,
            description: "clear sky".into(),
            humidity: 50,
            wind_speed: 3.0,
        };
        let output = format!("{report}");
        assert!(output.contains("☀️"));
        assert!(output.contains("20°C"));
    }

    #[test]
    fn empty_weather_array_defaults_to_unknown() {
        let json = r#"{
            "main": {"temp": 0.0, "humidity": 0},
            "weather": [],
            "wind": {"speed": 0.0},
            "name": "Nowhere"
        }"#;
        let api: ApiResponse = serde_json::from_str(json).unwrap();
        let report = WeatherReport::from(api);
        assert_eq!(report.description, "Unknown");
    }
}
```

---

### 최종 파일 구조

```
weather-cli/
├── Cargo.toml
├── src/
│   ├── main.rs        # CLI 엔트리 포인트 (clap)
│   ├── lib.rs         # 모듈 선언
│   ├── client.rs      # HTTP 클라이언트 (reqwest + tokio)
│   ├── weather.rs     # 데이터 타입 + From 구현 + 테스트
│   ├── display.rs     # 출력 포맷팅
│   └── error.rs       # WeatherError + Result 별칭
└── tests/
    └── integration.rs # 통합 테스트
```

C# 프로젝트 구조와의 비교:
```
WeatherCli/
├── WeatherCli.csproj
├── Program.cs
├── Services/
│   └── WeatherClient.cs
├── Models/
│   ├── ApiResponse.cs
│   └── WeatherReport.cs
└── Tests/
    └── WeatherTests.cs
```

**Rust 버전의 구조는 C#과 매우 흡사합니다.** 주요 차이점은 다음과 같습니다:
- 네임스페이스 대신 `mod` 선언을 사용합니다.
- 예외(exception) 대신 `Result<T, E>`를 사용합니다.
- AutoMapper 대신 `From` 트레이트를 사용합니다.
- 기본 내장 비동기 런타임 대신 명시적인 `#[tokio::main]`을 사용합니다.

### 보너스: 통합 테스트 스텁 (Integration Test Stub)

실제 서버를 호출하지 않고 공용 API를 테스트하기 위해 `tests/integration.rs`를 작성합니다.

```rust
// tests/integration.rs
use weather_cli::weather::WeatherReport;

#[test]
fn weather_report_display_roundtrip() {
    let report = WeatherReport {
        city: "Seattle".into(),
        temp_celsius: 12.3,
        description: "overcast clouds".into(),
        humidity: 82,
        wind_speed: 5.4,
    };

    let output = format!("{report}");
    assert!(output.contains("Seattle"));
    assert!(output.contains("12°C"));
    assert!(output.contains("82%"));
}
```

`cargo test`를 실행하면 — Rust는 `src/` 내의 테스트(`#[cfg(test)]` 모듈)와 `tests/` 내의 통합 테스트를 자동으로 찾아 실행합니다. xUnit이나 NUnit을 설정할 때와 비교하면 별도의 테스트 프레임워크 설정이 거의 필요 없음을 알 수 있습니다.

---

### 심화 도전 과제 (Extension Challenges)

도구가 정상 작동한다면, 다음 과제들을 통해 실력을 더 쌓아보세요:

1. **캐싱 기능 추가** — 마지막 API 응답을 파일에 저장합니다. 시작 시 파일이 10분 이내라면 HTTP 호출을 건너뛰고 캐시를 사용합니다. 이 과정에서 `std::fs`, `serde_json::to_writer`, `SystemTime`을 연습하게 됩니다.

2. **여러 도시 지원** — `--city "Seattle,Portland,Vancouver"`와 같이 입력받아 `tokio::join!`을 사용하여 모든 도시의 정보를 동시에 가져옵니다. 이 과정에서 동시 비동기 처리를 연습하게 됩니다.

3. **`--format json` 플래그 추가** — 사람이 읽는 텍스트 대신 `serde_json::to_string_pretty`를 사용하여 결과를 JSON으로 출력합니다. 이 과정에서 조건부 포맷팅과 `Serialize`를 연습하게 됩니다.

4. **심화 통합 테스트 작성** — `wiremock`을 사용하여 모의(mock) HTTP 서버를 만들고 전체 흐름을 테스트하는 `tests/integration.rs`를 작성합니다. 14-1장에서 다룬 `tests/` 디렉토리 패턴을 연습하게 됩니다.

***
