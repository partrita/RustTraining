# 열거형과 패턴 매칭: 단순 상수를 넘어선 강력한 도구

> **학습 목표:** Rust의 **대수적 데이터 타입(Algebraic Data Types)**으로서의 열거형을 배우고, 이를 C#의 클래스 상속이나 `switch` 문과 비교합니다. 모든 경우의 수를 강제하는 철저한 패턴 매칭을 통해 런타임 에러를 어떻게 방지하는지 마스터합니다.

---

### 1. 대수적 데이터 타입 (ADT): 데이터를 담는 열거형
C#의 열거형은 단순한 숫자 상수에 이름을 붙인 것이지만, Rust의 열거형은 **서로 다른 구조의 데이터를 포함**할 수 있습니다.

| **비교 항목** | **C# 열거형/상속** | **Rust 열거형 (Enum)** |
| :--- | :--- | :--- |
| **데이터 포함** | 클래스 상속으로 구현해야 함 | 각 변형(Variant)이 고유 데이터를 가짐 |
| **메모리 할당** | 클래스 사용 시 힙(Heap) 할당 | 구조체처럼 스택(Stack) 할당 가능 |
| **안전성 검사** | `default` 케이스 누락 시 위험 | **철저한 매칭(Exhaustive)** 컴파일 타임 검증 |

```rust
// [다양한 형태의 메시지를 하나의 타입으로 모델링]
enum Message {
    Quit,                       // 데이터 없음
    Move { x: i32, y: i32 },   // 구조체 형태
    Write(String),             // 튜플 형태
    ChangeColor(i32, i32, i32), // 여러 개의 값
}
```

---

### 2. `match` 표현식: 강력한 구조 분해
C#의 `switch`보다 훨씬 강력합니다. 데이터를 검사함과 동시에 그 내부의 값을 즉시 꺼내서 쓸 수 있습니다.

```rust
fn process_message(msg: Message) {
    match msg {
        Message::Quit => println!("종료합니다."),
        Message::Move { x, y } => println!("좌표 ({}, {})로 이동", x, y),
        Message::Write(text) => println!("글자 기록: {}", text),
        Message::ChangeColor(r, g, b) => println!("색상 변경: RGB({}, {}, {})", r, g, b),
    }
}
```

---

### 3. 유용한 패턴 매칭 기법
- **와일드카드 (`_`)**: "나머지 모든 경우"를 처리합니다. C#의 `default`와 같습니다.
- **매치 가드 (Match Guards)**: 패턴 뒤에 `if` 조건을 붙여 더 세밀하게 필터링합니다.
- **바인딩 (`@`)**: 값을 매칭함과 동시에 변수에 저장합니다.

```rust
match x {
    1..=5 => println!("1에서 5 사이"),
    n @ 10..=20 => println!("{}는 10에서 20 사이", n),
    n if n % 2 == 0 => println!("{}는 짝수", n),
    _ => println!("그 외"),
}
```

---

### 💡 실무 팁: `if let`으로 간결하게!
`match`를 써야 하지만 관심 있는 케이스가 딱 하나뿐일 때가 있습니다. 이럴 땐 `if let` 구문을 쓰면 코드가 훨씬 간결해집니다.

```rust
// [match 버전]
match some_option {
    Some(val) => println!("값: {}", val),
    _ => (), // 아무것도 안 함
}

// [if let 버전]
if let Some(val) = some_option {
    println!("값: {}", val);
}
```

