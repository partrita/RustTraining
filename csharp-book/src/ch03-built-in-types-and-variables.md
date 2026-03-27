## 변수와 가변성

> **학습 목표:** Rust의 변수 선언 및 가변성(Mutability) 모델을 C#의 `var`/`const`와 비교하여 이해합니다. 기본 타입 매핑, 매우 중요한 `String`과 `&str`의 차이점, 타입 추론, 그리고 Rust가 C#과 다르게 타입 캐스팅 및 변환을 처리하는 방식을 배웁니다.
>
> **난이도:** 🟢 초급

### C# 변수 선언
```csharp
// C# - 변수는 기본적으로 가변적입니다.
int count = 0;           // 가변 변수
count = 5;               // ✅ 작동함

// readonly 필드 (클래스 레벨에서만 가능, 로컬 변수에는 불가)
// readonly int maxSize = 100;  // 초기화 후 불변

const int BUFFER_SIZE = 1024; // 컴파일 타임 상수 (로컬 또는 필드로 작동)
```

### Rust 변수 선언
```rust
// Rust - 변수는 기본적으로 불변입니다.
let count = 0;           // 기본적으로 불변
// count = 5;            // ❌ 컴파일 에러: 불변 변수에 두 번 할당할 수 없음

let mut count = 0;       // 명시적인 가변 변수
count = 5;               // ✅ 작동함

const BUFFER_SIZE: usize = 1024; // 컴파일 타임 상수
```

### C# 개발자를 위한 핵심 사고 변화
```rust
// 'let'을 모든 변수에 적용된 C#의 readonly 필드 의미론으로 생각하세요.
let name = "홍길동";       // readonly 필드와 유사: 한 번 설정되면 변경 불가
let mut age = 30;        // int age = 30; 과 유사

// 변수 섀도잉 (Shadowing, Rust만의 특징)
let spaces = "   ";      // 문자열
let spaces = spaces.len(); // 이제 숫자(usize)입니다.
// 이는 값의 변경(Mutation)과는 다릅니다 - 아예 새로운 변수를 만드는 것입니다.
```

### 실전 예제: 카운터
```csharp
// C# 버전
public class Counter
{
    private int value = 0;
    
    public void Increment()
    {
        value++;  // 값 변경
    }
    
    public int GetValue() => value;
}
```

```rust
// Rust 버전
pub struct Counter {
    value: i32,  // 기본적으로 비공개(Private)
}

impl Counter {
    pub fn new() -> Counter {
        Counter { value: 0 }
    }
    
    pub fn increment(&mut self) {  // 값 변경을 위해 &mut가 필요함
        self.value += 1;
    }
    
    pub fn get_value(&self) -> i32 {
        self.value
    }
}
```

***

## 데이터 타입 비교

### 기본 타입

| C# 타입 | Rust 타입 | 크기 | 범위 |
|---------|-----------|------|-------|
| `byte` | `u8` | 8 bits | 0 ~ 255 |
| `sbyte` | `i8` | 8 bits | -128 ~ 127 |
| `short` | `i16` | 16 bits | -32,768 ~ 32,767 |
| `ushort` | `u16` | 16 bits | 0 ~ 65,535 |
| `int` | `i32` | 32 bits | -2³¹ ~ 2³¹-1 |
| `uint` | `u32` | 32 bits | 0 ~ 2³²-1 |
| `long` | `i64` | 64 bits | -2⁶³ ~ 2⁶³-1 |
| `ulong` | `u64` | 64 bits | 0 ~ 2⁶⁴-1 |
| `float` | `f32` | 32 bits | IEEE 754 |
| `double` | `f64` | 64 bits | IEEE 754 |
| `bool` | `bool` | 1 bit | true/false |
| `char` | `char` | 32 bits | 유니코드 스칼라 값 |

### 크기 타입 (Size Types - 중요!)
```csharp
// C# - int는 항상 32비트입니다.
int arrayIndex = 0;
long fileSize = file.Length;
```

```rust
// Rust - 크기 타입은 포인터 크기에 맞춰집니다 (32비트 또는 64비트).
let array_index: usize = 0;    // C 언어의 size_t와 유사
let file_size: u64 = file.len(); // 명시적인 64비트
```

