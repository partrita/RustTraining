## C++ → Rust 시맨틱 심층 분석

> **학습 내용:** Rust에 명확한 대응물이 없는 C++ 개념들(4가지 명명된 캐스트, SFINAE 대 트레이트 바운드, CRTP 대 연관 타입 등)에 대한 상세한 매핑과 번역 과정에서 자주 발생하는 마찰 지점들을 배웁니다.

아래 섹션들은 Rust에 명확한 1:1 대응물이 없는 C++ 개념들을 매핑합니다. 이러한 차이점들은 C++ 프로그래머가 번역 작업을 수행할 때 자주 혼란을 겪는 부분들입니다.

### 캐스팅 계층 구조: 4가지 C++ 캐스트 → Rust 대응물

C++에는 4가지 명명된 캐스트(named casts)가 있습니다. Rust는 이를 더 명시적인 서로 다른 메커니즘으로 대체합니다.

```cpp
// C++ 캐스팅 계층 구조
int i = static_cast<int>(3.14);            // 1. 숫자 형변환 / 업캐스트 (Numeric / up-cast)
Derived* d = dynamic_cast<Derived*>(base); // 2. 런타임 다운캐스팅 (Runtime downcasting)
int* p = const_cast<int*>(cp);              // 3. const 제거 (Cast away const)
auto* raw = reinterpret_cast<char*>(&obj); // 4. 비트 수준 재해석 (Bit-level reinterpretation)
```

| C++ 캐스트 | Rust 대응물 | 안전성 | 참고 |
|----------|----------------|--------|-------|
| `static_cast` (숫자) | `as` 키워드 | 안전하지만 절단/래핑 발생 가능 | `let i = 3.14_f64 as i32;` — 3으로 절단됨 |
| `static_cast` (숫자, 확인됨) | `From`/`Into` | 안전하며 컴파일 타임에 검증됨 | `let i: i32 = 42_u8.into();` — 확장 변환만 수행 |
| `static_cast` (숫자, 실패 가능) | `TryFrom`/`TryInto` | 안전하며 `Result`를 반환함 | `let i: u8 = 300_u16.try_into()?;` — Err 반환 |
| `dynamic_cast` (다운캐스트) | 열거형 `match` / `Any::downcast_ref` | 안전함 | 열거형은 패턴 매칭 사용, 트레이트 객체는 `Any` 사용 |
| `const_cast` | 대응물 없음 | | Rust의 안전한 코드에서는 `&`를 `&mut`로 바꿀 수 없습니다. 내부 가변성을 위해 `Cell`/`RefCell`을 사용하십시오. |
| `reinterpret_cast` | `std::mem::transmute` | **`unsafe`** | 비트 패턴을 재해석합니다. 거의 항상 잘못된 방식이며, `from_le_bytes()` 등을 선호하십시오. |

```rust
// Rust 대응물:

// 1. 숫자 캐스트 — as보다는 From/Into를 선호하십시오.
let widened: u32 = 42_u8.into();             // 실패 없는 확장 변환 — 항상 권장됨
let truncated = 300_u16 as u8;                // ⚠ 44로 래핑됨! 조용히 데이터 손실 발생
let checked: Result<u8, _> = 300_u16.try_into(); // Err — 안전한 실패 가능 변환

// 2. 다운캐스트: 열거형(권장) 또는 Any (타입 소거가 필요한 경우)
use std::any::Any;

fn handle_any(val: &dyn Any) {
    if let Some(s) = val.downcast_ref::<String>() {
        println!("문자열 감지: {s}");
    } else if let Some(n) = val.downcast_ref::<i32>() {
        println!("정수 감지: {n}");
    }
}

// 3. "const_cast" → 내부 가변성 (unsafe 불필요)
use std::cell::Cell;
struct Sensor {
    read_count: Cell<u32>,  // &self를 통해 수정 가능
}
impl Sensor {
    fn read(&self) -> f64 {
        self.read_count.set(self.read_count.get() + 1); // &mut self가 아닌 &self 사용
        42.0
    }
}

// 4. reinterpret_cast → transmute (almost never needed)
// 안전한 대안을 선호하십시오:
let bytes: [u8; 4] = 0x12345678_u32.to_ne_bytes();  // ✅ 안전함
let val = u32::from_ne_bytes(bytes);                   // ✅ 안전함
// unsafe { std::mem::transmute::<u32, [u8; 4]>(val) } // ❌ 피하십시오.
```

> **가이드라인**: 관용적인 Rust에서 `as`는 드물게 사용되어야 하며(확장은 `From`/`Into`, 축소는 `TryFrom`/`TryInto` 사용), `transmute`는 예외적인 상황에서만 사용해야 합니다. 또한 내부 가변성 타입들 덕분에 `const_cast`는 필요하지 않으므로 대응물이 없습니다.

---

### 전처리기(Preprocessor) → `cfg`, 기능 플래그(Feature Flags), 그리고 `macro_rules!`

C++은 조건부 컴파일, 상수 정의, 코드 생성을 위해 전처리기에 크게 의존합니다. Rust는 이 모든 것을 언어의 정식 기능(first-class features)으로 대체합니다.

