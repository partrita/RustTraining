## C++ 프로그래머를 위한 테스트 패턴

> **학습 내용:** Rust의 내장 테스트 프레임워크(`#[test]`, `#[should_panic]`, `Result`를 반환하는 테스트, 테스트 데이터를 위한 빌더 패턴, 트레이트 기반 모킹, `proptest`를 이용한 속성 기반 테스트, `insta`를 이용한 스냅샷 테스트, 통합 테스트 조직화)를 배웁니다. Google Test와 CMake를 대체하는 설정 없는 테스트 환경을 경험해 보세요.

C++ 테스트는 일반적으로 복잡한 빌드 통합이 필요한 외부 프레임워크(Google Test, Catch2, Boost.Test)에 의존합니다. Rust의 테스트 프레임워크는 **언어와 툴체인에 내장**되어 있어 의존성, CMake 통합, 테스트 러너 설정이 전혀 필요 없습니다.

### `#[test]` 외의 테스트 속성들

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_pass() {
        assert_eq!(2 + 2, 4);
    }

    // 패닉이 발생할 것을 기대함 — GTest의 EXPECT_DEATH와 유사
    #[test]
    #[should_panic]
    fn out_of_bounds_panics() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // 패닉 발생 — 테스트 통과
    }

    // 특정 메시지를 포함한 패닉을 기대함
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn specific_panic_message() {
        let v = vec![1, 2, 3];
        let _ = v[10];
    }

    // Result<(), E>를 반환하는 테스트 — unwrap() 대신 ? 사용 가능
    #[test]
    fn test_with_result() -> Result<(), String> {
        let value: u32 = "42".parse().map_err(|e| format!("{e}"))?;
        assert_eq!(value, 42);
        Ok(())
    }

    // 기본적으로 느린 테스트 건너뛰기 — `cargo test -- --ignored`로 실행
    #[test]
    #[ignore]
    fn slow_integration_test() {
        std::thread::sleep(std::time::Duration::from_secs(10));
    }
}
```

```bash
cargo test                          # 무시되지 않은 모든 테스트 실행
cargo test -- --ignored             # 무시된 테스트만 실행
cargo test -- --include-ignored     # 무시된 테스트를 포함한 모든 테스트 실행
cargo test test_name                # 이름 패턴이 일치하는 테스트 실행
cargo test -- --nocapture           # 테스트 중 println! 출력을 표시
cargo test -- --test-threads=1      # 테스트를 순차적으로 실행 (공유 상태가 있는 경우)
```

### 테스트 헬퍼: 테스트 데이터를 위한 빌더 패턴

C++에서는 Google Test 픽스처(`class MyTest : public ::testing::Test`)를 사용했을 것입니다. Rust에서는 빌더 함수나 `Default` 트레이트를 사용합니다.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 빌더 함수 — 합리적인 기본값으로 테스트 데이터를 생성
    fn make_gpu_event(severity: Severity, fault_code: u32) -> DiagEvent {
        DiagEvent {
            source: "accel_diag".to_string(),
            severity,
            message: format!("테스트 이벤트 FC:{fault_code}"),
            fault_code,
        }
    }

    // 재사용 가능한 테스트 픽스처 — 미리 생성된 이벤트 집합
    fn sample_events() -> Vec<DiagEvent> {
        vec![
            make_gpu_event(Severity::Critical, 67956),
            make_gpu_event(Severity::Warning, 32709),
            make_gpu_event(Severity::Info, 10001),
        ]
    }

    #[test]
    fn filter_critical_events() {
        let events = sample_events();
        let critical: Vec<_> = events.iter()
            .filter(|e| e.severity == Severity::Critical)
            .collect();
        assert_eq!(critical.len(), 1);
        assert_eq!(critical[0].fault_code, 67956);
    }
}
```

### 트레이트를 이용한 모킹(Mocking)

C++에서 모킹을 하려면 Google Mock 같은 프레임워크나 수동적인 가상 함수 재정의가 필요합니다. Rust에서는 의존성에 대해 트레이트를 정의하고 테스트에서 구현체를 교체합니다.

