## C# 개발자를 위한 필수 Rust 키워드

> **학습 목표:** Rust 키워드와 그에 대응하는 C# 개념을 빠르게 찾아볼 수 있는 참조 가이드입니다. 가시성 한정자, 소유권 관련 키워드, 제어 흐름, 타입 정의 및 패턴 매칭 구문을 다룹니다.
>
> **난이도:** 🟢 초급

Rust의 키워드와 그 용도를 이해하면 C# 개발자가 이 언어를 더 효과적으로 익히는 데 도움이 됩니다.

### 가시성 및 접근 제어 키워드

#### C# 접근 한정자
```csharp
public class Example
{
    public int PublicField;           // 어디서든 접근 가능
    private int privateField;        // 클래스 내부에서만 접근 가능
    protected int protectedField;    // 클래스 및 자식 클래스에서 접근 가능
    internal int internalField;      // 현재 어셈블리 내에서만 접근 가능
    protected internal int protectedInternalField; // 조합형
}
```

#### Rust 가시성 키워드
```rust
// pub - 항목을 공개함 (C#의 public과 유사)
pub struct PublicStruct {
    pub public_field: i32,           // 공개 필드
    private_field: i32,              // 기본적으로 비공개 (키워드 없음)
}

pub mod my_module {
    pub(crate) fn crate_public() {}     // 현재 크레이트 내에서 공개 (internal과 유사)
    pub(super) fn parent_public() {}    // 부모 모듈에 공개
    pub(self) fn self_public() {}       // 현재 모듈 내에서 공개 (비공개와 동일)
    
    pub use super::PublicStruct;        // 다시 내보내기 (using alias와 유사)
}

// C#의 protected에 직접 대응하는 개념은 없음 - 대신 구성을 사용함
```

### 메모리 및 소유권 키워드

#### C# 메모리 관련 키워드
```csharp
// ref - 참조에 의한 전달
public void Method(ref int value) { value = 10; }

// out - 출력 매개변수
public bool TryParse(string input, out int result) { /* */ }

// in - 읽기 전용 참조 (C# 7.2+)
public void ReadOnly(in LargeStruct data) { /* data 수정 불가 */ }
```

#### Rust 소유권 키워드
```rust
// & - 불변 참조 (C#의 in 매개변수와 유사)
fn read_only(data: &Vec<i32>) {
    println!("길이: {}", data.len()); // 읽기 가능, 수정 불가
}

// &mut - 가변 참조 (C#의 ref 매개변수와 유사)
fn modify(data: &mut Vec<i32>) {
    data.push(42); // 수정 가능
}

// move - 클로저에서 캡처한 변수의 소유권을 강제로 이전함
let data = vec![1, 2, 3];
let closure = move || {
    println!("{:?}", data); // data가 클로저 내부로 이동됨
};
// 이 지점부터 data에 더 이상 접근할 수 없음

// Box - 힙 할당 (참조 타입에 대한 C#의 new와 유사)
let boxed_data = Box::new(42); // 데이터를 힙에 할당
```

### 제어 흐름 키워드

#### C# 제어 흐름
```csharp
// return - 값을 반환하며 함수 종료
public int GetValue() { return 42; }

// yield return - 반복자(Iterator) 패턴
public IEnumerable<int> GetNumbers()
{
    yield return 1;
    yield return 2;
}

// break/continue - 루프 제어
foreach (var item in items)
{
    if (item == null) continue;
    if (item.Stop) break;
}
```

#### Rust 제어 흐름 키워드
```rust
// return - 명시적 반환 (일반적으로는 필요하지 않음)
fn get_value() -> i32 {
    return 42; // 명시적 반환
    // 또는 단순히: 42 (암시적 반환)
}

// break/continue - 값을 반환할 수 있는 루프 제어
fn find_value() -> Option<i32> {
    loop {
        let value = get_next();
        if value < 0 { continue; }
        if value > 100 { break None; }      // None을 반환하며 루프 종료
        if value == 42 { break Some(value); } // Some(value)를 반환하며 루프 종료
    }
}

// loop - 무한 루프 (while(true)와 유사)
loop {
    if condition { break; }
}

// while - 조건부 루프
while condition {
    // 코드
}

// for - 반복자 루프
for item in collection {
    // 코드
}
```