#### `#define` 상수 → `const` 또는 `const fn`

```cpp
// C++
#define MAX_RETRIES 5
#define BUFFER_SIZE (1024 * 64)
#define SQUARE(x) ((x) * (x))  // 매크로 — 단순 텍스트 치환, 타입 안전성 없음
```

```rust
// Rust — 타입 안전, 스코프 적용, 텍스트 치환 아님
const MAX_RETRIES: u32 = 5;
const BUFFER_SIZE: usize = 1024 * 64;
const fn square(x: u32) -> u32 { x * x }  // 컴파일 타임에 평가됨

// 상수 컨텍스트에서 사용 가능:
const AREA: u32 = square(12);  // 컴파일 타임에 계산됨
static BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
```

#### `#ifdef` / `#if` → `#[cfg()]` 및 `cfg!()`

```cpp
// C++
#ifdef DEBUG
    log_verbose("1단계 완료");
#endif

#if defined(LINUX) && !defined(ARM)
    use_x86_path();
#else
    use_generic_path();
#endif
```

```rust
// Rust — 속성(attribute) 기반의 조건부 컴파일
#[cfg(debug_assertions)]
fn log_verbose(msg: &str) { eprintln!("[상세] {msg}"); }

#[cfg(not(debug_assertions))]
fn log_verbose(_msg: &str) { /* 릴리스 빌드에서는 컴파일에서 제외됨 */ }

// 조건 결합:
#[cfg(all(target_os = "linux", target_arch = "x86_64"))]
fn use_x86_path() { /* ... */ }

#[cfg(not(all(target_os = "linux", target_arch = "x86_64")))]
fn use_generic_path() { /* ... */ }

// 런타임 확인 (조건은 컴파일 타임에 결정되지만 식 내부에서 사용 가능):
if cfg!(target_os = "windows") {
    println!("Windows에서 실행 중");
}
```

#### `Cargo.toml`의 기능 플래그 (Feature flags)

```toml
# Cargo.toml — #ifdef FEATURE_FOO를 대체
[features]
default = ["json"]
json = ["dep:serde_json"]       // 선택적 의존성
verbose-logging = []            // 추가 의존성 없는 플래그
gpu-support = ["dep:cuda-sys"]  // 선택적 GPU 지원
```

```rust
// 기능 플래그에 따른 조건부 코드:
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}

#[cfg(feature = "verbose-logging")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[상세] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose-logging"))]
macro_rules! verbose {
    ($($arg:tt)*) => { }; // 아무것도 생성하지 않음
}
```

#### `#define MACRO(x)` → `macro_rules!`

```cpp
// C++ — 단순 텍스트 치환, 에러가 발생하기 쉬움
#define DIAG_CHECK(cond, msg) \
    do { if (!(cond)) { log_error(msg); return false; } } while(0)
```

```rust
// Rust — 위생적(hygienic)이고, 타입이 검사되며, 구문 트리(syntax tree) 상에서 작동함
macro_rules! diag_check {
    ($cond:expr, $msg:expr) => {
        if !($cond) {
            log_error($msg);
            return Err(DiagError::CheckFailed($msg.to_string()));
        }
    };
}

fn run_test() -> Result<(), DiagError> {
    diag_check!(temperature < 85.0, "GPU 과열");
    diag_check!(voltage > 0.8, "레일 전압 낮음");
    Ok(())
}
```

| C++ 전처리기 | Rust 대응물 | 장점 |
|-----------------|----------------|-----------|
| `#define PI 3.14` | `const PI: f64 = 3.14;` | 타입과 스코프가 있고, 디버거에서 확인 가능 |
| `#define MAX(a,b) ((a)>(b)?(a):(b))` | `macro_rules!` 또는 제네릭 `fn max<T: Ord>` | 이중 평가(double-evaluation) 버그 없음 |
| `#ifdef DEBUG` | `#[cfg(debug_assertions)]` | 컴파일러가 검사하며 오타 위험 없음 |
| `#ifdef FEATURE_X` | `#[cfg(feature = "x")]` | Cargo가 기능을 관리하며 의존성 인식 가능 |
| `#include "header.h"` | `mod module;` + `use module::Item;` | 텍스트 삽입 없음, 순환 참조 문제 없음 |
| `#pragma once` | 필요 없음 | 각 `.rs` 파일은 모듈이며 정확히 한 번만 포함됨 |

---

### 헤더 파일 및 `#include` → 모듈 및 `use`

C++에서 컴파일 모델은 텍스트 삽입(textual inclusion)을 중심으로 작동합니다.

```cpp
// widget.h — Widget을 사용하는 모든 번역 단위가 이 파일을 포함함
#pragma once
#include <string>
#include <vector>

class Widget {
public:
    Widget(std::string name);
    void activate();
private:
    std::string name_;
    std::vector<int> data_;
};
```

```cpp
// widget.cpp — 별도의 정의부
#include "widget.h"
Widget::Widget(std::string name) : name_(std::move(name)) {}
void Widget::activate() { /* ... */ }
```

