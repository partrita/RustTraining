## 검사되지 않은 인덱싱(Unchecked Indexing) 피하기

> **학습 내용:** Rust에서 `vec[i]`가 왜 위험한지(범위 초과 시 패닉 발생), 그리고 `.get()`, 반복자(iterator), `HashMap`의 `entry()` API와 같은 안전한 대안들을 배웁니다. C++의 미정의 동작(undefined behavior)을 명시적인 처리 방식으로 대체합니다.

- C++에서 `vec[i]`와 `map[key]`는 범위 초과 시 미정의 동작을 일으키거나 키가 없을 때 자동으로 삽입을 수행합니다. 반면 Rust의 `[]` 연산자는 범위를 벗어나면 패닉(panic)을 발생시킵니다.
- **규칙**: 인덱스가 유효하다는 것을 *증명*할 수 있는 경우가 아니라면, `[]` 대신 `.get()`을 사용하십시오.

### C++ → Rust 비교
```cpp
// C++ — 조용한 미정의 동작(UB) 또는 자동 삽입
std::vector<int> v = {1, 2, 3};
int x = v[10];        // 미정의 동작! operator[]는 범위를 검사하지 않습니다.

std::map<std::string, int> m;
int y = m["missing"]; // 키가 없으면 값 0과 함께 조용히 삽입합니다!
```

```rust
// Rust — 안전한 대안
let v = vec![1, 2, 3];

// 나쁜 예: 인덱스가 범위를 벗어나면 패닉 발생
// let x = v[10];

// 좋은 예: Option<&i32>를 반환
let x = v.get(10);              // None — 패닉 없음
let x = v.get(1).copied().unwrap_or(0);  // 2, 또는 값이 없으면 0 반환
```

### 실제 예시: 실무 Rust 코드의 안전한 바이트 파싱
```rust
// 예시: diagnostics.rs
// 이진 SEL 레코드를 파싱 — 버퍼가 예상보다 짧을 수 있습니다.
let sensor_num = bytes.get(7).copied().unwrap_or(0);
let ppin = cpu_ppin.get(i).map(|s| s.as_str()).unwrap_or("");
```

### 실제 예시: `.and_then()`을 사용한 체인 형태의 안전한 조회
```rust
// 예시: profile.rs — 이중 조회: HashMap → Vec
pub fn get_processor(&self, location: &str) -> Option<&Processor> {
    self.processor_by_location
        .get(location)                              // HashMap → Option<&usize>
        .and_then(|&idx| self.processors.get(idx))   // Vec → Option<&Processor>
}
// 두 조회 모두 Option을 반환하므로 패닉이나 미정의 동작이 발생하지 않습니다.
```

### 실제 예시: 안전한 JSON 탐색
```rust
// 예시: framework.rs — 모든 JSON 키는 Option을 반환함
let manufacturer = product_fru
    .get("Manufacturer")            // Option<&Value>
    .and_then(|v| v.as_str())       // Option<&str>
    .unwrap_or(UNKNOWN_VALUE)       // &str (안전한 기본값)
    .to_string();
```
C++ 패턴인 `json["SystemInfo"]["ProductFru"]["Manufacturer"]`와 비교해 보십시오. C++에서는 키가 하나라도 없으면 `nlohmann::json::out_of_range` 예외가 발생합니다.

### `[]` 사용이 허용되는 경우
- **범위 검사 직후**: `if i < v.len() { v[i] }`
- **테스트 코드**: 패닉이 발생하는 것이 의도된 동작일 때
- **상수 인덱스**: `assert!(!v.is_empty());` 바로 뒤에서 `let first = v[0];`을 호출할 때

----

## unwrap_or를 사용한 안전한 값 추출

- `unwrap()`은 `None`이나 `Err`일 때 패닉을 일으킵니다. 실무 코드에서는 다음과 같은 안전한 대안들을 선호하십시오.

### unwrap 계열 메서드
| **메서드** | **None/Err 시 동작** | **사용 시점** |
|-----------|------------------------|-------------|
| `.unwrap()` | **패닉 발생** | 테스트 코드 전용, 또는 절대 실패하지 않음을 증명할 수 있을 때 |
| `.expect("메시지")` | 메시지와 함께 패닉 발생 | 패닉이 정당화될 때 사용하며, 그 이유를 설명하십시오. |
| `.unwrap_or(기본값)` | `기본값` 반환 | 저비용의 상수 기본값이 있을 때 |
| `.unwrap_or_else(|| 표현식)` | 클로저 호출 | 기본값을 계산하는 비용이 클 때 |
| `.unwrap_or_default()` | `Default::default()` 반환 | 타입이 `Default`를 구현할 때 |