### 타입 정의 키워드

#### C# 타입 키워드
```csharp
// class - 참조 타입
public class MyClass { }

// struct - 값 타입
public struct MyStruct { }

// interface - 계약 정의
public interface IMyInterface { }

// enum - 열거형
public enum MyEnum { Value1, Value2 }

// delegate - 함수 포인터
public delegate void MyDelegate(int value);
```

#### Rust 타입 키워드
```rust
// struct - 데이터 구조 (C#의 클래스와 구조체를 결합한 형태)
struct MyStruct {
    field: i32,
}

// enum - 대수적 데이터 타입 (C#의 열거형보다 훨씬 강력함)
enum MyEnum {
    Variant1,
    Variant2(i32),              // 데이터를 포함할 수 있음
    Variant3 { x: i32, y: i32 }, // 구조체 형태의 변형
}

// trait - 인터페이스 정의 (C# 인터페이스와 유사하지만 더 강력함)
trait MyTrait {
    fn method(&self);
    
    // 기본 구현 (C# 8+의 기본 인터페이스 메서드와 유사)
    fn default_method(&self) {
        println!("기본 구현");
    }
}

// type - 타입 별칭 (C#의 using alias와 유사)
type UserId = u32;
type Result<T> = std::result::Result<T, MyError>;

// impl - 구현 블록 (C#에는 직접 대응하는 개념이 없으며, 메서드가 별도로 정의됨)
impl MyStruct {
    fn new() -> MyStruct {
        MyStruct { field: 0 }
    }
}

impl MyTrait for MyStruct {
    fn method(&self) {
        println!("구현 완료");
    }
}
```

### 함수 정의 키워드

#### C# 함수 키워드
```csharp
// static - 클래스 메서드
public static void StaticMethod() { }

// virtual - 오버라이드 가능
public virtual void VirtualMethod() { }

// override - 기반 메서드 오버라이드
public override void VirtualMethod() { }

// abstract - 반드시 구현해야 함
public abstract void AbstractMethod();

// async - 비동기 메서드
public async Task<int> AsyncMethod() { return await SomeTask(); }
```

#### Rust 함수 키워드
```rust
// fn - 함수 정의 (C#의 메서드와 유사하지만 독립적으로 존재 가능)
fn regular_function() {
    println!("안녕하세요");
}

// const fn - 컴파일 타임 함수 (C#의 const와 유사하지만 함수에 적용)
const fn compile_time_function() -> i32 {
    42 // 컴파일 타임에 평가될 수 있음
}

// async fn - 비동기 함수 (C#의 async와 유사)
async fn async_function() -> i32 {
    some_async_operation().await
}

// unsafe fn - 메모리 안전성을 위반할 가능성이 있는 함수
unsafe fn unsafe_function() {
    // 안전하지 않은 작업 수행 가능
}

// extern fn - 외부 함수 인터페이스 (FFI)
extern "C" fn c_compatible_function() {
    // C 언어에서 호출 가능
}
```

### 변수 선언 키워드

#### C# 변수 키워드
```csharp
// var - 타입 추론
var name = "홍길동"; // string으로 추론됨

// const - 컴파일 타임 상수
const int MaxSize = 100;

// readonly - 런타임 상수 (필드에만 적용 가능, 로컬 변수에는 불가)
// readonly DateTime createdAt = DateTime.Now;

// static - 클래스 레벨 변수
static int instanceCount = 0;
```

#### Rust 변수 키워드
```rust
// let - 변수 바인딩 (C#의 var와 유사)
let name = "홍길동"; // 기본적으로 불변(Immutable)

// let mut - 가변 변수 바인딩
let mut count = 0; // 값 변경 가능
count += 1;

// const - 컴파일 타임 상수 (C#의 const와 유사)
const MAX_SIZE: usize = 100;

// static - 전역 변수 (C#의 static과 유사)
static INSTANCE_COUNT: std::sync::atomic::AtomicUsize = 
    std::sync::atomic::AtomicUsize::new(0);
```

### 패턴 매칭 키워드

