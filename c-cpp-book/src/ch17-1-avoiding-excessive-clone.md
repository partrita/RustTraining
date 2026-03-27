## 과도한 clone() 호출 피하기

> **학습 내용:** Rust에서 `.clone()`이 왜 코드 스멜(code smell)로 간주되는지, 불필요한 복사를 제거하기 위해 소유권 구조를 재설정하는 방법, 그리고 소유권 설계 문제를 나타내는 특정 패턴들을 배웁니다.

- C++에서 온 개발자에게 `.clone()`은 "그냥 복사하면 된다"는 식의 안전한 기본 선택처럼 느껴질 수 있습니다. 하지만 과도한 클로닝(cloning)은 소유권 문제를 숨기고 성능을 저하시킵니다.
- **경험 법칙 (Rule of thumb)**: 빌림 검사기(borrow checker)를 만족시키기 위해 clone을 사용하고 있다면, 아마도 소유권 구조를 재설정해야 할 시점일 것입니다.

### clone()이 잘못 사용되는 경우

```rust
// 나쁜 예: 단순히 읽기만 하는 함수에 전달하기 위해 String을 복사함
fn log_message(msg: String) {  // 불필요하게 소유권을 가져갑니다.
    println!("[로그] {}", msg);
}
let message = String::from("GPU 테스트 통과");
log_message(message.clone());  // 낭비: 완전히 새로운 String을 할당합니다.
log_message(message);           // 원본이 소비됨 — 이전의 clone은 무의미했습니다.
```

```rust
// 좋은 예: 빌림(borrow)을 사용 — 할당 발생 안 함 (zero allocation)
fn log_message(msg: &str) {    // 소유하지 않고 빌립니다.
    println!("[로그] {}", msg);
}
let message = String::from("GPU 테스트 통과");
log_message(&message);          // clone 없음, 할당 없음
log_message(&message);          // 다시 호출 가능 — message가 소비되지 않음
```

### 실제 예시: clone 대신 `&str` 반환하기
```rust
// 예시: healthcheck.rs — 빌려온 뷰를 반환하며, 할당이 발생하지 않음
pub fn serial_or_unknown(&self) -> &str {
    self.serial.as_deref().unwrap_or(UNKNOWN_VALUE)
}

pub fn model_or_unknown(&self) -> &str {
    self.model.as_deref().unwrap_or(UNKNOWN_VALUE)
}
```
C++라면 `const std::string&`이나 `std::string_view`를 반환했을 것입니다. 하지만 C++에서는 둘 다 수명 검사가 이루어지지 않습니다. Rust에서는 빌림 검사기가 반환된 `&str`이 `self`보다 오래 살아남을 수 없음을 보장합니다.

### 실제 예시: 정적 문자열 슬라이스 (Static string slices) — 힙 할당 없음
```rust
// 예시: healthcheck.rs — 컴파일 타임 문자열 테이블
const HBM_SCREEN_RECIPES: &[&str] = &[
    "hbm_ds_ntd", "hbm_ds_ntd_gfx", "hbm_dt_ntd", "hbm_dt_ntd_gfx",
    "hbm_burnin_8h", "hbm_burnin_24h",
];
```
C++에서는 보통 `std::vector<std::string>`을 사용하며 첫 사용 시 힙에 할당될 것입니다. Rust의 `&'static [&'static str]`은 읽기 전용 메모리에 위치하며 런타임 비용이 전혀 없습니다.

### clone()이 적절한 경우

| **상황** | **clone이 괜찮은 이유** | **예시** |
|--------------|--------------------|-----------|
| 스레딩을 위한 `Arc::clone()` | 참조 횟수만 증가시킴 (~1 ns), 데이터를 복사하지 않음 | `let flag = stop_flag.clone();` |
| 생성된 스레드로 데이터를 이동할 때 | 스레드가 자신만의 복사본을 필요로 함 | `let ctx = ctx.clone(); thread::spawn(move || { ... })` |
| `&self` 필드에서 값을 추출할 때 | 빌려온 상태에서 값을 이동(move)시킬 수 없음 | 소유권이 있는 `String`을 반환할 때의 `self.name.clone()` |
| `Option`으로 감싸진 작은 `Copy` 타입 | `.clone()`보다 `.copied()`가 더 명확함 | `Option<&u32>`를 `Option<u32>`로 바꿀 때의 `opt.get(0).copied()` |

