### Rust 배열(Array) 타입

> **학습 내용:** Rust의 핵심 데이터 구조 — 배열, 튜플, 슬라이스, 문자열, 구조체, `Vec`, `HashMap`을 배웁니다. 내용이 방대한 장이므로 `String` vs `&str`의 차이와 구조체가 어떻게 작동하는지에 집중하세요. 참조(References)와 빌림(Borrowing)은 7장에서 심도 있게 다시 다룰 것입니다.

- 배열은 동일한 타입의 고정된 수의 요소를 포함합니다.
    - 다른 모든 Rust 타입과 마찬가지로 배열은 기본적으로 불변입니다 (mut를 사용하지 않는 한).
    - 배열은 []를 사용하여 인덱싱되며 경계 체크가 수행됩니다. len() 메서드를 사용하여 배열의 길이를 얻을 수 있습니다.
```rust
    fn get_index(y : usize) -> usize {
        y+1        
    }
    
    fn main() {
        // 3개의 요소를 가진 배열을 초기화하고 모두 42로 설정합니다.
        let a : [u8; 3] = [42; 3];
        // 대체 구문
        // let a = [42u8, 42u8, 42u8];
        for x in a {
            println!("{x}");
        }
        let y = get_index(a.len());
        // 아래 줄의 주석을 해제하면 패닉(panic)이 발생합니다.
        //println!("{}", a[y]);
    }
```

----
### Rust 배열 타입 계속
- 배열은 중첩될 수 있습니다.
    - Rust에는 출력을 위한 여러 내장 포맷터가 있습니다. 아래에서 ```:?```는 ```debug``` 출력 포맷터입니다. ```:#?``` 포맷터는 ```pretty print```를 위해 사용될 수 있습니다. 이러한 포맷터는 타입별로 커스터마이징할 수 있습니다 (이에 대해서는 나중에 자세히 다룹니다).
```rust
    fn main() {
        let a = [
            [40, 0], // 중첩된 배열 정의
            [41, 0],
            [42, 1],
        ];
        for x in a {
            println!("{x:?}");
        }
    }
```
----
### Rust 튜플(Tuples)
- 튜플은 고정된 크기를 가지며 임의의 타입들을 하나의 복합 타입으로 그룹화할 수 있습니다.
    - 구성 타입들은 상대적 위치(.0, .1, .2, ...)로 인덱싱될 수 있습니다. 빈 튜플, 즉 ()은 유닛(unit) 값이라고 불리며 void 반환값에 해당합니다.
    - Rust는 변수를 개별 요소에 쉽게 바인딩할 수 있도록 튜플 구조 분해(destructuring)를 지원합니다.
```rust
fn get_tuple() -> (u32, bool) {
    (42, true)        
}

fn main() {
   let t : (u8, bool) = (42, true);
   let u : (u32, bool) = (43, false);
   println!("{}, {}", t.0, t.1);
   println!("{}, {}", u.0, u.1);
   let (num, flag) = get_tuple(); // 튜플 구조 분해
   println!("{num}, {flag}");
}
```

### Rust 참조자(References)
- Rust의 참조자는 몇 가지 주요 차이점을 제외하면 C의 포인터와 대략적으로 비슷합니다.
    - 어느 시점이든 한 변수에 대해 여러 개의 읽기 전용(불변) 참조자를 갖는 것은 합법적입니다. 참조자는 변수의 범위를 벗어나 생존할 수 없습니다 (이것은 **수명(Lifetime)**이라고 불리는 핵심 개념이며 나중에 자세히 다룹니다).
    - 가변 변수에 대해서는 오직 하나의 쓰기 가능(가변) 참조자만 허용되며, 다른 어떤 참조자와도 겹쳐서는 안 됩니다.
```rust
fn main() {
    let mut a = 42;
    {
        let b = &a;
        let c = b;
        println!("{} {}", *b, *c); // 컴파일러가 자동으로 *c를 역참조합니다.
        // b가 여전히 범위(scope) 내에 있으므로 아래 코드는 불법입니다.
        // let d = &mut a;
    }
    let d = &mut a; // OK: b와 c가 범위 내에 없습니다.
    *d = 43;
}
```

