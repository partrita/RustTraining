# Rust의 인터페이스: 트레이트(Traits)

> **학습 목표:** Rust에서 다형성을 구현하는 핵심 도구인 **트레이트**를 배웁니다. 트레이트가 어떻게 인터페이스, 추상 클래스, 그리고 연산자 오버로딩의 역할을 수행하는지 이해하고, 정적 디스패치(제네릭)와 동적 디스패치(`dyn Trait`)의 차이를 명확히 구분합니다. C++ 개발자에게 트레이트는 가상 함수, CRTP, 컨셉(Concepts)을 대체하는 강력한 수단입니다.

---

### 트레이트의 기본 개념
트레이트는 특정 타입이 '할 수 있는 행동'을 정의합니다. 다른 언어의 인터페이스와 유사하지만, 더 유연하고 강력합니다.

```rust
trait Animal {
    // 반드시 구현해야 하는 메서드
    fn speak(&self);
    
    // 기본 구현 (옵션: 필요에 따라 재정의 가능)
    fn sleep(&self) {
        println!("잠을 잡니다...");
    }
}

struct Cat;
struct Dog;

impl Animal for Cat {
    fn speak(&self) { println!("야옹"); }
}

impl Animal for Dog {
    fn speak(&self) { println!("멍멍!"); }
}

fn main() {
    let kitty = Cat;
    let puppy = Dog;
    
    kitty.speak();
    puppy.speak();
    puppy.sleep(); // 기본 구현 사용
}
```

---

# 💡 C++ 상속 vs Rust 트레이트

C++는 **상속(Inheritance)**을 통해 "A는 B다(IS-A)" 관계를 형성하지만, Rust는 **트레이트(Trait)**를 통해 "A는 B라는 행동을 할 수 있다(CAN-DO)" 관계를 지향합니다.

| **비교 항목** | **C++ 상속 (OOP)** | **Rust 트레이트 (Comp.)** |
| :--- | :--- | :--- |
| **관계 모델** | 클래스 계층 구조 (부모-자식) | 타입과 행동의 조합 (Data + Behavior) |
| **다형성 방식** | 가상 함수 테이블 (vtable) 기반 | 정적 디스패치(제네릭)가 기본 |
| **결합도** | 강한 결합 (계층 구조에 종속됨) | 느슨한 결합 (필요한 트레이트만 구현) |
| **메모리** | 종속적 (힙 할당 및 포인터 선호) | 독립적 (스택 할당 및 제로 코스트) |

```mermaid
graph TD
    subgraph "C++: 상속 계층 (IS-A)"
        C_BASE["Animal<br/>(추상 클래스)"] --> C_CAT["Cat (상속)"]
        C_BASE --> C_DOG["Dog (상속)"]
        C_VT["Virtual Table<br/>(런타임 오버헤드)"]
    end
    
    subgraph "Rust: 트레이트 구현 (CAN-DO)"
        R_TRAIT["trait Animal<br/>(행동 정의)"]
        R_CAT["struct Cat<br/>(데이터만)"] -.->|"impl"| R_TRAIT
        R_DOG["struct Dog<br/>(데이터만)"] -.->| "impl"| R_TRAIT
        R_OPT["정적 최적화<br/>(제로 코스트)"]
    end

    style C_VT fill:#ffa07a,color:#000
    style R_OPT fill:#91e5a3,color:#000
```

---

# 제네릭과 트레이트 경계 (Trait Bounds)

제네릭 함수를 작성할 때, 특정 트레이트를 구현한 타입만 인자로 받도록 제한할 수 있습니다. 이를 **트레이트 경계**라고 합니다.

```rust
use std::fmt::Display;

// T는 반드시 Display 트레이트를 구현한 타입이어야 합니다.
fn print_info<T: Display>(item: T) {
    println!("정보: {item}");
}

// 여러 개의 경계가 필요할 때는 where 절을 쓰면 깔끔합니다.
fn compare_and_print<T>(a: T, b: T) 
where 
    T: Display + PartialOrd 
{
    if a > b {
        println!("{a}가 {b}보다 큽니다.");
    }
}
```

---

# 연산자 오버로딩 (Operator Overloading)