### 실제 예시: 안전한 기본값을 사용한 파싱
```rust
// 예시: peripherals.rs
// 정규표현식 캡처 그룹이 일치하지 않을 수 있으므로 안전한 기본값을 제공합니다.
let bus_hex = caps.get(1).map(|m| m.as_str()).unwrap_or("00");
let fw_status = caps.get(5).map(|m| m.as_str()).unwrap_or("0x0");
let bus = u8::from_str_radix(bus_hex, 16).unwrap_or(0);
```

### 실제 예시: 기본 구조체를 반환하는 `unwrap_or_else`
```rust
// 예시: framework.rs
// 전체 기능을 Option을 반환하는 클로저로 감쌉니다.
// 하나라도 실패하면 기본 구조체를 반환합니다.
(|| -> Option<BaseboardFru> {
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    // ... .get()? 체인을 사용하여 필드 추출
    Some(baseboard_fru)
})()
.unwrap_or_else(|| BaseboardFru {
    manufacturer: String::new(),
    model: String::new(),
    product_part_number: String::new(),
    serial_number: String::new(),
    asset_tag: String::new(),
})
```

### 실제 예시: JSON 역직렬화 시의 `unwrap_or_default`
```rust
// 예시: framework.rs
// JSON 설정 파싱에 실패하면 크래시 없이 Default 값으로 대체합니다.
Ok(json) => serde_json::from_str(&json).unwrap_or_default(),
```
C++에서는 `nlohmann::json::parse()`를 `try/catch`로 감싸고 catch 블록에서 수동으로 기본값을 생성해야 합니다.

----

## 함수형 변환: map, map_err, find_map

- `Option`과 `Result`의 메서드들을 사용하면 값을 꺼내지(unwrap) 않고도 내부의 값을 변환할 수 있습니다. 이를 통해 중첩된 `if/else`를 선형적인 체인으로 대체할 수 있습니다.

### 빠른 참조
| **메서드** | **대상** | **역할** | **C++ 대응 패턴** |
|-----------|-------|---------|-------------------|
| `.map(|v| ...)` | `Option` / `Result` | `Some`/`Ok` 값 변환 | `if (opt) { *opt = transform(*opt); }` |
| `.map_err(|e| ...)` | `Result` | `Err` 값 변환 | catch 블록에서 컨텍스트 추가 |
| `.and_then(|v| ...)` | `Option` / `Result` | `Option`/`Result`를 반환하는 연산 체이닝 | 중첩된 if 검사 |
| `.find_map(|v| ...)` | 반복자 (Iterator) | `find`와 `map`을 한 번에 수행 | `if + break`가 포함된 루프 |
| `.filter(|v| ...)` | `Option` / 반복자 | 조건에 맞는 값만 유지 | `if (!predicate) return nullopt;` |
| `.ok()?` | `Result` | `Result → Option` 변환 및 `None` 전파 | `if (result.has_error()) return nullopt;` |

### 실제 예시: JSON 필드 추출을 위한 `.and_then()` 체인
```rust
// 예시: framework.rs — 기본값을 포함하여 시리얼 번호 찾기
let sys_info = json.get("SystemInfo")?;

// 먼저 BaseboardFru.BoardSerialNumber 시도
if let Some(serial) = sys_info
    .get("BaseboardFru")
    .and_then(|b| b.get("BoardSerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)     // 비어 있지 않고 유효한 시리얼만 수락
{
    return Some(serial.to_string());
}

// 실패 시 BoardFru.SerialNumber 시도
sys_info
    .get("BoardFru")
    .and_then(|b| b.get("SerialNumber"))
    .and_then(|v| v.as_str())
    .filter(valid_serial)
    .map(|s| s.to_string())   // Some인 경우에만 &str → String 변환
```
C++에서는 `if (json.contains("BaseboardFru")) { if (json["BaseboardFru"].contains("BoardSerialNumber")) { ... } }`와 같은 "이프 피라미드"가 만들어집니다.