### 타입 추론
```csharp
// C# - var 키워드
var name = "홍길동";        // string
var count = 42;           // int
var price = 29.99;        // double
```

```rust
// Rust - 자동 타입 추론
let name = "홍길동";        // &str (문자열 슬라이스)
let count = 42;           // i32 (기본 정수 타입)
let price = 29.99;        // f64 (기본 부동 소수점 타입)

// 명시적인 타입 어노테이션
let count: u32 = 42;
let price: f32 = 29.99;
```

### 배열 및 컬렉션 개요
```csharp
// C# - 참조 타입, 힙에 할당됨
int[] numbers = new int[5];        // 고정 크기
List<int> list = new List<int>();  // 동적 크기
```

```rust
// Rust - 다양한 옵션 존재
let numbers: [i32; 5] = [1, 2, 3, 4, 5];  // 스택 배열, 고정 크기
let mut list: Vec<i32> = Vec::new();       // 힙 벡터, 동적 크기
```

***

## 문자열 타입: String vs &str

이 부분은 C# 개발자들에게 가장 혼란스러운 개념 중 하나이므로 주의 깊게 살펴보겠습니다.

### C# 문자열 처리
```csharp
// C# - 단순한 문자열 모델
string name = "홍길동";           // 문자열 리터럴
string greeting = "안녕하세요, " + name;  // 문자열 연결
string upper = name.ToUpper();  // 메서드 호출
```

### Rust 문자열 타입
```rust
// Rust - 두 가지 주요 문자열 타입

// 1. &str (문자열 슬라이스) - C#의 ReadOnlySpan<char>와 유사
let name: &str = "홍길동";        // 문자열 리터럴 (불변, 빌림)

// 2. String - StringBuilder나 가변 문자열과 유사
let mut greeting = String::new();       // 빈 문자열
greeting.push_str("안녕하세요, ");      // 뒤에 추가
greeting.push_str(name);               // 뒤에 추가

// 또는 직접 생성
let greeting = String::from("안녕하세요, 홍길동");
let greeting = "안녕하세요, 홍길동".to_string();  // &str을 String으로 변환
```

### 언제 어떤 것을 사용해야 할까요?

| 시나리오 | 사용 타입 | C# 대응 개념 |
|----------|-----|---------------|
| 문자열 리터럴 | `&str` | `string` 리터럴 |
| 함수 매개변수 (읽기 전용) | `&str` | `string` 또는 `ReadOnlySpan<char>` |
| 소유권을 가지고 수정 가능한 문자열 | `String` | `StringBuilder` |
| 소유권을 가진 문자열을 반환할 때 | `String` | `string` |

### 실전 예제
```rust
// 어떤 문자열 타입이든 받아들이는 함수
fn greet(name: &str) {  // String과 &str 모두 수용 가능
    println!("안녕하세요, {}님!", name);
}

fn main() {
    let literal = "홍길동";                    // &str
    let owned = String::from("성춘향");        // String
    
    greet(literal);                          // 작동함
    greet(&owned);                           // 작동함 (String을 &str로 빌림)
    greet("이몽룡");                         // 작동함
}

// 소유권을 가진 문자열을 반환하는 함수
fn create_greeting(name: &str) -> String {
    format!("안녕하세요, {}님!", name)  // format! 매크로는 String을 반환함
}
```

### C# 개발자를 위한 팁
```rust
// &str은 ReadOnlySpan<char>와 같이 문자열 데이터를 바라보는 '뷰(View)'라고 생각하세요.
// String은 여러분이 직접 소유하고 수정할 수 있는 char[]와 비슷합니다.

let borrowed: &str = "저는 이 데이터를 소유하지 않습니다";
let owned: String = String::from("저는 이 데이터를 소유합니다");

// 서로 변환하기
let owned_copy: String = borrowed.to_string();  // 소유권이 있는 String으로 복사
let borrowed_view: &str = &owned;               // String에서 데이터를 빌려오기
```

***

## 출력 및 문자열 포맷팅