Rust에는 **헤더 파일도, 전방 선언(forward declaration)도, 인클루드 가드(include guard)도 없습니다**.

```rust
// src/widget.rs — 선언과 정의가 한 파일에 공존함
pub struct Widget {
    name: String,         // 기본적으로 비공개(private)
    data: Vec<i32>,
}

impl Widget {
    pub fn new(name: String) -> Self {
        Widget { name, data: Vec::new() }
    }
    pub fn activate(&self) { /* ... */ }
}
```

```rust
// src/main.rs — 모듈 경로를 통해 임포트
mod widget;  // 컴파일러에게 src/widget.rs를 포함하도록 지시함
use widget::Widget;

fn main() {
    let w = Widget::new("sensor".to_string());
    w.activate();
}
```

| C++ | Rust | 장점 |
|-----|------|-----------------|
| `#include "foo.h"` | 부모의 `mod foo;` + `use foo::Item;` | 텍스트 삽입 없음, ODR 위반 없음 |
| `#pragma once` / 인클루드 가드 | 필요 없음 | 각 `.rs` 파일은 모듈이며 한 번만 컴파일됨 |
| 전방 선언 | 필요 없음 | 컴파일러가 크레이트 전체를 보므로 순서가 중요하지 않음 |
| `class Foo;` (불완전 타입) | 필요 없음 | 선언과 정의를 분리할 필요가 없음 |
| 클래스당 `.h` + `.cpp` | 단일 `.rs` 파일 | 선언과 정의가 일치하지 않는 버그가 없음 |
| `using namespace std;` | `use std::collections::HashMap;` | 항상 명시적이며 전역 네임스페이스 오염이 없음 |
| 중첩된 `namespace a::b` | 중첩된 `mod a { mod b { } }` 또는 `a/b.rs` | 파일 시스템이 모듈 트리를 그대로 반영함 |

---

### `friend` 및 접근 제어 → 모듈 가시성 (Visibility)

C++은 비공개 멤버에 대한 접근 권한을 특정 클래스나 함수에 부여하기 위해 `friend`를 사용합니다. Rust에는 `friend` 키워드가 없으며, 대신 **접근 제어가 모듈 단위로 이루어집니다**.

```cpp
// C++
class Engine {
    friend class Car;   // Car는 비공개 멤버에 접근할 수 있습니다.
    int rpm_;
    void set_rpm(int r) { rpm_ = r; }
public:
    int rpm() const { return rpm_; }
};
```

```rust
// Rust — 같은 모듈 내의 항목들은 모든 필드에 접근할 수 있으므로 `friend`가 필요 없습니다.
mod vehicle {
    pub struct Engine {
        rpm: u32,  // 모듈 내에서는 공개되지만 모듈 밖에서는 비공개입니다.
    }

    impl Engine {
        pub fn new() -> Self { Engine { rpm: 0 } }
        pub fn rpm(&self) -> u32 { self.rpm }
    }

    pub struct Car {
        engine: Engine,
    }

    impl Car {
        pub fn new() -> Self { Car { engine: Engine::new() } }
        pub fn accelerate(&mut self) {
            self.engine.rpm = 3000; // ✅ 같은 모듈 — 필드에 직접 접근 가능
        }
        pub fn rpm(&self) -> u32 {
            self.engine.rpm  // ✅ 같은 모듈 — 비공개 필드 읽기 가능
        }
    }
}

fn main() {
    let mut car = vehicle::Car::new();
    car.accelerate();
    // car.engine.rpm = 9000;  // ❌ 컴파일 에러: `engine`은 비공개임
    println!("RPM: {}", car.rpm()); // ✅ Car의 공개 메서드 사용
}
```

| C++ 접근 제어 | Rust 대응물 | 범위 |
|-----------|----------------|-------|
| `private` | (기본값, 키워드 없음) | 동일 모듈 내에서만 접근 가능 |
| `protected` | 직접적인 대응물 없음 | 부모 모듈 접근을 위해 `pub(super)` 사용 |
| `public` | `pub` | 어디서나 접근 가능 |
| `friend class Foo` | `Foo`를 같은 모듈에 배치 | 모듈 수준의 프라이버시가 friend를 대체 |
| — | `pub(crate)` | 크레이트 내에서는 보이지만 외부 의존성에는 숨김 |
| — | `pub(super)` | 부모 모듈에만 공개 |
| — | `pub(in crate::path)` | 특정 모듈 하위 트리 내에서만 공개 |

> **핵심 통찰**: C++의 프라이버시는 클래스 단위입니다. Rust의 프라이버시는 모듈 단위입니다. 즉, 어떤 타입들을 같은 모듈에 둘지 결정함으로써 접근 권한을 제어하며, 같은 모듈에 있는 타입들은 서로의 비공개 필드에 자유롭게 접근할 수 있습니다.

---

### `volatile` → 원자적 연산(Atomics) 및 `read_volatile`/`write_volatile`

