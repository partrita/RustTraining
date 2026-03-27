## 트레이트(Traits) - Rust의 인터페이스

> **학습 내용:** 트레이트와 C# 인터페이스의 비교, 기본 메서드 구현, 트레이트 객체(`dyn Trait`)와 제네릭 바운드(`impl Trait`)의 차이, 파생(Derived) 트레이트, 주요 표준 라이브러리 트레이트, 연관 타입(Associated types), 그리고 트레이트를 이용한 연산자 오버로딩.
>
> **난이도:** 🟡 중급

트레이트는 공유된 동작을 정의하는 Rust의 방식입니다. C#의 인터페이스와 유사하지만 훨씬 더 강력한 기능을 제공합니다.

### C# 인터페이스와의 비교
```csharp
// C# 인터페이스 정의
public interface IAnimal
{
    string Name { get; }
    void MakeSound();
    
    // 기본 구현 (C# 8+)
    string Describe()
    {
        return $"{Name}이(가) 소리를 냅니다.";
    }
}

// C# 인터페이스 구현
public class Dog : IAnimal
{
    public string Name { get; }
    
    public Dog(string name)
    {
        Name = name;
    }
    
    public void MakeSound()
    {
        Console.WriteLine("멍멍!");
    }
    
    // 기본 구현을 오버라이드할 수 있음
    public string Describe()
    {
        return $"{Name}은(는) 충직한 강아지입니다.";
    }
}

// 제네릭 제약 조건
public void ProcessAnimal<T>(T animal) where T : IAnimal
{
    animal.MakeSound();
    Console.WriteLine(animal.Describe());
}
```

### Rust 트레이트 정의 및 구현
```rust
// 트레이트 정의
trait Animal {
    fn name(&self) -> &str;
    fn make_sound(&self);
    
    // 기본 구현
    fn describe(&self) -> String {
        format!("{}이(가) 소리를 냅니다", self.name())
    }
    
    // 다른 트레이트 메서드를 사용하는 기본 구현
    fn introduce(&self) {
        println!("안녕, 나는 {}야.", self.name());
        self.make_sound();
    }
}

// 구조체 정의
#[derive(Debug)]
struct Dog {
    name: String,
    breed: String,
}

impl Dog {
    fn new(name: String, breed: String) -> Dog {
        Dog { name, breed }
    }
}

// 트레이트 구현
impl Animal for Dog {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("멍멍!");
    }
    
    // 기본 구현 오버라이드
    fn describe(&self) -> String {
        format!("{}은(는) 충직한 {}종 강아지입니다", self.name, self.breed)
    }
}

// 또 다른 구현
#[derive(Debug)]
struct Cat {
    name: String,
    indoor: bool,
}

impl Animal for Cat {
    fn name(&self) -> &str {
        &self.name
    }
    
    fn make_sound(&self) {
        println!("야옹!");
    }
    
    // describe()의 기본 구현을 사용함
}

// 트레이트 바운드(trait bounds)를 가진 제네릭 함수
fn process_animal<T: Animal>(animal: &T) {
    animal.make_sound();
    println!("{}", animal.describe());
    animal.introduce();
}

// 다중 트레이트 바운드
fn process_animal_debug<T: Animal + std::fmt::Debug>(animal: &T) {
    println!("디버그 정보: {:?}", animal);
    process_animal(animal);
}

fn main() {
    let dog = Dog::new("바둑이".to_string(), "골든 리트리버".to_string());
    let cat = Cat { name: "나비".to_string(), indoor: true };
    
    process_animal(&dog);
    process_animal(&cat);
    
    process_animal_debug(&dog);
}
```

### 트레이트 객체와 동적 디스패치(Dynamic Dispatch)
```csharp
// C# 동적 다형성
public void ProcessAnimals(List<IAnimal> animals)
{
    foreach (var animal in animals)
    {
        animal.MakeSound(); // 동적 디스패치
        Console.WriteLine(animal.Describe());
    }
}

// 사용 예시
var animals = new List<IAnimal>
{
    new Dog("바둑이"),
    new Cat("나비"),
    new Dog("렉스")
};

ProcessAnimals(animals);
```

