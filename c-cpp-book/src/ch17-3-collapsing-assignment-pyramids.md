## 클로저를 사용한 대입 피라미드 축소하기

> **학습 내용:** Rust의 식(expression) 기반 문법과 클로저를 사용하여, C++에서 흔히 나타나는 깊게 중첩된 `if/else` 검증 체인을 깔끔하고 선형적인 코드로 바꾸는 방법을 배웁니다.

- C++에서는 특히 검증이나 폴백(fallback) 로직이 포함된 경우 변수에 값을 할당하기 위해 여러 블록으로 구성된 `if/else` 체인이 필요한 경우가 많습니다. Rust의 식 기반 문법과 클로저는 이러한 구조를 평면적이고 선형적인 코드로 축소해 줍니다.

### 패턴 1: `if` 식을 사용한 튜플 대입
```cpp
// C++ — 여러 블록의 if/else 체인에 걸쳐 세 개의 변수를 설정합니다.
uint32_t fault_code;
const char* der_marker;
const char* action;
if (is_c44ad) {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
} else if (error.is_hardware_error()) {
    fault_code = 67956; der_marker = "CSI_ERR"; action = "Replace GPU";
} else {
    fault_code = 32709; der_marker = "CSI_WARN"; action = "No action";
}
```

```rust
// Rust 대응 예시: accel_fieldiag.rs
// 단일 식(expression)으로 세 변수를 한 번에 할당합니다.
let (fault_code, der_marker, recommended_action) = if is_c44ad {
    (32709u32, "CSI_WARN", "No action")
} else if error.is_hardware_error() {
    (67956u32, "CSI_ERR", "Replace GPU")
} else {
    (32709u32, "CSI_WARN", "No action")
};
```

### 패턴 2: 실패 가능한 체인을 위한 IIFE (즉시 실행 함수 식)
```cpp
// C++ — JSON 탐색을 위한 "죽음의 피라미드"
std::string get_part_number(const nlohmann::json& root) {
    if (root.contains("SystemInfo")) {
        auto& sys = root["SystemInfo"];
        if (sys.contains("BaseboardFru")) {
            auto& bb = sys["BaseboardFru"];
            if (bb.contains("ProductPartNumber")) {
                return bb["ProductPartNumber"].get<std::string>();
            }
        }
    }
    return "UNKNOWN";
}
```

```rust
// Rust 대응 예시: framework.rs
// 클로저와 ? 연산자를 사용하여 피라미드를 선형적인 코드로 축소합니다.
let part_number = (|| -> Option<String> {
    let path = self.args.sysinfo.as_ref()?;
    let content = std::fs::read_to_string(path).ok()?;
    let json: serde_json::Value = serde_json::from_str(&content).ok()?;
    let ppn = json
        .get("SystemInfo")?
        .get("BaseboardFru")?
        .get("ProductPartNumber")?
        .as_str()?;
    Some(ppn.to_string())
})()
.unwrap_or_else(|| "UNKNOWN".to_string());
```
클로저는 `Option<String>` 스코프를 생성하며, 각 단계에서 `?` 연산자가 실패 시 조기 반환(early exit)을 수행합니다. `.unwrap_or_else()`는 마지막에 한 번만 폴백 값을 제공합니다.

### 패턴 3: 수동 루프와 push_back을 대체하는 반복자 체인
```cpp
// C++ — 중간 변수가 포함된 수동 루프
std::vector<std::tuple<std::vector<std::string>, std::string, std::string>> gpu_info;
for (const auto& [key, info] : gpu_pcie_map) {
    std::vector<std::string> bdfs;
    // ... bdf_path를 bdfs로 파싱하는 과정
    std::string serial = info.serial_number.value_or("UNKNOWN");
    std::string model = info.model_number.value_or(model_name);
    gpu_info.push_back({bdfs, serial, model});
}
```

```rust
// Rust 대응 예시: peripherals.rs
// 단일 체인: values() → map → collect
let gpu_info: Vec<(Vec<String>, String, String, String)> = self
    .gpu_pcie_map
    .values()
    .map(|info| {
        let bdfs: Vec<String> = info.bdf_path
            .split(')')
            .filter(|s| !s.is_empty())
            .map(|s| s.trim_start_matches('(').to_string())
            .collect();
        let serial = info.serial_number.clone()
            .unwrap_or_else(|| "UNKNOWN".to_string());
        let model = info.model_number.clone()
            .unwrap_or_else(|| model_name.to_string());
        let gpu_bdf = format!("{}:{}:{}.{}",
            info.bdf.segment, info.bdf.bus, info.bdf.device, info.bdf.function);
        (bdfs, serial, model, gpu_bdf)
    })
    .collect();
```