----
# Rust 슬라이스(Slices)
- Rust 참조자를 사용하여 배열의 부분 집합을 생성할 수 있습니다.
    - 컴파일 타임에 결정되는 고정된 길이를 갖는 배열과 달리, 슬라이스는 임의의 크기를 가질 수 있습니다. 내부적으로 슬라이스는 슬라이스의 길이와 원본 배열의 시작 요소에 대한 포인터를 포함하는 "뚱뚱한 포인터(fat-pointer)"로 구현됩니다.
```rust
fn main() {
    let a = [40, 41, 42, 43];
    let b = &a[1..a.len()]; // 원본의 두 번째 요소부터 시작하는 슬라이스
    let c = &a[1..]; // 위와 동일
    let d = &a[..]; // &a[0..] 또는 &a[0..a.len()]과 동일
    println!("{b:?} {c:?} {d:?}");
}
```
----
# Rust 상수(Constants) 및 정적 변수(Statics)
- ```const``` 키워드는 상수 값을 정의하는 데 사용됩니다. 상수 값은 **컴파일 타임**에 평가되며 프로그램에 인라인(inline)됩니다.
- ```static``` 키워드는 C/C++와 같은 언어의 전역 변수에 해당하는 것을 정의하는 데 사용됩니다. 정적 변수는 주소 지정이 가능한 메모리 위치를 가지며 한 번 생성되어 프로그램의 전체 수명 동안 유지됩니다.
```rust
const SECRET_OF_LIFE: u32 = 42;
static GLOBAL_VARIABLE : u32 = 2;
fn main() {
    println!("생명의 비밀은 {}입니다.", SECRET_OF_LIFE);
    println!("전역 변수의 값은 {GLOBAL_VARIABLE}입니다.")
}
```

----
# Rust 문자열: String vs &str

- Rust에는 서로 다른 목적을 가진 **두 가지** 문자열 타입이 있습니다.
    - `String` — 소유권이 있고, 힙에 할당되며, 크기 조절이 가능합니다 (C의 `malloc`된 버퍼나 C++의 `std::string`과 유사).
    - `&str` — 빌려온 것이며, 가벼운 참조입니다 (C의 길이를 포함한 `const char*`나 C++의 `std::string_view`와 유사. 하지만 `&str`은 **수명 체크**가 이루어지므로 절대 댕글링되지 않습니다).
    - C의 null로 끝나는 문자열과 달리, Rust 문자열은 길이를 추적하며 유효한 UTF-8임이 보장됩니다.

> **C++ 개발자라면:** `String` ≈ `std::string`, `&str` ≈ `std::string_view`. `std::string_view`와 달리 `&str`은 빌림 검사기에 의해 전체 수명 동안 유효함이 보장됩니다.

## String vs &str: 소유 vs 빌림

