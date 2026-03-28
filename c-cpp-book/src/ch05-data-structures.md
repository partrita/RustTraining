# Rust의 핵심 데이터 구조와 컬렉션

> **학습 목표:** Rust를 구성하는 다양한 데이터 구조(배열, 튜플, 슬라이스, 문자열, 구조체, `Vec`, `HashMap`)를 익힙니다. 내용이 방대한 장이므로 특히 `String`과 `&str`의 차이점, 그리고 구조체의 동작 방식에 집중해 주세요. 참조(References)와 빌림(Borrowing) 개념은 7장에서 더욱 심도 있게 다룰 예정입니다.

---

### Rust 배열 (Arrays)
배열은 **동일한 타입**의 요소를 **고정된 개수**만큼 담는 구조입니다.

- **주요 특징**
    - 다른 타입과 마찬가지로 기본적으로 **불변(Immutable)**입니다. (`mut` 없이 선언 시)
    - 대괄호 `[]`를 사용해 인덱스로 접근하며, 실행 시 항상 **경계 검사(Bounds Check)**를 수행합니다.
    - `len()` 메서드로 배열의 길이를 알 수 있습니다.

```rust
fn get_next_index(current: usize) -> usize {
    current + 1        
}

fn main() {
    // 값이 42인 요소 3개를 가진 배열 초기화 [타입; 개수]
    let a: [u8; 3] = [42; 3];
    
    // 일반적인 초기화 방식
    // let a = [42u8, 42u8, 42u8];
    
    for x in a {
        println!("요소: {x}");
    }
    
    let next_idx = get_next_index(a.len());
    // 아래 주석을 해제하면 실행 시 인덱스 초과로 패닉(Panic)이 발생합니다.
    // println!("{}", a[next_idx]);
}
```

- **다차원 배열**: 배열은 중첩하여 선언할 수 있습니다.
    - Rust는 디버깅을 위한 포맷터(`:?`, `:#?`)를 제공합니다.
```rust
fn main() {
    let matrix = [
        [40, 0], 
        [41, 0],
        [42, 1],
    ];
    for row in matrix {
        println!("행 데이터: {row:?}"); // :?는 디버그 출력 양식입니다.
    }
}
```

---

### 튜플 (Tuples)
튜플은 **다양한 타입**의 값을 하나의 복합 타입으로 묶을 때 사용하며, 크기는 고정됩니다.

- **주요 특징**
    - 각 요소는 마침표와 인덱스(`.0`, `.1` 등)를 통해 접근합니다.
    - 빈 튜플 `()`은 **유닛(Unit)** 값이라 부르며, C/C++의 `void`와 유사한 용도로 쓰입니다.
    - **구조 분해(Destructuring)**를 통해 튜플의 값을 개별 변수로 쉽게 분리할 수 있습니다.

```rust
fn get_result() -> (u32, bool) {
    (42, true)        
}

fn main() {
   let t: (u8, bool) = (42, true);
   println!("인덱스 접근: {}, {}", t.0, t.1);
   
   let (num, flag) = get_result(); // 구조 분해 할당
   println!("구조 분해 결과: {num}, {flag}");
}
```

---

### 참조자 (References)
Rust의 참조자는 C의 포인터와 개념적으로 유사하지만, 안전성을 위해 엄격한 규칙이 적용됩니다.

- **빌림(Borrowing) 규칙**
    - **공유 참조 (`&T`)**: 동시에 여러 개의 읽기 전용 참조자를 가질 수 있습니다.
    - **가변 참조 (`&mut T`)**: 특정 시점에 단 **하나**의 가변 참조자만 허용되며, 다른 참조자와 공존할 수 없습니다.
    - **수명(Lifetime)**: 참조자는 자신이 가리키는 원본 변수보다 더 오래 살아남을 수 없습니다. (7장에서 상세히 다룸)

```rust
fn main() {
    let mut a = 42;
    {
        let b = &a; // 공유 참조 생성
        let c = b;  // 참조 복사
        println!("값 확인: {} {}", *b, *c); 
        // b가 유효한 동안에는 아래와 같은 가변 참조 생성이 금지됩니다.
        // let d = &mut a; 
    }
    // b와 c의 범위가 끝났으므로 가변 참조 생성이 가능해집니다.
    let d = &mut a; 
    *d = 43;
}
```

---

### 슬라이스 (Slices)
슬라이스는 컬렉션의 연속된 일부분을 가리키는 참조입니다.

- **특징**
    - 배열과 달리 컴파일 타임에 크기를 알 필요가 없습니다.
    - 내부적으로는 시작 위치를 가리키는 포인터와 길이를 담은 **'뚱뚱한 포인터(Fat-pointer)'** 구조입니다.