C++에서 `volatile`은 컴파일러가 읽기/쓰기 연산을 최적화하여 제거하지 않도록 지시하며, 주로 메모리 맵 하드웨어 레지스터(MMIO)에 사용됩니다. **Rust에는 `volatile` 키워드가 없습니다.**

```cpp
// C++: 하드웨어 레지스터를 위한 volatile
volatile uint32_t* const GPIO_REG = reinterpret_cast<volatile uint32_t*>(0x4002'0000);
*GPIO_REG = 0x01;              // 쓰기 연산이 최적화로 제거되지 않음
uint32_t val = *GPIO_REG;     // 읽기 연산이 최적화로 제거되지 않음
```

```rust
// Rust: 명시적인 volatile 연산 — unsafe 코드 내에서만 가능
use std::ptr;

const GPIO_REG: *mut u32 = 0x4002_0000 as *mut u32;

// 안전성(SAFETY): GPIO_REG는 유효한 메모리 맵 I/O 주소여야 합니다.
unsafe {
    ptr::write_volatile(GPIO_REG, 0x01);   // 쓰기 연산이 최적화로 제거되지 않음
    let val = ptr::read_volatile(GPIO_REG); // 읽기 연산이 최적화로 제거되지 않음
}
```

**동시성 공유 상태**(C++ `volatile`의 또 다른 흔한 용도)의 경우, Rust는 원자적 연산(atomics)을 사용합니다.

```cpp
// C++: volatile은 스레드 안전성을 위해 충분하지 않습니다 (흔한 실수입니다!).
volatile bool stop_flag = false;  // ❌ 데이터 경합 발생 — C++11 이상에서 미정의 동작

// 올바른 C++ 방식:
std::atomic<bool> stop_flag{false};
```

```rust
// Rust: 원자적 연산은 스레드 간 가변 상태를 공유하는 유일한 방법입니다.
use std::sync::atomic::{AtomicBool, Ordering};

static STOP_FLAG: AtomicBool = AtomicBool::new(false);

// 다른 스레드에서:
STOP_FLAG.store(true, Ordering::Release);

// 확인 시:
if STOP_FLAG.load(Ordering::Acquire) {
    println!("중지 중");
}
```

| C++ 용도 | Rust 대응물 | 참고 |
|-----------|----------------|-------|
| 하드웨어 레지스터용 `volatile` | `ptr::read_volatile` / `ptr::write_volatile` | `unsafe` 필요 — MMIO에 적합한 방식 |
| 스레드 신호용 `volatile` | `AtomicBool` / `AtomicU32` 등 | C++에서도 이 용도로 `volatile`을 쓰는 것은 잘못되었습니다! |
| `std::atomic<T>` | `std::sync::atomic::AtomicT` | 동일한 시맨틱, 동일한 메모리 순서(ordering) |
| `std::atomic<T>::load(memory_order_acquire)` | `AtomicT::load(Ordering::Acquire)` | 1:1 매핑 |

---

### `static` 변수 → `static`, `const`, `LazyLock`, `OnceLock`

#### 기본 `static` 및 `const`

```cpp
// C++
const int MAX_RETRIES = 5;                    // 컴파일 타임 상수
static std::string CONFIG_PATH = "/etc/app";  // 정적 초기화 — 실행 순서가 불명확함!
```

```rust
// Rust
const MAX_RETRIES: u32 = 5;                   // 컴파일 타임 상수, 인라인화됨
static CONFIG_PATH: &str = "/etc/app";         // 'static 수명, 고정된 주소
```

#### 정적 초기화 순서 문제 (Static initialization order fiasco)

C++에는 서로 다른 번역 단위에 있는 전역 생성자들이 **정해지지 않은 순서**로 실행된다는 고질적인 문제가 있습니다. Rust는 `static` 값이 반드시 컴파일 타임 상수여야 한다는 제약을 통해 이 문제를 원천적으로 방지합니다 (생성자 호출 불가).

런타임에 초기화되는 전역 변수의 경우, `LazyLock` (Rust 1.80 이상) 또는 `OnceLock`을 사용하십시오.

```rust
use std::sync::LazyLock;

// C++의 `static std::regex`에 대응 — 첫 접근 시 초기화되며 스레드 안전함
static CONFIG_REGEX: LazyLock<regex::Regex> = LazyLock::new(|| {
    regex::Regex::new(r"^[a-z]+_diag$").expect("유효하지 않은 정규표현식")
});

fn is_valid_diag(name: &str) -> bool {
    CONFIG_REGEX.is_match(name)  // 첫 호출 시 초기화되고, 이후 호출은 빠릅니다.
}
```

```rust
use std::sync::OnceLock;

// OnceLock: 한 번만 초기화되며, 런타임 데이터로부터 설정 가능함
static DB_CONN: OnceLock<String> = OnceLock::new();

fn init_db(connection_string: &str) {
    DB_CONN.set(connection_string.to_string())
        .expect("DB_CONN이 이미 초기화되었습니다");
}

fn get_db() -> &'static str {
    DB_CONN.get().expect("DB가 초기화되지 않았습니다")
}
```

