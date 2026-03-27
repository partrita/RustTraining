## 로깅과 트레이싱 (Logging and Tracing): syslog/printf → `log` + `tracing`

> **학습 내용:** Rust의 2계층 로깅 아키텍처(파사드 + 백엔드), `log` 및 `tracing` 크레이트, 스팬(span)을 사용한 구조화된 로깅, 그리고 이것이 어떻게 `printf`/`syslog` 디버깅을 대체하는지 배웁니다.

C++ 진단 코드는 대개 `printf`, `syslog` 또는 커스텀 로깅 프레임워크를 사용합니다.
Rust는 표준화된 2계층 로깅 아키텍처를 가지고 있습니다: **파사드(facade)** 크레이트(`log` 또는 `tracing`)와 **백엔드(backend)**(실제 로거 구현체)입니다.

### `log` 파사드 — Rust의 범용 로깅 API

`log` 크레이트는 syslog의 심각도 레벨과 유사한 매크로들을 제공합니다. 라이브러리는 `log` 매크로를 사용하고, 실행 파일(binary)은 백엔드를 선택합니다.

```rust
// Cargo.toml
// [dependencies]
// log = "0.4"
// env_logger = "0.11"    # 여러 백엔드 중 하나

use log::{info, warn, error, debug, trace};

fn check_sensor(id: u32, temp: f64) {
    trace!("센서 {id} 읽는 중");           // 가장 세밀한 정보
    debug!("센서 {id} 원시 값: {temp}"); // 개발 시 상세 정보

    if temp > 85.0 {
        warn!("센서 {id} 고온 감지: {temp}°C");
    }
    if temp > 95.0 {
        error!("센서 {id} 위험 수치: {temp}°C — 셧다운 시작");
    }
    info!("센서 {id} 점검 완료");     // 정상 작동
}

fn main() {
    // 백엔드 초기화 — 보통 main()에서 한 번 수행합니다.
    env_logger::init();  // RUST_LOG 환경 변수로 제어 가능

    check_sensor(0, 72.5);
    check_sensor(1, 91.0);
}
```

```bash
# 환경 변수를 통해 로그 레벨 제어
RUST_LOG=debug cargo run          # debug 레벨 이상 표시
RUST_LOG=warn cargo run           # warn 및 error만 표시
RUST_LOG=my_crate=trace cargo run # 모듈별 필터링
RUST_LOG=my_crate::gpu=debug,warn cargo run  # 여러 레벨 조합
```

### C++ 비교

| C++ | Rust (`log`) | 참고 |
|-----|-------------|-------|
| `printf("DEBUG: %s\n", msg)` | `debug!("{msg}")` | 컴파일 타임에 포맷 검사 수행 |
| `syslog(LOG_ERR, "...")` | `error!("...")` | 백엔드가 출력 위치 결정 |
| 로그 호출 주변의 `#ifdef DEBUG` | `trace!` / `debug!`는 max_level 설정 시 컴파일에서 제외됨 | 비활성화 시 비용 발생 안 함 (Zero-cost) |
| 커스텀 `Logger::log(level, msg)` | `log::info!("...")` — 모든 크레이트가 동일한 API 사용 | 범용 파사드, 교체 가능한 백엔드 |
| 파일별 로그 상세도 설정 | `RUST_LOG=crate::module=level` | 환경 변수 기반, 재컴파일 불필요 |

### `tracing` 크레이트 — 스팬(span)을 사용한 구조화된 로깅

`tracing`은 **구조화된 필드(structured fields)**와 **스팬(span, 시간 기반 스코프)**을 통해 `log`의 기능을 확장합니다. 이는 컨텍스트를 추적해야 하는 진단 코드에서 특히 유용합니다.