```rust
// Rust 트레이트 객체를 이용한 동적 디스패치
fn process_animals(animals: &[Box<dyn Animal>]) {
    for animal in animals {
        animal.make_sound(); // 동적 디스패치
        println!("{}", animal.describe());
    }
}

// 대안: 참조자를 사용한 방식
fn process_animal_refs(animals: &[&dyn Animal]) {
    for animal in animals {
        animal.make_sound();
        println!("{}", animal.describe());
    }
}

fn main() {
    // Box<dyn Trait> 사용
    let animals: Vec<Box<dyn Animal>> = vec![
        Box::new(Dog::new("바둑이".to_string(), "골든 리트리버".to_string())),
        Box::new(Cat { name: "나비".to_string(), indoor: true }),
        Box::new(Dog::new("렉스".to_string(), "저먼 셰퍼드".to_string())),
    ];
    
    process_animals(&animals);
    
    // 참조자 사용
    let dog = Dog::new("바둑이".to_string(), "골든 리트리버".to_string());
    let cat = Cat { name: "나비".to_string(), indoor: true };
    
    let animal_refs: Vec<&dyn Animal> = vec![&dog, &cat];
    process_animal_refs(&animal_refs);
}
```

### 파생 트레이트 (Derived Traits)
```rust
// 공통 트레이트를 자동으로 파생(derive)시킴
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Person {
    name: String,
    age: u32,
}

// 위 구문이 생성하는 코드(간략화됨):
impl std::fmt::Debug for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Person")
            .field("name", &self.name)
            .field("age", &self.age)
            .finish()
    }
}

impl Clone for Person {
    fn clone(&self) -> Self {
        Person {
            name: self.name.clone(),
            age: self.age,
        }
    }
}

impl PartialEq for Person {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.age == other.age
    }
}

// 사용 예시
fn main() {
    let person1 = Person {
        name: "Alice".to_string(),
        age: 30,
    };
    
    let person2 = person1.clone(); // Clone 트레이트 사용
    
    println!("{:?}", person1); // Debug 트레이트 사용
    println!("일치 여부: {}", person1 == person2); // PartialEq 트레이트 사용
}
```

### 주요 표준 라이브러리 트레이트
```rust
use std::collections::HashMap;

// 사용자 친화적 출력을 위한 Display 트레이트
impl std::fmt::Display for Person {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} ({}세)", self.name, self.age)
    }
}

// 타입 변환을 위한 From 트레이트
impl From<(String, u32)> for Person {
    fn from((name, age): (String, u32)) -> Self {
        Person { name, age }
    }
}

// From을 구현하면 Into 트레이트는 자동으로 구현됩니다.
fn create_person() {
    let person: Person = ("Alice".to_string(), 30).into();
    println!("{}", person);
}

// Iterator 트레이트 구현
struct PersonIterator {
    people: Vec<Person>,
    index: usize,
}

impl Iterator for PersonIterator {
    type Item = Person;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.people.len() {
            let person = self.people[self.index].clone();
            self.index += 1;
            Some(person)
        } else {
            None
        }
    }
}

impl Person {
    fn iterator(people: Vec<Person>) -> PersonIterator {
        PersonIterator { people, index: 0 }
    }
}

fn main() {
    let people = vec![
        Person::from(("Alice".to_string(), 30)),
        Person::from(("Bob".to_string(), 25)),
        Person::from(("Charlie".to_string(), 35)),
    ];
    
    // 커스텀 반복자 사용
    for person in Person::iterator(people.clone()) {
        println!("{}", person); // Display 트레이트 사용
    }
}
```

***


<details>
<summary><strong>🏋️ 연습 문제: 트레이트 기반 그리기 시스템</strong> (클릭하여 확장)</summary>

