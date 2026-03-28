# From과 Into: 우아한 타입 변환

> **학습 목표:** Rust에서 타입을 변환하는 가장 표준적인 방법인 **`From`**과 **`Into`** 트레이트를 배웁니다. C#의 암시적/명시적 캐스팅과 비교하며, 실패할 수 있는 변환을 안전하게 처리하는 `TryFrom`과 문자열 파싱을 위한 `FromStr` 활용법을 익힙니다.

---

### 1. `From`과 `Into`: 하나를 구현하면 둘을 얻는다
C#에서는 `implicit` 또는 `explicit` 연산자를 정의하여 타입을 변환하지만, Rust는 트레이트를 사용합니다. 특히 **`From`을 구현하면 `Into`는 컴파일러가 자동으로 구현**해 줍니다.

- **`From<T>`**: "T로부터 나를 만든다." (생성자 역할)
- **`Into<T>`**: "나를 T로 변환한다." (소비자 역할)

```rust
// [From 구현 예시]
impl From<i32> for Number {
    fn from(item: i32) -> Self {
        Number { value: item }
    }
}

// [Into 사용 예시]
let num: Number = 5.into(); // From<i32>가 있으므로 자동으로 작동
```

---

### 2. `TryFrom`과 `TryInto`: 실패를 고려한 변환
모든 변환이 항상 성공하는 것은 아닙니다. 예를 들어, `i64`를 `u32`로 바꿀 때 값이 너무 크면 실패할 수 있습니다. 이때는 `Result`를 반환하는 `TryFrom`을 사용합니다.

```rust
use std::convert::TryFrom;

let big_num: i64 = 1_000_000_000_000;
let try_u32 = u32::try_from(big_num); // Result<u32, Error> 반환
```

---

### 3. 문자열 변환 (`Display`와 `FromStr`)
- **`Display`**: 구조체를 문자열로 바꾸고 싶을 때 구현합니다. 구현하면 자동으로 `.to_string()` 메서드가 생깁니다.
- **`FromStr`**: 문자열을 구조체로 파싱하고 싶을 때 구현합니다. 구현하면 자동으로 `.parse()` 메서드가 생깁니다.

```rust
// [문자열 파싱 예시]
let my_val: MyStruct = "100".parse().expect("파싱 실패");
```

---

### 💡 실무 팁: `impl Into<T>`를 인자로 받기
함수가 특정 타입 `T`뿐만 아니라 그 타입으로 변환될 수 있는 **모든 타입**을 인자로 받게 하고 싶다면 `impl Into<T>`를 사용하세요. 호출하는 쪽에서 훨씬 유연하게 인자를 넘길 수 있습니다.

```rust
fn greet(name: impl Into<String>) {
    let name_str = name.into();
    println!("안녕하세요, {}님!", name_str);
}

greet("앨리스"); // &str 전달 가능
greet(String::from("밥")); // String 전달 가능
```