> **실무 패턴**: 문자열 처리가 실무 코드에서 serde와 어떻게 작동하는지는 [JSON 처리: nlohmann::json → serde](ch17-2-avoiding-unchecked-indexing.md#json-handling-nlohmannjson--serde)를 참조하세요.

| **관점** | **C `char*`** | **C++ `std::string`** | **Rust `String`** | **Rust `&str`** |
|------------|--------------|----------------------|-------------------|----------------|
| **메모리** | 수동 (`malloc`/`free`) | 힙 할당, 버퍼 소유 | 힙 할당, 자동 해제 | 빌려온 참조 (수명 체크됨) |
| **가변성** | 포인터를 통해 항상 가변 | 가변 | `mut`가 있으면 가변 | 항상 불변 |
| **크기 정보** | 없음 (`'\0'`에 의존) | 길이 및 용량 추적 | 길이 및 용량 추적 | 길이 추적 (뚱뚱한 포인터) |
| **인코딩** | 지정되지 않음 (보통 ASCII) | 지정되지 않음 (보통 ASCII) | 유효한 UTF-8 보장 | 유효한 UTF-8 보장 |
| **Null 종료자** | 필요함 | 필요함 (`c_str()`) | 사용 안 함 | 사용 안 함 |

```rust
fn main() {
    // &str - 문자열 슬라이스 (빌려온 것, 불변, 보통 문자열 리터럴)
    let greeting: &str = "Hello";  // 읽기 전용 메모리를 가리킴

    // String - 소유권이 있고, 힙 할당되며, 크기 조절 가능
    let mut owned = String::from(greeting);  // 데이터를 힙으로 복사
    owned.push_str(", World!");        // 문자열 확장
    owned.push('!');                   // 단일 문자 추가

    // String과 &str 사이의 변환
    let slice: &str = &owned;          // String -> &str (비용 없음, 단순히 빌림)
    let owned2: String = slice.to_string();  // &str -> String (할당 발생)
    let owned3: String = String::from(slice); // 위와 동일

    // 문자열 연결 (주의: +는 왼쪽 피연산자를 소비함)
    let hello = String::from("Hello");
    let world = String::from(", World!");
    let combined = hello + &world;  // hello는 이동됨(소비됨), world는 빌려옴
    // println!("{hello}");  // 컴파일 에러: hello가 이동되었음

    // 이동 문제를 피하려면 format!을 사용하세요
    let a = String::from("Hello");
    let b = String::from("World");
    let combined = format!("{a}, {b}!");  // a와 b 모두 소비되지 않음

    println!("{combined}");
}
```

## 문자열을 `[]`로 인덱싱할 수 없는 이유
```rust
fn main() {
    let s = String::from("hello");
    // let c = s[0];  // 컴파일되지 않습니다! Rust 문자열은 바이트 배열이 아니라 UTF-8입니다.

    // 안전한 대안:
    let first_char = s.chars().next();           // Option<char>: Some('h')
    let as_bytes = s.as_bytes();                 // &[u8]: 원시 UTF-8 바이트
    let substring = &s[0..1];                    // &str: "h" (바이트 범위, 반드시 유효한 UTF-8 경계여야 함)

    println!("첫 번째 문자: {:?}", first_char);
    println!("바이트: {:?}", &as_bytes[..5]);
}
```

## 연습 문제: 문자열 조작

🟢 **초급**
- 문자열에서 공백으로 구분된 단어의 개수를 세는 `fn count_words(text: &str) -> usize` 함수를 작성하세요.
- 가장 긴 단어를 반환하는 `fn longest_word(text: &str) -> &str` 함수를 작성하세요. (힌트: 수명에 대해 생각해야 합니다. 왜 반환 타입이 `String`이 아니라 `&str`이어야 할까요?)

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
fn count_words(text: &str) -> usize {
    text.split_whitespace().count()
}

fn longest_word(text: &str) -> &str {
    text.split_whitespace()
        .max_by_key(|word| word.len())
        .unwrap_or("")
}

fn main() {
    let text = "the quick brown fox jumps over the lazy dog";
    println!("단어 개수: {}", count_words(text));       // 9
    println!("가장 긴 단어: {}", longest_word(text));     // "jumps"
}
```

</details>

# Rust 구조체(Structs)
- ```struct``` 키워드는 사용자 정의 구조체 타입을 선언합니다.
    - ```struct``` 멤버는 이름을 가질 수도 있고, 익명(튜플 구조체)일 수도 있습니다.
- C++와 같은 언어와 달리 Rust에는 "데이터 상속"이라는 개념이 없습니다.
```rust
fn main() {
    struct MyStruct {
        num: u32,
        is_secret_of_life: bool,
    }
    let x = MyStruct {
        num: 42,
        is_secret_of_life: true,
    };
    let y = MyStruct {
        num: x.num,
        is_secret_of_life: x.is_secret_of_life,
    };
    let z = MyStruct { num: x.num, ..x }; // ..은 나머지 필드를 복사함을 의미합니다.
    println!("{} {} {}", x.num, y.is_secret_of_life, z.num);
}
```

# Rust 튜플 구조체
- Rust 튜플 구조체는 튜플과 유사하며 개별 필드에 이름이 없습니다.
    - 튜플과 마찬가지로 개별 요소는 .0, .1, .2, ...를 사용하여 접근합니다. 튜플 구조체의 일반적인 용도는 기본 타입을 감싸서 커스텀 타입을 만드는 것입니다. **이는 동일한 타입의 서로 다른 값들을 섞어 쓰는 것을 방지하는 데 유용할 수 있습니다.**
```rust
struct WeightInGrams(u32);
struct WeightInMilligrams(u32);
fn to_weight_in_grams(kilograms: u32) -> WeightInGrams {
    WeightInGrams(kilograms * 1000)
}

fn to_weight_in_milligrams(w : WeightInGrams) -> WeightInMilligrams  {
    WeightInMilligrams(w.0 * 1000)
}

fn main() {
    let x = to_weight_in_grams(42);
    let y = to_weight_in_milligrams(x);
    // let z : WeightInGrams = x;  // 컴파일 에러: x가 to_weight_in_milligrams()로 이동되었음
    // let a : WeightInGrams = y;   // 컴파일 에러: 타입 불일치 (WeightInMilligrams vs WeightInGrams)
}
```


**참고**: `#[derive(...)]` 속성은 구조체와 열거형에 대해 일반적인 트레이트 구현을 자동으로 생성합니다. 이 과정 전반에서 이를 보게 될 것입니다.
```rust
#[derive(Debug, Clone, PartialEq)]
struct Point { x: i32, y: i32 }

fn main() {
    let p = Point { x: 1, y: 2 };
    println!("{:?}", p);           // Debug: #[derive(Debug)] 덕분에 작동함
    let p2 = p.clone();           // Clone: #[derive(Clone)] 덕분에 작동함
    assert_eq!(p, p2);            // PartialEq: #[derive(PartialEq)] 덕분에 작동함
}
```
나중에 트레이트 시스템을 심도 있게 다루겠지만, `#[derive(Debug)]`는 매우 유용하므로 여러분이 만드는 거의 모든 `struct`와 `enum`에 추가하는 것이 좋습니다.

# Rust Vec 타입
- ```Vec<T>``` 타입은 동적 힙 할당 버퍼를 구현합니다 (C의 수동 관리형 `malloc`/`realloc` 배열이나 C++의 `std::vector`와 유사).
    - 고정된 크기의 배열과 달리 `Vec`은 런타임에 늘어나거나 줄어들 수 있습니다.
    - `Vec`은 데이터를 소유하며 메모리 할당/해제를 자동으로 관리합니다.
- 공통 연산: `push()`, `pop()`, `insert()`, `remove()`, `len()`, `capacity()`
```rust
fn main() {
    let mut v = Vec::new();    // 빈 벡터, 사용법에 따라 타입 추론됨
    v.push(42);                // 끝에 요소 추가 - Vec<i32>
    v.push(43);                
    
    // 안전한 순회 (권장)
    for x in &v {              // 요소를 빌려오며, 벡터를 소비하지 않음
        println!("{x}");
    }
    
    // 초기화 단축어
    let mut v2 = vec![1, 2, 3, 4, 5];           // 초기화를 위한 매크로
    let v3 = vec![0; 10];                       // 10개의 0
    
    // 안전한 접근 메서드 (인덱싱보다 권장됨)
    match v2.get(0) {
        Some(first) => println!("첫 번째 요소: {first}"),
        None => println!("빈 벡터입니다."),
    }
    
    // 유용한 메서드들
    println!("길이: {}, 용량: {}", v2.len(), v2.capacity());
    if let Some(last) = v2.pop() {             // 마지막 요소를 제거하고 반환
        println!("꺼낸 요소: {last}");
    }
    
    // 위험함: 직접 인덱싱 (패닉이 발생할 수 있음!)
    // println!("{}", v2[100]);  // 런타임에 패닉 발생
}
```
> **실무 패턴**: 실무 Rust 코드의 안전한 `.get()` 패턴은 [검사되지 않은 인덱싱 방지](ch17-2-avoiding-unchecked-indexing.md#avoiding-unchecked-indexing)를 참조하세요.

# Rust HashMap 타입
- ```HashMap```은 제네릭 ```key``` -> ```value``` 조회를 구현합니다 (```사전(dictionary)``` 또는 ```맵(map)```이라고도 함).
```rust
fn main() {
    use std::collections::HashMap;  // Vec과 달리 명시적 임포트 필요
    let mut map = HashMap::new();       // 빈 HashMap 할당
    map.insert(40, false);  // 타입이 int -> bool로 추론됨
    map.insert(41, false);
    map.insert(42, true);
    for (key, value) in map {
        println!("{key} {value}");
    }
    let map = HashMap::from([(40, false), (41, false), (42, true)]);
    if let Some(x) = map.get(&43) {
        println!("43은 {x:?}에 매핑되었습니다.");
    } else {
        println!("43에 대한 매핑을 찾을 수 없습니다.");
    }
    let x = map.get(&43).or(Some(&false));  // 키를 찾을 수 없을 때의 기본값
    println!("{x:?}"); 
}
```

# 연습 문제: Vec 및 HashMap

🟢 **초급**
- 몇 개의 항목이 있는 ```HashMap<u32, bool>```을 생성하세요 (일부 값은 ```true```이고 일부는 ```false```여야 함). 해시맵의 모든 요소를 순회하면서 키는 하나의 ```Vec```에, 값은 다른 ```Vec```에 넣으세요.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
use std::collections::HashMap;

fn main() {
    let map = HashMap::from([(1, true), (2, false), (3, true), (4, false)]);
    let mut keys = Vec::new();
    let mut values = Vec::new();
    for (k, v) in &map {
        keys.push(*k);
        values.push(*v);
    }
    println!("키 목록:   {keys:?}");
    println!("값 목록: {values:?}");

    // 대안: unzip()과 함께 반복자 사용하기
    let (keys2, values2): (Vec<u32>, Vec<bool>) = map.into_iter().unzip();
    println!("키 목록 (unzip):   {keys2:?}");
    println!("값 목록 (unzip): {values2:?}");
}
```

</details>

---

## 심층 분석: C++ 참조 vs Rust 참조자

> **C++ 개발자라면:** C++ 프로그래머들은 종종 Rust의 `&T`가 C++의 `T&`처럼 작동한다고 가정합니다. 표면적으로는 비슷하지만 혼란을 야기하는 근본적인 차이점이 있습니다. C 개발자는 이 섹션을 건너뛰셔도 됩니다 — Rust 참조자는 [소유권 및 빌림](ch07-ownership-and-borrowing.md)에서 다룹니다.

#### 1. Rvalue 참조 또는 유니버설 참조 없음

C++에서 `&&`는 문맥에 따라 두 가지 의미를 갖습니다.

```cpp
// C++: &&는 상황에 따라 다른 것을 의미합니다:
int&& rref = 42;           // Rvalue 참조 — 임시 객체에 바인딩됨
void process(Widget&& w);   // Rvalue 참조 — 호출자가 std::move를 해야 함

// 유니버설 (전달) 참조 — 추론된 템플릿 문맥:
template<typename T>
void forward(T&& arg) {     // rvalue 참조가 아님! T& 또는 T&&로 추론됨
    inner(std::forward<T>(arg));  // 완벽한 전달(Perfect forwarding)
}
```

**Rust에는 이러한 것이 전혀 존재하지 않습니다.** `&&`는 단순히 논리 AND 연산자일 뿐입니다.

```rust
// Rust: &&는 단순히 불리언 AND입니다.
let a = true && false; // false

// Rust에는 rvalue 참조, 유니버설 참조, 완벽한 전달이 없습니다.
// 대신:
//   - Copy가 아닌 타입에 대해서는 이동(Move)이 기본입니다 (std::move가 필요 없음).
//   - 제네릭 + 트레이트 경계가 유니버설 참조를 대체합니다.
//   - 임시 객체 바인딩 구분이 없습니다 — 값은 값일 뿐입니다.

fn process(w: Widget) { }      // 소유권을 가짐 (C++의 값 매개변수 + 암시적 이동과 유사)
fn process_ref(w: &Widget) { } // 불변으로 빌림 (C++의 const T&와 유사)
fn process_mut(w: &mut Widget) { } // 가변으로 빌림 (C++의 T&와 유사하지만 독점적임)
```

| C++ 개념 | Rust 대응 개념 | 참고 |
|-------------|-----------------|-------|
| `T&` (lvalue 참조) | `&T` 또는 `&mut T` | Rust는 공유 vs 독점으로 나눕니다. |
| `T&&` (rvalue 참조) | 그냥 `T` | 값으로 받기 = 소유권 가지기 |
| 템플릿에서의 `T&&` (유니버설 참조) | `impl Trait` 또는 `<T: Trait>` | 제네릭이 전달을 대체합니다. |
| `std::move(x)` | `x` (그냥 사용) | 이동이 기본입니다. |
| `std::forward<T>(x)` | 필요 없음 | 전달할 유니버설 참조가 없습니다. |

#### 2. 이동은 비트 단위임 — 이동 생성자 없음

C++에서 이동은 *사용자 정의 연산* (이동 생성자 / 이동 대입)입니다. Rust에서 이동은 항상 값의 **비트 단위 memcpy**이며, 소스(원본)는 무효화됩니다.

```rust
// Rust 이동 = 바이트를 memcpy하고 원본을 무효화로 표시
let s1 = String::from("hello");
let s2 = s1; // s1의 바이트가 s2의 스택 슬롯으로 복사됨
              // s1은 이제 무효함 — 컴파일러가 이를 강제함
// println!("{s1}"); // ❌ 컴파일 에러: 이동 후 사용된 값
```

```cpp
// C++ 이동 = 이동 생성자 호출 (사용자 정의!)
std::string s1 = "hello";
std::string s2 = std::move(s1); // string의 이동 생성자 호출
// s1은 이제 "유효하지만 지정되지 않은 상태"인 좀비가 됨
std::cout << s1; // 컴파일됨! 무언가 출력됨 (보통 빈 문자열)
```

**결과**:
- Rust에는 Rule of Five가 없습니다 (복사 생성자, 이동 생성자, 복사=, 이동=, 소멸자를 정의할 필요 없음).
- 이동된 "좀비" 상태가 없습니다 — 컴파일러가 단순히 접근을 차단합니다.
- 이동에 대한 `noexcept` 고려 사항이 없습니다 — 비트 단위 복사는 예외를 던질 수 없습니다.

#### 3. 자동 역참조(Auto-Deref): 컴파일러가 간접 참조를 꿰뚫어 봄

Rust는 `Deref` 트레이트를 통해 여러 층의 포인터/래퍼를 자동으로 역참조합니다. 이는 C++에 대응하는 개념이 없습니다.

```rust
use std::sync::{Arc, Mutex};

// 중첩된 래핑: Arc<Mutex<Vec<String>>>
let data = Arc::new(Mutex::new(vec!["hello".to_string()]));

// C++라면 각 층마다 명시적인 잠금 해제와 수동 역참조가 필요했을 것입니다.
// Rust에서 컴파일러는 Arc → Mutex → MutexGuard → Vec 순으로 자동 역참조합니다.
let guard = data.lock().unwrap(); // Arc가 Mutex로 자동 역참조됨
let first: &str = &guard[0];      // MutexGuard→Vec (Deref), Vec[0] (Index),
                                   // &String→&str (Deref 강제 변환)
println!("첫 번째 요소: {first}");

// 메서드 호출 시에도 자동 역참조가 일어납니다.
let boxed_string = Box::new(String::from("hello"));
println!("길이: {}", boxed_string.len());  // Box→String으로 변환 후 String::len() 호출
// (*boxed_string).len()이나 boxed_string->len()을 쓸 필요가 없습니다.
```

**Deref 강제 변환(Deref coercion)**은 함수 인자에도 적용됩니다 — 컴파일러가 타입이 일치하도록 역참조를 삽입합니다.

```rust
fn greet(name: &str) {
    println!("안녕하세요, {name}님");
}

fn main() {
    let owned = String::from("Alice");
    let boxed = Box::new(String::from("Bob"));
    let arced = std::sync::Arc::new(String::from("Carol"));

    greet(&owned);  // &String → &str (1회 역참조 강제 변환)
    greet(&boxed);  // &Box<String> → &String → &str (2회 역참조 강제 변환)
    greet(&arced);  // &Arc<String> → &String → &str (2회 역참조 강제 변환)
    greet("Dave");  // 이미 &str이므로 강제 변환 필요 없음
}
// C++라면 각 경우에 대해 .c_str()이나 명시적 변환이 필요했을 것입니다.
```

**Deref 체인**: `x.method()`를 호출할 때, Rust의 메서드 확인은 수신자 타입 `T`, 그다음 `&T`, 그다음 `&mut T`를 시도합니다. 일치하는 것이 없으면 `Deref` 트레이트를 통해 역참조하고 대상 타입에 대해 반복합니다. 이것이 여러 층을 거쳐 계속되므로 `Box<Vec<T>>`가 `Vec<T>`처럼 "그냥 작동"하는 것입니다. Deref *강제 변환*(함수 인자용)은 별개이지만 관련된 메커니즘으로, `Deref` 구현을 연결하여 `&Box<String>`을 `&str`로 자동 변환합니다.

#### 4. Null 참조 없음, Optional 참조 없음

```cpp
// C++: 참조는 null일 수 없지만, 포인터는 가능하며 그 경계가 모호합니다.
Widget& ref = *ptr;  // ptr이 null이라면 → UB
Widget* opt = nullptr;  // 포인터를 통한 "선택적" 참조
```

```rust
// Rust: 참조자는 항상 유효합니다 — 빌림 검사기에 의해 보장됩니다.
// 안전한 코드에서 null이나 댕글링 참조를 만들 방법은 없습니다.
let r: &i32 = &42; // 항상 유효함

// "선택적 참조"는 명시적입니다:
let opt: Option<&Widget> = None; // 의도가 명확하며 null 포인터가 아님
if let Some(w) = opt {
    w.do_something(); // 존재할 때만 도달 가능함
}
```

#### 5. 참조자는 재할당(Reseat)될 수 없음

```cpp
// C++: 참조자는 별칭(alias)입니다 — 다시 바인딩될 수 없습니다.
int a = 1, b = 2;
int& r = a;
r = b;  // b의 값을 a에 대입하는 것이지, r을 다시 바인딩하는 것이 아닙니다!
// a는 이제 2이고, r은 여전히 a를 가리킵니다.
```

```rust
// Rust: let 바인딩은 섀도잉될 수 있지만, 참조자는 다른 규칙을 따릅니다.
let a = 1;
let b = 2;
let r = &a;
// r = &b;   // ❌ 불변 변수에 할당할 수 없습니다.
let r = &b;  // ✅ 하지만 r을 새로운 바인딩으로 섀도잉할 수 있습니다.
             // 이전 바인딩은 사라지며, 재할당(reseat)되는 것이 아닙니다.

// mut를 사용하는 경우:
let mut r = &a;
r = &b;      // ✅ r이 이제 b를 가리킵니다 — 이것은 다시 바인딩하는 것입니다 (대입이 아님).
```

> **정신적 모델**: C++에서 참조자는 한 객체에 대한 영구적인 별칭입니다. Rust에서 참조자는 수명 보장이 있는 값(포인터)이며, 일반적인 변수 바인딩 규칙을 따릅니다 — 기본적으로 불변이며, `mut`로 선언된 경우에만 다시 바인딩이 가능합니다.