```rust
// 프로덕션용 트레이트
trait SensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String>;
}

// 프로덕션용 실제 구현
struct HwSensorReader;
impl SensorReader for HwSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        // 실제 하드웨어 호출...
        Ok(72.5)
    }
}

// 테스트용 모크 — 예측 가능한 값을 반환
#[cfg(test)]
struct MockSensorReader {
    temperatures: std::collections::HashMap<u32, f64>,
}

#[cfg(test)]
impl SensorReader for MockSensorReader {
    fn read_temperature(&self, sensor_id: u32) -> Result<f64, String> {
        self.temperatures.get(&sensor_id)
            .copied()
            .ok_or_else(|| format!("알 수 없는 센서 {sensor_id}"))
    }
}

// 테스트 대상 함수 — 리더(reader)에 대해 제네릭함
fn check_overtemp(reader: &impl SensorReader, ids: &[u32], threshold: f64) -> Vec<u32> {
    ids.iter()
        .filter(|&&id| reader.read_temperature(id).unwrap_or(0.0) > threshold)
        .copied()
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detect_overtemp_sensors() {
        let mut mock = MockSensorReader { temperatures: Default::default() };
        mock.temperatures.insert(0, 72.5);
        mock.temperatures.insert(1, 91.0);  // 임계값 초과
        mock.temperatures.insert(2, 65.0);

        let hot = check_overtemp(&mock, &[0, 1, 2], 80.0);
        assert_eq!(hot, vec![1]);
    }
}
```

### 테스트에서의 임시 파일 및 디렉토리

C++ 테스트는 종종 플랫폼별 임시 디렉토리를 사용합니다. Rust에는 `tempfile` 크레이트가 있습니다.

```rust
// Cargo.toml: [dev-dependencies]
// tempfile = "3"

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;
    use std::io::Write;

    #[test]
    fn parse_config_from_file() -> Result<(), Box<dyn std::error::Error>> {
        // 드롭될 때 자동으로 삭제되는 임시 파일 생성
        let mut file = NamedTempFile::new()?;
        writeln!(file, r#"{{"sku": "ServerNode", "level": "Quick"}}"#)?;

        let config = load_config(file.path().to_str().unwrap())?;
        assert_eq!(config.sku, "ServerNode");
        Ok(())
        // file이 여기서 삭제됨 — 별도의 정리 코드 불필요
    }
}
```

### `proptest`를 이용한 속성 기반 테스트

특정 테스트 케이스를 작성하는 대신, 모든 입력에 대해 유지되어야 하는 **속성**을 기술합니다. `proptest`는 무작위 입력을 생성하고 최소한의 실패 사례를 찾아냅니다.

```rust
// Cargo.toml: [dev-dependencies]
// proptest = "1"

#[cfg(test)]
mod tests {
    use proptest::prelude::*;

    fn parse_and_format(n: u32) -> String {
        format!("{n}")
    }

    proptest! {
        #[test]
        fn roundtrip_u32(n: u32) {
            let formatted = parse_and_format(n);
            let parsed: u32 = formatted.parse().unwrap();
            prop_assert_eq!(n, parsed);
        }

        #[test]
        fn string_contains_no_null(s in "[a-zA-Z0-9 ]{0,100}") {
            prop_assert!(!s.contains('\0'));
        }
    }
}
```

### `insta`를 이용한 스냅샷 테스트

복잡한 출력(JSON, 포맷팅된 문자열)을 생성하는 테스트의 경우, `insta`가 참조 스냅샷을 자동으로 생성하고 관리합니다.

```rust
// Cargo.toml: [dev-dependencies]
// insta = { version = "1", features = ["json"] }

#[cfg(test)]
mod tests {
    use insta::assert_json_snapshot;

    #[test]
    fn der_entry_format() {
        let entry = DerEntry {
            fault_code: 67956,
            component: "GPU".to_string(),
            message: "ECC 에러 감지됨".to_string(),
        };
        // 첫 실행 시: tests/snapshots/에 스냅샷 파일 생성
        // 이후 실행 시: 저장된 스냅샷과 비교
        assert_json_snapshot!(entry);
    }
}
```

```bash
cargo insta test              # 테스트를 실행하고 새롭거나 변경된 스냅샷 검토
cargo insta review            # 스냅샷 변경 사항을 대화형으로 검토
```

