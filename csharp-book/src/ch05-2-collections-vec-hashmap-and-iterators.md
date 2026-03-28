# 컬렉션과 반복자: `Vec`, `HashMap`, 그리고 LINQ를 넘어서

> **학습 목표:** C#의 `List<T>`와 `Dictionary<K, V>`에 대응하는 Rust의 핵심 컬렉션을 배우고, 메모리 효율적인 반복자(Iterator) 활용법을 익힙니다. 특히 Rust가 인덱스 접근 시 예외 대신 `Option`을 반환하는 이유와 소유권이 컬렉션 조작에 미치는 영향을 이해합니다.

---

### 1. `Vec<T>` vs `List<T>`
C#의 `List`처럼 동적으로 크기가 변하는 배열이지만, 소유권 규칙이 엄격하게 적용됩니다.

| **비교 항목** | **C# `List<T>`** | **Rust `Vec<T>`** |
| :--- | :--- | :--- |
| **할당 위치** | 항상 힙(Heap) | 항상 힙(Heap) |
| **전달 방식** | 참조 복사 (원본 공유) | 소유권 이동(Move) 또는 빌림(&) |
| **생성 매크로** | `new List<int> {1, 2}` | `vec![1, 2]` |
| **크기 확인** | `.Count` | `.len()` |

```rust
// [소유권 이동 주의!]
let numbers = vec![1, 2, 3];
process_data(numbers); // 소유권이 함수로 넘어감
// println!("{:?}", numbers); // ❌ 에러: numbers는 이제 함수 내부의 것입니다.
```

---

### 2. `HashMap<K, V>` vs `Dictionary<K, V>`
키-값 쌍을 저장하는 컬렉션입니다. Rust의 `HashMap`은 기본적으로 강력한 보안(DoS 공격 방어)을 위해 `SipHash`를 사용합니다.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert("Alice", 100);

// Entry API: "값이 없으면 넣고 있으면 수정해"를 한 번에!
scores.entry("Bob").or_insert(80);
*scores.entry("Alice").or_default() += 10;
```

---

### 3. 안전한 접근: 예외(Exception) 대신 `Option`
C#에서 범위 밖의 인덱스에 접근하면 `IndexOutOfRangeException`이 발생하지만, Rust는 이를 안전하게 처리하도록 강제합니다.

```rust
let v = vec![1, 2, 3];

// 1. 위험한 접근 (C# 스타일)
// let val = v[10]; // ❌ 인덱스 범위 초과 시 패닉(프로그램 종료)

// 2. 안전한 접근 (Rust 권장)
if let Some(val) = v.get(10) {
    println!("값: {}", val);
} else {
    println!("값이 없습니다.");
}
```

---

### 4. 반복자(Iterator)와 LINQ
Rust의 반복자는 C#의 LINQ 표현력과 C 수준의 성능을 동시에 제공합니다.

| **구분** | **C# (LINQ)** | **Rust (Iterator)** |
| :--- | :--- | :--- |
| **필터링** | `.Where(x => ...)` | `.filter(|x| ...)` |
| **변환** | `.Select(x => ...)` | `.map(|x| ...)` |
| **수집** | `.ToList()`, `.ToArray()` | `.collect()` |
| **성능** | 델리게이트 호출 오버헤드 | **단형성화(Monomorphization)**로 최적화됨 |

```rust
// [LINQ 스타일의 Rust 코드]
let numbers = vec![1, 2, 3, 4, 5];
let doubled_evens: Vec<i32> = numbers.iter()
    .filter(|&&x| x % 2 == 0) // 짝수만 골라서
    .map(|&x| x * 2)          // 2배로 만든 뒤
    .collect();               // 다시 벡터로 수집
```

---

### 💡 실무 팁: `into_iter()` vs `iter()`
- `iter()`: 요소를 **빌려와서** 순회합니다. 원본 벡터를 계속 쓸 수 있습니다.
- `into_iter()`: 요소를 **소비하며(소유권 획득)** 순회합니다. 순회 후 원본 벡터는 사라집니다.
- 데이터를 단순히 읽기만 한다면 항상 `iter()`를 먼저 고려하세요.

