# 사례 연구 3: 프레임워크 통신 → 수명 기반 빌림 (Lifetime borrowing)

> **학습 내용:** C++의 생포인터(raw-pointer) 프레임워크 통신 패턴을 Rust의 수명(lifetime) 기반 빌림 시스템으로 전환하는 방법을 배웁니다. 이를 통해 제로 비용 추상화(zero-cost abstractions)를 유지하면서 댕글링 포인터(dangling pointer) 위험을 제거할 수 있습니다.

## C++ 패턴: 프레임워크에 대한 생포인터
```cpp
// C++ 원본: 모든 진단 모듈이 프레임워크에 대한 생포인터를 저장합니다.
class DiagBase {
protected:
    DiagFramework* m_pFramework;  // 생포인터 — 소유권자가 누구인가요?
public:
    DiagBase(DiagFramework* fw) : m_pFramework(fw) {}
    
    void LogEvent(uint32_t code, const std::string& msg) {
        m_pFramework->GetEventLog()->Record(code, msg);  // 프레임워크가 아직 유효하기를 바랄 뿐입니다!
    }
};
// 문제점: m_pFramework는 수명 보장이 없는 생포인터입니다.
// 모듈이 여전히 프레임워크를 참조하고 있는데 프레임워크가 먼저 파괴되면 → 미정의 동작(Undefined Behavior, UB) 발생
```

## Rust 해결책: 수명 기반 빌림을 사용하는 DiagContext
```rust
// 예시: module.rs — 저장하지 말고 빌리십시오.

/// 실행 중 진단 모듈에 전달되는 컨텍스트입니다.
/// 수명 'a는 프레임워크가 컨텍스트보다 더 오래 생존함을 보장합니다.
pub struct DiagContext<'a> {
    pub der_log: &'a mut EventLogManager,
    pub config: &'a ModuleConfig,
    pub framework_opts: &'a HashMap<String, String>,
}

/// 모듈은 프레임워크 포인터를 저장하지 않고 컨텍스트를 매개변수로 받습니다.
pub trait DiagModule {
    fn id(&self) -> &str;
    fn execute(&mut self, ctx: &mut DiagContext) -> DiagResult<()>;
    fn pre_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
    fn post_execute(&mut self, _ctx: &mut DiagContext) -> DiagResult<()> {
        Ok(())
    }
}
```

### 핵심 통찰 (Key Insight)
- C++ 모듈은 프레임워크에 대한 포인터를 **저장**합니다 (위험: 프레임워크가 먼저 파괴되면 어떻게 될까요?).
- Rust 모듈은 함수 매개변수로 컨텍스트를 **받습니다**. 빌림 검사기(borrow checker)는 호출 중에 프레임워크가 살아 있음을 보장합니다.
- 생포인터도, 수명 모호성도, "아직 살아 있기를 바라는" 불확실성도 없습니다.

----

# 사례 연구 4: 거대 객체(God object) → 조합 가능한 상태 (Composable state)

## C++ 패턴: 단일 프레임워크 클래스 (Monolithic Framework Class)
```cpp
// C++ 원본: 프레임워크가 모든 것을 담당하는 거대 객체(god object)입니다.
class DiagFramework {
    // 상태 모니터 트랩 처리 (Health-monitor trap processing)
    std::vector<AlertTriggerInfo> m_alertTriggers;
    std::vector<WarnTriggerInfo> m_warnTriggers;
    bool m_healthMonHasBootTimeError;
    uint32_t m_healthMonActionCounter;
    
    // GPU 진단
    std::map<uint32_t, GpuPcieInfo> m_gpuPcieMap;
    bool m_isRecoveryContext;
    bool m_healthcheckDetectedDevices;
    // ... 30개 이상의 GPU 관련 필드
    
    // PCIe 트리
    std::shared_ptr<CPcieTreeLinux> m_pPcieTree;
    
    // 이벤트 로깅
    CEventLogMgr* m_pEventLogMgr;
    
    // ... 기타 여러 메서드
    void HandleGpuEvents();
    void HandleNicEvents();
    void RunGpuDiag();
    // 모든 것이 서로에게 의존합니다.
};
```

