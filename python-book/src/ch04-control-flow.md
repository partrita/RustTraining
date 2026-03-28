# 제어 흐름: 조건문, 반복문, 그리고 표현식

> **학습 목표:** Rust의 `if`, `for`, `match` 등이 파이썬과 어떻게 다른지 배옵니다. 특히 모든 블록이 값을 반환할 수 있는 '표현식(Expression)' 중심의 사고방식을 익히고, 세미콜론 유무에 따른 반환값의 차이를 명확히 이해합니다.

---

### 1. 조건문 (`if`, `else if`, `else`)
파이썬의 `elif` 대신 `else if`를 사용하며, 조건식에 괄호는 필요 없지만 실행 블록의 중괄호(`{}`)는 필수입니다.

```python
# [Python]
if temperature > 30:
    status = "hot"
else:
    status = "ok"
```

```rust
// [Rust] if는 표현식입니다. 값을 바로 변수에 할당할 수 있습니다.
let status = if temperature > 30 {
    "hot"
} else {
    "ok" // 세미콜론이 없으면 이 값이 반환됩니다.
};
```

---

### 2. 반복문과 반복자
파이썬의 `range`, `enumerate`, 리스트 컴프리헨션 등이 Rust에서 어떻게 구현되는지 비교해 봅니다.

| **파이썬** | **Rust** | **비고** |
| :--- | :--- | :--- |
| `for i in range(5):` | `for i in 0..5 {` | `0..5`는 상한 미포함, `0..=5`는 포함 |
| `enumerate(list)` | `.iter().enumerate()` | 인덱스와 값을 동시에 추출 |
| `[x**2 for x in r]` | `.map(|x| x * x).collect()` | 반복자 체인을 이용한 지연 연산 |
| `while True:` | `loop {` | Rust는 무한 루프 전용 키워드 `loop` 권장 |

#### 💡 실무 팁: `loop`에서 값 반환하기
Rust의 `loop`는 `break`와 함께 값을 반환할 수 있습니다. 이는 특정 조건을 만족할 때까지 재시도하는 로직에서 매우 유용합니다.

```rust
let result = loop {
    let input = get_input();
    if let Ok(num) = input.parse::<i32>() {
        break num; // 숫자를 파싱하면 루프를 종료하고 값을 반환
    }
};
```

---

### 3. 표현식 vs 문장 (The Semicolon Rule)
Rust에서 가장 중요한 규칙 중 하나입니다.
- **표현식(Expression)**: 세미콜론 없이 끝나며 **값을 반환**합니다. (함수의 마지막 줄 등)
- **문장(Statement)**: 세미콜론으로 끝나며 **값을 반환하지 않습니다** (단순 실행).

```rust
fn add(a: i32, b: i32) -> i32 {
    a + b // 세미콜론이 없으므로 a + b 결과가 return됨
}
```

---

### 4. 함수와 메서드
파이썬과 달리 매개변수와 반환값의 타입을 생략할 수 없습니다.

- **`fn`**: 함수를 정의합니다.
- **`&self` vs `&mut self`**: 메서드에서 객체를 읽기 전용으로 쓸지(`&self`), 수정할지(`&mut self`) 명확히 구분해야 합니다. 파이썬은 모든 `self`가 가변적이지만 Rust는 엄격히 제한합니다.

---

### 💡 실무 팁: 파이썬의 `match-case` vs Rust `match`
파이썬 3.10에 도입된 `match-case`는 Rust의 `match`를 벤치마킹한 것입니다. Rust의 `match`는 모든 경우의 수를 처리하지 않으면 컴파일조차 되지 않으므로, 런타임에 처리되지 않은 케이스로 인한 버그가 발생하지 않습니다.