| C++ | Rust | 참고 |
|-----|------|-------|
| `const int X = 5;` | `const X: i32 = 5;` | 둘 다 컴파일 타임 상수. Rust는 타입 명시가 필요함 |
| `constexpr int X = 5;` | `const X: i32 = 5;` | Rust의 `const`는 항상 constexpr임 |
| `static int count = 0;` (파일 스코프) | `static COUNT: AtomicI32 = AtomicI32::new(0);` | 가변 정적 변수는 `unsafe` 또는 원자적 연산 필요 |
| `static std::string s = "hi";` | `static S: &str = "hi";` 또는 `LazyLock<String>` | 단순한 경우 런타임 생성자가 필요 없음 |
| `static MyObj obj;` (복잡한 초기화) | `static OBJ: LazyLock<MyObj> = LazyLock::new(|| { ... });` | 스레드 안전, 지연 초기화, 순서 문제 없음 |
| `thread_local` | `thread_local! { static X: Cell<u32> = Cell::new(0); }` | 동일한 시맨틱 |

---

### `constexpr` → `const fn`

C++ `constexpr`는 컴파일 타임 평가를 위해 함수와 변수를 표시합니다. Rust는 같은 목적으로 `const fn`과 `const`를 사용합니다.

```cpp
// C++
constexpr int factorial(int n) {
    return n <= 1 ? 1 : n * factorial(n - 1);
}
constexpr int val = factorial(5);  // 컴파일 타임에 120으로 계산됨
```

```rust
// Rust
const fn factorial(n: u32) -> u32 {
    if n <= 1 { 1 } else { n * factorial(n - 1) }
}
const VAL: u32 = factorial(5);  // 컴파일 타임에 120으로 계산됨

// 배열 크기나 match 패턴에서도 사용 가능:
const LOOKUP: [u32; 5] = [factorial(1), factorial(2), factorial(3),
                           factorial(4), factorial(5)];
```

| C++ | Rust | 참고 |
|-----|------|-------|
| `constexpr int f()` | `const fn f() -> i32` | 동일한 의도 — 컴파일 타임 평가 가능 |
| `constexpr` 변수 | `const` 변수 | Rust의 `const`는 항상 컴파일 타임 상수임 |
| `consteval` (C++20) | 대응물 없음 | `const fn`은 런타임에도 실행될 수 있음 |
| `if constexpr` (C++17) | 대응물 없음 (`cfg!` 또는 제네릭 사용) | 트레이트 특수화(specialization)가 일부 용도 대체 |
| `constinit` (C++20) | 상수 초기화 값을 가진 `static` | Rust의 `static`은 기본적으로 상수 초기화가 필수임 |

> **`const fn`의 현재 제약 사항** (Rust 1.82 기준 안정화 상태):
> - 트레이트 메서드 호출 불가 (상수 컨텍스트에서 `Vec`의 `.len()` 등 호출 불가)
> - 힙 할당 불가 (`Box::new`, `Vec::new` 등은 const가 아님)
> - 부동 소수점 연산 — **Rust 1.82에서 안정화됨**
> - `for` 루프 사용 불가 (재귀 또는 수동 인덱스를 사용하는 `while` 루프 사용)

---

### SFINAE 및 `enable_if` → 트레이트 바운드(Trait Bounds) 및 `where` 절

C++에서 SFINAE(Substitution Failure Is Not An Error)는 조건부 제네릭 프로그래밍의 핵심 메커니즘입니다. 강력하지만 가독성이 매우 떨어지는 것으로 악명이 높습니다. Rust는 이를 **트레이트 바운드**로 완전히 대체합니다.

```cpp
// C++: SFINAE 기반 조건부 함수 (C++20 이전)
template<typename T,
         std::enable_if_t<std::is_integral_v<T>, int> = 0>
T double_it(T val) { return val * 2; }

template<typename T,
         std::enable_if_t<std::is_floating_point_v<T>, int> = 0>
T double_it(T val) { return val * 2.0; }

// C++20 컨셉(concepts) — 깔끔해졌지만 여전히 장황함:
template<std::integral T>
T double_it(T val) { return val * 2; }
```

```rust
// Rust: 트레이트 바운드 — 가독성이 높고 조합 가능하며 훌륭한 에러 메시지 제공
use std::ops::Mul;

fn double_it<T: Mul<Output = T> + From<u8>>(val: T) -> T {
    val * T::from(2)
}

// 복잡한 바운드의 경우 where 절 사용:
fn process<T>(val: T) -> String
where
    T: std::fmt::Display + Clone + Send,
{
    format!("처리 중: {}", val)
}

// 개별 impl을 통한 조건부 동작 (SFINAE 오버로딩 대체):
trait Describable {
    fn describe(&self) -> String;
}

impl Describable for u32 {
    fn describe(&self) -> String { format!("정수: {self}") }
}

impl Describable for f64 {
    fn describe(&self) -> String { format!("실수: {self:.2}") }
}
```

