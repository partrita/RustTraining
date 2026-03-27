## `Vec<T>` vs `List<T>`

> **학습 목표:** `Vec<T>`와 `List<T>`, `HashMap`과 `Dictionary`를 비교하고, 안전한 접근 패턴(Rust가 예외를 던지는 대신 `Option`을 반환하는 이유)과 컬렉션이 소유권에 미치는 영향을 배웁니다.
>
> **난이도:** 🟢 초급

`Vec<T>`는 C#의 `List<T>`에 대응하는 Rust의 컬렉션이지만, 소유권 의미론이 적용됩니다.

### C# `List<T>`
```csharp
// C# List<T> - 참조 타입, 힙에 할당됨
var numbers = new List<int>();
numbers.Add(1);
numbers.Add(2);
numbers.Add(3);

// 메서드에 전달 - 참조가 복사됨
ProcessList(numbers);
Console.WriteLine(numbers.Count);  // 여전히 접근 가능함

void ProcessList(List<int> list)
{
    list.Add(4);  // 원본 리스트가 수정됨
    Console.WriteLine($"메서드 내 개수: {list.Count}");
}
```

### Rust `Vec<T>`
```rust
// Rust Vec<T> - 소유권이 있는 타입, 힙에 할당됨
let mut numbers = Vec::new();
numbers.push(1);
numbers.push(2);
numbers.push(3);

// 소유권을 가져가는 메서드
process_vec(numbers);
// println!("{:?}", numbers);  // ❌ 에러: numbers가 이동(Move)됨

// 빌림을 사용하는 메서드
let mut numbers = vec![1, 2, 3];  // 편의를 위한 vec! 매크로
process_vec_borrowed(&mut numbers);
println!("{:?}", numbers);  // ✅ 여전히 접근 가능함

fn process_vec(mut vec: Vec<i32>) {  // 소유권을 가져옴
    vec.push(4);
    println!("메서드 내 개수: {}", vec.len());
    // vec은 여기서 메모리에서 해제(Drop)됨
}

fn process_vec_borrowed(vec: &mut Vec<i32>) {  // 가변으로 빌려옴
    vec.push(4);
    println!("메서드 내 개수: {}", vec.len());
}
```

### 벡터 생성 및 초기화
```csharp
// C# List 초기화
var numbers = new List<int> { 1, 2, 3, 4, 5 };
var empty = new List<int>();
var sized = new List<int>(10);  // 초기 용량(Capacity) 설정

// 다른 컬렉션으로부터 생성
var fromArray = new List<int>(new[] { 1, 2, 3 });
```

```rust
// Rust Vec 초기화
let numbers = vec![1, 2, 3, 4, 5];  // vec! 매크로
let empty: Vec<i32> = Vec::new();   // 빈 벡터는 타입 명시가 필요함
let sized = Vec::with_capacity(10); // 미리 용량 할당

// 반복자(Iterator)로부터 생성
let from_range: Vec<i32> = (1..=5).collect();
let from_array = vec![1, 2, 3];
```

### 주요 작업 비교
```csharp
// C# List 작업
var list = new List<int> { 1, 2, 3 };

list.Add(4);                    // 요소 추가
list.Insert(0, 0);              // 특정 인덱스에 삽입
list.Remove(2);                 // 첫 번째 일치 항목 삭제
list.RemoveAt(1);               // 특정 인덱스 항목 삭제
list.Clear();                   // 모든 항목 삭제

int first = list[0];            // 인덱스로 접근
int count = list.Count;         // 개수 확인
bool contains = list.Contains(3); // 포함 여부 확인
```

```rust
// Rust Vec 작업
let mut vec = vec![1, 2, 3];

vec.push(4);                    // 요소 추가
vec.insert(0, 0);               // 특정 인덱스에 삽입
vec.retain(|&x| x != 2);        // 요소 유지/삭제 (함수형 스타일)
vec.remove(1);                  // 특정 인덱스 항목 삭제
vec.clear();                    // 모든 항목 삭제

let first = vec[0];             // 인덱스 접근 (범위 벗어나면 패닉 발생)
let safe_first = vec.get(0);    // 안전한 접근, Option<&T> 반환
let count = vec.len();          // 개수 확인
let contains = vec.contains(&3); // 포함 여부 확인
```

### 안전한 접근 패턴
```csharp
// C# - 예외 기반의 경계 검사
public int SafeAccess(List<int> list, int index)
{
    try
    {
        return list[index];
    }
    catch (ArgumentOutOfRangeException)
    {
        return -1;  // 기본값 반환
    }
}
```

```rust
// Rust - Option 기반의 안전한 접근
fn safe_access(vec: &Vec<i32>, index: usize) -> Option<i32> {
    vec.get(index).copied()  // Option<i32> 반환
}

fn main() {
    let vec = vec![1, 2, 3];
    
    // 안전한 접근 패턴
    match vec.get(10) {
        Some(value) => println!("값: {}", value),
        None => println!("인덱스 범위를 벗어남"),
    }
    
    // 또는 unwrap_or 사용
    let value = vec.get(10).copied().unwrap_or(-1);
    println!("값: {}", value);
}
```

