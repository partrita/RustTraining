# Rust 열거형(Enum) 타입

> **학습 내용:** Rust 열거형을 구별된 공용체(discriminated unions, 올바르게 구현된 태그가 있는 공용체)로 배우고, 철저한 패턴 매칭을 위한 `match`, 그리고 열거형이 어떻게 컴파일러가 강제하는 안전성을 갖추고 C++ 클래스 계층 구조와 C의 태그가 있는 공용체를 대체하는지 알아봅니다.

- 열거형 타입은 구별된 공용체입니다. 즉, 어떤 변형이 활성 상태인지 식별하는 태그를 가진 여러 가능한 서로 다른 타입들의 합 타입(sum type)입니다.
    - C 개발자에게: Rust의 열거형은 데이터를 가질 수 있습니다 (올바르게 구현된 태그가 있는 공용체 — 컴파일러가 어떤 변형이 활성 상태인지 추적함).
    - C++ 개발자에게: Rust의 열거형은 `std::variant`와 비슷하지만 철저한 패턴 매칭을 지원하며, `std::get` 예외나 `std::visit`의 번잡함이 없습니다.
    - `enum`의 크기는 가능한 가장 큰 타입의 크기와 같습니다. 개별 변형들은 서로 관련이 없으며 완전히 다른 타입을 가질 수 있습니다.
    - `enum` 타입은 이 언어의 가장 강력한 기능 중 하나입니다. C++의 전체 클래스 계층 구조를 대체할 수 있습니다 (이에 대한 자세한 내용은 사례 연구에서 다룹니다).
```rust
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let a = Numbers::Zero;
    let b = Numbers::SmallNumber(42);
    let c : Numbers = a; // OK -- a의 타입은 Numbers입니다.
    let d : Numbers = b; // OK -- b의 타입은 Numbers입니다.
}
```
----
# Rust match 문
- Rust의 ```match```는 강력한 기능을 갖춘 C의 "switch"에 해당합니다.
    - ```match```는 단순 데이터 타입, ```struct```, ```enum```에 대한 패턴 매칭에 사용될 수 있습니다.
    - ```match``` 문은 철저(exhaustive)해야 합니다. 즉, 주어진 ```타입```에 대해 가능한 모든 경우를 다루어야 합니다. ```_```는 "그 외 모든 경우"를 위한 와일드카드로 사용될 수 있습니다.
    - ```match```는 값을 생성할 수 있지만, 모든 가지(```=>```)는 동일한 타입의 값을 반환해야 합니다.

```rust
fn main() {
    let x = 42;
    // 이 경우, _는 명시적으로 나열된 것들을 제외한 모든 숫자를 다룹니다.
    let is_secret_of_life = match x {
        42 => true, // 반환 타입은 불리언 값
        _ => false, // 반환 타입은 불리언 값
        // 반환 타입이 불리언이 아니므로 컴파일되지 않습니다.
        // _ => 0  
    };
    println!("{is_secret_of_life}");
}
```

# Rust match 문
- ```match```는 범위, 불리언 필터, ```if``` 가드(guard) 문을 지원합니다.
```rust
fn main() {
    let x = 42;
    match x {
        // =41은 포함 범위를 보장합니다.
        0..=41 => println!("생명의 비밀보다 작음"),
        42 => println!("생명의 비밀"),
        _ => println!("생명의 비밀보다 큼"),
    }
    let y = 100;
    match y {
        100 if x == 43 => println!("y는 100% 생명의 비밀이 아님"),
        100 if x == 42 => println!("y는 100% 생명의 비밀임"),
        _ => (),    // 아무것도 하지 않음
    }
}
```

# Rust match 문
- ```match```와 ```enums```는 종종 함께 결합됩니다.
    - match 문은 포함된 값을 변수에 "바인딩"할 수 있습니다. 값이 중요하지 않은 경우 ```_```를 사용하세요.
    - ```matches!``` 매크로는 특정 변형과 일치하는지 확인하는 데 사용될 수 있습니다.
```rust
fn main() {
    enum Numbers {
        Zero,
        SmallNumber(u8),
        BiggerNumber(u32),
        EvenBiggerNumber(u64),
    }
    let b = Numbers::SmallNumber(42);
    match b {
        Numbers::Zero => println!("Zero"),
        Numbers::SmallNumber(value) => println!("작은 숫자 {value}"),
        Numbers::BiggerNumber(_) | Numbers::EvenBiggerNumber(_) => println!("BiggerNumber 또는 EvenBiggerNumber 중 하나"),
    }
    
    // 특정 변형에 대한 불리언 테스트
    if matches!(b, Numbers::Zero | Numbers::SmallNumber(_)) {
        println!("Zero 또는 작은 숫자와 일치함");
    }
}
```