C# 개발자들은 `Console.WriteLine`과 문자열 보간(`$""`)을 자주 사용합니다. Rust의 포맷팅 시스템도 그만큼 강력하지만, 매크로와 포맷 지정자(Format specifier)를 사용합니다.

### 기본 출력
```csharp
// C# 출력
Console.Write("줄바꿈 없음");
Console.WriteLine("줄바꿈 포함");
Console.Error.WriteLine("표준 에러로 출력");

// 문자열 보간 (C# 6+)
string name = "앨리스";
int age = 30;
Console.WriteLine($"{name}는 {age}살입니다");
```

```rust
// Rust 출력 — 모두 매크로입니다 (!가 붙음)
print!("줄바꿈 없음");              // → 표준 출력(stdout), 줄바꿈 없음
println!("줄바꿈 포함");           // → 표준 출력 + 줄바꿈
eprint!("표준 에러로 출력");        // → 표준 에러(stderr), 줄바꿈 없음  
eprintln!("줄바꿈 포함 에러 출력"); // → 표준 에러 + 줄바꿈

// 문자열 포맷팅 ($"" 보간과 유사)
let name = "앨리스";
let age = 30;
println!("{name}는 {age}살입니다");     // 인라인 변수 캡처 (Rust 1.58+)
println!("{}는 {}살입니다", name, age); // 위치 인자 방식

// format!은 출력하는 대신 String을 반환합니다.
let msg = format!("{name}는 {age}살입니다");
```

### 포맷 지정자
```csharp
// C# 포맷 지정자
Console.WriteLine($"{price:F2}");         // 소수점 고정:  29.99
Console.WriteLine($"{count:D5}");         // 숫자 패딩: 00042
Console.WriteLine($"{value,10}");         // 우측 정렬, 너비 10
Console.WriteLine($"{value,-10}");        // 좌측 정렬, 너비 10
Console.WriteLine($"{hex:X}");            // 16진수:    FF
Console.WriteLine($"{ratio:P1}");         // 퍼센트:     85.0%
```

```rust
// Rust 포맷 지정자
println!("{price:.2}");          // 소수점 2자리:  29.99
println!("{count:05}");          // 0으로 채움, 너비 5: 00042
println!("{value:>10}");         // 우측 정렬, 너비 10
println!("{value:<10}");         // 좌측 정렬, 너비 10
println!("{value:^10}");         // 중앙 정렬, 너비 10
println!("{hex:#X}");            // 접두사 포함 16진수: 0xFF
println!("{hex:08X}");           // 0으로 채운 16진수: 000000FF
println!("{bits:#010b}");        // 접두사 포함 2진수: 0b00001010
println!("{big}", big = 1_000_000); // 이름 지정 인자
```

### Debug vs Display 출력
```rust
// {:?}  — Debug 트레이트 (개발자용, 자동 파생 가능)
// {:#?} — 예쁘게 출력되는 Debug (들여쓰기, 다중 행)
// {}    — Display 트레이트 (사용자용, 수동 구현 필요)

#[derive(Debug)] // Debug 출력을 자동으로 생성함
struct Point { x: f64, y: f64 }

let p = Point { x: 1.5, y: 2.7 };

println!("{:?}", p);   // Point { x: 1.5, y: 2.7 }   — 조밀한 디버그 출력
println!("{:#?}", p);  // Point {                     — 예쁜 디버그 출력
                        //     x: 1.5,
                        //     y: 2.7,
                        // }
// println!("{}", p);  // ❌ 에러: Point가 Display를 구현하지 않음

// 사용자용 출력을 위해 Display 구현하기:
use std::fmt;

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
println!("{}", p);    // (1.5, 2.7)  — 친숙한 출력
```

```csharp
// C# 대응 개념:
// {:?}  ≈ object.GetType().ToString() 또는 리플렉션을 통한 덤프
// {}    ≈ object.ToString()
// C#에서는 ToString()을 오버라이드하지만, Rust에서는 Display를 구현합니다.
```

### 빠른 참조 표