### 실제 예시: 스레드 공유를 위한 Arc::clone
```rust
// 예시: workload.rs — Arc::clone은 비용이 저렴함 (참조 횟수 증가)
let stop_flag = Arc::new(AtomicBool::new(false));
let stop_flag_clone = stop_flag.clone();   // 약 1 ns, 데이터 복사 없음
let ctx_clone = ctx.clone();               // 스레드로 이동시키기 위해 컨텍스트 복제

let sensor_handle = thread::spawn(move || {
    // ... stop_flag_clone과 ctx_clone을 사용합니다.
});
```

### 체크리스트: clone을 해야 할까요?
1. **`String` / `T` 대신 `&str` / `&T`를 받을 수 있나요?** → 복제하지 말고 빌리십시오.
2. **두 명의 소유자가 필요 없도록 구조를 변경할 수 있나요?** → 참조로 전달하거나 스코프(scope)를 활용하십시오.
3. **이것이 `Arc::clone()`인가요?** → 괜찮습니다. O(1) 연산입니다.
4. **데이터를 스레드나 클로저로 이동시키고 있나요?** → clone이 필요합니다.
5. **빈번하게 호출되는 루프(hot loop) 안에서 clone을 하고 있나요?** → 프로파일링을 해보고 빌림이나 `Cow<T>` 사용을 고려하십시오.

----

## `Cow<'a, T>`: 쓰기 시 복제 (Clone-on-Write) — 가능하면 빌리고, 필요할 때만 복제하기

`Cow` (Clone on Write)는 **빌려온 참조** 또는 **소유한 값** 중 하나를 가질 수 있는 열거형입니다. 이는 Rust에서 "가능한 경우 할당을 피하되, 수정이 필요한 경우에만 할당한다"는 개념을 구현한 것입니다. C++에는 직접적인 대응물이 없으며, 상황에 따라 `const std::string&` 또는 `std::string`을 반환하는 함수와 가장 비슷합니다.

### `Cow`가 존재하는 이유

```rust
// Cow가 없는 경우 — 항상 빌리거나 항상 복제하는 것 중 하나를 선택해야 합니다.
fn normalize(s: &str) -> String {          // 항상 할당이 발생합니다!
    if s.contains(' ') {
        s.replace(' ', "_")               // 새로운 String (할당 필요)
    } else {
        s.to_string()                     // 불필요한 할당!
    }
}

// Cow를 사용하는 경우 — 변경되지 않았을 때는 빌리고, 수정되었을 때만 할당합니다.
use std::borrow::Cow;

fn normalize(s: &str) -> Cow<'_, str> {
    if s.contains(' ') {
        Cow::Owned(s.replace(' ', "_"))    // 할당 발생 (수정 필요)
    } else {
        Cow::Borrowed(s)                   // 할당 없음 (그대로 전달)
    }
}
```

### `Cow` 작동 방식

```rust
use std::borrow::Cow;

// Cow<'a, str>은 본질적으로 다음과 같습니다:
// enum Cow<'a, str> {
//     Borrowed(&'a str),     // 비용 없는 참조
//     Owned(String),          // 힙에 할당된 소유권이 있는 값
// }

fn greet(name: &str) -> Cow<'_, str> {
    if name.is_empty() {
        Cow::Borrowed("stranger")         // 정적 문자열 — 할당 없음
    } else if name.starts_with(' ') {
        Cow::Owned(name.trim().to_string()) // 수정됨 — 할당 필요
    } else {
        Cow::Borrowed(name)               // 그대로 전달 — 할당 없음
    }
}

fn main() {
    let g1 = greet("Alice");     // Cow::Borrowed("Alice")
    let g2 = greet("");          // Cow::Borrowed("stranger")
    let g3 = greet(" Bob ");     // Cow::Owned("Bob")
    
    // Cow<str>은 Deref<Target = str>를 구현하므로 &str처럼 사용할 수 있습니다.
    println!("Hello, {g1}!");    // 작동함 — Cow가 자동으로 &str로 역참조(deref)됩니다.
    println!("Hello, {g2}!");
    println!("Hello, {g3}!");
}
```

