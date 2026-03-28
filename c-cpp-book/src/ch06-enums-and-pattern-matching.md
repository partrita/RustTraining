# 패턴 매칭과 열거형 (Enums)

> **학습 목표:** Rust의 열거형을 단순한 상수의 집합이 아닌, 데이터를 가질 수 있는 **'구별된 공용체(Discriminated Unions)'**로 이해합니다. 또한 모든 경우의 수를 철저히 검사하는 `match` 구문의 강력함을 체감하고, 열거형이 어떻게 C++의 복잡한 클래스 계층 구조를 우아하게 대체하는지 알아봅니다.

---

### Rust의 열거형 (Enums)
Rust의 열거형은 합 타입(Sum type)으로, 여러 가능한 변형(Variant) 중 하나가 활성화된 상태임을 나타냅니다.

- **C 개발자 관점**: 데이터와 태그를 함께 갖춘 '안전한 공용체(Tagged Union)'입니다. 어떤 데이터가 유효한지 컴파일러가 직접 추적합니다.
- **C++ 개발자 관점**: `std::variant`와 유사하지만, 훨씬 간결한 문법과 컴파일 단계의 철저한 패턴 매칭을 지원합니다. (`std::visit`의 번거로움이 없습니다.)
- **주요 특징**
    - 각 변형은 서로 다른 타입과 크기의 데이터를 가질 수 있습니다.
    - 열거형 전체의 크기는 가장 큰 변형의 크기에 태그(Tag) 크기가 더해진 값과 같습니다.
    - 클래스 상속 구조를 사용하던 많은 패턴을 더 안전하고 간결한 열거형 구조로 대체할 수 있습니다.

```rust
fn main() {
    enum WebEvent {
        // 데이터가 없는 단순 유닛 변형
        PageLoad,
        // 문자열 데이터를 포함하는 변형
        KeyPress(char),
        // 이름이 있는 필드를 포함하는 구조체형 변형
        Click { x: i64, y: i64 },
    }

    let load = WebEvent::PageLoad;
    let press = WebEvent::KeyPress('q');
    let click = WebEvent::Click { x: 10, y: 20 };
}
```

---

### 강력한 제어 흐름: match 문
`match`는 C의 `switch`를 현대적으로 재해석한 도구로, 데이터의 구조를 파헤치고 해당 구조에 맞는 로직을 실행하는 데 특화되어 있습니다.

- **핵심 규칙**
    - **철저성(Exhaustiveness)**: 가능한 모든 경우의 수를 빠짐없이 다뤄야 합니다. 누락 시 컴파일 에러가 발생합니다.
    - **결과값 반환**: `match` 블록 전체가 하나의 표현식으로서 값을 반환할 수 있습니다. (단, 모든 가지의 반환 타입이 일치해야 함)
    - **와일드카드(`_`)**: "그 외의 모든 경우"를 처리할 때 유용하게 쓰입니다.

```rust
fn main() {
    let x = 42;
    
    let result_msg = match x {
        42 => "정답입니다!", 
        0..=41 => "너무 작아요.",
        _ => "너무 커요.", // 모든 숫자를 다루기 위한 와일드카드
    };
    
    println!("{result_msg}");
}
```

---

### match 문의 고급 기능들

1.  **조건부 필터 (Match Guards)**: 패턴 일치 후에 추가적인 조건을 검사합니다.
```rust
match x {
    n if n % 2 == 0 => println!("짝수 패턴: {n}"),
    n => println!("홀수 패턴: {n}"),
}
```

2.  **값 바인딩 (Binding)**: 매칭된 내부 데이터를 변수에 담아 바로 사용할 수 있습니다.
```rust
enum Message {
    Quit,
    Move { x: i32, y: i32 },
}

let msg = Message::Move { x: 10, y: 20 };

match msg {
    Message::Move { x, y } => println!("좌표 이동: ({}, {})", x, y),
    Message::Quit => println!("종료"),
}
```

3.  **`matches!` 매크로**: 특정 패턴과 일치하는지 여부를 `bool` 값으로 간단히 확인합니다.
```rust
if matches!(msg, Message::Move { .. }) {
    println!("이동 메시지입니다.");
}
```

---