```rust
fn main() {
    let a = [40, 41, 42, 43];
    let b = &a[1..3];   // 인덱스 1부터 2까지 (41, 42)
    let c = &a[1..];    // 인덱스 1부터 끝까지
    let d = &a[..];     // 전체 범위
    println!("슬라이스 결과: {b:?} {c:?} {d:?}");
}
```

---

### 상수(Constants)와 정적 변수(Statics)

- **`const`**: **컴파일 타임**에 평가되는 상수로, 사용되는 모든 곳에 인라인(Inline)됩니다.
- **`static`**: 프로그램의 **전체 실행 수명** 동안 고정된 메모리 주소를 가지는 전역 변수입니다.

```rust
const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_COUNTER: u32 = 2;

fn main() {
    println!("상수 값: {}", SECRET_OF_LIFE);
    println!("정적 변수 주소 기반 접근: {GLOBAL_COUNTER}")
}
```

---

# Rust 문자열 관리: String vs &str

Rust에는 용도에 따른 두 가지 핵심 문자열 타입이 있습니다.

1.  **`String`**: 소유권을 가지며, 힙(Heap)에 할당되고 크기 조절이 가능한 문자열 버퍼입니다. (C++의 `std::string`과 유사)
2.  **`&str`**: 고정된 문자열 데이터에 대한 참조(슬라이스)입니다. 메모리를 직접 소유하지 않으며 수명 검사를 통해 안전성을 보장받습니다. (C++의 `std::string_view`와 유사하지만 훨씬 안전함)

> **핵심 차이점**: `&str`은 컴파일 단계에서 유효성이 철저히 보증되어 댕글링 포인터 문제가 원천 차단됩니다. 또한 모든 Rust 문자열은 **UTF-8 인코딩**을 준수해야 합니다.

### 비교 요약

| **항목** | **C `char*`** | **C++ `std::string`** | **Rust `String`** | **Rust `&str`** |
| :--- | :--- | :--- | :--- | :--- |
| **메모리** | 수동 관리 | 힙 할당, 소유권 관리 | 힙 할당, 자동 해제 | 참조 (수명 관리) |
| **가변성** | 항상 가능 (포인터) | 가변적 | `mut` 선언 시 가변 | 항상 불변 |
| **크기 정보** | `'\0'` 기반 유추 | 자동 추적 | 자동 추적 | 길이 포함 (Fat-pointer) |
| **인코딩** | 보통 ASCII (불분명) | 보통 ASCII (불분명) | **UTF-8 보장** | **UTF-8 보장** |
| **Null 종료자** | 필수 | `c_str()` 시 필요 | 없음 | 없음 |

```rust
fn main() {
    // &str: 문자열 리터럴은 읽기 전용 영역을 가리키는 슬라이스입니다.
    let greeting: &str = "Hello"; 

    // String: 데이터를 힙으로 복사하여 소유하며, 수정이 가능합니다.
    let mut owned = String::from(greeting); 
    owned.push_str(", World!");
    owned.push('!'); 

    // 상호 변환
    let slice: &str = &owned;           // String -> &str (단순 빌림, 추가 비용 없음)
    let owned2: String = slice.to_string(); // &str -> String (새로운 힙 메모리 할당)

    // 문자열 연결 시 주의사항
    let hello = String::from("Hello");
    let world = String::from(", World!");
    // '+' 연산 시 왼쪽 피연산자의 소유권이 이동됩니다.
    let combined = hello + &world; 
    // println!("{hello}"); // 에러! hello는 combined로 소유권이 이동되었습니다.

    // 안전하고 편리한 결합 방식: format! 매크로 활용
    let a = String::from("Hello");
    let b = String::from("World");
    let res = format!("{a}, {b}!"); // 원본 변수들의 소유권이 유지됩니다.
}
```

### 문자열 인덱싱이 금지된 이유
Rust 문자열은 단순한 바이트 배열이 아닌 가변 길이 인코딩인 **UTF-8**입니다. 따라서 `s[0]`과 같은 O(1) 인덱싱은 반환하려는 데이터가 한 글자인지, 바이트의 일부인지 모호하기 때문에 언어 차원에서 허용하지 않습니다.

- **안전한 접근 방법**
```rust
fn main() {
    let s = String::from("안녕하세요");
    
    // 1. 반복자(Iterator) 사용 (가장 안전함)
    let first_char = s.chars().next(); // Option<char> 반환
    
    // 2. 바이트 단위 접근이 필요한 경우
    let bytes = s.as_bytes(); 
    
    // 3. 특정 범위를 슬라이스로 가져오기 (경계 오류 시 패닉 발생 주의)
    let sub = &s[0..3]; // 한글 한 글자는 UTF-8에서 3바이트입니다.
}
```

---

# 구조체 (Structs)

Rust의 구조체는 데이터 상속 개념 없이 **데이터의 조합(Composition)**에 집중합니다.

