# 10-1. 제네릭(Generics): 제로 비용 추상화 🟡

> **학습 목표:**
> - 제네릭 타입 매개변수와 이를 최저 부하로 처리하는 **단형성화(Monomorphization)** 기술을 배웁니다.
> - **트레이트 경계(Trait Bounds)**를 통해 제네릭에 기능을 부여하는 법을 익힙니다.
> - C++ 템플릿의 복잡한 에러 메시지나 SFINAE 고민 없이 안전하게 공용 로직을 작성하는 방법을 알아봅니다.
> - **타입 상태(Type State)** 패턴을 통해 런타임 오류를 컴파일 타임으로 옮기는 고급 기법을 살펴봅니다.

---

### 제네릭: "코드 한 번 짜서 여러 타입에 쓰기"
제네릭은 데이터 타입만 다를 뿐 로직이 동일한 함수나 구조체를 재사용할 때 사용합니다. C++의 템플릿(Template)과 개념적으로 가장 가깝습니다.

- **표기법**: `<T>`와 같이 꺾쇠괄호 안에 식별자를 넣어 표현합니다.
- **작동 원리 (단형성화)**: Rust 컴파일러는 빌드 시점에 사용된 구체적인 타입별로 코드를 각각 생성합니다. 따라서 런타임 오버헤드가 전혀 없으며, C++ 템플릿과 성능 면에서 동일합니다.

```rust
// 타입 T를 받아 순서를 바꿔서 반환하는 제네릭 함수
fn swap_pair<T>(left: T, right: T) -> (T, T) {
    (right, left)
}

fn main() {
    let a = swap_pair(true, false);     // T는 bool로 결정됨
    let b = swap_pair("hello", "rust"); // T는 &str로 결정됨
    println!("{a:?}, {b:?}");
}
```

---

### 구조체와 메서드에서의 제네릭
구조체 전체를 제네릭으로 정의하거나, 특정 타입에 대해서만 특별한 기능을 추가(특수화)할 수 있습니다.

```rust
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

// 오직 f32 타입의 Point에 대해서만 동작하는 메서드 정의
impl Point<f32> {
    fn origin_check(&self) -> bool {
        self.x == 0.0 && self.y == 0.0
    }
}
```

---

### 트레이트 경계 (Trait Bounds)
제네릭 타입 `T`가 아무런 제약이 없다면, 함수 내부에서 `T`에 대해 어떤 연산(출력, 비교 등)도 수행할 수 없습니다. 이를 해결하기 위해 "T는 최소한 이 트레이트는 구현해야 한다"는 제약을 겁니다.

```rust
trait Area {
    fn compute(&self) -> f64;
}

// T는 반드시 Area 트레이트를 구현한 타입이어야 함
fn print_area<T: Area>(item: &T) {
    println!("면적: {}", item.compute());
}

// 여러 제약이 있을 때는 'where' 절을 쓰면 코드가 훨씬 깔끔해집니다.
fn process_item<T>(item: T) 
where 
    T: Area + std::fmt::Display + Clone 
{
    // ... 로직 수행 ...
}
```

---

### 🚀 고급 패턴: 타입 상태(Type State) 머신
Rust의 제네릭과 소유권을 결합하면 **컴파일 타임에 상태 전이를 강제**하는 안전한 상태 머신을 만들 수 있습니다. C++ 환경에서 런타임에 체크하던 로직을 컴파일 타임으로 옮길 수 있는 강력한 방법입니다.

```rust
use std::marker::PhantomData;

// 상태 표시용 구조체 (마커)
struct Idle;
struct Flying;

struct Drone<S> {
    id: u32,
    _state: PhantomData<S>, // 런타임 크기는 0인 표시용 필드
}

impl Drone<Idle> {
    fn new(id: u32) -> Self {
        Self { id, _state: PhantomData }
    }

    // self를 소비(Consume)하여 Idle 드론을 없애고 Flying 드론을 반환함
    fn takeoff(self) -> Drone<Flying> {
        println!("드론 {} 이륙!", self.id);
        Drone { id: self.id, _state: PhantomData }
    }
}

impl Drone<Flying> {
    fn land(self) -> Drone<Idle> {
        println!("드론 {} 착륙 중...", self.id);
        Drone { id: self.id, _state: PhantomData }
    }
}

fn main() {
    let drone = Drone::new(1);
    
    // drone.land(); // 컴파일 에러! 대기 중인(Idle) 드론은 착륙할 수 없습니다.
    
    let flying_drone = drone.takeoff(); 
    let _idle_drone = flying_drone.land();
}
```

---

### 💡 실무 팁: C++ 템플릿 vs Rust 제네릭
1.  **에러 메시지**: C++는 템플릿 인스턴스화 과정에서 수천 줄의 에러가 나기도 하지만, Rust는 트레이트 경계를 통해 **함수 정의 시점**에 에러를 잡아내 훨씬 명확한 가이드를 제공합니다.
2.  **SFINAE 대체**: C++의 난해한 SFINAE 기법 대신, Rust는 명시적인 트레이트 구현과 `where` 절을 통해 조건부 기능을 훨씬 우아하고 가독성 있게 구현합니다.
3.  **예측 가능성**: Rust 제네릭은 정의된 경계 내에서만 동작하므로, 의도치 않은 타입이 들어와서 발생하는 기괴한 코너 케이스를 방지합니다.

---

### 📌 요약
- 제네릭은 **단형성화**를 통해 런타임 부하 없이 작동합니다.
- **트레이트 경계**는 제네릭 타입에게 '능력'을 부여하는 방법입니다.
- **타입 상태 머신**은 런타임 오류를 원천 차단하는 고급 설계 기법입니다.
- 복잡한 제약 조건은 **`where` 절**을 활용해 깔끔하게 정리하세요.