### 패턴 4: 루프와 `if (condition) continue`를 대체하는 `.filter().collect()`
```cpp
// C++
std::vector<TestResult*> failures;
for (auto& t : test_results) {
    if (!t.is_pass()) {
        failures.push_back(&t);
    }
}
```

```rust
// Rust 대응 예시: accel_diag/src/healthcheck.rs
pub fn failed_tests(&self) -> Vec<&TestResult> {
    self.test_results.iter().filter(|t| !t.is_pass()).collect()
}
```

### 요약: 각 패턴을 사용하는 시점
| **C++ 패턴** | **Rust 대체 방식** | **핵심 장점** |
|----------------|---------------------|-----------------|
| 여러 블록의 변수 할당 | `let (a, b) = if ... { } else { };` | 모든 변수가 원자적으로 바인딩됨 |
| 중첩된 `if (contains)` 피라미드 | `?` 연산자를 포함한 IIFE 클로저 | 선형적이고 평면적인 구조, 조기 반환 가능 |
| `for` 루프 + `push_back` | `.iter().map(||).collect()` | 중간 단계의 가변(mutable) Vec이 필요 없음 |
| `for` + `if (cond) continue` | `.iter().filter(||).collect()` | 선언적인 의도 표현 |
| `for` + `if + break` (첫 번째 항목 찾기) | `.iter().find_map(||)` | 한 번의 패스로 검색과 변환 수행 |

----

# 종합 실습: 진단 이벤트 파이프라인

🔴 **도전 과제** — 열거형, 트레이트, 반복자, 에러 처리, 제네릭을 결합한 통합 실습입니다.

이 실습에서는 열거형, 트레이트, 반복자, 에러 처리, 제네릭을 모두 활용합니다. 실제 Rust 실무 코드에서 사용되는 패턴과 유사한 간단한 진단 이벤트 처리 파이프라인을 구축해 봅니다.

**요구 사항:**
1. `Display`를 구현한 `enum Severity { Info, Warning, Critical }`와 `source: String`, `severity: Severity`, `message: String`, `fault_code: u32`를 포함하는 `struct DiagEvent`를 정의하십시오.
2. `fn should_include(&self, event: &DiagEvent) -> bool` 메서드를 가진 `trait EventFilter`를 정의하십시오.
3. 두 가지 필터를 구현하십시오: `SeverityFilter` (특정 심각도 이상의 이벤트만 허용)와 `SourceFilter` (특정 소스 문자열의 이벤트만 허용).
4. **모든** 필터를 통과한 이벤트에 대해 포맷된 보고서 라인을 반환하는 `fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String>` 함수를 작성하십시오.
5. `"source:severity:fault_code:message"` 형식의 문자열을 파싱하는 `fn parse_event(line: &str) -> Result<DiagEvent, String>` 함수를 작성하십시오 (잘못된 입력에 대해서는 `Err` 반환).

**시작 코드:**
```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        todo!()
    }
}

#[derive(Debug, Clone)]
struct DiagEvent {
    source: String,
    severity: Severity,
    message: String,
    fault_code: u32,
}

trait EventFilter {
    fn should_include(&self, event: &DiagEvent) -> bool;
}

struct SeverityFilter {
    min_severity: Severity,
}
// TODO: SeverityFilter에 대해 EventFilter 구현

struct SourceFilter {
    source: String,
}
// TODO: SourceFilter에 대해 EventFilter 구현

fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
    // TODO: 모든 필터를 통과하는 이벤트를 필터링하고, 다음 형식으로 포맷팅하십시오.
    // "[SEVERITY] source (FC:fault_code): message"
    todo!()
}

fn parse_event(line: &str) -> Result<DiagEvent, String> {
    // "source:severity:fault_code:message" 파싱
    // 유효하지 않은 입력에 대해 Err 반환
    todo!()
}

fn main() {
    let raw_lines = vec![
        "accel_diag:Critical:67956:ECC uncorrectable error detected",
        "nic_diag:Warning:32709:Link speed degraded",
        "accel_diag:Info:10001:Self-test passed",
        "cpu_diag:Critical:55012:Thermal throttling active",
        "accel_diag:Warning:32710:PCIe link width reduced",
    ];

    // 모든 라인을 파싱하고, 성공한 것들만 수집하며 에러는 보고합니다.
    let events: Vec<DiagEvent> = raw_lines.iter()
        .filter_map(|line| match parse_event(line) {
            Ok(e) => Some(e),
            Err(e) => { eprintln!("파싱 에러: {e}"); None }
        })
        .collect();

    // 필터 적용: accel_diag 소스의 Warning 이상 이벤트만 추출
    let sev_filter = SeverityFilter { min_severity: Severity::Warning };
    let src_filter = SourceFilter { source: "accel_diag".to_string() };
    let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];

    let report = process_events(&events, &filters);
    for line in &report {
        println!("{line}");
    }
    println!("--- {}개의 이벤트가 일치함 ---", report.len());
}
```