#### C# 패턴 매칭 (C# 8+)
```csharp
// switch 표현식
string result = value switch
{
    1 => "하나",
    2 => "둘",
    _ => "기타"
};

// is 패턴
if (obj is string str)
{
    Console.WriteLine(str.Length);
}
```

#### Rust 패턴 매칭 키워드
```rust
// match - 패턴 매칭 (C#의 switch와 유사하지만 훨씬 강력함)
let result = match value {
    1 => "하나",
    2 => "둘",
    3..=10 => "3에서 10 사이", // 범위 패턴
    _ => "기타", // 와일드카드 (C#의 _와 유사)
};

// if let - 조건부 패턴 매칭
if let Some(value) = optional {
    println!("값 획득: {}", value);
}

// while let - 패턴 매칭 루프
while let Some(item) = iterator.next() {
    println!("항목: {}", item);
}

// 패턴을 사용한 let - 구조 분해
let (x, y) = point; // 튜플 구조 분해
let Some(value) = optional else {
    return; // 패턴이 일치하지 않을 경우 조기 반환 (Early return)
};
```

### 메모리 안전성 키워드

#### C# 메모리 키워드
```csharp
// unsafe - 안전성 검사 비활성화
unsafe
{
    int* ptr = &variable;
    *ptr = 42;
}

// fixed - 매니지드 메모리 고정
unsafe
{
    fixed (byte* ptr = array)
    {
        // ptr 사용
    }
}
```

#### Rust 안전성 키워드
```rust
// unsafe - 빌림 검사기 비활성화 (주의해서 사용하세요!)
unsafe {
    let ptr = &variable as *const i32;
    let value = *ptr; // 원시 포인터(Raw pointer) 역참조
}

// 원시 포인터 타입 (C#에는 직접 대응하는 개념이 없으며, 보통은 필요하지 않음)
let ptr: *const i32 = &42;  // 불변 원시 포인터
let ptr: *mut i32 = &mut 42; // 가변 원시 포인터
```

### C#에는 없는 일반적인 Rust 키워드

```rust
// where - 제네릭 제약 조건 (C#의 where보다 유연함)
fn generic_function<T>() 
where 
    T: Clone + Send + Sync,
{
    // T는 Clone, Send, Sync 트레이트를 구현해야 함
}

// dyn - 동적 트레이트 객체 (C#의 object와 유사하지만 타입 안전함)
let drawable: Box<dyn Draw> = Box::new(Circle::new());

// Self - 구현 중인 타입 자체를 참조 (C#의 this와 유사하지만 타입에 대해 사용)
impl MyStruct {
    fn new() -> Self { // Self = MyStruct
        Self { field: 0 }
    }
}

// self - 메서드 수신자(Receiver)
impl MyStruct {
    fn method(&self) { }        // 불변 빌림
    fn method_mut(&mut self) { } // 가변 빌림  
    fn consume(self) { }        // 소유권 가져오기
}

// crate - 현재 크레이트의 루트를 참조
use crate::models::User; // 크레이트 루트로부터의 절대 경로

// super - 부모 모듈을 참조
use super::utils; // 부모 모듈로부터 임포트
```

### C# 개발자를 위한 키워드 요약

| 용도 | C# | Rust | 주요 차이점 |
|---------|----|----|----------------|
| 가시성 | `public`, `private`, `internal` | `pub`, 기본 비공개 | `pub(crate)` 등을 통해 더 세밀하게 제어 가능 |
| 변수 | `var`, `readonly`, `const` | `let`, `let mut`, `const` | 기본적으로 불변 |
| 함수 | `method()` | `fn` | 독립 함수(Standalone) 존재 가능 |
| 타입 | `class`, `struct`, `interface` | `struct`, `enum`, `trait` | Enum은 대수적 데이터 타입임 |
| 제네릭 | `<T> where T : IFoo` | `<T> where T: Foo` | 제약 조건이 더 유연함 |
| 참조 | `ref`, `out`, `in` | `&`, `&mut` | 컴파일 타임에 빌림 검사 수행 |
| 패턴 | `switch`, `is` | `match`, `if let` | 철저한(Exhaustive) 매칭 필수 |

***