| C# | Rust | 출력 결과 |
|----|------|--------|
| `Console.WriteLine(x)` | `println!("{x}")` | Display 포맷팅 |
| `$"{x}"` (문자열 보간) | `format!("{x}")` | `String` 반환 |
| `x.ToString()` | `x.to_string()` | `Display` 트레이트 필요 |
| `ToString()` 오버라이드 | `impl Display` | 사용자용 출력 정의 |
| 디버거 뷰 | `{:?}` 또는 `dbg!(x)` | 개발자용 디버그 출력 |
| `String.Format("{0:F2}", x)` | `format!("{x:.2}")` | 포맷팅된 `String` 반환 |
| `Console.Error.WriteLine` | `eprintln!()` | 표준 에러로 출력 |

***

## 타입 캐스팅 및 변환

C#에는 암시적 변환, 명시적 캐스트 `(int)x`, 그리고 `Convert.To*()`가 있습니다. Rust는 더 엄격하여 숫자 간의 암시적 변환이 존재하지 않습니다.

### 숫자 변환
```csharp
// C# — 암시적 및 명시적 변환
int small = 42;
long big = small;              // 암시적 확대 변환: OK
double d = small;              // 암시적 확대 변환: OK
int truncated = (int)3.14;     // 명시적 축소 변환: 3
byte b = (byte)300;            // 소리 없는 오버플로: 44

// 안전한 변환
if (int.TryParse("42", out int parsed)) { /* ... */ }
```

```rust
// Rust — 모든 숫자 변환은 명시적이어야 합니다.
let small: i32 = 42;
let big: i64 = small as i64;       // 확대 변환: 'as'를 사용한 명시적 변환
let d: f64 = small as f64;         // 정수에서 부동 소수점: 명시적
let truncated: i32 = 3.14_f64 as i32; // 축소 변환: 3 (절삭됨)
let b: u8 = 300_u16 as u8;        // 오버플로: 44로 순환(Wrap)됨 (C#의 unchecked와 유사)

// TryFrom을 사용한 안전한 변환
use std::convert::TryFrom;
let safe: Result<u8, _> = u8::try_from(300_u16); // Err — 범위를 벗어남
let ok: Result<u8, _>   = u8::try_from(42_u16);  // Ok(42)

// 문자열 파싱 — bool + out 매개변수가 아닌 Result를 반환함
let parsed: Result<i32, _> = "42".parse::<i32>();   // Ok(42)
let bad: Result<i32, _>    = "abc".parse::<i32>();  // Err(ParseIntError)

// 터보피쉬(Turbofish) 구문 사용 예:
let n = "42".parse::<f64>().unwrap(); // 42.0
```

### 문자열 변환
```csharp
// C#
int n = 42;
string s = n.ToString();          // "42"
string formatted = $"{n:X}";
int back = int.Parse(s);          // 42 또는 예외 발생
bool ok = int.TryParse(s, out int result);
```

```rust
// Rust — Display를 통한 to_string(), FromStr를 통한 parse()
let n: i32 = 42;
let s: String = n.to_string();            // "42" (Display 트레이트 사용)
let formatted = format!("{n:X}");         // "2A"
let back: i32 = s.parse().unwrap();       // 42 또는 패닉(Panic) 발생
let result: Result<i32, _> = s.parse();   // Ok(42) — 안전한 버전

// &str ↔ String 변환 (Rust에서 가장 흔한 변환)
let owned: String = "hello".to_string();    // &str → String
let owned2: String = String::from("hello"); // &str → String (동일함)
let borrowed: &str = &owned;               // String → &str (비용 없음, 빌림일 뿐임)
```

### 참조 변환 (상속 캐스팅 없음!)
```csharp
// C# — 업캐스팅과 다운캐스팅
Animal a = new Dog();              // 업캐스트 (암시적)
Dog d = (Dog)a;                    // 다운캐스트 (명시적, 예외 발생 가능)
if (a is Dog dog) { /* ... */ }    // 패턴 매칭을 통한 안전한 다운캐스트
```

```rust
// Rust — 상속이 없으므로 업캐스팅/다운캐스팅도 없음
// 다형성을 위해 트레이트 객체(Trait object)를 사용함:
let animal: Box<dyn Animal> = Box::new(Dog);

// "다운캐스팅"을 하려면 Any 트레이트가 필요함 (거의 필요하지 않음):
use std::any::Any;
if let Some(dog) = animal_any.downcast_ref::<Dog>() {
    // dog 사용
}
// 실무에서는 다운캐스팅 대신 열거형(Enum)을 사용하세요:
enum Animal {
    Dog(Dog),
    Cat(Cat),
}
match animal {
    Animal::Dog(d) => { /* d 사용 */ }
    Animal::Cat(c) => { /* c 사용 */ }
}
```