# Rust match 문
- ```match```는 구조 분해(destructuring)와 슬라이스를 사용하여 매칭을 수행할 수도 있습니다.
```rust
fn main() {
    struct Foo {
        x: (u32, bool),
        y: u32
    }
    let f = Foo {x: (42, true), y: 100};
    match f {
        // x의 값을 tuple이라는 변수에 캡처합니다.
        Foo{y: 100, x : tuple} => println!("x 매칭됨: {tuple:?}"),
        _ => ()
    }
    let a = [40, 41, 42];
    match a {
        // 슬라이스의 마지막 요소가 42여야 합니다. @는 매칭된 값을 바인딩하는 데 사용됩니다.
        [rest @ .., 42] => println!("{rest:?}"),
        // 슬라이스의 첫 번째 요소가 42여야 합니다. @는 매칭된 값을 바인딩하는 데 사용됩니다.
        [42, rest @ ..] => println!("{rest:?}"),
        _ => (),
    }
}
```

# 연습 문제: match 및 열거형을 사용하여 덧셈과 뺄셈 구현하기

🟢 **초급**

- 부호 없는 64비트 숫자에 대해 산술 연산을 수행하는 함수를 작성하세요.
- **단계 1**: 연산을 위한 열거형 정의:
```rust
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}
```
- **단계 2**: 결과 열거형 정의:
```rust
enum CalcResult {
    Ok(u64),                    // 성공적인 결과
    Invalid(String),            // 유효하지 않은 연산에 대한 에러 메시지
}
```
- **단계 3**: `calculate(op: Operation) -> CalcResult` 구현
    - Add의 경우: Ok(sum) 반환
    - Subtract의 경우: 첫 번째 >= 두 번째이면 Ok(difference) 반환, 그렇지 않으면 Invalid("Underflow") 반환
- **힌트**: 함수에서 패턴 매칭을 사용하세요:
```rust
match op {
    Operation::Add(a, b) => { /* 코드 작성 */ },
    Operation::Subtract(a, b) => { /* 코드 작성 */ },
}
```

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
enum Operation {
    Add(u64, u64),
    Subtract(u64, u64),
}

enum CalcResult {
    Ok(u64),
    Invalid(String),
}

fn calculate(op: Operation) -> CalcResult {
    match op {
        Operation::Add(a, b) => CalcResult::Ok(a + b),
        Operation::Subtract(a, b) => {
            if a >= b {
                CalcResult::Ok(a - b)
            } else {
                CalcResult::Invalid("Underflow".to_string())
            }
        }
    }
}

fn main() {
    match calculate(Operation::Add(10, 20)) {
        CalcResult::Ok(result) => println!("10 + 20 = {result}"),
        CalcResult::Invalid(msg) => println!("에러: {msg}"),
    }
    match calculate(Operation::Subtract(5, 10)) {
        CalcResult::Ok(result) => println!("5 - 10 = {result}"),
        CalcResult::Invalid(msg) => println!("에러: {msg}"),
    }
}
// 출력:
// 10 + 20 = 30
// 에러: Underflow
```

</details>

# Rust 연관 메서드(Associated methods)
- ```impl```은 ```struct```, ```enum``` 등의 타입에 연관된 메서드를 정의할 수 있습니다.
    - 메서드는 선택적으로 ```self```를 매개변수로 가질 수 있습니다. ```self```는 개념적으로 C에서 구조체에 대한 포인터를 첫 번째 매개변수로 전달하거나 C++의 ```this```와 유사합니다.
    - ```self```에 대한 참조는 불변(기본값: ```&self```), 가변(```&mut self```), 또는 ```self```(소유권 이전)일 수 있습니다.
    - ```Self``` 키워드는 해당 타입을 나타내는 단축어로 사용될 수 있습니다.
```rust
struct Point {x: u32, y: u32}
impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point {x, y}
    }
    fn increment_x(&mut self) {
        self.x += 1;
    }
}
fn main() {
    let mut p = Point::new(10, 20);
    p.increment_x();
}
```

# 연습 문제: Point 덧셈 및 변환

🟡 **중급** — 메서드 시그니처에서 이동(move)과 빌림(borrow)의 차이를 이해해야 합니다.
- ```Point```에 대해 다음 연관 메서드를 구현하세요.
    - ```add()```는 다른 ```Point```를 받아 x와 y 값을 제자리(in place)에서 증가시킵니다. (힌트: ```&mut self``` 사용)
    - ```transform()```은 기존 ```Point```를 소비하고 (힌트: ```self``` 사용) x와 y를 제곱하여 새로운 ```Point```를 반환합니다.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
struct Point { x: u32, y: u32 }

impl Point {
    fn new(x: u32, y: u32) -> Self {
        Point { x, y }
    }
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }
    fn transform(self) -> Point {
        Point { x: self.x * self.x, y: self.y * self.y }
    }
}

fn main() {
    let mut p1 = Point::new(2, 3);
    let p2 = Point::new(10, 20);
    p1.add(&p2);
    println!("add 후: x={}, y={}", p1.x, p1.y);           // x=12, y=23
    let p3 = p1.transform();
    println!("transform 후: x={}, y={}", p3.x, p3.y);     // x=144, y=529
    // p1은 더 이상 접근할 수 없습니다 — transform()이 이를 소비했기 때문입니다.
}
```

</details>

----
