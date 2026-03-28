# 열거형과 패턴 매칭: 데이터에 의미 부여하기

> **학습 목표:** 파이썬 3.10에 도입된 `match-case`와 `Union` 타입의 모체가 된 Rust의 강력한 **열거형(Enum)**과 **패턴 매칭**을 배웁니다. 특히 데이터가 포함된 열거형과 `None` 에러를 원천 차단하는 `Option<T>`의 활용법을 익힙니다.

---

### 1. 데이터가 포함된 열거형 (Enum with Data)
파이썬의 `Union` 타입이나 클래스 상속으로 처리하던 복잡한 상태를 Rust는 하나의 열거형으로 우아하게 표현합니다.

```python
# [Python 3.10+] Union 타입과 match
from typing import Union
Shape = Union[Circle, Rectangle]

def area(shape: Shape):
    match shape:
        case Circle(r): return 3.14 * r * r
        case Rectangle(w, h): return w * h
    # 만약 새로운 도형이 추가되었는데 여기서 누락해도 경고가 없습니다.
```

```rust
// [Rust] 데이터가 포함된 열거형
enum Shape {
    Circle(f64),
    Rectangle(f64, f64),
    Triangle { base: f64, height: f64 },
}

fn area(shape: &Shape) -> f64 {
    match shape {
        Shape::Circle(r) => 3.14 * r * r,
        Shape::Rectangle(w, h) => w * h,
        Shape::Triangle { base, height } => 0.5 * base * height,
        // 모든 경우(Circle, Rectangle, Triangle)를 처리하지 않으면 빌드가 안 됩니다.
    }
}
```

---

### 2. 철저한 매칭 (Exhaustive Matching)
파이썬의 `match`는 처리되지 않은 케이스가 있어도 런타임에 조용히 `None`을 반환하고 넘어가지만, Rust는 **철저함(Exhaustiveness)**을 강제합니다. 새로운 상태가 추가되면 컴파일 에러를 통해 수정해야 할 모든 지점을 알려줍니다.

---

### 3. `None` 안전성: Option<T>
파이썬의 가장 큰 골칫거리인 `None` 참조 에러(`AttributeError`)를 Rust는 `Option<T>` 열거형으로 해결합니다.

| **패턴** | **Python** | **Rust** |
| :--- | :--- | :--- |
| **값 확인** | `if x is not None:` | `if let Some(x) = opt {` |
| **기본값 제공** | `x or default` | `opt.unwrap_or(default)` |
| **값 변환** | `f(x) if x else None` | `opt.map(f)` |
| **조기 반환** | `if x is None: return` | `let x = opt?;` |

#### 💡 실무 팁: `?` 연산자의 위력
함수 내에서 `None`을 만나면 바로 함수를 종료하고 `None`을 반환하고 싶을 때, 파이썬처럼 복잡한 `if x is None` 문을 쓸 필요가 없습니다. 그냥 변수 뒤에 `?`를 붙이세요. (단, 함수의 반환 타입이 `Option`이어야 합니다.)

---

### 4. 패턴 매칭의 다양한 기법
- **가드(Guard)**: `match x { n if n > 0 => ... }` 처럼 추가 조건을 붙일 수 있습니다. (파이썬의 `case x if x > 0:`)
- **와일드카드(`_`)**: 처리할 필요가 없는 나머지 모든 경우를 묶습니다.
- **범위 매칭**: `1..=10 => ...` 처럼 연속된 숫자를 쉽게 매칭합니다.

---

### 💡 실무 팁: 열거형을 통한 상태 설계
단순히 "에러", "성공" 같은 문자열 상수를 쓰는 대신, `Enum`을 사용해 보세요. 데이터와 상태를 하나로 묶어 관리할 수 있어 코드가 훨씬 견고해지고 가독성이 좋아집니다.