### 빠른 참조 표

| C# | Rust | 비고 |
|----|------|-------|
| `(int)x` | `x as i32` | 절삭/순환 캐스트 |
| 암시적 확대 변환 | 반드시 `as` 사용 | 숫자 간 암시적 변환 없음 |
| `Convert.ToInt32(x)` | `i32::try_from(x)` | 안전함, `Result` 반환 |
| `int.Parse(s)` | `s.parse::<i32>().unwrap()` | 실패 시 패닉 발생 |
| `int.TryParse(s, out n)` | `s.parse::<i32>()` | `Result<i32, _>` 반환 |
| `(Dog)animal` | 지원 안 함 | 열거형이나 `Any` 사용 |
| `as Dog` / `is Dog` | `downcast_ref::<Dog>()` | `Any` 트레이트 경유; 열거형 권장 |

***

## 주석과 문서화

### 일반 주석
```csharp
// C# 주석
// 단일 행 주석
/* 다중 행
   주석 */

/// <summary>
/// XML 문서 주석
/// </summary>
/// <param name="name">사용자 이름</param>
/// <returns>인사말 문자열</returns>
public string Greet(string name)
{
    return $"안녕하세요, {name}님!";
}
```

```rust
// Rust 주석
// 단일 행 주석
/* 다중 행
   주석 */

/// 문서 주석 (C#의 ///와 유사)
/// 사용자의 이름으로 인사말을 생성합니다.
/// 
/// # 인자
/// 
/// * `name` - 문자열 슬라이스 형태의 사용자 이름
/// 
/// # 반환값
/// 
/// 인사말이 담긴 `String`
/// 
/// # 예제
/// 
/// ```
/// let greeting = greet("앨리스");
/// assert_eq!(greeting, "안녕하세요, 앨리스님!");
/// ```
pub fn greet(name: &str) -> String {
    format!("안녕하세요, {}님!", name)
}
```

### 문서 생성
```bash
# 문서 생성 (C#의 XML 문서와 유사)
cargo doc --open

# 문서 테스트 실행
cargo test --doc
```

---

## 연습 문제

<details>
<summary><strong>🏋️ 실습: 타입 안전 온도 변환기</strong> (펼치기)</summary>

다음을 수행하는 Rust 프로그램을 작성해 보세요:
1. 섭씨 절대 영도(`-273.15`)를 위한 `const` 선언
2. 변환이 몇 번 수행되었는지 기록하는 `static` 카운터 선언 (`AtomicU32` 사용)
3. 절대 영도 미만의 온도는 `f64::NAN`을 반환하여 거부하는 `celsius_to_fahrenheit(c: f64) -> f64` 함수 작성
4. 문자열 `"98.6"`을 `f64`로 파싱한 후 변환하는 과정에서 섀도잉(Shadowing) 활용 사례 보여주기

<details>
<summary>🔑 해답</summary>

```rust
use std::sync::atomic::{AtomicU32, Ordering};

const ABSOLUTE_ZERO_C: f64 = -273.15;
static CONVERSION_COUNT: AtomicU32 = AtomicU32::new(0);

fn celsius_to_fahrenheit(c: f64) -> f64 {
    if c < ABSOLUTE_ZERO_C {
        return f64::NAN;
    }
    CONVERSION_COUNT.fetch_add(1, Ordering::Relaxed);
    c * 9.0 / 5.0 + 32.0
}

fn main() {
    let temp = "98.6";           // &str
    let temp: f64 = temp.parse().unwrap(); // f64로 섀도잉
    let temp = celsius_to_fahrenheit(temp); // 화씨 온도로 섀도잉
    println!("{temp:.1}°F");
    println!("변환 횟수: {}", CONVERSION_COUNT.load(Ordering::Relaxed));
}
```

</details>
</details>

***