<details><summary>해설 (클릭하여 확장)</summary>

```rust
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Severity {
    Info,
    Warning,
    Critical,
}

impl fmt::Display for Severity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Severity::Info => write!(f, "INFO"),
            Severity::Warning => write!(f, "WARNING"),
            Severity::Critical => write!(f, "CRITICAL"),
        }
    }
}

impl Severity {
    fn from_str(s: &str) -> Result<Self, String> {
        match s {
            "Info" => Ok(Severity::Info),
            "Warning" => Ok(Severity::Warning),
            "Critical" => Ok(Severity::Critical),
            other => Err(format!("알 수 없는 심각도: {other}")),
        }
    }
}

#[derive(Debug, Clone)]
struct DiagEvent {
    source: String,
    severity: Severity,
    message: String,
    fault_code: u32,
}

trait EventFilter {
    fn should_include(&self, event: &DiagEvent) -> bool;
}

struct SeverityFilter {
    min_severity: Severity,
}

impl EventFilter for SeverityFilter {
    fn should_include(&self, event: &DiagEvent) -> bool {
        event.severity >= self.min_severity
    }
}

struct SourceFilter {
    source: String,
}

impl EventFilter for SourceFilter {
    fn should_include(&self, event: &DiagEvent) -> bool {
        event.source == self.source
    }
}

fn process_events(events: &[DiagEvent], filters: &[&dyn EventFilter]) -> Vec<String> {
    events.iter()
        .filter(|e| filters.iter().all(|f| f.should_include(e)))
        .map(|e| format!("[{}] {} (FC:{}): {}", e.severity, e.source, e.fault_code, e.message))
        .collect()
}

fn parse_event(line: &str) -> Result<DiagEvent, String> {
    let parts: Vec<&str> = line.splitn(4, ':').collect();
    if parts.len() != 4 {
        return Err(format!("콜론으로 구분된 4개의 필드가 필요합니다. (입력 필드 수: {})", parts.len()));
    }
    let fault_code = parts[2].parse::<u32>()
        .map_err(|e| format!("유효하지 않은 결함 코드 '{}': {e}", parts[2]))?;
    Ok(DiagEvent {
        source: parts[0].to_string(),
        severity: Severity::from_str(parts[1])?,
        fault_code,
        message: parts[3].to_string(),
    })
}

fn main() {
    let raw_lines = vec![
        "accel_diag:Critical:67956:ECC uncorrectable error detected",
        "nic_diag:Warning:32709:Link speed degraded",
        "accel_diag:Info:10001:Self-test passed",
        "cpu_diag:Critical:55012:Thermal throttling active",
        "accel_diag:Warning:32710:PCIe link width reduced",
    ];

    let events: Vec<DiagEvent> = raw_lines.iter()
        .filter_map(|line| match parse_event(line) {
            Ok(e) => Some(e),
            Err(e) => { eprintln!("파싱 에러: {e}"); None }
        })
        .collect();

    let sev_filter = SeverityFilter { min_severity: Severity::Warning };
    let src_filter = SourceFilter { source: "accel_diag".to_string() };
    let filters: Vec<&dyn EventFilter> = vec![&sev_filter, &src_filter];

    let report = process_events(&events, &filters);
    for line in &report {
        println!("{line}");
    }
    println!("--- {}개의 이벤트가 일치함 ---", report.len());
}
// 출력:
// [CRITICAL] accel_diag (FC:67956): ECC uncorrectable error detected
// [WARNING] accel_diag (FC:32710): PCIe link width reduced
// --- 2개의 이벤트가 일치함 ---
```

</details>

----
