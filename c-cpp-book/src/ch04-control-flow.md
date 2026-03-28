# Rust의 제어 흐름 (Control Flow)

> **학습 목표:** Rust의 제어 구조를 익힙니다. 특히 모든 제어 흐름이 값을 반환하는 **'표현식(Expression)'**이라는 점이 C/C++와 어떻게 다른지 살펴봅니다. `if/else`, `loop`, `while`, `for`, `match`의 기본 사용법과 특징을 이해합니다.

---

### 조건문: if 키워드
Rust에서 `if`는 단순한 문장이 아니라 **표현식**입니다. 즉, 연산 결과로 값을 산출하며 이를 변수에 직접 할당할 수도 있습니다.

```rust
fn main() {
    let x = 42;
    
    // 기본적인 if/else 구조
    if x < 42 {
        println!("생명의 비밀(42)보다 작습니다.");
    } else if x == 42 {
        println!("정답입니다! 생명의 비밀을 찾았습니다.");
    } else {
        println!("생명의 비밀보다 큽니다.");
    }

    // if를 이용한 값 할당: C/C++의 삼항 연산자와 유사하게 작동합니다.
    let is_secret_of_life = if x == 42 { true } else { false };
    println!("상태: {}", is_secret_of_life);
}
```
[▶ Rust Playground에서 테스트해 보세요.](https://play.rust-lang.org/)

---

### 반복문: while과 for 루프
특정 조건이 만족되는 동안 또는 데이터 범위를 순회할 때 사용합니다.

- **while 루프**: 조건이 참인 동안 반복합니다.
```rust
fn main() {
    let mut x = 40;
    while x != 42 {
        println!("증가 중: {}", x);
        x += 1;
    }
}
```
- **for 루프**: 일정 범위(Range)를 순회할 때 가장 효율적입니다.
```rust
fn main() {
    // 40부터 42까지 출력 (43은 포함되지 않음)
    // 마지막 값까지 포함하려면 40..=43과 같이 작성합니다.
    for x in 40..43 {
        println!("현재 값: {}", x);
    } 
}
```

---

### 무한 루프: loop 키워드
`loop`는 명시적인 `break`를 만날 때까지 무한히 실행됩니다. C++의 `while(true)`보다 더 안전하고 의도가 명확합니다.

```rust
fn main() {
    let mut x = 40;
    
    // 루프에 'label: loop와 같이 레이블을 붙여 중첩 루프를 제어할 수 있습니다.
    loop {
        if x == 42 {
            // 루프를 종료하며 값을 반환할 수도 있습니다: break x;
            break; 
        }
        x += 1;
    }
}
```
- **주요 특징**
    - `break` 문 뒤에 값을 붙여 `loop` 표현식 자체의 결과값으로 전달할 수 있습니다.
    - `continue`는 현재 반복을 건너뛰고 다음 반복의 시작으로 돌아갑니다.
    - 중첩된 루프에서 `break 'label` 형식을 사용하여 특정 부모 루프를 한 번에 빠져나갈 수 있습니다.

---

### Rust의 핵심: 표현식 블록 (Expression Blocks)
Rust의 중괄호 `{}`로 묶인 블록은 그 자체가 하나의 표현식입니다. 블록 내부의 **마지막 표현식(세미콜론이 없는 줄)**이 해당 블록의 결과값이 됩니다.

```rust
fn main() {
    let x = {
        let y = 40;
        y + 2 // 세미콜론(;)을 붙이지 않아야 결과값으로 반환됩니다.
    };
    
    println!("산출된 값: {x}"); // 42가 출력됩니다.
}
```

- **함수에서의 활용**: Rust는 함수 마지막에 `return` 키워드를 생략하는 것을 권장합니다.
```rust
fn is_secret_of_life(x: u32) -> bool {
    // 세미콜론을 생략함으로써 결괏값을 반환합니다.
    x == 42 
}

fn main() {
    println!("결과: {}", is_secret_of_life(42));
}
```
이러한 방식은 코드를 더 간결하게 만들어주며, 세미콜론의 유무가 '반환'과 '단순 실행'의 의미를 결정하는 매우 중요한 역할을 합니다.