| C++ 템플릿 메타프로그래밍 | Rust 대응물 | 가독성 |
|-----------------------------|----------------|-------------|
| `std::enable_if_t<cond>` | `where T: Trait` | 🟢 명확한 의미 전달 |
| `std::is_integral_v<T>` | 숫자 트레이트 또는 특정 타입에 대한 바운드 | 🟢 `_v` / `_t` 접미사 없음 |
| SFINAE 오버로드 세트 | 별도의 `impl Trait for ConcreteType` 블록 | 🟢 각 구현이 독립적임 |
| `if constexpr (std::is_same_v<T, int>)` | 트레이트 구현을 통한 특수화 | 🟢 컴파일 타임에 디스패치됨 |
| C++20 `concept` | `trait` | 🟢 거의 동일한 의도 |
| `requires` 절 | `where` 절 | 🟢 위치와 구문이 유사함 |
| 템플릿 내부 깊은 곳에서 컴파일 실패 | 호출 지점에서 트레이트 불일치로 실패 | 🟢 200줄의 에러 폭포가 없음 |

> **핵심 통찰**: C++20의 컨셉(concepts)이 Rust 트레이트와 가장 유사합니다. 만약 C++20 컨셉에 익숙하다면, Rust 트레이트는 덕 타이핑(duck typing) 대신 일관된 구현 모델(trait impls)을 가진, 1.0 버전부터 언어의 핵심이었던 기능이라고 생각하면 됩니다.

---

### `std::function` → 함수 포인터, `impl Fn`, 그리고 `Box<dyn Fn>`

C++ `std::function<R(Args...)>`는 타입이 소거된 호출 가능 객체(callable)입니다. Rust에는 세 가지 옵션이 있으며 각각의 장단점이 있습니다.

```cpp
// C++: 만능 도구 (힙 할당 발생, 타입 소거)
#include <functional>
std::function<int(int)> make_adder(int n) {
    return [n](int x) { return x + n; };
}
```

```rust
// Rust 옵션 1: 함수 포인터 — 단순함, 캡처 없음, 할당 없음
fn add_one(x: i32) -> i32 { x + 1 }
let f: fn(i32) -> i32 = add_one;
println!("{}", f(5)); // 6

// Rust 옵션 2: impl Fn — 단일화(monomorphized), 오버헤드 없음, 캡처 가능
fn apply(val: i32, f: impl Fn(i32) -> i32) -> i32 { f(val) }
let n = 10;
let result = apply(5, |x| x + n);  // 클로저가 `n`을 캡처함

// Rust 옵션 3: Box<dyn Fn> — 타입 소거됨, 힙 할당 발생 (std::function과 유사)
fn make_adder(n: i32) -> Box<dyn Fn(i32) -> i32> {
    Box::new(move |x| x + n)
}
let adder = make_adder(10);
println!("{}", adder(5));  // 15

// 서로 다른 호출 가능 객체들을 저장할 때 (vector<function<int(int)>>와 유사):
let callbacks: Vec<Box<dyn Fn(i32) -> i32>> = vec![
    Box::new(|x| x + 1),
    Box::new(|x| x * 2),
    Box::new(make_adder(100)),
];
for cb in &callbacks {
    println!("{}", cb(5));  // 6, 10, 105
}
```

| 사용 시점 | C++ 대응물 | Rust 선택지 |
|------------|---------------|-------------|
| 최상위 함수, 캡처 없음 | 함수 포인터 | `fn(Args) -> Ret` |
| 호출 가능 객체를 받는 제네릭 함수 | 템플릿 매개변수 | `impl Fn(Args) -> Ret` (정적 디스패치) |
| 제네릭의 트레이트 바운드 | `template<typename F>` | `F: Fn(Args) -> Ret` |
| 저장된 호출 객체, 타입 소거 필요 | `std::function<R(Args)>` | `Box<dyn Fn(Args) -> Ret>` |
| 상태를 변경하는 콜백 | 가변 람다를 가진 `std::function` | `Box<dyn FnMut(Args) -> Ret>` |
| 일회성 콜백 (소비됨) | 이동된 `std::function` | `Box<dyn FnOnce(Args) -> Ret>` |

> **성능 참고**: `impl Fn`은 오버헤드가 전혀 없습니다(C++ 템플릿처럼 단일화됨). `Box<dyn Fn>`은 `std::function`과 동일한 오버헤드(가상 함수 테이블 + 힙 할당)를 가집니다. 서로 다른 여러 타입의 호출 객체를 저장해야 하는 경우가 아니라면 `impl Fn`을 우선적으로 고려하십시오.

---

### 컨테이너 매핑: C++ STL → Rust `std::collections`