### 실제 활용 사례: 설정값 정규화

```rust
use std::borrow::Cow;

/// SKU 이름을 정규화합니다: 공백 제거, 소문자 변환.
/// 이미 정규화되어 있다면 Cow::Borrowed를 반환합니다 (할당 없음).
fn normalize_sku(sku: &str) -> Cow<'_, str> {
    let trimmed = sku.trim();
    if trimmed == sku && sku.chars().all(|c| c.is_lowercase() || !c.is_alphabetic()) {
        Cow::Borrowed(sku)   // 이미 정규화됨 — 할당 없음
    } else {
        Cow::Owned(trimmed.to_lowercase())  // 수정 필요 — 할당 발생
    }
}

fn main() {
    let s1 = normalize_sku("server-x1");   // Borrowed — 할당 없음
    let s2 = normalize_sku("  Server-X1 "); // Owned — 할당 필요
    println!("{s1}, {s2}"); // "server-x1, server-x1"
}
```

### `Cow`를 사용하는 시점

| **상황** | **Cow를 사용할까요?** |
|--------------|---------------|
| 함수가 대부분의 경우 입력값을 변경하지 않고 반환할 때 | ✅ 예 — 불필요한 clone 방지 |
| 문자열 파싱 또는 정규화 (trim, 소문자 변환, 치환 등) | ✅ 예 — 입력값이 이미 유효한 경우가 많음 |
| 항상 수정이 발생하여 모든 경로에서 할당이 일어날 때 | ❌ 아니요 — 그냥 `String`을 반환하십시오. |
| 단순히 값을 전달만 하고 절대 수정하지 않을 때 | ❌ 아니요 — 그냥 `&str`을 반환하십시오. |
| 구조체에 장기적으로 데이터를 저장할 때 | ❌ 아니요 — 소유권이 있는 `String`을 사용하십시오. |

> **C++ 비교**: `Cow<str>`은 자동 역참조 기능이 있고 값 접근을 위한 보일러플레이트 코드가 없는 `std::variant<std::string_view, std::string>`을 반환하는 함수와 비슷합니다.

----

## `Weak<T>`: 참조 순환 끊기 — Rust의 `weak_ptr`

`Weak<T>`는 C++의 `std::weak_ptr<T>`에 대응하는 Rust의 개념입니다. 이는 `Rc<T>` 또는 `Arc<T>` 값에 대한 소유하지 않는 참조를 유지합니다. `Weak` 참조가 남아 있더라도 원래 값은 할당 해제될 수 있으며, 값이 사라진 경우 `upgrade()`를 호출하면 `None`이 반환됩니다.

### `Weak`가 존재하는 이유

`Rc<T>`와 `Arc<T>`는 두 값이 서로를 가리킬 경우 참조 순환(reference cycle)을 생성합니다. 이 경우 참조 횟수가 절대 0이 되지 않아 둘 다 드롭되지 않고 메모리 누수가 발생합니다. `Weak`는 이 순환을 끊어줍니다.

```rust
use std::rc::{Rc, Weak};
use std::cell::RefCell;

#[derive(Debug)]
struct Node {
    value: String,
    parent: RefCell<Weak<Node>>,      // Weak — 부모가 드롭되는 것을 막지 않습니다.
    children: RefCell<Vec<Rc<Node>>>,  // Strong — 부모가 자식을 소유합니다.
}

impl Node {
    fn new(value: &str) -> Rc<Node> {
        Rc::new(Node {
            value: value.to_string(),
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(Vec::new()),
        })
    }

    fn add_child(parent: &Rc<Node>, child: &Rc<Node>) {
        // 자식은 부모에 대해 약한 참조(weak reference)를 가집니다 (순환 없음).
        *child.parent.borrow_mut() = Rc::downgrade(parent);
        // 부모는 자식에 대해 강한 참조(strong reference)를 가집니다.
        parent.children.borrow_mut().push(Rc::clone(child));
    }
}

fn main() {
    let root = Node::new("root");
    let child = Node::new("child");
    Node::add_child(&root, &child);

    // upgrade()를 통해 자식에서 부모로 접근합니다.
    if let Some(parent) = child.parent.borrow().upgrade() {
        println!("자식의 부모: {}", parent.value); // "root"
    }
    
    println!("Root 강한 참조 횟수: {}", Rc::strong_count(&root));  // 1
    println!("Root 약한 참조 횟수: {}", Rc::weak_count(&root));      // 1
}
```