**도전 과제**: `area()` 메서드와 `draw()` 기본 메서드를 가진 `Drawable` 트레이트를 구현하십시오. `Circle`과 `Rect` 구조체를 만들고, `&[Box<dyn Drawable>]`를 인자로 받아 총 면적을 출력하는 함수를 작성하십시오.

<details>
<summary>🔑 정답</summary>

```rust
use std::f64::consts::PI;

trait Drawable {
    fn area(&self) -> f64;

    fn draw(&self) {
        println!("면적이 {:.2}인 도형을 그립니다.", self.area());
    }
}

struct Circle { radius: f64 }
struct Rect   { w: f64, h: f64 }

impl Drawable for Circle {
    fn area(&self) -> f64 { PI * self.radius * self.radius }
}

impl Drawable for Rect {
    fn area(&self) -> f64 { self.w * self.h }
}

fn total_area(shapes: &[Box<dyn Drawable>]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

fn main() {
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rect { w: 4.0, h: 6.0 }),
        Box::new(Circle { radius: 2.0 }),
    ];
    for s in &shapes { s.draw(); }
    println!("총 면적: {:.2}", total_area(&shapes));
}
```

**핵심 요점**:
- `dyn Trait`은 런타임 다형성을 제공합니다 (C#의 `IDrawable`과 유사).
- `Box<dyn Trait>`은 힙에 할당되며, 서로 다른 타입들이 섞인 컬렉션을 만들 때 필요합니다.
- 기본 메서드(Default methods)는 C# 8 이상의 기본 인터페이스 메서드와 똑같이 작동합니다.

</details>
</details>

### 연관 타입(Associated Types): 타입 멤버를 가진 트레이트

C# 인터페이스에는 연관 타입이 없지만, Rust 트레이트에는 있습니다. 이것이 `Iterator`가 작동하는 방식입니다.

```rust
// Iterator 트레이트는 연관 타입 'Item'을 가집니다.
trait Iterator {
    type Item;                         // 각 구현체는 Item이 무엇인지 정의합니다.
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter { max: u32, current: u32 }

impl Iterator for Counter {
    type Item = u32;                   // 이 Counter는 u32 값을 생성합니다.
    fn next(&mut self) -> Option<u32> {
        if self.current < self.max {
            self.current += 1;
            Some(self.current)
        } else {
            None
        }
    }
}
```

C#에서 `IEnumerator<T>`는 이를 위해 제네릭 매개변수(`T`)를 사용합니다. Rust의 연관 타입은 다릅니다. `Iterator`는 구현당 *하나의* `Item` 타입만 가질 수 있으며, 트레이트 수준의 제네릭 매개변수가 아닙니다. 이는 트레이트 바운드를 더 단순하게 만듭니다: C#의 `IEnumerable<int>` 대신 `impl Iterator<Item = u32>`와 같이 사용합니다.

### 트레이트를 통한 연산자 오버로딩

C#에서는 `public static MyType operator+(MyType a, MyType b)`와 같이 정의합니다. Rust에서는 모든 연산자가 `std::ops`에 정의된 트레이트에 매핑됩니다.

```rust
use std::ops::Add;

#[derive(Debug, Clone, Copy)]
struct Vec2 { x: f64, y: f64 }

impl Add for Vec2 {
    type Output = Vec2;
    fn add(self, rhs: Vec2) -> Vec2 {
        Vec2 { x: self.x + rhs.x, y: self.y + rhs.y }
    }
}

let a = Vec2 { x: 1.0, y: 2.0 };
let b = Vec2 { x: 3.0, y: 4.0 };
let c = a + b;  // <Vec2 as Add>::add(a, b)를 호출합니다.
```

| C# | Rust | 비고 |
|----|------|-------|
| `operator+` | `impl Add` | `self`를 값으로 받음 — `Copy` 타입이 아니면 소유권 소비 |
| `operator==` | `impl PartialEq` | 보통 `#[derive(PartialEq)]`로 해결 |
| `operator<` | `impl PartialOrd` | 보통 `#[derive(PartialOrd)]`로 해결 |
| `ToString()` | `impl fmt::Display` | `println!("{}", x)` 등에서 사용 |
| 암시적 변환 | 대응 기능 없음 | Rust는 암시적 변환이 없음 — `From`/`Into` 사용 |

### 일관성(Coherence): 고아 규칙(Orphan Rule)

트레이트나 타입 중 하나를 자신이 소유하고 있을 때만 해당 타입에 대해 트레이트를 구현할 수 있습니다. 이는 여러 크레이트에서 구현이 충돌하는 것을 방지합니다.

```rust
// ✅ 가능 — MyType을 소유함
impl Display for MyType { ... }

// ✅ 가능 — MyTrait을 소유함
impl MyTrait for String { ... }

// ❌ 불가능 — Display도 String도 소유하지 않음
impl Display for String { ... }
```

C#에는 이러한 제한이 없습니다. 어떤 코드에서든 어떤 타입에 대해서도 확장 메서드를 추가할 수 있으며, 이는 모호함을 유발할 수 있습니다.

<!-- ch10.0a: impl Trait and Dispatch Strategies -->
## `impl Trait`: 박싱 없이 트레이트 반환하기

C# 인터페이스는 항상 반환 타입으로 사용될 수 있습니다. Rust에서 트레이트를 반환하려면 정적 디스패치(`impl Trait`)를 쓸지 동적 디스패치(`dyn Trait`)를 쓸지 결정해야 합니다.

### 인자 위치의 `impl Trait` (제네릭의 축약형)
```rust
// 이 두 코드는 동일합니다:
fn print_animal(animal: &impl Animal) { animal.make_sound(); }
fn print_animal<T: Animal>(animal: &T)  { animal.make_sound(); }

// impl Trait은 제네릭 매개변수를 위한 문법적 설탕(syntactic sugar)입니다.
// 컴파일러는 각 구체 타입에 대해 전문화된 복사본을 생성합니다 (단형성화, monomorphization).
```

### 반환 위치의 `impl Trait` (핵심 차이점)
```rust
// 구체 타입을 노출하지 않고 반복자를 반환합니다.
fn even_squares(limit: u32) -> impl Iterator<Item = u32> {
    (0..limit)
        .filter(|n| n % 2 == 0)
        .map(|n| n * n)
}
// 호출자는 "Iterator<Item = u32>를 구현하는 어떤 타입"으로만 보게 됩니다.
// 실제 타입(Filter<Map<Range<u32>, ...>>)은 이름을 붙이기 매우 복잡하지만, impl Trait이 이를 해결해 줍니다.

fn main() {
    for n in even_squares(20) {
        print!("{n} ");
    }
    // 출력: 0 4 16 36 64 100 144 196 256 324
}
```

```csharp
// C# — 인터페이스 반환 (항상 동적 디스패치, 힙 할당된 반복자 객체)
public IEnumerable<int> EvenSquares(int limit) =>
    Enumerable.Range(0, limit)
        .Where(n => n % 2 == 0)
        .Select(n => n * n);
// 반환 타입은 IEnumerable 인터페이스 뒤에 구체적인 반복자를 숨깁니다.
// Rust의 Box<dyn Trait>과 달리 C#은 명시적으로 박싱하지 않으며 런타임이 할당을 처리합니다.
```

### 클로저 반환: `impl Fn` vs `Box<dyn Fn>`
```rust
// 클로저 반환 — 클로저 타입은 이름을 붙일 수 없으므로 impl Fn이 필수적입니다.
fn make_adder(x: i32) -> impl Fn(i32) -> i32 {
    move |y| x + y
}

let add5 = make_adder(5);
println!("{}", add5(3)); // 8 출력

// 조건에 따라 서로 다른 클로저를 반환해야 한다면 Box가 필요합니다:
fn choose_op(add: bool) -> Box<dyn Fn(i32, i32) -> i32> {
    if add {
        Box::new(|a, b| a + b)
    } else {
        Box::new(|a, b| a * b)
    }
}
// impl Trait은 단일 구체 타입을 요구하지만, 서로 다른 클로저는 서로 다른 타입이기 때문입니다.
```

```csharp
// C# — 델리게이트가 이를 자연스럽게 처리합니다 (항상 힙 할당됨).
Func<int, int> MakeAdder(int x) => y => x + y;
Func<int, int, int> ChooseOp(bool add) => add ? (a, b) => a + b : (a, b) => a * b;
```

### 디스패치 결정: `impl Trait` vs `dyn Trait` vs 제네릭

이는 C# 개발자가 Rust에서 즉시 마주하게 되는 설계 결정 사항입니다. 다음 가이드를 참고하십시오.

```mermaid
graph TD
    START["함수가 트레이트 기반 타입을<br/>받거나 반환하는가?"]
    POSITION["인자 위치인가, 반환 위치인가?"]
    ARG_SAME["모든 호출자가<br/>동일한 타입을 전달하는가?"]
    RET_SINGLE["항상 동일한 구체<br/>타입을 반환하는가?"]
    COLLECTION["컬렉션이나 구조체<br/>필드에 저장하는가?"]

    GENERIC["제네릭 사용<br/><code>fn foo&lt;T: Trait&gt;(x: T)</code>"]
    IMPL_ARG["impl Trait 사용<br/><code>fn foo(x: impl Trait)</code>"]
    IMPL_RET["impl Trait 사용<br/><code>fn foo() -> impl Trait</code>"]
    DYN_BOX["Box&lt;dyn Trait&gt; 사용<br/>동적 디스패치"]
    DYN_REF["&dyn Trait 사용<br/>대여된 동적 디스패치"]

    START --> POSITION
    POSITION -->|인자| ARG_SAME
    POSITION -->|반환| RET_SINGLE
    ARG_SAME -->|"예 (문법적 설탕)"| IMPL_ARG
    ARG_SAME -->|"복잡한 바운드/다중 사용"| GENERIC
    RET_SINGLE -->|예| IMPL_RET
    RET_SINGLE -->|"아니요 (조건부 타입)"| DYN_BOX
    RET_SINGLE -->|"이종(Heterogeneous) 컬렉션"| COLLECTION
    COLLECTION -->|소유| DYN_BOX
    COLLECTION -->|대여| DYN_REF

    style GENERIC fill:#c8e6c9,color:#000
    style IMPL_ARG fill:#c8e6c9,color:#000
    style IMPL_RET fill:#c8e6c9,color:#000
    style DYN_BOX fill:#fff3e0,color:#000
    style DYN_REF fill:#fff3e0,color:#000
```

| 방식 | 디스패치 | 할당 | 사용 시점 |
|----------|----------|------------|-------------|
| `fn foo<T: Trait>(x: T)` | 정적 (단형성화됨) | 스택 | 다중 트레이트 바운드, Turbofish 필요, 동일 타입 재사용 시 |
| `fn foo(x: impl Trait)` | 정적 (단형성화됨) | 스택 | 단순 바운드, 깔끔한 문법, 일회성 매개변수 |
| `fn foo() -> impl Trait` | 정적 | 스택 | 단일 구체 반환 타입, 반복자, 클로저 |
| `fn foo() -> Box<dyn Trait>` | 동적 (vtable) | **힙** | 서로 다른 반환 타입, 컬렉션 내의 트레이트 객체 |
| `&dyn Trait` / `&mut dyn Trait` | 동적 (vtable) | 할당 없음 | 대여된 이종 참조자, 함수 매개변수 |

```rust
// 요약: 가장 빠른 속도부터 가장 유연한 방식까지
fn static_dispatch(x: impl Display)             { /* 가장 빠름, 할당 없음 */ }
fn generic_dispatch<T: Display + Clone>(x: T)    { /* 가장 빠름, 다중 바운드 */ }
fn dynamic_dispatch(x: &dyn Display)             { /* vtable 조회, 할당 없음 */ }
fn boxed_dispatch(x: Box<dyn Display>)           { /* vtable 조회 + 힙 할당 */ }
```

***
