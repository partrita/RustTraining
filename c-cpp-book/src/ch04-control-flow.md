# Rust if 키워드

> **학습 내용:** Rust의 제어 흐름 구조를 배웁니다 — 표현식으로서의 `if`/`else`, `loop`/`while`/`for`, `match`, 그리고 C/C++의 대응 개념과 어떻게 다른지 알아봅니다. 핵심 통찰: 대부분의 Rust 제어 흐름은 값을 반환합니다.

- Rust에서 ```if```는 실제로는 표현식(expression)입니다. 즉, 값을 할당하는 데 사용될 수 있으며 동시에 문장(statement)처럼 동작하기도 합니다. [▶ 직접 시도해 보기](https://play.rust-lang.org/)

```rust
fn main() {
    let x = 42;
    if x < 42 {
        println!("생명의 비밀보다 작습니다.");
    } else if x == 42 {
        println!("생명의 비밀과 같습니다.");
    } else {
        println!("생명의 비밀보다 큽니다.");
    }
    let is_secret_of_life = if x == 42 {true} else {false};
    println!("{}", is_secret_of_life);
}
```

# while 및 for를 사용한 Rust 루프
- ```while``` 키워드는 표현식이 참인 동안 루프를 도는 데 사용됩니다.
```rust
fn main() {
    let mut x = 40;
    while x != 42 {
        x += 1;
    }
}
```
- ```for``` 키워드는 범위를 순회하는 데 사용될 수 있습니다.
```rust
fn main() {
    // 43은 출력되지 않습니다. 마지막 요소를 포함하려면 40..=43을 사용하세요.
    for x in 40..43 {
        println!("{}", x);
    } 
}
```

# loop를 사용한 Rust 루프
- ```loop``` 키워드는 ```break```를 만날 때까지 무한 루프를 생성합니다.
```rust
fn main() {
    let mut x = 40;
    // 루프에 선택적 레이블을 지정하려면 아래를 'here: loop로 변경하세요.
    loop {
        if x == 42 {
            break; // x의 값을 반환하려면 break x;를 사용하세요.
        }
        x += 1;
    }
}
```
- ```break``` 문은 ```loop``` 표현식의 값을 할당하는 데 사용할 수 있는 선택적 표현식을 포함할 수 있습니다.
- ```continue``` 키워드는 ```loop```의 처음으로 돌아가는 데 사용됩니다.
- 루프 레이블은 ```break``` 또는 ```continue```와 함께 사용될 수 있으며 중첩 루프를 다룰 때 유용합니다.

# Rust 표현식 블록(Expression Blocks)
- Rust 표현식 블록은 단순히 ```{}```로 감싸진 일련의 표현식입니다. 평가된 값은 단순히 블록의 마지막 표현식입니다.
```rust
fn main() {
    let x = {
        let y = 40;
        y + 2 // 주의: ;을 생략해야 합니다.
    };
    // Python 스타일의 출력을 확인해 보세요.
    println!("{x}");
}
```
- Rust 스타일은 함수에서 ```return``` 키워드를 생략하기 위해 이를 사용합니다.
```rust
fn is_secret_of_life(x: u32) -> bool {
    // if x == 42 {true} else {false}와 같습니다.
    x == 42 // 주의: ;을 생략해야 합니다.
}
fn main() {
    println!("{}", is_secret_of_life(42));
}
```