### C++ 비교

```cpp
// C++ — shared_ptr 순환을 끊기 위한 weak_ptr
struct Node {
    std::string value;
    std::weak_ptr<Node> parent;                  // Weak — 소유권 없음
    std::vector<std::shared_ptr<Node>> children;  // Strong — 자식 소유

    static auto create(const std::string& v) {
        return std::make_shared<Node>(Node{v, {}, {}});
    }
};

auto root = Node::create("root");
auto child = Node::create("child");
child->parent = root;          // weak_ptr 대입
root->children.push_back(child);

if (auto p = child->parent.lock()) {   // lock() → shared_ptr 또는 null
    std::cout << "부모: " << p->value << std::endl;
}
```

| C++ | Rust | 참고 |
|-----|------|-------|
| `shared_ptr<T>` | `Rc<T>` (단일 스레드) / `Arc<T>` (멀티 스레드) | 동일한 시맨틱 |
| `weak_ptr<T>` | `Rc::downgrade()` / `Arc::downgrade()`를 통한 `Weak<T>` | 동일한 시맨틱 |
| `weak_ptr::lock()` → `shared_ptr` 또는 null | `Weak::upgrade()` → `Option<Rc<T>>` | 드롭된 경우 `None` |
| `shared_ptr::use_count()` | `Rc::strong_count()` | 동일한 의미 |

### `Weak`를 사용하는 시점

| **상황** | **패턴** |
|--------------|-----------|
| 부모 ↔ 자식 트리 관계 | 부모는 `Rc<Child>`, 자식은 `Weak<Parent>` 소유 |
| 옵저버 패턴 / 이벤트 리스너 | 이벤트 소스는 `Weak<Observer>`, 옵저버는 `Rc<Source>` 소유 |
| 할당 해제를 막지 않는 캐시 | `HashMap<Key, Weak<Value>>` — 항목이 자연스럽게 만료됨 |
| 그래프 구조의 순환 끊기 | 교차 링크는 `Weak`, 트리 에지는 `Rc`/`Arc` 사용 |

> 새로운 코드의 트리 구조에서는 `Rc/Weak`보다 **아레나(Arena) 패턴**(사례 연구 2)을 권장합니다. `Vec<T>`와 인덱스를 사용하는 것이 더 단순하고 빠르며 참조 횟수 계산 오버헤드가 없습니다. 동적인 수명을 가진 공유 소유권이 필요할 때만 `Rc/Weak`를 사용하십시오.

----

## Copy 대 Clone, PartialEq 대 Eq — 언제 무엇을 derive 할 것인가

- **Copy ≈ C++의 trivially copyable (커스텀 복사 생성자/소멸자 없음).** `int`, `enum`, 단순한 POD 구조체와 같은 타입들입니다. 컴파일러가 비트 단위의 `memcpy`를 자동으로 생성합니다. Rust에서도 `Copy`는 동일한 개념입니다. `let b = a;` 대입 시 암시적으로 비트 단위 복사가 이루어지며 두 변수 모두 유효하게 남습니다.
- **Clone ≈ C++의 복사 생성자 / `operator=` 깊은 복사 (deep-copy).** C++ 클래스에 커스텀 복사 생성자가 있는 경우(예: `std::vector` 멤버를 깊은 복사하는 경우), Rust에서는 `Clone`을 구현하는 것과 같습니다. 반드시 `.clone()`을 명시적으로 호출해야 합니다. Rust는 비용이 많이 드는 복사를 `=` 뒤에 숨기지 않습니다.
- **핵심 차이점:** C++에서는 단순 복사와 깊은 복사 모두 동일한 `=` 문법을 통해 암시적으로 발생합니다. Rust는 선택을 강제합니다. `Copy` 타입은 조용히 복사(저비용)되고, `Copy`가 아닌 타입은 기본적으로 **이동(move)**하며, 비싼 복제가 필요한 경우 `.clone()`을 명시적으로 사용해야 합니다.
- 마찬가지로 C++의 `operator==`는 `a == a`가 항상 성립하는 타입(정수 등)과 그렇지 않은 타입(NaN이 있는 `float` 등)을 구분하지 않습니다. Rust는 이를 `PartialEq`와 `Eq`로 구분하여 인코딩합니다.