| C++ STL 컨테이너 | Rust 대응물 | 참고 |
|------------------|----------------|-------|
| `std::vector<T>` | `Vec<T>` | 거의 동일한 API. Rust는 기본적으로 범위 검사 수행 |
| `std::array<T, N>` | `[T; N]` | 스택에 할당되는 고정 크기 배열 |
| `std::deque<T>` | `std::collections::VecDeque<T>` | 링 버퍼. 양끝에서 효율적인 push/pop 가능 |
| `std::list<T>` | `std::collections::LinkedList<T>` | Rust에서는 거의 쓰이지 않음 — 대개 `Vec`이 더 빠름 |
| `std::forward_list<T>` | 대응물 없음 | `Vec` 또는 `VecDeque` 사용 |
| `std::unordered_map<K, V>` | `std::collections::HashMap<K, V>` | 기본값으로 `SipHash` 사용 (DoS 내성) |
| `std::map<K, V>` | `std::collections::BTreeMap<K, V>` | B-트리; 키 정렬됨; `K: Ord` 필요 |
| `std::unordered_set<T>` | `std::collections::HashSet<T>` | `T: Hash + Eq` 필요 |
| `std::set<T>` | `std::collections::BTreeSet<T>` | 정렬된 집합; `T: Ord` 필요 |
| `std::priority_queue<T>` | `std::collections::BinaryHeap<T>` | 기본적으로 최대 힙 (C++과 동일) |
| `std::stack<T>` | `.push()` / `.pop()`을 사용하는 `Vec<T>` | 별도의 스택 타입이 필요 없음 |
| `std::queue<T>` | `.push_back()` / `.pop_front()`를 사용하는 `VecDeque<T>` | 별도의 큐 타입이 필요 없음 |
| `std::string` | `String` | UTF-8 보장, 널 문자로 종료되지 않음 |
| `std::string_view` | `&str` | 빌려온 UTF-8 슬라이스 |
| `std::span<T>` (C++20) | `&[T]` / `&mut [T]` | Rust의 슬라이스는 1.0 버전부터 핵심 타입이었음 |
| `std::tuple<A, B, C>` | `(A, B, C)` | 언어 차원의 구문 지원, 구조 분해 가능 |
| `std::pair<A, B>` | `(A, B)` | 단순한 2-요소 튜플 |
| `std::bitset<N>` | 표준 라이브러리 대응물 없음 | `bitvec` 크레이트나 `[u8; N/8]` 사용 |

**주요 차이점**:
- Rust의 `HashMap`/`HashSet`은 `K: Hash + Eq`를 요구합니다. C++에서는 해시 불가능한 키를 쓰면 STL 내부 깊은 곳에서 템플릿 에러가 나지만, Rust는 타입 시스템 수준에서 이를 강제합니다.
- `Vec` 인덱싱(`v[i]`)은 기본적으로 범위 초과 시 패닉을 발생시킵니다. `Option<&T>`를 위해 `.get(i)`를 쓰거나 반복자를 사용하여 범위 검사를 완전히 피하십시오.
- `std::multimap`이나 `std::multiset`은 없습니다 — `HashMap<K, Vec<V>>` 또는 `BTreeMap<K, Vec<V>>`를 사용하십시오.

---

### 예외 안전성 (Exception Safety) → 패닉 안전성 (Panic Safety)

C++은 세 가지 수준의 예외 안전성(Abrahams guarantees)을 정의합니다.

| C++ 수준 | 의미 | Rust 대응물 |
|----------|---------|----------------|
| **No-throw** | 절대 예외를 던지지 않음 | 절대 패닉이 발생하지 않음 (`Result` 반환) |
| **Strong** (commit-or-rollback) | 예외 시 상태가 변경되지 않음 | 소유권 모델 덕분에 자연스러움 — `?`로 조기 반환 시 부분 생성된 값들이 드롭됨 |
| **Basic** | 예외 시 불변성이 유지됨 | Rust의 기본값 — `Drop`이 실행되며 메모리 누수 없음 |

#### Rust의 소유권 모델이 돕는 방식

```rust
// 별도의 노력 없이도 강력한 보장 제공 — file.write() 실패 시 config는 변경되지 않음
fn update_config(config: &mut Config, path: &str) -> Result<(), Error> {
    let new_data = fetch_from_network()?; // Err 시 조기 반환, config는 수정 전 상태
    let validated = validate(new_data)?;   // Err 시 조기 반환, config는 수정 전 상태
    *config = validated;                   // 성공 시에만 도달 (커밋)
    Ok(())
}
```

C++에서 강력한 보장을 달성하려면 수동으로 롤백하거나 copy-and-swap 이디엄을 써야 합니다. Rust에서는 `?` 전파를 통해 대부분의 코드에서 기본적으로 강력한 보장을 얻을 수 있습니다.

#### `catch_unwind` — Rust의 `catch(...)` 대응물

```rust
use std::panic;

// 패닉 캡처 (C++의 catch(...)와 유사) — 드물게 사용됨
let result = panic::catch_unwind(|| {
    // 패닉이 발생할 수 있는 코드
    let v = vec![1, 2, 3];
    v[10]  // 패닉 발생! (인덱스 범위 초과)
});

match result {
    Ok(val) => println!("결과: {val}"),
    Err(_) => eprintln!("패닉 캡처됨 — 정리 완료"),
}
```

#### `UnwindSafe` — 타입을 패닉 안전으로 표시하기