### 실제 예시: `find_map` — 한 번의 패스로 검색과 변환 수행
```rust
// 예시: context.rs — 센서와 소유자가 일치하는 SDR 레코드 찾기
pub fn find_for_event(&self, sensor_number: u8, owner_id: u8) -> Option<&SdrRecord> {
    self.by_sensor.get(&sensor_number).and_then(|indices| {
        indices.iter().find_map(|&i| {
            let record = &self.records[i];
            if record.sensor_owner_id() == Some(owner_id) {
                Some(record)
            } else {
                None
            }
        })
    })
}
```
`find_map`은 `find`와 `map`이 합쳐진 형태입니다. 첫 번째 일치하는 항목에서 멈추고 이를 변환합니다. C++에서는 `if`와 `break`가 있는 `for` 루프에 해당합니다.

### 실제 예시: 에러 컨텍스트를 위한 `map_err`
```rust
// 예시: main.rs — 전파하기 전 에러에 컨텍스트 추가
let json_str = serde_json::to_string_pretty(&config)
    .map_err(|e| format!("설정 직렬화 실패: {}", e))?;
```
`serde_json::Error`를 무엇이 실패했는지에 대한 컨텍스트가 포함된 설명적인 `String` 에러로 변환합니다.

----

## JSON 처리: nlohmann::json → serde

- C++ 팀은 보통 JSON 파싱에 `nlohmann::json`을 사용합니다. Rust는 **serde** + **serde_json**을 사용하는데, JSON 스키마가 **타입 시스템**에 인코딩되어 있어 더 강력합니다.

### C++ (nlohmann) 대 Rust (serde) 비교

```cpp
// nlohmann::json을 사용한 C++ — 런타임 필드 접근
#include <nlohmann/json.hpp>
using json = nlohmann::json;

struct Fan {
    std::string logical_id;
    std::vector<std::string> sensor_ids;
};

Fan parse_fan(const json& j) {
    Fan f;
    f.logical_id = j.at("LogicalID").get<std::string>();    // 없으면 예외 발생
    if (j.contains("SDRSensorIdHexes")) {                   // 수동 기본값 처리
        f.sensor_ids = j["SDRSensorIdHexes"].get<std::vector<std::string>>();
    }
    return f;
}
```

```rust
// serde를 사용한 Rust — 컴파일 타임 스키마, 자동 필드 매핑
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Fan {
    pub logical_id: String,
    #[serde(rename = "SDRSensorIdHexes", default)]  // JSON 키 → Rust 필드 매핑
    pub sensor_ids: Vec<String>,                     // 키가 없으면 빈 Vec 반환
    #[serde(default)]
    pub sensor_names: Vec<String>,                   // 키가 없으면 빈 Vec 반환
}

// 단 한 줄로 전체 파싱 함수를 대체합니다:
let fan: Fan = serde_json::from_str(json_str)?;
```

### 주요 serde 속성 (실무 Rust 코드 예시)

| **속성** | **목적** | **C++ 대응 패턴** |
|--------------|------------|--------------------|
| `#[serde(default)]` | 누락된 필드에 `Default::default()` 사용 | `if (j.contains(key)) { ... } else { default; }` |
| `#[serde(rename = "Key")]` | JSON 키 이름을 Rust 필드 이름으로 매핑 | 수동 `j.at("Key")` 접근 |
| `#[serde(flatten)]` | 알 수 없는 키들을 `HashMap`으로 흡수 | `for (auto& [k,v] : j.items()) { ... }` |
| `#[serde(skip)]` | 해당 필드를 직렬화/역직렬화에서 제외 | JSON에 저장하지 않음 |
| `#[serde(tag = "type")]` | 내부 태그형 열거형 (판별 필드) | `if (j["type"] == "gpu") { ... }` |

### 실제 예시: 전체 설정 구조체
```rust
// 예시: diag.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiagConfig {
    pub sku: SkuConfig,
    #[serde(default)]
    pub level: DiagLevel,            // 없으면 DiagLevel::default()
    #[serde(default)]
    pub modules: ModuleConfig,       // 없으면 ModuleConfig::default()
    #[serde(default)]
    pub output_dir: String,          // 없으면 ""
    #[serde(default, flatten)]
    pub options: HashMap<String, serde_json::Value>,  // 알 수 없는 키들을 흡수
}

// 로딩 코드는 3줄이면 충분합니다 (C++ nlohmann의 경우 20줄 이상 소요):
let content = std::fs::read_to_string(path)?;
let config: DiagConfig = serde_json::from_str(&content)?;
Ok(config)
```