### C++ vs Rust 테스트 비교

| **C++ (Google Test)** | **Rust** | **비고** |
|----------------------|---------|----------|
| `TEST(Suite, Name) { }` | `#[test] fn name() { }` | 별도의 슈트/클래스 계층 구조 불필요 |
| `ASSERT_EQ(a, b)` | `assert_eq!(a, b)` | 내장 매크로, 프레임워크 불필요 |
| `ASSERT_NEAR(a, b, eps)` | `assert!((a - b).abs() < eps)` | 또는 `approx` 크레이트 사용 |
| `EXPECT_THROW(expr, type)` | `#[should_panic(expected = "...")]` | 세밀한 제어가 필요하면 `catch_unwind` 사용 |
| `EXPECT_DEATH(expr, "msg")` | `#[should_panic(expected = "msg")]` | |
| `class Fixture : public ::testing::Test` | 빌더 함수 + `Default` | 상속 불필요 |
| Google Mock `MOCK_METHOD` | 트레이트 + 테스트용 구현 | 더 명시적이며 매크로 마법이 없음 |
| `INSTANTIATE_TEST_SUITE_P` | `proptest!` 또는 매크로로 생성된 테스트 | 매개변수화된 테스트 |
| `SetUp()` / `TearDown()` | `Drop`을 통한 RAII — 자동 정리 | 테스트 종료 시 변수 드롭 |
| 별도의 테스트 바이너리 + CMake | `cargo test` — 설정 제로 | |
| `ctest --output-on-failure` | `cargo test -- --nocapture` | |

----

### 통합 테스트: `tests/` 디렉토리

단위 테스트는 코드와 함께 `#[cfg(test)]` 모듈 안에 위치합니다. **통합 테스트**는 크레이트 루트의 별도 `tests/` 디렉토리에 위치하며, 외부 사용자처럼 라이브러리의 공개 API만을 테스트합니다.

```
my_crate/
├── src/
│   └── lib.rs          # 라이브러리 코드
├── tests/
│   ├── smoke.rs        # 각 .rs 파일은 별도의 테스트 바이너리가 됨
│   ├── regression.rs
│   └── common/
│       └── mod.rs      # 공유 테스트 헬퍼 (그 자체로 테스트는 아님)
└── Cargo.toml
```

```rust
// tests/smoke.rs — 외부 사용자 관점에서 크레이트 테스트
use my_crate::DiagEngine;  // 공개 API만 접근 가능

#[test]
fn engine_starts_successfully() {
    let engine = DiagEngine::new("test_config.json");
    assert!(engine.is_ok());
}

#[test]
fn engine_rejects_invalid_config() {
    let engine = DiagEngine::new("nonexistent.json");
    assert!(engine.is_err());
}
```

```rust
// tests/common/mod.rs — 공유 헬퍼, 테스트 바이너리로 컴파일되지 않음
pub fn setup_test_environment() -> tempfile::TempDir {
    let dir = tempfile::tempdir().unwrap();
    std::fs::write(dir.path().join("config.json"), r#"{"log_level": "debug"}"#).unwrap();
    dir
}
```

```rust
// tests/regression.rs — 공유 헬퍼 사용 가능
mod common;

#[test]
fn regression_issue_42() {
    let env = common::setup_test_environment();
    let engine = my_crate::DiagEngine::new(
        env.path().join("config.json").to_str().unwrap()
    );
    assert!(engine.is_ok());
}
```

**통합 테스트 실행:**
```bash
cargo test                          # 단위 테스트와 통합 테스트 모두 실행
cargo test --test smoke             # tests/smoke.rs만 실행
cargo test --test regression        # tests/regression.rs만 실행
cargo test --lib                    # 단위 테스트만 실행 (통합 테스트 제외)
```

> **단위 테스트와의 핵심 차이점**: 통합 테스트는 비공개 함수나 `pub(crate)` 항목에 접근할 수 없습니다. 이는 공개 API가 충분한지 검증하도록 강제하며, 이는 매우 가치 있는 설계 신호입니다. C++로 치면 `friend` 접근 권한 없이 공개 헤더만을 대상으로 테스트하는 것과 같습니다.

----
