# Rust 제네릭(Generics)

> **학습 내용:** 제네릭 타입 매개변수, 단형성화(monomorphization, 제로 코스트 제네릭), 트레이트 경계(trait bounds)를 배웁니다. 또한 Rust 제네릭이 C++ 템플릿과 비교했을 때 더 나은 에러 메시지를 제공하고 SFINAE가 없는 등의 장점을 알아봅니다.

- 제네릭을 사용하면 동일한 알고리즘이나 데이터 구조를 여러 데이터 타입에 걸쳐 재사용할 수 있습니다.
    - 제네릭 매개변수는 ```<>``` 안의 식별자로 나타납니다. 예: ```<T>```. 매개변수는 임의의 유효한 식별자 이름을 가질 수 있지만, 간결함을 위해 보통 짧게 유지합니다.
    - 컴파일러는 컴파일 타임에 단형성화를 수행합니다. 즉, 발견되는 ```T```의 모든 변형에 대해 새로운 타입을 생성합니다.
```rust
// <T> 타입의 left와 right로 구성된 <T> 타입의 튜플을 반환합니다.
fn pick<T>(x: u32, left: T, right: T) -> (T, T) {
   if x == 42 {
    (left, right) 
   } else {
    (right, left)
   }
}
fn main() {
    let a = pick(42, true, false);
    let b = pick(42, "hello", "world");
    println!("{a:?}, {b:?}");
}
```

# Rust 제네릭
- 제네릭은 데이터 타입과 연관 메서드에도 적용될 수 있습니다. 특정 ```<T>```(예: ```f32``` vs. ```u32```)에 대해 구현을 특수화(specialize)하는 것도 가능합니다.
```rust
#[derive(Debug)] // 이에 대해서는 나중에 설명합니다.
struct Point<T> {
    x : T,
    y : T,
}
impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point {x, y}
    }
    fn set_x(&mut self, x: T) {
         self.x = x;       
    }
    fn set_y(&mut self, y: T) {
         self.y = y;       
    }
}
impl Point<f32> {
    fn is_secret(&self) -> bool {
        self.x == 42.0
    }    
}
fn main() {
    let mut p = Point::new(2, 4); // i32
    let q = Point::new(2.0, 4.0); // f32
    p.set_x(42);
    p.set_y(43);
    println!("{p:?} {q:?} {}", q.is_secret());
}
```

# 연습 문제: 제네릭

🟢 **초급**
- ```Point``` 타입을 수정하여 x와 y에 서로 다른 두 가지 타입(```T```와 ```U```)을 사용하도록 만드세요.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
#[derive(Debug)]
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn new(x: T, y: U) -> Self {
        Point { x, y }
    }
}

fn main() {
    let p1 = Point::new(42, 3.14);        // Point<i32, f64>
    let p2 = Point::new("hello", true);   // Point<&str, bool>
    let p3 = Point::new(1u8, 1000u64);    // Point<u8, u64>
    println!("{p1:?}");
    println!("{p2:?}");
    println!("{p3:?}");
}
// 출력:
// Point { x: 42, y: 3.14 }
// Point { x: "hello", y: true }
// Point { x: 1, y: 1000 }
```

</details>

### Rust 트레이트와 제네릭의 결합
- 트레이트를 사용하여 제네릭 타입에 제약 조건(constraints)을 걸 수 있습니다.
- 제약 조건은 제네릭 타입 매개변수 뒤에 ```:```를 사용하거나 ```where``` 절을 사용하여 지정할 수 있습니다. 다음은 ```ComputeArea``` ```트레이트```를 구현한 모든 타입 ```T```를 인자로 받는 제네릭 함수 ```get_area```를 정의합니다.
```rust
    trait ComputeArea {
        fn area(&self) -> u64;
    }
    fn get_area<T: ComputeArea>(t: &T) -> u64 {
        t.area()
    }
```
- [▶ Rust Playground에서 시도해 보기](https://play.rust-lang.org/)

### Rust 트레이트와 제네릭의 결합
- 여러 개의 트레이트 제약 조건을 가질 수도 있습니다.
```rust
trait Fish {}
trait Mammal {}
struct Shark;
struct Whale;
impl Fish for Shark {}
impl Fish for Whale {}
impl Mammal for Whale {}
fn only_fish_and_mammals<T: Fish + Mammal>(_t: &T) {}
fn main() {
    let w = Whale {};
    only_fish_and_mammals(&w);
    let _s = Shark {};
    // 컴파일되지 않음
    only_fish_and_mammals(&_s);
}
```

### 데이터 타입에서의 Rust 트레이트 제약
- 데이터 타입에서도 트레이트 제약 조건을 제네릭과 결합할 수 있습니다.
- 다음 예제에서는 ```PrintDescription``` ```트레이트```와, 해당 트레이트에 의해 제약되는 멤버를 가진 제네릭 ```구조체``` ```Shape```를 정의합니다.
```rust
trait PrintDescription {
    fn print_description(&self);
}
struct Shape<S: PrintDescription> {
    shape: S,
}
// PrintDescription을 구현하는 모든 타입에 대한 제네릭 Shape 구현
impl<S: PrintDescription> Shape<S> {
    fn print(&self) {
        self.shape.print_description();
    }
}
```
- [▶ Rust Playground에서 시도해 보기](https://play.rust-lang.org/)

# 연습 문제: 트레이트 제약 및 제네릭

🟡 **중급**
- ```CipherText```를 구현하는 제네릭 멤버 ```cipher```를 가진 ```구조체```를 구현하세요.
```rust
trait CipherText {
    fn encrypt(&self);
}
// TO DO
//struct Cipher<>