### Copy 대 Clone

| | **Copy** | **Clone** |
|---|---------|----------|
| **작동 방식** | 비트 단위 memcpy (암시적) | 커스텀 로직 (명시적 `.clone()`) |
| **발생 시점** | 대입 시: `let b = a;` | `.clone()` 호출 시에만 |
| **복사/클론 후** | `a`와 `b` 모두 유효함 | `a`와 `b` 모두 유효함 |
| **둘 다 없을 때** | `let b = a;`는 `a`를 **이동(move)**시킴 (`a`는 사라짐) | `let b = a;`는 `a`를 **이동(move)**시킴 (`a`는 사라짐) |
| **허용 대상** | 힙 데이터가 없는 타입 | 모든 타입 |
| **C++ 비유** | Trivially copyable / POD 타입 (커스텀 복사 생성자 없음) | 커스텀 복사 생성자 (깊은 복사) |

### 실제 예시: Copy — 단순 열거형
```rust
// fan_diag/src/sensor.rs — 모든 유닛 변형이 1바이트에 들어감
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum FanStatus {
    #[default]
    Normal,
    Low,
    High,
    Missing,
    Failed,
    Unknown,
}

let status = FanStatus::Normal;
let copy = status;   // 암시적 복사 — status는 여전히 유효함
println!("{:?} {:?}", status, copy);  // 둘 다 작동함
```

### 실제 예시: Copy — 정수 페이로드가 있는 열거형
```rust
// 예시: healthcheck.rs — u32 페이로드는 Copy이므로 열거형 전체도 Copy 가능
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum HealthcheckStatus {
    Pass,
    ProgramError(u32),
    DmesgError(u32),
    RasError(u32),
    OtherError(u32),
    Unknown,
}
```

### 실제 예시: Clone 전용 — 힙 데이터가 있는 구조체
```rust
// 예시: components.rs — String 필드가 Copy를 막음
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FruData {
    pub technology: DeviceTechnology,
    pub physical_location: String,      // ← String: 힙에 할당되므로 Copy 불가
    pub expected: bool,
    pub removable: bool,
}
// let a = fru_data;   → 이동(MOVE) (fru_data는 사라짐)
// let a = fru_data.clone();  → 복제(CLONE) (fru_data는 여전히 유효, 새로운 힙 할당 발생)
```

### 규칙: Copy가 가능할까요?
```text
해당 타입에 String, Vec, Box, HashMap,
Rc, Arc 또는 다른 힙 소유 타입이 포함되어 있습니까?
    예 → Clone만 가능 (Copy 불가)
    아니요 → Copy를 derive 할 수 있음 (타입이 작다면 권장됨)
```

### PartialEq 대 Eq

| | **PartialEq** | **Eq** |
|---|--------------|-------|
| **기능** | `==` 및 `!=` 연산자 제공 | "동등성은 반사적(reflexive)"이라는 마커 |
| **반사성 (a == a)?** | 보장되지 않음 | **보장됨** |
| **중요한 이유** | `f32::NAN != f32::NAN` | `HashMap` 키는 `Eq`를 **요구함** |
| **derive 시점** | 거의 항상 | `f32`/`f64` 필드가 없는 경우 |
| **C++ 비유** | `operator==` | 직접적인 대응물 없음 (C++는 확인하지 않음) |