## Rust 해결책: 조합 가능한 상태 구조체
```rust
// 예시: main.rs — 상태를 집중된 구조체들로 분해합니다.

#[derive(Default)]
struct HealthMonitorState {
    alert_triggers: Vec<AlertTriggerInfo>,
    warn_triggers: Vec<WarnTriggerInfo>,
    health_monitor_action_counter: u32,
    health_monitor_has_boot_time_error: bool,
    // 상태 모니터와 관련된 필드만 포함합니다.
}

#[derive(Default)]
struct GpuDiagState {
    gpu_pcie_map: HashMap<u32, GpuPcieInfo>,
    is_recovery_context: bool,
    healthcheck_detected_devices: bool,
    // GPU와 관련된 필드만 포함합니다.
}

/// 프레임워크는 모든 것을 평면적으로 소유하는 대신 이러한 상태들을 조합합니다.
struct DiagFramework {
    ctx: DiagContext,             // 실행 컨텍스트
    args: Args,                   // CLI 인자
    pcie_tree: Option<DeviceTree>,  // shared_ptr이 필요 없습니다.
    event_log_mgr: EventLogManager,   // 생포인터가 아닌 소유된 객체입니다.
    fc_manager: FcManager,        // 결함 코드(Fault code) 관리
    health: HealthMonitorState,   // 상태 모니터 상태 — 별도의 구조체
    gpu: GpuDiagState,           // GPU 상태 — 별도의 구조체
}
```

### 핵심 통찰 (Key Insight)
- **테스트 가능성**: 각 상태 구조체를 독립적으로 단위 테스트(unit-test)할 수 있습니다.
- **가독성**: `m_alertTriggers` 대신 `self.health.alert_triggers`를 사용함으로써 소유권이 명확해집니다.
- **두려움 없는 리팩터링**: `GpuDiagState`를 변경해도 상태 모니터 처리에 실수로 영향을 줄 수 없습니다.
- **메서드 난립 방지**: 상태 모니터 상태만 필요한 함수는 프레임워크 전체가 아닌 `&mut HealthMonitorState`를 인자로 받습니다.

----

# 사례 연구 5: 트레이트 객체(Trait objects) — 정말로 필요할 때

- 모든 것을 열거형(enum)으로 만들어서는 안 됩니다! **진단 모듈 플러그인 시스템**은 트레이트 객체를 사용하는 정당한 사례입니다.
- 이유: 진단 모듈은 **확장에 열려(open for extension)** 있어야 하기 때문입니다. 즉, 프레임워크를 수정하지 않고도 새로운 모듈을 추가할 수 있어야 합니다.

```rust
// 예시: framework.rs — 여기서는 Vec<Box<dyn DiagModule>>이 적합합니다.
pub struct DiagFramework {
    modules: Vec<Box<dyn DiagModule>>,        // 런타임 다형성
    pre_diag_modules: Vec<Box<dyn DiagModule>>,
    event_log_mgr: EventLogManager,
    // ...
}

impl DiagFramework {
    /// 진단 모듈 등록 — DiagModule을 구현하는 모든 타입 가능
    pub fn register_module(&mut self, module: Box<dyn DiagModule>) {
        info!("모듈 등록 중: {}", module.id());
        self.modules.push(module);
    }
}
```

### 각 패턴을 사용하는 시점

| **사용 사례** | **패턴** | **이유** |
|-------------|-----------|--------|
| 컴파일 타임에 알려진 고정된 변형 세트 | `enum` + `match` | 철저한 검사 가능, 가상 함수 테이블(vtable) 없음 |
| 하드웨어 이벤트 타입 (Degrade, Fatal, Boot, ...) | `enum GpuEventKind` | 모든 변형을 알고 있으며 성능이 중요함 |
| PCIe 장치 타입 (GPU, NIC, Switch, ...) | `enum PcieDeviceKind` | 고정된 세트이며, 각 변형이 서로 다른 데이터를 가짐 |
| 플러그인/모듈 시스템 (확장에 열려 있음) | `Box<dyn Trait>` | 프레임워크 수정 없이 새로운 모듈 추가 가능 |
| 테스트 모킹 (Mocking) | `Box<dyn Trait>` | 테스트 더블(test double) 주입 가능 |