```
- 다음으로, ```cipher```의 ```encrypt```를 호출하는 ```encrypt``` 메서드를 해당 ```구조체```의 ```impl```에 구현하세요.
```rust
// TO DO
impl for Cipher<> {}
```
- 다음으로, ```CipherOne```과 ```CipherTwo```라는 두 구조체에 ```CipherText```를 구현하세요 (단순히 ```println()```만 해도 됩니다). ```CipherOne```과 ```CipherTwo```를 생성하고, ```Cipher```를 사용하여 호출해 보세요.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
trait CipherText {
    fn encrypt(&self);
}

struct Cipher<T: CipherText> {
    cipher: T,
}

impl<T: CipherText> Cipher<T> {
    fn encrypt(&self) {
        self.cipher.encrypt();
    }
}

struct CipherOne;
struct CipherTwo;

impl CipherText for CipherOne {
    fn encrypt(&self) {
        println!("CipherOne 암호화가 적용되었습니다.");
    }
}

impl CipherText for CipherTwo {
    fn encrypt(&self) {
        println!("CipherTwo 암호화가 적용되었습니다.");
    }
}

fn main() {
    let c1 = Cipher { cipher: CipherOne };
    let c2 = Cipher { cipher: CipherTwo };
    c1.encrypt();
    c2.encrypt();
}
// 출력:
// CipherOne 암호화가 적용되었습니다.
// CipherTwo 암호화가 적용되었습니다.
```

</details>

### Rust 타입 상태(Type state) 패턴 및 제네릭
- Rust 타입을 사용하여 *컴파일 타임*에 상태 머신 전이를 강제할 수 있습니다.
    - 예를 들어 ```Idle```(대기)과 ```Flying```(비행)이라는 두 상태를 가진 ```Drone```을 생각해 보세요. ```Idle``` 상태에서는 ```takeoff()``` 메서드만 허용되고, ```Flying``` 상태에서는 ```land()```만 허용하려고 합니다.
    
- 한 가지 방법은 다음과 같이 상태 머신을 모델링하는 것입니다.
```rust
enum DroneState {
    Idle,
    Flying
}
struct Drone {x: u64, y: u64, z: u64, state: DroneState}  // x, y, z는 좌표
```
- 이 방식은 상태 머신 의미론을 강제하기 위해 많은 런타임 체크가 필요합니다 — 왜 그런지 [▶ 시도해 보세요](https://play.rust-lang.org/).

### Rust 타입 상태 패턴 제네릭
- 제네릭을 사용하면 *컴파일 타임*에 상태 머신을 강제할 수 있습니다. 이를 위해 ```PhantomData<T>```라는 특별한 제네릭을 사용해야 합니다.
- ```PhantomData<T>```는 ```제로 사이즈(zero-sized)``` 마커 데이터 타입입니다. 이 경우 ```Idle```과 ```Flying``` 상태를 나타내는 데 사용되지만, 런타임 크기는 ```0```입니다.
- ```takeoff```와 ```land``` 메서드가 ```self```를 매개변수로 받는다는 점에 유의하세요. 이를 ```소비(consuming)```라고 합니다 (빌림을 사용하는 ```&self```와 대조적입니다). 기본적으로 ```Drone<Idle>```에 대해 ```takeoff()```를 호출하면 ```Drone<Flying>```만 반환받을 수 있고 그 반대도 마찬가지입니다.
```rust
struct Drone<T> {x: u64, y: u64, z: u64, state: PhantomData<T> }
impl Drone<Idle> {
    fn takeoff(self) -> Drone<Flying> {...}
}
impl Drone<Flying> {
    fn land(self) -> Drone<Idle> { ...}
}
```
    - [▶ Rust Playground에서 시도해 보기](https://play.rust-lang.org/)

### Rust 타입 상태 패턴 제네릭
- 핵심 요약:
    - 상태를 구조체(제로 사이즈)로 표현할 수 있습니다.
    - 상태 ```T```를 ```PhantomData<T>```(제로 사이즈)와 결합할 수 있습니다.
    - 상태 머신의 특정 단계에 대한 메서드를 구현하는 것은 이제 ```impl State<T>```의 문제입니다.
    - 한 상태에서 다른 상태로 전이하기 위해 ```self```를 소비하는 메서드를 사용합니다.
    - 이는 우리에게 ```제로 코스트(zero cost)``` 추상화를 제공합니다. 컴파일러는 컴파일 타임에 상태 머신을 강제할 수 있으며, 상태가 올바르지 않으면 메서드를 호출하는 것이 불가능합니다.

### Rust 빌더(Builder) 패턴
- ```self```를 소비하는 방식은 빌더 패턴에도 유용할 수 있습니다.
- 수십 개의 핀이 있는 GPIO 설정을 생각해 보세요. 핀은 high 또는 low로 설정될 수 있습니다 (기본값은 low).
```rust
#[derive(default)]
enum PinState {
    #[default]
    Low,
    High,
} 
#[derive(default)]
struct GPIOConfig {
    pin0: PinState,
    pin1: PinState
    ... 
}
```
- 빌더 패턴을 사용하여 체이닝 방식으로 GPIO 설정을 구성할 수 있습니다 — [▶ 직접 시도해 보세요](https://play.rust-lang.org/).