```rust
// Cargo.toml
// [dependencies]
// tracing = "0.1"
// tracing-subscriber = { version = "0.3", features = ["env-filter"] }

use tracing::{info, warn, error, instrument, info_span};

#[instrument(skip(data), fields(gpu_id = gpu_id, data_len = data.len()))]
fn run_gpu_test(gpu_id: u32, data: &[u8]) -> Result<(), String> {
    info!("GPU 테스트 시작");

    let span = info_span!("ecc_check", gpu_id);
    let _guard = span.enter();  // 이 스코프 안의 모든 로그에는 gpu_id가 포함됩니다.

    if data.is_empty() {
        error!(gpu_id, "테스트 데이터가 제공되지 않았습니다.");
        return Err("데이터 없음".to_string());
    }

    // 구조화된 필드 — 단순 문자열 보간이 아닌 기계가 파싱 가능한 형태
    info!(
        gpu_id,
        temp_celsius = 72.5,
        ecc_errors = 0,
        "ECC 체크 통과"
    );

    Ok(())
}

fn main() {
    // tracing subscriber 초기화
    tracing_subscriber::fmt()
        .with_env_filter("debug")  // 또는 RUST_LOG 환경 변수 사용
        .with_target(true)          // 모듈 경로 표시
        .with_thread_ids(true)      // 스레드 ID 표시
        .init();

    let _ = run_gpu_test(0, &[1, 2, 3]);
}
```

`tracing-subscriber`를 통한 출력 예시:
```rust
2026-02-15T10:30:00.123Z DEBUG ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}: my_crate: GPU 테스트 시작
2026-02-15T10:30:00.124Z  INFO ThreadId(01) run_gpu_test{gpu_id=0 data_len=3}:ecc_check{gpu_id=0}: my_crate: ECC 체크 통과 gpu_id=0 temp_celsius=72.5 ecc_errors=0
```

### `#[instrument]` — 자동 스팬 생성

`#[instrument]` 속성은 함수 이름과 인자들을 포함하는 스팬을 자동으로 생성해 줍니다.

```rust
use tracing::instrument;

#[instrument]
fn parse_sel_record(record_id: u16, sensor_type: u8, data: &[u8]) -> Result<(), String> {
    // 이 함수 내부의 모든 로그에는 다음 정보가 자동으로 포함됩니다:
    // record_id, sensor_type, 그리고 data (Debug 구현 시)
    tracing::debug!("SEL 레코드 파싱 중");
    Ok(())
}

// skip: 크거나 민감한 인자를 스팬에서 제외
// fields: 계산된 필드 추가
#[instrument(skip(raw_buffer), fields(buf_len = raw_buffer.len()))]
fn decode_ipmi_response(raw_buffer: &[u8]) -> Result<Vec<u8>, String> {
    tracing::trace!("{} 바이트 디코딩 중", raw_buffer.len());
    Ok(raw_buffer.to_vec())
}
```

### `log` 대 `tracing` — 무엇을 사용할 것인가

| 비교 항목 | `log` | `tracing` |
|--------|-------|-----------|
| **복잡성** | 단순함 — 5개의 매크로 | 더 풍부함 — 스팬, 필드, instrument |
| **구조화된 데이터** | 문자열 보간만 가능 | 키-값 필드: `info!(gpu_id = 0, "메시지")` |
| **타이밍 / 스팬** | 지원 안 함 | 지원함 — `#[instrument]`, `span.enter()` |
| **비동기 지원** | 기본적인 수준 | 수준 높음 — `.await`를 가로질러 스팬 전파 |
| **호환성** | 범용 파사드 | `log`와 호환 가능 (`log` 브릿지 제공) |
| **사용 시점** | 간단한 애플리케이션, 라이브러리 | 진단 도구, 비동기 코드, 관측성(observability) 필요 시 |

> **권장 사항**: 운영 환경에서 사용되는 진단용 프로젝트(구조화된 출력이 필요한 진단 도구)에는 `tracing`을 사용하십시오. 의존성을 최소화하고 싶은 간단한 라이브러리에는 `log`를 사용하십시오. `tracing`은 호환 계층을 포함하고 있으므로, `log` 매크로를 사용하는 라이브러리들도 `tracing` 구독자(subscriber)와 함께 작동할 수 있습니다.

### 백엔드 옵션

| 백엔드 크레이트 | 출력 위치 | 사용 사례 |
|--------------|--------|----------|
| `env_logger` | stderr (색상 지원) | 개발 단계, 간단한 CLI 도구 |
| `tracing-subscriber` | stderr (포맷팅 지원) | `tracing`을 사용하는 운영 환경 |
| `syslog` | 시스템 syslog | 리눅스 시스템 서비스 |
| `tracing-journald` | systemd 저널 | systemd로 관리되는 서비스 |
| `tracing-appender` | 순환 로그 파일 (Rotating) | 장시간 실행되는 데몬 |
| `tracing-opentelemetry` | OpenTelemetry 수집기 | 분산 트레이싱 |

----
