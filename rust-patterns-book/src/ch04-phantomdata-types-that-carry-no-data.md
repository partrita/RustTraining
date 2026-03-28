# 4. PhantomData: 데이터를 갖지 않는 타입 🔴

> **학습 목표:**
> - `PhantomData<T>`가 왜 존재하는지, 그리고 이것이 해결하는 세 가지 문제를 이해합니다.
> - 컴파일 타임에 스코프를 강제하기 위한 **수명 브랜딩(Lifetime branding)**을 익힙니다.
> - 차원이 안전한 연산을 위한 **단위 시스템(Unit-of-measure)** 패턴을 배웁니다.
> - **변형성(Variance: 공변, 반공변, 불변)**의 개념과 `PhantomData`를 통한 제어 방법을 마스터합니다.

---

### PhantomData가 해결하는 것

`PhantomData<T>`는 크기가 0인 타입(ZST)으로, 컴파일러에게 "이 구조체는 실제로 `T`를 포함하지는 않지만, 논리적으로는 `T`와 연관되어 있다"라고 알려주는 역할을 합니다. 이는 메모리를 전혀 사용하지 않으면서 **변형성(Variance)**, **드롭 체크(Drop check)**, **자동 트레이트 추론**에 영향을 미칩니다.

```rust
use std::marker::PhantomData;

// PhantomData가 없는 경우:
struct Slice<'a, T> {
    ptr: *const T,
    len: usize,
    // 문제: 컴파일러는 이 구조체가 'a 수명 동안 데이터를 빌려오고 있는지,
    // 또는 드롭 체크 시 T를 고려해야 하는지 알 수 없습니다.
}

// PhantomData를 사용하는 경우:
struct Slice<'a, T> {
    ptr: *const T,
    len: usize,
    _marker: PhantomData<&'a T>,
    // 이제 컴파일러는 다음을 알게 됩니다:
    // 1. 이 구조체는 'a 수명의 데이터를 빌려온다.
    // 2. 'a에 대해 공변(Covariant)이다 (수명이 줄어들 수 있음).
    // 3. 드롭 체크 시 T 타입을 고려한다.
}
```

**PhantomData의 세 가지 역할:**

| 역할 | 예시 | 설명 |
| :--- | :--- | :--- |
| **수명 바인딩** | `PhantomData<&'a T>` | 구조체가 `'a` 수명을 빌리고 있는 것으로 취급함 |
| **소유권 시뮬레이션** | `PhantomData<T>` | 드롭 체크 시 구조체가 `T`를 소유한 것으로 간주함 |
| **변형성 제어** | `PhantomData<fn(T)>` | 구조체를 `T`에 대해 반공변(Contravariant)으로 만듦 |

---

### 단위 시스템 (Unit-of-Measure) 패턴

서로 호환되지 않는 단위(미터, 초 등)를 섞어서 연산하는 실수를 컴파일 타임에 방지할 수 있습니다. 런타임 비용은 제로입니다.

```rust
use std::marker::PhantomData;

struct Meters;
struct Seconds;

#[derive(Debug, Clone, Copy)]
struct Quantity<Unit> {
    value: f64,
    _unit: PhantomData<Unit>,
}

fn main() {
    let dist = Quantity::<Meters>::new(100.0);
    let time = Quantity::<Seconds>::new(9.58);
    
    // let nonsense = dist + time; // ❌ 컴파일 에러: Meters와 Seconds는 더할 수 없음
}
```
> **타입 시스템의 마법**: `PhantomData<Meters>`는 크기가 0이므로, `Quantity<Meters>`는 메모리 상에서 `f64`와 동일한 레이아웃을 가집니다. 성능 저하 없이 완벽한 타입 안전성을 제공합니다.

---

### 변형성(Variance) — 왜 PhantomData의 타입 파라미터가 중요한가?

변형성은 제네릭 타입이 하위 타입이나 상위 타입으로 대체될 수 있는지를 결정합니다. Rust에서 하위 타입은 보통 "더 긴 수명을 가진 타입"을 의미합니다.

#### 세 가지 변형성 요약:

| 변형성 | 의미 | "대체가 가능한가?" | Rust 예시 |
| :--- | :--- | :--- | :--- |
| **공변 (Covariant)** | 하위 타입 관계가 유지됨 | `'long`을 `'short`가 필요한 곳에 사용 ✅ | `&'a T`, `Vec<T>`, `Box<T>` |
| **반공변 (Contravariant)** | 하위 타입 관계가 역전됨 | `'short`를 `'long`이 필요한 곳에 사용 ✅ | `fn(T)` (인자 위치) |
| **불변 (Invariant)** | 대체 불가능 | 수명이 정확히 일치해야 함 ✅ | `&mut T`, `Cell<T>` |

#### PhantomData 변형성 치트 시트:

| PhantomData 타입 | T에 대한 변형성 | 'a에 대한 변형성 | 사용 시점 |
| :--- | :--- | :--- | :--- |
| `PhantomData<T>` | **공변** | — | `T`를 논리적으로 소유할 때 |
| `PhantomData<&'a T>` | **공변** | **공변** | `T`를 `'a` 수명 동안 빌릴 때 |
| `PhantomData<&'a mut T>` | **불변** | **공변** | `T`를 가변으로 빌릴 때 |
| `PhantomData<*const T>` | **공변** | — | 소유하지 않는 불변 포인터 |
| `PhantomData<*mut T>` | **불변** | — | 소유하지 않는 가변 포인터 |
| `PhantomData<fn(T)>` | **반공변** | — | `T`가 인자 위치에 올 때 (콜백 등) |

---

### 📝 연습 문제: PhantomData를 활용한 단위 시스템 확장 ★★ (~30분)

단위 시스템 패턴을 확장하여 다음을 지원해 보세요:
- `Meters`, `Seconds`, `Kilograms` 단위 정의
- 동일 단위 간의 덧셈 지원
- 곱셈 지원: `Meters * Meters = SquareMeters`
- 나눗셈 지원: `Meters / Seconds = MetersPerSecond`

---

### 📌 요약
- `PhantomData<T>`는 런타임 비용 없이 타입/수명 정보를 전달합니다.
- 수명 브랜딩, 변형성 제어, 단위 시스템 패턴 등에 사용됩니다.
- **드롭 체크**: `PhantomData<T>`는 해당 타입이 논리적으로 `T`를 소유하고 있음을 컴파일러에게 알려줍니다.