***

## HashMap vs Dictionary

HashMap은 C#의 `Dictionary<K,V>`에 대응하는 Rust의 컬렉션입니다.

### C# Dictionary
```csharp
// C# Dictionary<TKey, TValue>
var scores = new Dictionary<string, int>
{
    ["앨리스"] = 100,
    ["밥"] = 85,
    ["찰리"] = 92
};

// 추가/업데이트
scores["데이브"] = 78;
scores["앨리스"] = 105;  // 기존 값 업데이트

// 안전한 접근
if (scores.TryGetValue("이브", out int score))
{
    Console.WriteLine($"이브의 점수: {score}");
}
else
{
    Console.WriteLine("이브를 찾을 수 없음");
}

// 반복문
foreach (var kvp in scores)
{
    Console.WriteLine($"{kvp.Key}: {kvp.Value}");
}
```

### Rust HashMap
```rust
use std::collections::HashMap;

// HashMap 생성 및 초기화
let mut scores = HashMap::new();
scores.insert("앨리스".to_string(), 100);
scores.insert("밥".to_string(), 85);
scores.insert("찰리".to_string(), 92);

// 또는 반복자로부터 생성
let scores: HashMap<String, i32> = [
    ("앨리스".to_string(), 100),
    ("밥".to_string(), 85),
    ("찰리".to_string(), 92),
].into_iter().collect();

// 추가/업데이트
let mut scores = scores;  // 가변으로 변경
scores.insert("데이브".to_string(), 78);
scores.insert("앨리스".to_string(), 105);  // 기존 값 업데이트

// 안전한 접근
match scores.get("이브") {
    Some(score) => println!("이브의 점수: {}", score),
    None => println!("이브를 찾을 수 없음"),
}

// 반복문
for (name, score) in &scores {
    println!("{}: {}", name, score);
}
```

### HashMap 작업
```csharp
// C# Dictionary 작업
var dict = new Dictionary<string, int>();

dict["key"] = 42;                    // 삽입/업데이트
bool exists = dict.ContainsKey("key"); // 존재 여부 확인
bool removed = dict.Remove("key");    // 삭제
dict.Clear();                        // 전체 삭제

// 기본값과 함께 가져오기
int value = dict.GetValueOrDefault("missing", 0);
```

```rust
use std::collections::HashMap;

// Rust HashMap 작업
let mut map = HashMap::new();

map.insert("key".to_string(), 42);   // 삽입/업데이트
let exists = map.contains_key("key"); // 존재 여부 확인
let removed = map.remove("key");      // 삭제, Option<V> 반환
map.clear();                         // 전체 삭제

// Entry API를 사용한 고급 작업
let mut map = HashMap::new();
map.entry("key".to_string()).or_insert(42);  // 없으면 삽입
map.entry("key".to_string()).and_modify(|v| *v += 1); // 있으면 수정

// 기본값과 함께 가져오기
let value = map.get("missing").copied().unwrap_or(0);
```

### HashMap 키와 값의 소유권
```rust
// HashMap에서의 소유권 이해
fn ownership_example() {
    let mut map = HashMap::new();
    
    // String 키와 값의 소유권이 맵 내부로 이동함
    let key = String::from("이름");
    let value = String::from("앨리스");
    
    map.insert(key, value);
    // println!("{}", key);   // ❌ 에러: key가 이동됨
    // println!("{}", value); // ❌ 에러: value가 이동됨
    
    // 참조를 통한 접근
    if let Some(name) = map.get("이름") {
        println!("이름: {}", name);  // 값을 빌려옴
    }
}

// &str 키 사용 (소유권 이전 없음)
fn string_slice_keys() {
    let mut map = HashMap::new();
    
    map.insert("이름", "앨리스");     // &str 키와 값
    map.insert("나이", "30");
    
    // 문자열 리터럴은 소유권 문제가 발생하지 않음
    println!("이름 존재 여부: {}", map.contains_key("이름"));
}
```

***

## 컬렉션 다루기

### 반복 패턴
```csharp
// C# 반복 패턴
var numbers = new List<int> { 1, 2, 3, 4, 5 };

// 인덱스를 포함한 for 루프
for (int i = 0; i < numbers.Count; i++)
{
    Console.WriteLine($"인덱스 {i}: {numbers[i]}");
}

// foreach 루프
foreach (int num in numbers)
{
    Console.WriteLine(num);
}

// LINQ 메서드
var doubled = numbers.Select(x => x * 2).ToList();
var evens = numbers.Where(x => x % 2 == 0).ToList();
```

```rust
// Rust 반복 패턴
let numbers = vec![1, 2, 3, 4, 5];

// 인덱스를 포함한 for 루프
for (i, num) in numbers.iter().enumerate() {
    println!("인덱스 {}: {}", i, num);
}

// 값에 대한 for 루프
for num in &numbers {  // 각 요소를 빌려옴
    println!("{}", num);
}

// 반복자 메서드 (LINQ와 유사)
let doubled: Vec<i32> = numbers.iter().map(|x| x * 2).collect();
let evens: Vec<i32> = numbers.iter().filter(|&x| x % 2 == 0).cloned().collect();

// 또는 더 효율적으로, 소유권을 소비하는 반복자 사용
let doubled: Vec<i32> = numbers.into_iter().map(|x| x * 2).collect();
```