### 복합적인 패턴 매칭 (구조 분해와 슬라이스)
구조체 내부의 튜플이나 배열의 일부분만을 목표로 매칭을 시도할 수도 있습니다.

```rust
fn main() {
    struct State {
        info: (u32, bool),
        tag: u32
    }
    
    let s = State { info: (42, true), tag: 100 };
    
    match s {
        // tag가 100인 경우만 info 튜플을 추출
        State { tag: 100, info } => println!("데이터 발견: {info:?}"),
        _ => ()
    }

    let arr = [1, 2, 3];
    match arr {
        // @ 기호는 특정 패턴에 매칭된 전체 값을 변수에 바인딩합니다.
        [first, rest @ ..] => println!("첫 요소: {first}, 나머지: {rest:?}"),
    }
}
```

---

### 실습 연습: 계산기 구현 (열거형과 match 조합)

🟢 **초급 과정**
부호 없는 64비트 정수를 계산하는 미니 계산기를 만들어 봅니다.

1.  **연산 정의**: `Add`, `Subtract` 변형을 가진 `Operation` 열거형을 만드세요.
2.  **결과 정의**: 성공 시 `Ok(u64)`, 실패(언더플로 등) 시 `Invalid(String)`을 반환하는 `CalcResult` 열거형을 만드세요.
3.  **함수 구현**: `calculate(op: Operation) -> CalcResult` 함수를 완성하세요.

<details><summary>💡 정답 및 해설 보기</summary>

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
                CalcResult::Invalid("뺄셈 결과가 음수(Underflow)일 수 없습니다.".to_string())
            }
        }
    }
}

fn main() {
    let result = calculate(Operation::Subtract(5, 10));
    match result {
        CalcResult::Ok(val) => println!("결과: {val}"),
        CalcResult::Invalid(err) => println!("오류 발생: {err}"),
    }
}
```
</details>

---

# 연관 메서드 (Associated Methods)

`impl` 블록을 사용하면 특정 구조체나 열거형에 메서드를 붙일 수 있습니다. C++의 멤버 함수와 비슷하지만, 데이터와 로직을 더욱 유연하게 결합합니다.

- **`self` 이해하기**
    - `&self`: 데이터만 읽고 싶을 때 (가장 흔함)
    - `&mut self`: 데이터를 수정해야 할 때
    - `self`: 객체의 **소유권을 가져와 소비**하고 싶을 때 (예: 타입 변환)

```rust
struct Point { x: i32, y: i32 }

impl Point {
    // 인스턴스 생성자 (관습적으로 new라고 부름)
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    // &mut self를 사용하여 상태 변경
    fn move_to(&mut self, next_x: i32, next_y: i32) {
        self.x = next_x;
        self.y = next_y;
    }
}
```

---

### 실습 연습: 상태 변환과 소유권

🟡 **중급 과정** — 메서드 호출 시 인자가 '복사'되는지 '이동'되는지 구분해야 합니다.

1.  `Point`에 `add(&mut self, other: &Point)` 메서드를 추가하여 값을 누적하세요.
2.  `Point`에 `transform(self) -> Point` 메서드를 추가하세요. 이 메서드는 호출된 인객체를 소멸시키고 각 좌표를 제곱한 **새로운** Point를 반환해야 합니다.

<details><summary>💡 정답 및 해설 보기</summary>

```rust
struct Point { x: i32, y: i32 }

impl Point {
    fn add(&mut self, other: &Point) {
        self.x += other.x;
        self.y += other.y;
    }

    // self를 인자로 받으므로, 이 함수가 끝나면 원래의 Point는 사라집니다.
    fn transform(self) -> Point {
        Point {
            x: self.x * self.x,
            y: self.y * self.y,
        }
    }
}

fn main() {
    let mut p1 = Point { x: 2, y: 3 };
    let p2 = Point { x: 10, y: 20 };
    
    p1.add(&p2); // p1 값이 변경됨
    
    let p3 = p1.transform(); // p1은 여기서 '소비'되어 더 이상 쓸 수 없습니다.
    // println!("{}", p1.x); // 컴파일 에러!
    println!("변환된 결과: ({}, {})", p3.x, p3.y);
}
```
</details>