### 연습 문제: 번역하기 전에 생각하십시오
다음 C++ 코드가 주어졌을 때:
```cpp
class Shape { public: virtual double area() = 0; };
class Circle : public Shape { double r; double area() override { return 3.14*r*r; } };
class Rect : public Shape { double w, h; double area() override { return w*h; } };
std::vector<std::unique_ptr<Shape>> shapes;
```
**질문**: Rust로 번역할 때 `enum Shape`를 사용해야 할까요, 아니면 `Vec<Box<dyn Shape>>`를 사용해야 할까요?

<details><summary>해설 (클릭하여 확장)</summary>

**정답**: `enum Shape` — 도형의 세트가 **닫혀(closed)** 있기(컴파일 타임에 알려져 있기) 때문입니다. 사용자가 런타임에 새로운 도형 타입을 추가할 수 있어야 할 때만 `Box<dyn Shape>`를 사용하십시오.

```rust
// 올바른 Rust 번역:
enum Shape {
    Circle { r: f64 },
    Rect { w: f64, h: f64 },
}

impl Shape {
    fn area(&self) -> f64 {
        match self {
            Shape::Circle { r } => std::f64::consts::PI * r * r,
            Shape::Rect { w, h } => w * h,
        }
    }
}

fn main() {
    let shapes: Vec<Shape> = vec![
        Shape::Circle { r: 5.0 },
        Shape::Rect { w: 3.0, h: 4.0 },
    ];
    for shape in &shapes {
        println!("면적: {:.2}", shape.area());
    }
}
// 출력:
// 면적: 78.54
// 면적: 12.00
```

</details>

----

# 번역 지표 및 교훈

## 우리가 배운 것
1. **기본적으로 열거형 디스패치(enum dispatch)를 사용하십시오** — 약 10만 줄의 C++ 코드에서 `Box<dyn Trait>`가 정말로 필요한 경우는 약 25군데(플러그인 시스템, 테스트 모크)뿐이었습니다. 나머지 약 900개의 가상 메서드는 match를 사용하는 열거형으로 바뀌었습니다.
2. **아레나(Arena) 패턴으로 참조 순환을 제거하십시오** — `shared_ptr`과 `enable_shared_from_this`는 불명확한 소유권의 징후입니다. 먼저 데이터의 **소유권자**가 누구인지 생각하십시오.
3. **포인터를 저장하지 말고 컨텍스트를 전달하십시오** — 수명이 제한된 `DiagContext<'a>`는 모든 모듈에 `Framework*`를 저장하는 것보다 안전하고 명확합니다.
4. **거대 객체(god object)를 분해하십시오** — 구조체에 30개 이상의 필드가 있다면, 그것은 아마도 트렌치코트를 입은 3~4개의 구조체일 가능성이 높습니다.
5. **컴파일러는 당신의 페어 프로그래머입니다** — 약 400개의 `dynamic_cast` 호출은 약 400개의 잠재적인 런타임 실패를 의미했습니다. Rust에서 `dynamic_cast`에 해당하는 기능이 없다는 것은 런타임 타입 에러가 없음을 의미합니다.

## 가장 어려웠던 부분
- **수명 주석 (Lifetime annotations)**: 생포인터에 익숙해져 있을 때는 빌림을 올바르게 설정하는 데 시간이 걸립니다. 하지만 일단 컴파일이 되면, 그것은 올바른 코드입니다.
- **빌림 검사기와의 싸움**: `&mut self`를 동시에 두 군데에서 사용하고 싶어지는 경우가 있습니다. 해결책: 상태를 별도의 구조체로 분해하십시오.
- **직역하고 싶은 유혹**: 모든 곳에 `Vec<Box<dyn Base>>`를 쓰고 싶어집니다. "이 변형 세트가 닫혀 있는가?"라고 자문해 보십시오. 만약 그렇다면 열거형을 사용하십시오.

## C++ 팀을 위한 권장 사항
1. 거대 객체가 아닌 작고 독립적인 모듈부터 시작하십시오.
2. 데이터 구조를 먼저 번역한 다음 동작을 번역하십시오.
3. 컴파일러의 안내를 따르십시오. 에러 메시지는 매우 훌륭합니다.
4. `dyn Trait`를 고려하기 전에 `enum`을 먼저 검토하십시오.
5. 통합하기 전에 [Rust playground](https://play.rust-lang.org/)를 사용하여 패턴을 프로토타이핑하십시오.

----
