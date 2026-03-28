# C# 개발자를 위한 필수 크레이트 가이드

> **학습 목표:** .NET 생태계의 주요 라이브러리에 대응하는 Rust의 핵심 **크레이트(Crates)**들을 알아봅니다. 특히 JSON 처리를 위한 `serde`의 강력한 속성 시스템을 심층 분석하여, 복잡한 데이터 구조를 어떻게 우아하게 직렬화/역직렬화하는지 익힙니다.

---

### 1. 주요 라이브러리 대응표

| **기능** | **C# (.NET)** | **Rust (Crate)** | **비고** |
| :--- | :--- | :--- | :--- |
| **JSON 처리** | `System.Text.Json` | **`serde`** / `serde_json` | Rust 생태계의 표준 |
| **HTTP 클라이언트** | `HttpClient` | **`reqwest`** | 비동기 지원, 사용성 우수 |
| **비동기 런타임** | .NET ThreadPool | **`tokio`** | 가장 널리 쓰이는 실행기 |
| **데이터베이스** | Entity Framework | **`sqlx`** / `diesel` | 컴파일 타임 쿼리 검사 연동 |
| **에러 처리** | Custom Exceptions | **`thiserror`** / `anyhow` | 에러 정의 및 전파 최적화 |
| **날짜/시간** | `DateTime` / `TimeSpan` | **`chrono`** | 유연한 시간대 처리 |
| **병렬 처리** | `Parallel.ForEach` | **`rayon`** | 데이터 기반 병렬 루프 |

---

### 2. Serde 심층 분석: JSON 처리의 마스터
C#의 `[JsonPropertyName]`과 같은 속성들을 Rust에서는 어떻게 사용하는지 핵심만 정리했습니다.

#### 주요 속성 (Attributes)
- **`#[serde(rename = "...")]`**: JSON 필드 이름과 구조체 필드 이름이 다를 때 사용합니다.
- **`#[serde(default)]`**: JSON에 필드가 없을 때 `Default` 트레이트를 사용하여 값을 채웁니다.
- **`#[serde(skip)]`**: 직렬화/역직렬화에서 해당 필드를 완전히 무시합니다.
- **`#[serde(rename_all = "camelCase")]`**: 모든 필드를 한꺼번에 카멜 케이스로 변환합니다.

#### 열거형(Enum) 처리의 강력함
Rust의 열거형은 데이터를 가질 수 있으므로, Serde는 이를 처리하는 독특한 방식을 제공합니다.
- **내부 태그 방식 (`tag = "type"`)**: `{"type": "admin", "id": 1}` 처럼 필드 하나를 구분자로 씁니다.
- **인접 태그 방식 (`tag = "t", content = "c"`)**: `{"t": "success", "c": {...}}` 처럼 타입과 내용을 분리합니다.

---

### 💡 실무 팁: `sqlx`의 장점
C#의 Entity Framework는 런타임에 문제를 발견하는 경우가 많지만, Rust의 **`sqlx`**는 컴파일 타임에 실제 데이터베이스에 접속하여 **SQL 문법과 컬럼 타입이 올바른지 검사**합니다. 덕분에 런타임에 "컬럼 이름이 틀렸어요" 같은 에러를 볼 일이 거의 없습니다.