### Iterator vs IntoIterator vs Iter
```rust
// 다양한 반복 메서드 이해하기
fn iteration_methods() {
    let vec = vec![1, 2, 3, 4, 5];
    
    // 1. iter() - 요소를 빌려옴 (&T)
    for item in vec.iter() {
        println!("{}", item);  // item 타입은 &i32
    }
    // vec은 여기서 여전히 사용 가능함
    
    // 2. into_iter() - 소유권을 가져옴 (T)
    for item in vec.into_iter() {
        println!("{}", item);  // item 타입은 i32
    }
    // vec은 여기서 더 이상 사용 불가능함
    
    let mut vec = vec![1, 2, 3, 4, 5];
    
    // 3. iter_mut() - 가변으로 빌려옴 (&mut T)
    for item in vec.iter_mut() {
        *item *= 2;  // item 타입은 &mut i32
    }
    println!("{:?}", vec);  // [2, 4, 6, 8, 10]
}
```

### 결과 수집하기
```csharp
// C# - 잠재적 에러가 포함된 컬렉션 처리
public List<int> ParseNumbers(List<string> inputs)
{
    var results = new List<int>();
    foreach (string input in inputs)
    {
        if (int.TryParse(input, out int result))
        {
            results.Add(result);
        }
        // 유효하지 않은 입력은 조용히 무시함
    }
    return results;
}
```

```rust
// Rust - collect를 사용한 명시적 에러 핸들링
fn parse_numbers(inputs: Vec<String>) -> Result<Vec<i32>, std::num::ParseIntError> {
    inputs.into_iter()
        .map(|s| s.parse::<i32>())  // Result<i32, ParseIntError> 반환
        .collect()                  // Result<Vec<i32>, ParseIntError>로 수집됨
}

// 대안: 에러 필터링하기
fn parse_numbers_filter(inputs: Vec<String>) -> Vec<i32> {
    inputs.into_iter()
        .filter_map(|s| s.parse::<i32>().ok())  // Ok인 값만 유지
        .collect()
}

fn main() {
    let inputs = vec!["1".to_string(), "2".to_string(), "invalid".to_string(), "4".to_string()];
    
    // 첫 번째 에러 발생 시 실패하는 버전
    match parse_numbers(inputs.clone()) {
        Ok(numbers) => println!("모두 파싱됨: {:?}", numbers),
        Err(error) => println!("파싱 에러: {}", error),
    }
    
    // 에러를 건너뛰는 버전
    let numbers = parse_numbers_filter(inputs);
    println!("성공적으로 파싱된 항목들: {:?}", numbers);  // [1, 2, 4]
}
```

---

## 연습 문제

<details>
<summary><strong>🏋️ 실습: LINQ를 반복자로 변환하기</strong> (펼치기)</summary>

다음 C# LINQ 쿼리를 관용적인 Rust 반복자로 변환해 보세요:

```csharp
var result = students
    .Where(s => s.Grade >= 90)
    .OrderByDescending(s => s.Grade)
    .Select(s => $"{s.Name}: {s.Grade}")
    .Take(3)
    .ToList();
```

다음 구조체를 사용하세요:
```rust
struct Student { name: String, grade: u32 }
```

점수가 90점 이상인 상위 3명의 학생을 `"이름: 점수"` 형식의 `Vec<String>`으로 반환하세요.

<details>
<summary>🔑 해답</summary>

```rust
#[derive(Debug)]
struct Student { name: String, grade: u32 }

fn top_students(students: &mut [Student]) -> Vec<String> {
    students.sort_by(|a, b| b.grade.cmp(&a.grade)); // 내림차순 정렬
    students.iter()
        .filter(|s| s.grade >= 90)
        .take(3)
        .map(|s| format!("{}: {}", s.name, s.grade))
        .collect()
}

fn main() {
    let mut students = vec![
        Student { name: "앨리스".into(), grade: 95 },
        Student { name: "밥".into(), grade: 88 },
        Student { name: "찰리".into(), grade: 92 },
        Student { name: "데이브".into(), grade: 97 },
        Student { name: "이브".into(), grade: 91 },
    ];
    let result = top_students(&mut students);
    assert_eq!(result, vec!["데이브: 97", "앨리스: 95", "찰리: 92"]);
    println!("{result:?}");
}
```

**C#과의 핵심 차이점**: Rust 반복자는 LINQ처럼 지연 실행(Lazy)되지만, `.sort_by()`는 즉시 실행(Eager)되며 제자리에서(In-place) 정렬을 수행합니다. 즉, 지연 실행되는 `OrderBy`는 없습니다. 먼저 정렬을 수행한 뒤 지연 실행되는 작업들을 연결해야 합니다.

</details>
</details>

***
