# 데이터 구조와 컬렉션: 효율적인 설계 기법

> **학습 목표:** Rust의 튜플, 배열, 슬라이스, 벡터를 C#의 대응 개념과 비교하며 익힙니다. 특히 C# 클래스와 Rust 구조체의 메모리 배치 차이를 이해하고, 컴파일 타임에 비즈니스 규칙을 강제하는 **뉴타입(Newtype) 패턴**을 마스터합니다.

---

### 1. 튜플과 구조 분해 (Tuples & Destructuring)
C#의 `ValueTuple`처럼 Rust의 튜플도 여러 타입의 값을 하나로 묶는 가벼운 방법입니다.

- **C#**: `(int age, string name) = (30, "Alice");` (항상 가변적)
- **Rust**: `let (age, name) = (30, "Alice");` (기본 불변, 구조 분해로 즉시 사용 가능)

```rust
// [튜플 활용 예시]
fn get_coordinates() -> (i32, i32) { (10, 20) }

let (x, y) = get_coordinates(); // 구조 분해
println!("x: {}, y: {}", x, y);

let result = get_coordinates();
println!("x: {}", result.0); // 인덱스로 접근 가능
```

---

### 2. 뉴타입(Newtype) 패턴: 제로 비용 타입 안전성
C#에서 `string email`과 `string address`를 실수로 섞어 쓰는 것을 방지하려면 런타임 검사가 필요하지만, Rust는 이를 타입 시스템 차원에서 해결합니다.

```rust
struct Email(String);
struct UserId(u64);

fn send_welcome(id: UserId, email: Email) { /* ... */ }

// Email과 UserId는 컴파일 타임에 엄격히 구분됩니다.
// 하지만 런타임에는 그냥 String과 u64일 뿐이므로 오버헤드가 '0'입니다.
```

---

### 3. 배열, 벡터, 슬라이스
데이터의 수명을 누가 관리하느냐에 따라 세 가지로 나뉩니다.

| **종류** | **C# 대응 개념** | **메모리 위치** | **특징** |
| :--- | :--- | :--- | :--- |
| **배열** (`[T; N]`) | `T[]` (고정) | 스택(Stack) | 컴파일 타임에 크기가 정해져야 함 |
| **벡터** (`Vec<T>`) | `List<T>` | 힙(Heap) | 실행 중 크기 확장 가능, 소유권 가짐 |
| **슬라이스** (`&[T]`) | `Span<T>` | 빌려옴 | 기존 데이터의 일부를 바라보는 뷰(View) |

---

### 4. 구조체(Struct) vs 클래스(Class)
가장 근본적인 차이는 **메모리 레이아웃**입니다.

- **C# 클래스**: 항상 힙에 존재하며, 객체 헤더와 가상 함수 테이블(VTable) 포인터를 가집니다. (항상 참조 타입)
- **Rust 구조체**: 기본적으로 스택에 존재하며, 오직 필드 데이터만 가집니다. 헤더 오버헤드가 없습니다.

```rust
struct Person {
    name: String,
    age: u32,
}

impl Person {
    // 팩토리 메서드 (정적 메서드와 유사)
    fn new(name: &str, age: u32) -> Self {
        Self { name: name.to_string(), age }
    }

    // 인스턴스 메서드 (self를 받음)
    fn celebrate_birthday(&mut self) {
        self.age += 1;
    }
}
```

---

### 💡 실무 팁: `&[T]`를 매개변수로 쓰세요
함수에서 데이터를 읽기만 한다면 `&Vec<T>` 대신 `&[T]`(슬라이스)를 받으세요. 이렇게 선언하면 `Vec` 뿐만 아니라 일반 배열, 심지어 벡터의 일부분까지도 모두 인자로 받을 수 있어 유연성이 극대화됩니다.