Rust에서 `+`, `-`, `*` 등 모든 연산자는 `std::ops` 모듈의 트레이트와 매핑됩니다. 마법 같은 문법 대신, 정해진 트레이트를 구현하기만 하면 연산자 기능을 부여할 수 있습니다.

| **연산자** | **Rust 트레이트** | **C++ 대응** |
| :--- | :--- | :--- |
| `+` | `Add` | `operator+` |
| `*` (곱셈) | `Mul` | `operator*` |
| `==` | `PartialEq` | `operator==` |
| `[]` | `Index` | `operator[]` |
| `*` (역참조) | `Deref` | `operator*` (포인터) |

```rust
use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Vec2 { x: f64, y: f64 }

impl Add for Vec2 {
    type Output = Self; // 연관 타입: 연산 결과물 타입 정의
    
    fn add(self, other: Self) -> Self {
        Self { x: self.x + other.x, y: self.y + other.y }
    }
}

fn main() {
    let v1 = Vec2 { x: 1.0, y: 2.0 };
    let v2 = Vec2 { x: 3.0, y: 4.0 };
    let v3 = v1 + v2; // Add 트레이트 덕분에 가능
    println!("{v3:?}");
}
```

---

# 정적 디스패치 vs 동적 디스패치

Rust는 다형성을 처리하는 두 가지 명확한 길을 제시합니다.

1.  **정적 디스패치 (`impl Trait`)**: 컴파일 타임에 각 타입별로 함수를 복제(단형성화)하여 최적화합니다. 성능이 가장 뛰어나며 기본적으로 사용해야 하는 방식입니다.
2.  **동적 디스패치 (`dyn Trait`)**: 실행 시점에 vtable을 통해 함수를 찾습니다. 서로 다른 타입들을 하나의 컬렉션(예: `Vec<Box<dyn Animal>>`)에 담아야 할 때 유일할 때 사용합니다.

| **구분** | **정적 디스패치 (제네릭)** | **동적 디스패치 (Trait Object)** |
| :--- | :--- | :--- |
| **문법** | `fn foo(item: impl Trait)` | `fn foo(item: &dyn Trait)` |
| **성능** | 제로 코스트 (인라이닝 가능) | 약간의 간접 참조 오버헤드 |
| **유연성** | 컴파일 시 타입이 고정됨 | 런타임에 다양한 타입 수용 가능 |
| **비유** | C++ 템플릿 | C++ 가상 함수(Virtual Function) |

---

# 📝 실전 연습: 로깅 트레이트 시스템 구축

🟡 **중급 과정** — 아래의 설계에 따라 다차원 로깅 시스템을 구현해 보세요.

1.  `Logger` 트레이트 정의: `fn log(&self, msg: &str)` 메서드를 가집니다.
2.  `ConsoleLogger` 구현: 표준 출력으로 메시지를 찍습니다.
3.  `FileLogger` 구현: "파일에 기록 중: <메시지>"라고 출력합니다.
4.  `run_app` 함수 작성: `impl Logger`를 인자로 받아 로그를 남깁니다.

```rust
trait Logger {
    fn log(&self, msg: &str);
}

struct ConsoleLogger;
impl Logger for ConsoleLogger {
    fn log(&self, msg: &str) {
        println!("[콘솔 로그] {msg}");
    }
}

struct FileLogger;
impl Logger for FileLogger {
    fn log(&self, msg: &str) {
        println!("[파일 기록] {msg}");
    }
}

// 정적 디스패치를 사용한 제네릭 함수
fn run_app(logger: &impl Logger) {
    logger.log("애플리케이션이 시작되었습니다.");
}

fn main() {
    let console = ConsoleLogger;
    let file = FileLogger;
    
    run_app(&console);
    run_app(&file);
}
```

---

# 💡 고아 규칙 (Orphan Rules)

Rust에서는 **"내가 정의한 타입에 외부 트레이트를 구현"**하거나, **"외부 타입에 내가 정의한 트레이트를 구현"**하는 것만 허용됩니다. 
- 예: `u32`(외부 타입)에 `Add`(외부 트레이트)를 다시 구현하는 것은 불가능합니다. 이는 서로 다른 라이브러리들이 연산자 정의를 마음대로 덮어씌워 충돌이 발생하는 것을 방지하는 중요한 안전 장치입니다.