### 실제 예시: Eq — HashMap 키로 사용됨
```rust
// hms_trap/src/cpu_handler.rs — Hash는 Eq를 요구함
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CpuFaultType {
    InvalidFaultType,
    CpuCperFatalErr,
    CpuLpddr5UceErr,
    CpuC2CUceFatalErr,
    // ...
}
// 사용 예: HashMap<CpuFaultType, FaultHandler>
// HashMap 키는 반드시 Eq + Hash를 구현해야 함 — PartialEq만으로는 컴파일되지 않음
```

### 실제 예시: Eq가 불가능한 경우 — f32 포함
```rust
// 예시: types.rs — f32가 Eq를 막음
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TemperatureSensors {
    pub warning_threshold: Option<f32>,   // ← f32는 NaN ≠ NaN 성립
    pub critical_threshold: Option<f32>,  // ← Eq를 derive 할 수 없음
    pub sensor_names: Vec<String>,
}
// HashMap 키로 사용할 수 없음. Eq를 derive 할 수 없음.
// 이유: f32::NAN == f32::NAN이 거짓이 되어 반사성 원칙을 위반하기 때문.
```

### PartialOrd 대 Ord

| | **PartialOrd** | **Ord** |
|---|---------------|--------|
| **기능** | `<`, `>`, `<=`, `>=` | `.sort()`, `BTreeMap` 키 |
| **전체 순서 (Total ordering)?** | 아니요 (비교 불가능한 쌍이 있을 수 있음) | **예** (모든 쌍을 비교 가능) |
| **f32/f64 포함 여부** | PartialOrd만 가능 (NaN이 순서를 깨뜨림) | Ord를 derive 할 수 없음 |

### 실제 예시: Ord — 심각도 순위
```rust
// hms_trap/src/fault.rs — 변형의 선언 순서가 심각도를 정의함
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum FaultSeverity {
    Info,      // 가장 낮음 (판별값 0)
    Warning,   //           (판별값 1)
    Error,     //           (판별값 2)
    Critical,  // 가장 높음 (판별값 3)
}
// FaultSeverity::Info < FaultSeverity::Critical → 참
// 활용 예: if severity >= FaultSeverity::Error { escalate(); }
```

### 실제 예시: Ord — 비교를 위한 진단 레벨
```rust
// 예시: orchestration.rs
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum GpuDiagLevel {
    #[default]
    Quick,     // 가장 낮음
    Standard,
    Extended,
    Full,      // 가장 높음
}
// 활용 예: if requested_level >= GpuDiagLevel::Extended { run_extended_tests(); }
```

### Derive 결정 트리

```text
                        새로운 타입
                            │
                   String/Vec/Box를 포함합니까?
                      /              \
                    예                아니요
                     │                  │
              Clone만 가능         Clone + Copy
                     │                  │
              f32/f64를 포함합니까?   f32/f64를 포함합니까?
                /          \         /          \
              예            아니요     예            아니요
               │             │      │             │
         PartialEq       PartialEq  PartialEq  PartialEq
         만 가능         + Eq       만 가능    + Eq
                          │                      │
                    정렬이 필요한가요?      정렬이 필요한가요?
                      /       \               /       \
                    예        아니요           예        아니요
                     │          │              │          │
               PartialOrd      완료         PartialOrd    완료
               + Ord                        + Ord
                     │                        │
               맵 키로 사용                  맵 키로 사용
               해야 하나요?                  해야 하나요?
                  │                            │
                + Hash                       + Hash
```

### 빠른 참조: 실무 Rust 코드의 일반적인 derive 조합

| **타입 카테고리** | **일반적인 derive** | **예시** |
|-------------------|--------------------|------------|
| 단순 상태 열거형 | `Copy, Clone, PartialEq, Eq, Default` | `FanStatus` |
| HashMap 키로 쓰이는 열거형 | `Copy, Clone, PartialEq, Eq, Hash` | `CpuFaultType`, `SelComponent` |
| 정렬 가능한 심각도 열거형 | `Copy, Clone, PartialEq, Eq, PartialOrd, Ord` | `FaultSeverity`, `GpuDiagLevel` |
| String을 포함한 데이터 구조체 | `Clone, Debug, Serialize, Deserialize` | `FruData`, `OverallSummary` |
| 직렬화 가능한 설정값 | `Clone, Debug, Default, Serialize, Deserialize` | `DiagConfig` |

----