```rust
use std::panic::UnwindSafe;

// &mut 뒤에 있는 타입들은 기본적으로 UnwindSafe가 아닙니다.
// 패닉으로 인해 데이터가 부분적으로 수정된 상태로 남았을 수 있기 때문입니다.
fn safe_execute<F: FnOnce() + UnwindSafe>(f: F) {
    let _ = std::panic::catch_unwind(f);
}

// 코드를 검토한 후 안전하다고 판단되면 AssertUnwindSafe로 덮어쓸 수 있습니다.
use std::panic::AssertUnwindSafe;
let mut data = vec![1, 2, 3];
let _ = std::panic::catch_unwind(AssertUnwindSafe(|| {
    data.push(4);
}));
```

| C++ 예외 패턴 | Rust 대응물 |
|-----------------------|-----------------|
| `throw MyException()` | `return Err(MyError::...)` (권장) 또는 `panic!("...")` |
| `try { } catch (const E& e)` | `match result { Ok(v) => ..., Err(e) => ... }` 또는 `?` |
| `catch (...)` | `std::panic::catch_unwind(...)` |
| `noexcept` | `-> Result<T, E>` (에러는 예외가 아닌 값임) |
| 스택 되감기 시 RAII 정리 | 패닉 되감기 시 `Drop::drop()` 실행 |
| `std::uncaught_exceptions()` | `std::thread::panicking()` |
| `-fno-exceptions` 컴파일 플래그 | `Cargo.toml` 프로필의 `panic = "abort"` |

> **결론**: Rust에서 대부분의 코드는 예외 대신 `Result<T, E>`를 사용하여 에러 경로를 명시적이고 조합 가능하게 만듭니다. `panic!`은 루틴한 에러가 아닌 버그(`assert!` 실패 등)를 위해 예약되어 있습니다. 이는 "예외 안전성"이 큰 문제가 되지 않음을 의미합니다 — 소유권 시스템이 정리를 자동으로 처리하기 때문입니다.

---

## C++에서 Rust로의 마이그레이션 패턴

### 빠른 참조: C++ → Rust 이디엄 매핑

| **C++ 패턴** | **Rust 이디엄** | **참고** |
|----------------|---------------|----------|
| `class Derived : public Base` | `enum Variant { A {...}, B {...} }` | 닫힌 세트에는 열거형을 선호하십시오. |
| `virtual void method() = 0` | `trait MyTrait { fn method(&self); }` | 개방형/확장 가능 인터페이스에 사용하십시오. |
| `dynamic_cast<Derived*>(ptr)` | `match value { Variant::A(data) => ..., }` | 철저한 검사, 런타임 실패 없음 |
| `vector<unique_ptr<Base>>` | `Vec<Box<dyn Trait>>` | 정말로 다형성이 필요할 때만 사용하십시오. |
| `shared_ptr<T>` | `Rc<T>` 또는 `Arc<T>` | 우선 `Box<T>`나 소유된 값을 고려하십시오. |
| `enable_shared_from_this<T>` | 아레나 패턴 (`Vec<T>` + 인덱스) | 참조 순환을 완전히 제거합니다. |
| 모든 클래스에 `Base* m_pFramework` | `fn execute(&mut self, ctx: &mut Context)` | 포인터를 저장하지 말고 컨텍스트를 전달하십시오. |
| `try { } catch (...) { }` | `match result { Ok(v) => ..., Err(e) => ... }` | 또는 전파를 위해 `?`를 사용하십시오. |
| `std::optional<T>` | `Option<T>` | `match`가 강제되므로 None을 잊을 수 없습니다. |
| `const std::string&` 매개변수 | `&str` 매개변수 | `String`과 `&str` 모두 수용 가능합니다. |
| `enum class Foo { A, B, C }` | `enum Foo { A, B, C }` | Rust 열거형은 데이터도 담을 수 있습니다. |
| `auto x = std::move(obj)` | `let x = obj;` | 이동이 기본이며, `std::move`가 필요 없습니다. |
| CMake + make + lint | `cargo build / test / clippy / fmt` | 하나의 도구로 모든 작업을 수행합니다. |

### 마이그레이션 전략
1. **데이터 타입부터 시작하십시오**: 구조체와 열거형을 먼저 번역하십시오 — 소유권에 대해 생각하게 해줍니다.
2. **팩토리를 열거형으로 전환하십시오**: 팩토리가 서로 다른 파생 타입을 생성한다면, `enum` + `match` 구조가 적합할 가능성이 높습니다.
3. **거대 객체(god object)를 구성된 구조체로 전환하십시오**: 관련 필드들을 집중된 구조체들로 그룹화하십시오.
4. **포인터를 빌림(borrow)으로 대체하십시오**: 저장된 `Base*` 포인터를 수명이 제한된 `&'a T` 빌림으로 전환하십시오.
5. **`Box<dyn Trait>` 사용을 절제하십시오**: 플러그인 시스템이나 테스트 모킹에만 사용하십시오.
6. **컴파일러의 안내를 따르십시오**: Rust의 에러 메시지는 매우 훌륭합니다 — 주의 깊게 읽어보십시오.

---
