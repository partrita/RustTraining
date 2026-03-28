# C++ 프로그래머를 위한 Rust 테스트 패턴

> **학습 목표:** Rust의 강력한 내장 테스트 프레임워크를 활용하여 고품질의 코드를 작성하는 방법을 배웁니다. `#[test]`부터 `#[should_panic]`, 트레이트 기반 모킹(Mocking), 속성 기반 테스트(`proptest`), 스냅샷 테스트(`insta`) 등 현대적인 테스트 기법들을 다룹니다. Google Test나 CMake 설정 없이도 동작하는 '무설정(Zero-config)' 테스트 환경을 경험해 보세요.

---

### Rust 테스트 시스템의 철학
C++에서는 Google Test, Catch2 같은 외부 프레임워크를 설치하고 CMake 파일에 복잡하게 연결해야 했습니다. Rust는 **언어와 도구(Cargo) 자체에 테스트 시스템이 내장**되어 있어, 추가 설정 없이 즉시 테스트를 시작할 수 있습니다.

### 핵심 테스트 속성(Attributes)

```rust
#[cfg(test)]
mod tests {
    use super::*; // 부모 모듈의 요소를 가져옴

    #[test]
    fn basic_assertion() {
        assert_eq!(2 + 2, 4); // 기본적인 일치 확인
    }

    // 패닉(Panic) 발생 여부 테스트 — GTest의 EXPECT_DEATH와 유사
    #[test]
    #[should_panic(expected = "index out of bounds")]
    fn test_out_of_bounds() {
        let v = vec![1, 2, 3];
        let _ = v[10]; // 여기서 패닉이 발생해야 테스트 통과
    }

    // Result 타입을 반환하는 테스트 — ? 연산자를 활용해 깔끔하게 작성 가능
    #[test]
    fn test_with_result() -> Result<(), String> {
        let val: u32 = "42".parse().map_err(|e| e.to_string())?;
        assert_eq!(val, 42);
        Ok(())
    }

    // 시간이 오래 걸리는 테스트 제외 — `cargo test -- --ignored`로 별도 실행
    #[test]
    #[ignore]
    fn heavy_integration_test() {
        // 복잡하고 느린 시뮬레이션 로직
    }
}
```

### 자주 쓰는 테스트 명령어
- `cargo test`: (무시된 테스트를 제외한) 모든 테스트 실행
- `cargo test -- --ignored`: `#[ignore]` 처리된 테스트만 실행
- `cargo test pattern`: 함수 이름에 'pattern'이 포함된 테스트만 실행
- `cargo test -- --nocapture`: 테스트 통과 시에도 `println!` 출력을 화면에 표시

---

### 1. 테스트 데이터 관리: 빌더 패턴과 피처
C++에서는 `testing::Test` 클래스를 상속받아 픽스처(Fixture)를 만들었지만, Rust는 **빌더 함수**나 **`Default` 트레이트**, 그리고 **`Drop` 트레이트(자동 정리)**를 활용합니다.

```rust
#[cfg(test)]
mod tests {
    use super::*;

    // 테스트용 데이터 생성 헬퍼
    fn create_test_event(code: u32) -> Event {
        Event {
            id: code,
            name: format!("테스트-{code}"),
            is_active: true,
        }
    }

    #[test]
    fn filter_active_events() {
        let events = vec![create_test_event(1), create_test_event(2)];
        assert_eq!(events.len(), 2);
    }
}
```

### 2. 의존성 주입과 모킹(Mocking)
C++에서 Google Mock을 쓰듯, Rust에서는 **트레이트(Trait)**로 인터페이스를 정의하고 테스트용 가짜 구현체를 주입합니다.

```rust
// 1. 공통 인터페이스 정의
trait Device {
    fn read(&self) -> f64;
}

// 2. 실제 장치 구현
struct RealSensor;
impl Device for RealSensor {
    fn read(&self) -> f64 { 72.5 } // 실제 센서 호출 로직
}

// 3. 테스트용 모크 장치
#[cfg(test)]
struct MockSensor { value: f64 }
#[cfg(test)]
impl Device for MockSensor {
    fn read(&self) -> f64 { self.value }
}

// 4. 테스트 대상 함수 (트레이트를 받는 제네릭 함수)
fn check_status(sensor: &impl Device) -> String {
    if sensor.read() > 80.0 { "위험".into() } else { "정상".into() }
}

#[cfg(test)]
#[test]
fn test_sensor_alert() {
    let mock = MockSensor { value: 95.0 };
    assert_eq!(check_status(&mock), "위험");
}
```

---

### 3. 더욱 정교한 테스트 기법들

- **임시 파일 활용**: `tempfile` 크레이트를 쓰면 테스트 종료 시 소멸자(`Drop`)를 통해 생성된 파일이 자동으로 지워집니다. 별도의 `TearDown()`이 필요 없습니다.
- **속성 기반 테스트 (`proptest`)**: 특정 값을 하나씩 넣는 대신, "모든 0~100 사이의 입력에 대해 결과가 참이어야 한다"는 속성을 정의합니다. 데이터 기반 버그를 찾는 데 매우 강력합니다.
- **스냅샷 테스트 (`insta`)**: JSON이나 긴 로그처럼 결과값이 복잡할 때, '기준 스냅샷' 파일과 비교하여 변경 사항을 검토합니다.

---

### C++ (Google Test) vs Rust 테스트 비교 요약

| **항목** | **C++ (Google Test)** | **Rust** | **비고** |
| :--- | :--- | :--- | :--- |
| **테스트 정의** | `TEST(Suite, Name) { ... }` | `#[test] fn name() { ... }` | 상속이나 복잡한 클래스 구조 불필요 |
| **일치 확인** | `ASSERT_EQ(a, b)` | `assert_eq!(a, b)` | 언어 내장 매크로로 깔끔하게 처리 |
| **예외/패직 검사** | `EXPECT_DEATH(expr, "msg")` | `#[should_panic(expected = "msg")]` | 런타임 오류 시나리오 테스트 |
| **픽스처 관리** | `SetUp()` / `TearDown()` | **`Drop` 트레이트 (RAII)** | 테스트 변수가 사라질 때 자동 정리 |
| **모킹 방식** | Google Mock 매크로 활용 | **트레이트 및 제네릭 활용** | 매크로 마법 없이 명시적인 코드 작 |
| **설정 방식** | CMakeLists.txt에 추가 | **설정 불필요 (`cargo test`)** | 생산성 향상의 핵심 포인트 |

---

### 통합 테스트 (Integration Tests)

단위 테스트가 코드 옆에 위치한다면, **통합 테스트**는 프로젝트 루트의 `tests/` 폴더에 위치합니다. 

- **외 관점 테스트**: 통합 테스트는 사용자의 관점에서 라이브러리의 **공개 API만** 호출할 수 있습니다. 비공개 함수는 접근 불가합니다.
- **구성**: `tests/` 내부의 각 `.rs` 파일은 독립적인 테스트 바이너리로 빌드됩니다.

```text
my_project/
├── src/
│   └── lib.rs       # 단위 테스트 포함 가능
├── tests/
│   ├── api_test.rs  # 라이브러리 전체 동작 테스트
│   └── common/      # 테스트용 공유 헬퍼 로직
└── Cargo.toml
```

> **설계 제언**: 통합 테스트에서 코드가 잘 안 돌아간다면, 공개 API가 직관적이지 않거나 불충분하다는 중요한 신호일 수 있습니다. C++에서 `friend` 클래스 없이 공개 헤더만으로 테스트하는 환경과 유사합니다.
