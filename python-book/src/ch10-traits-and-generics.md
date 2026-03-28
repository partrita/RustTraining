# 트레이트와 제네릭: 유연하고 안전한 설계

> **학습 목표:** 파이썬의 '덕 타이핑(Duck Typing)'과 Rust의 **트레이트(Trait)** 방식을 비교하며 정적 타입 시스템에서의 추상화를 배웁니다. 제네릭을 통해 코드 재사용성을 높이고, 컴파일 타임에 모든 타입 제약을 검증하는 법을 익힙니다.

---

### 1. 트레이트 vs 덕 타이핑
파이썬은 "오리처럼 걷고 오리처럼 울면 오리다"라고 런타임에 판단하지만, Rust는 "오리처럼 행동하려면 반드시 `오리(Duck)` 트레이트를 구현해야 한다"라고 컴파일 타임에 선언합니다.

```python
# [Python] 덕 타이핑 (런타임에 메서드 유무 확인)
def total_area(shapes):
    return sum(s.area() for s in shapes)

# area() 메서드가 없는 객체가 들어오면 런타임에 에러!
```

```rust
// [Rust] 트레이트 (컴파일 타임에 계약 명시)
trait HasArea {
    fn area(&self) -> f64;
}

fn total_area(shapes: &[&dyn HasArea]) -> f64 {
    shapes.iter().map(|s| s.area()).sum()
}

// HasArea를 구현하지 않은 타입을 넣으면 컴파일 에러!
```

---

### 2. 제네릭과 트레이트 바운드
파이썬의 `TypeVar`나 `Generic`과 유사하지만, Rust는 훨씬 더 강력하고 안전한 제약을 제공합니다.

```rust
// "T는 반드시 Display와 Debug를 구현해야 한다"는 제약을 겁니다.
fn log_and_print<T>(item: &T)
where
    T: std::fmt::Display + std::fmt::Debug,
{
    println!("로그: {:?}", item);
    println!("출력: {}", item);
}
```

---

### 3. 주요 표준 트레이트 (파이썬의 매직 메서드 대응)
파이썬에서 `__str__`이나 `__add__` 같은 '던더(Dunder)' 메서드로 하던 일들을 Rust에서는 트레이트 구현으로 처리합니다.

| **Rust 트레이트** | **파이썬 매직 메서드** | **용도** |
| :--- | :--- | :--- |
| **`Display`** | `__str__` | 사람이 읽기 좋은 문자열 출력 |
| **`Debug`** | `__repr__` | 개발자용 디버깅 출력 (`{:?}`) |
| **`PartialEq` / `Eq`** | `__eq__` | 값 비교 (`==`, `!=`) |
| **`PartialOrd` / `Ord`** | `__lt__`, `__gt__` 등 | 크기 비교 및 정렬 |
| **`Add`, `Sub` 등** | `__add__`, `__sub__` | 연산자 오버로딩 |
| **`Iterator`** | `__iter__`, `__next__` | 반복문 처리 |
| **`Clone`** | `copy.deepcopy()` | 데이터 깊은 복사 |

---

### 4. 정적 디스패치 vs 동적 디스패치
- **정적 디스패치 (`impl Trait`)**: 컴파일 타임에 각 타입에 맞는 코드를 생성합니다. 실행 속도가 가장 빠릅니다. (기본값)
- **동적 디스패치 (`dyn Trait`)**: 런타임에 실제 타입을 확인하여 호출합니다. 파이썬의 기본 작동 방식과 비슷합니다.

---

### 💡 실무 팁: `# [derive(...)]` 활용하기
`Debug`, `Clone`, `PartialEq` 등 자주 쓰이는 트레이트들은 일일이 구현할 필요 없이 구조체 위에 `#[derive(Debug, Clone)]` 처럼 한 줄만 추가하면 컴파일러가 알아서 구현해 줍니다. 파이썬의 `dataclass`보다 훨씬 강력하고 편리합니다.

