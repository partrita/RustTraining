# 제네릭 제약 조건: `where`와 트레이트 바운드

> **학습 목표:** Rust의 제네릭 제약 조건을 C#의 `where` 절과 비교하며 배웁니다. 더 복잡한 제약 조건을 깔끔하게 표현하는 `where` 문법, 타입 간의 관계를 정의하는 연관 타입(Associated Types), 그리고 고급 기능인 고차 트레이트 바운드(HRTBs)를 익힙니다.

---

### 1. 트레이트 바운드 (Trait Bounds)
C#의 `where T : IInterface`처럼, Rust는 `T: Trait` 형식을 사용하여 제네릭 타입이 특정 기능을 가져야 함을 명시합니다.

```rust
// [간단한 인라인 바운드]
fn print_it<T: Display>(item: T) {
    println!("{}", item);
}

// [다중 제약 조건]
fn clone_and_print<T: Display + Clone>(item: T) {
    let cloned = item.clone();
    println!("{}", cloned);
}
```

---

### 2. `where` 절: 복잡한 제약 조건 정리하기
제약 조건이 많아지면 함수 시그니처가 너무 길어져 읽기 힘들어집니다. 이때 `where` 절을 사용하면 코드가 훨씬 깔끔해집니다.

```rust
// [C#의 where와 매우 유사한 구조]
fn complex_function<T, U>(t: T, u: U) -> i32
where
    T: Display + Clone,
    U: Debug + PartialEq,
{
    // 로직 구현...
}
```

---

### 3. 조건부 트레이트 구현 (Blanket Implementation)
Rust에서는 특정한 조건을 만족하는 타입들에 대해 한꺼번에 트레이트를 구현할 수 있습니다. 이는 C#에는 없는 매우 강력한 기능입니다.

```rust
// "Display를 구현하는 모든 타입 T에 대해, ToString 트레이트를 자동으로 구현하라."
impl<T: Display> ToString for T {
    // ...
}
```

---

### 4. 연관 타입 (Associated Types)
제네릭 매개변수가 너무 많아지는 것을 방지하기 위해 트레이트 내부에 타입을 정의하는 방식입니다. `Iterator` 트레이트가 대표적인 예입니다.

```rust
trait MyTrait {
    type Output; // 연관 타입 정의
    fn compute(&self) -> Self::Output;
}

impl MyTrait for MyStruct {
    type Output = i32; // 구체적인 타입 결정
    fn compute(&self) -> i32 { 42 }
}
```

---

### 💡 실무 팁: `new()` 제약 조건 대신 `Default`
C#의 `where T : new()`는 매개변수 없는 생성자를 요구합니다. Rust에서는 이를 **`Default` 트레이트**가 담당합니다. `T: Default` 바운드를 걸고 `T::default()`를 호출하면 됩니다.