### `#[serde(tag = "type")]`를 사용한 열거형 역직렬화
```rust
// 예시: components.rs
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]                   // JSON 예시: {"type": "Gpu", "product": ...}
pub enum PcieDeviceKind {
    Gpu { product: GpuProduct, manufacturer: GpuManufacturer },
    Nic { product: NicProduct, manufacturer: NicManufacturer },
    NvmeDrive { drive_type: StorageDriveType, capacity_gb: u32 },
    // ... 9개 이상의 변형
}
// serde가 "type" 필드를 보고 자동으로 분기합니다 — 수동 if/else 체인이 필요 없습니다.
```
C++에서는 `if (j["type"] == "Gpu") { parse_gpu(j); } else if (j["type"] == "Nic") { parse_nic(j); } ...`와 같이 작성해야 합니다.

# 연습 문제: serde를 사용한 JSON 역직렬화

- 다음 JSON으로부터 역직렬화할 수 있는 `ServerConfig` 구조체를 정의하십시오:
```json
{
    "hostname": "diag-node-01",
    "port": 8080,
    "debug": true,
    "modules": ["accel_diag", "nic_diag", "cpu_diag"]
}
```
- `#[derive(Deserialize)]`와 `serde_json::from_str()`를 사용하여 파싱하십시오.
- `debug` 필드에 `#[serde(default)]`를 추가하여 값이 없을 때 기본값 `false`를 가지도록 하십시오.
- **보너스**: `#[serde(default)]`를 사용하고 기본값이 `Quick`인 `enum DiagLevel { Quick, Full, Extended }` 필드를 추가하십시오.

**시작 코드** (`cargo add serde --features derive` 및 `cargo add serde_json` 필요):
```rust
use serde::Deserialize;

// TODO: Default를 구현한 DiagLevel 열거형 정의

// TODO: serde 속성을 사용한 ServerConfig 구조체 정의

fn main() {
    let json_input = r#"{
        "hostname": "diag-node-01",
        "port": 8080,
        "debug": true,
        "modules": ["accel_diag", "nic_diag", "cpu_diag"]
    }"#;

    // TODO: 역직렬화 및 설정 출력
    // TODO: "debug" 필드가 없는 JSON 파싱 시도 — 기본값 false 확인
}
```

<details><summary>해설 (클릭하여 확장)</summary>

```rust
use serde::Deserialize;

#[derive(Debug, Deserialize, Default)]
enum DiagLevel {
    #[default]
    Quick,
    Full,
    Extended,
}

#[derive(Debug, Deserialize)]
struct ServerConfig {
    hostname: String,
    port: u16,
    #[serde(default)]       // 누락 시 false로 기본 설정
    debug: bool,
    modules: Vec<String>,
    #[serde(default)]       // 누락 시 DiagLevel::Quick으로 기본 설정
    level: DiagLevel,
}

fn main() {
    let json_input = r#"{
        "hostname": "diag-node-01",
        "port": 8080,
        "debug": true,
        "modules": ["accel_diag", "nic_diag", "cpu_diag"]
    }"#;

    let config: ServerConfig = serde_json::from_str(json_input)
        .expect("JSON 파싱 실패");
    println!("{config:#?}");

    // 선택적 필드가 누락된 경우 테스트
    let minimal = r#"{
        "hostname": "node-02",
        "port": 9090,
        "modules": []
    }"#;
    let config2: ServerConfig = serde_json::from_str(minimal)
        .expect("최소 JSON 파싱 실패");
    println!("debug (기본값): {}", config2.debug);    // false
    println!("level (기본값): {:?}", config2.level);  // Quick
}
// 출력:
// ServerConfig {
//     hostname: "diag-node-01",
//     port: 8080,
//     debug: true,
//     modules: ["accel_diag", "nic_diag", "cpu_diag"],
//     level: Quick,
// }
// debug (기본값): false
// level (기본값): Quick
```

</details>

----