```rust
struct MyData {
    id: u32,
    is_active: bool,
}

fn main() {
    // 1. 인스턴스 생성
    let data = MyData { id: 1, is_active: true };
    
    // 2. 다른 인스턴스를 바탕으로 나머지 필드 채우기 (Struct Update Syntax)
    let next_data = MyData { id: 2, ..data }; // id만 바꾸고 나머지는 data에서 복사
    
    println!("ID: {}, 활성: {}", next_data.id, next_data.is_active);
}
```

### 튜플 구조체 (Tuple Structs)
필드에 이름이 없는 구조체로, 특정 타입을 명확히 구분하는 **뉴타입(Newtype) 패턴**에 주로 쓰입니다.

```rust
struct WeightInGrams(u32);
struct DistanceInMeters(u32);

fn process_weight(w: WeightInGrams) { /* ... */ }

fn main() {
    let w = WeightInGrams(500);
    let d = DistanceInMeters(500);
    
    // process_weight(d); // 컴파일 에러! 타입이 달라 섞어 쓸 수 없습니다.
}
```

---

# 동적 배열: Vec\<T\>

`Vec<T>`는 런타임에 크기가 변할 수 있는 힙 할당 배열입니다. (C++의 `std::vector`와 거의 동일하게 작동합니다.)

```rust
fn main() {
    let mut v = Vec::new(); // 빈 벡터 생성
    v.push(10);
    v.push(20);
    
    // 1. 안전한 순회: 참조(&)를 사용하여 벡터의 소유권을 유지합니다.
    for x in &v {
        println!("요소: {x}");
    }
    
    // 2. 매크로를 이용한 간편한 초기화
    let v2 = vec![1, 2, 3, 4, 5];
    let v3 = vec![0; 10]; // 0으로 10개 채우기
    
    // 3. 안전한 요소 접근 (인덱싱 호출보다 .get() 사용 권장)
    if let Some(val) = v2.get(0) {
        println!("첫 번째 값: {val}");
    }
}
```

---

# 키-값 저장소: HashMap

`HashMap`은 키를 사용해 값을 빠르게 조회할 수 있는 구조입니다. (사용 전 `use std::collections::HashMap;` 필요)

```rust
use std::collections::HashMap;

fn main() {
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    // 값 가져오기: Option을 반환하므로 안전하게 처리해야 합니다.
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    println!("{} 팀 점수: {}", team_name, score);
}
```

---

# 💡 심층 분석: C++ 대비 Rust 참조자의 특징

C++ 개발자라면 Rust 참조자의 동작 방식이 표면적으로 비슷해 보여 혼란을 겪을 수 있습니다. 다음 핵심 차이점을 꼭 숙지하세요.

#### 1. Rvalue 참조 및 완벽한 전달(Perfect Forwarding) 개념 무방
Rust에는 `&&` 구문(Rvalue 참조)이 없습니다.
- **C++**: `T&&`를 사용해 이동이나 템플릿의 유니버설 참조를 구현합니다.
- **Rust**: **이동(Move)이 기본 동작**이므로 `std::move` 같은 키워드 없이 바로 데이터를 넘기면 됩니다. 복잡한 전달 매커니즘 대신 제네릭과 트레이트 경계를 활용합니다.

#### 2. 이동은 항상 비트 복사(Memcpy)
C++ 소멸자나 이동 생성자는 개발자가 직접 로직을 짤 수 있지만, Rust의 이동은 언제나 **단순 바이트 복체**입니다. 이동된 원본 변수는 즉시 무효화되어 '좀비 객체'가 발생할 여지를 차단합니다. (따라서 'Rule of Five' 고민이 필요 없습니다.)

#### 3. 자동 역참조 (Auto-Deref)
Rust는 `Deref` 트레이트를 통해 스마트 포인터나 기술적인 래핑을 자동으로 꿰뚫어 봅니다.
- **예**: `Arc<Mutex<Vec<T>>>`를 가지고 있을 때, C++라면 각 계층마다 `.lock()`이나 역참조 연산자를 복잡하게 써야 하지만, Rust는 메서드 호출 시 필요한 계층까지 자동으로 역참조하여 `Vec`의 메서드를 바로 쓸 수 있게 해줍니다.

#### 4. 참조자의 재할당 (Reseat) 가능성
- **C++**: 참조자는 한 번 바인딩되면 다른 객체를 가리킬 수 없습니다. (`ref = b`는 별칭 대상의 값을 바꾸는 것임)
- **Rust**: 참조자 자체를 `mut`로 선언하면(`let mut r = &a;`), 나중에 `r = &b;`와 같이 다른 대상을 가리키도록 **재할당**할 수 있습니다. 즉, Rust 참조자는 일종의 '안전성이 보장된 포인터'와 더 유사하게 작동합니다.
