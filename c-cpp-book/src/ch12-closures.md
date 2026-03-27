## Rust 클로저(Closures)

> **학습 내용:** 익명 함수로서의 클로저, 세 가지 캡처 트레이트(`Fn`, `FnMut`, `FnOnce`), `move` 클로저를 배웁니다. 또한 수동으로 `[&]`/`[=]`를 명시하는 대신 자동 캡처 분석을 수행하는 Rust 클로저를 C++ 람다와 비교해 봅니다.

- 클로저는 주변 환경을 캡처할 수 있는 익명 함수입니다.
    - C++ 대응 개념: 람다 (`[&](int x) { return x + 1; }`)
    - 핵심 차이점: Rust 클로저는 컴파일러가 자동으로 선택하는 **세 가지** 캡처 트레이트(`Fn`, `FnMut`, `FnOnce`)를 가집니다.
    - C++의 캡처 모드(`[=]`, `[&]`, `[this]`)는 수동적이며 실수하기 쉽습니다 (댕글링 `[&]` 위험!).
    - Rust의 빌림 검사기는 컴파일 타임에 댕글링 캡처를 방지합니다.
- 클로저는 `||` 기호로 식별할 수 있습니다. 매개변수 타입은 `||` 안에 기술하며 타입 추론을 사용할 수 있습니다.
- 클로저는 반복자(다음 주제)와 함께 자주 사용됩니다.
```rust
fn add_one(x: u32) -> u32 {
    x + 1
}
fn main() {
    let add_one_v1 = |x : u32| {x + 1}; // 타입을 명시적으로 지정
    let add_one_v2 = |x| {x + 1};   // 호출 지점에서 타입이 추론됨
    let add_one_v3 = |x| x+1;   // 한 줄짜리 함수인 경우 허용됨
    println!("{} {} {} {}", add_one(42), add_one_v1(42), add_one_v2(42), add_one_v3(42) );
}
```


# 연습 문제: 클로저 및 캡처링

🟡 **중급**

- 바깥쪽 범위의 `String`을 캡처하여 그 뒤에 내용을 덧붙이는 클로저를 만드세요 (힌트: `move` 사용).
- 클로저 벡터 `Vec<Box<dyn Fn(i32) -> i32>>`를 생성하세요. 이 벡터에는 1을 더하는 클로저, 2를 곱하는 클로저, 그리고 입력값을 제곱하는 클로저가 포함되어야 합니다. 벡터를 순회하며 숫자 5에 각각의 클로저를 적용해 보세요.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
fn main() {
    // 1부: String을 캡처하여 내용을 덧붙이는 클로저
    let mut greeting = String::from("Hello");
    let mut append = |suffix: &str| {
        greeting.push_str(suffix);
    };
    append(", world");
    append("!");
    println!("{greeting}");  // "Hello, world!"

    // 2부: 클로저 벡터
    let operations: Vec<Box<dyn Fn(i32) -> i32>> = vec![
        Box::new(|x| x + 1),      // 1 더하기
        Box::new(|x| x * 2),      // 2 곱하기
        Box::new(|x| x * x),      // 제곱하기
    ];

    let input = 5;
    for (i, op) in operations.iter().enumerate() {
        println!("작업 {i} (입력 {input}): {}", op(input));
    }
}
// 출력:
// Hello, world!
// 작업 0 (입력 5): 6
// 작업 1 (입력 5): 10
// 작업 2 (입력 5): 25
```

</details>

# Rust 반복자(Iterators)
- 반복자는 Rust의 가장 강력한 기능 중 하나입니다. 필터링(```filter()```), 변환(```map()```), 필터 후 변환(```filter_and_map()```), 검색(```find()```) 등 컬렉션에 대한 작업을 매우 우아하게 수행할 수 있게 해줍니다.
- 아래 예제에서 ```|&x| *x >= 42```는 동일한 비교를 수행하는 클로저입니다. ```|x| println!("{x}")``` 역시 또 다른 클로저입니다.
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    for x in &a {
        if *x >= 42 {
            println!("{x}");
        }
    }
    // 위와 동일한 동작
    a.iter().filter(|&x| *x >= 42).for_each(|x| println!("{x}"))
}
```

# Rust 반복자
- 반복자의 핵심 특징은 대부분 **지연(lazy)** 방식이라는 점입니다. 즉, 평가될 때까지는 아무 일도 하지 않습니다. 예를 들어, ```a.iter().filter(|&x| *x >= 42);```는 ```for_each``` 같은 소비 메서드 없이는 *아무 작업도* 수행하지 않았을 것입니다. Rust 컴파일러는 이러한 상황을 감지하면 명시적인 경고를 내보냅니다.
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    // 각 요소에 1을 더하고 출력
    let _ = a.iter().map(|x|x + 1).for_each(|x|println!("{x}"));
    let found = a.iter().find(|&x|*x == 42);
    println!("{found:?}");
    // 요소 개수 세기
    let count = a.iter().count();
    println!("{count}");
}
```

# Rust 반복자
- ```collect()``` 메서드를 사용하여 결과를 별도의 컬렉션으로 모을 수 있습니다.
    - 아래 코드의 ```Vec<_>```에서 ```_```는 ```map```이 반환하는 타입에 대한 와일드카드 역할을 합니다. 예를 들어, ```map```에서 ```String```을 반환할 수도 있습니다.
```rust
fn main() {
    let a = [0, 1, 2, 3, 42, 43];
    let squared_a : Vec<_> = a.iter().map(|x|x*x).collect();
    for x in &squared_a {
        println!("{x}");
    }
    let squared_a_strings : Vec<_> = a.iter().map(|x|(x*x).to_string()).collect();
    // 이것들은 실제로는 문자열 표현들입니다.
    for x in &squared_a_strings {
        println!("{x}");
    }
}
```

# 연습 문제: Rust 반복자

🟢 **초급**
- 홀수와 짝수가 섞인 정수 배열을 생성하세요. 배열을 순회하면서 짝수만 담긴 벡터와 홀수만 담긴 벡터 두 개로 분리하세요.
- 이 작업을 단일 패스로 수행할 수 있나요 (힌트: ```partition()``` 사용)?

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
fn main() {
    let numbers = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // 접근 방식 1: 수동 순회
    let mut evens = Vec::new();
    let mut odds = Vec::new();
    for n in numbers {
        if n % 2 == 0 {
            evens.push(n);
        } else {
            odds.push(n);
        }
    }
    println!("짝수: {evens:?}");
    println!("홀수: {odds:?}");

    // 접근 방식 2: partition()을 이용한 단일 패스
    let (evens, odds): (Vec<i32>, Vec<i32>) = numbers
        .into_iter()
        .partition(|n| n % 2 == 0);
    println!("짝수 (partition): {evens:?}");
    println!("홀수 (partition): {odds:?}");
}
// 출력:
// 짝수: [2, 4, 6, 8, 10]
// 홀수: [1, 3, 5, 7, 9]
// 짝수 (partition): [2, 4, 6, 8, 10]
// 홀수 (partition): [1, 3, 5, 7, 9]
```

</details>

> **실무 패턴**: 실제 운영 환경의 Rust 코드에서 사용되는 반복자 체인(`.map().collect()`, `.filter().collect()`, `.find_map()`) 예시는 [클로저를 이용한 할당 피라미드 축소](ch17-3-collapsing-assignment-pyramids.md#collapsing-assignment-pyramids-with-closures)를 참조하세요.

### 반복자 강력한 도구들: C++ 루프를 대체하는 메서드들

다음 반복자 어댑터들은 Rust 운영 환경의 코드에서 *광범위하게* 사용됩니다. C++에는 `<algorithm>`과 C++20의 ranges가 있지만, Rust의 반복자 체인은 더 결합하기 쉽고 보편적으로 사용됩니다.

#### `enumerate` — 인덱스 + 값 (`for (int i = 0; ...)` 대체)

```rust
let sensors = vec!["temp0", "temp1", "temp2"];
for (idx, name) in sensors.iter().enumerate() {
    println!("센서 {idx}: {name}");
}
// 센서 0: temp0
// 센서 1: temp1
// 센서 2: temp2
```

C++ 대응 코드: `for (size_t i = 0; i < sensors.size(); ++i) { auto& name = sensors[i]; ... }`

#### `zip` — 두 반복자의 요소를 쌍으로 묶기 (병렬 인덱스 루프 대체)

```rust
let names = ["gpu0", "gpu1", "gpu2"];
let temps = [72.5, 68.0, 75.3];

let report: Vec<String> = names.iter()
    .zip(temps.iter())
    .map(|(name, temp)| format!("{name}: {temp}°C"))
    .collect();
println!("{report:?}");
// ["gpu0: 72.5°C", "gpu1: 68.0°C", "gpu2: 75.3°C"]

// 더 짧은 반복자 쪽에서 멈추므로 인덱스 범위 초과 위험이 없습니다.
```

C++ 대응 코드: `for (size_t i = 0; i < std::min(names.size(), temps.size()); ++i) { ... }`

#### `flat_map` — 맵 수행 후 중첩된 컬렉션 평탄화

```rust
// 각 GPU는 여러 PCIe BDF를 가집니다. 모든 GPU의 모든 BDF를 수집합니다.
let gpu_bdfs = vec![
    vec!["0000:01:00.0", "0000:02:00.0"],
    vec!["0000:41:00.0"],
    vec!["0000:81:00.0", "0000:82:00.0"],
];

let all_bdfs: Vec<&str> = gpu_bdfs.iter()
    .flat_map(|bdfs| bdfs.iter().copied())
    .collect();
println!("{all_bdfs:?}");
// ["0000:01:00.0", "0000:02:00.0", "0000:41:00.0", "0000:81:00.0", "0000:82:00.0"]
```

C++ 대응 코드: 중첩된 `for` 루프를 돌며 단일 벡터에 푸시.

#### `chain` — 두 반복자 연결하기

```rust
let critical_gpus = vec!["gpu0", "gpu3"];
let warning_gpus = vec!["gpu1", "gpu5"];

// 모든 플래그가 지정된 GPU를 처리하며, 위급(critical) 상태부터 먼저 처리합니다.
for gpu in critical_gpus.iter().chain(warning_gpus.iter()) {
    println!("플래그 지정됨: {gpu}");
}
```

#### `windows` 및 `chunks` — 슬라이스에 대한 슬라이딩/고정 크기 뷰

```rust
let temps = [70, 72, 75, 73, 71, 68, 65];

// windows(3): 크기 3의 슬라이딩 윈도우 — 추세 감지
let rising = temps.windows(3)
    .any(|w| w[0] < w[1] && w[1] < w[2]);
println!("상승 추세 감지됨: {rising}"); // true (70 < 72 < 75)

// chunks(2): 고정 크기 그룹 — 쌍으로 처리
for pair in temps.chunks(2) {
    println!("쌍: {pair:?}");
}
// 쌍: [70, 72]
// 쌍: [75, 73]
// 쌍: [71, 68]
// 쌍: [65]       ← 마지막 덩어리는 더 작을 수 있음
```

C++ 대응 코드: `i`와 `i+1`/`i+2`를 사용한 수동 인덱스 연산.

#### `fold` — 단일 값으로 축적 (`std::accumulate` 대체)

```rust
let errors = vec![
    ("gpu0", 3u32),
    ("gpu1", 0),
    ("gpu2", 7),
    ("gpu3", 1),
];

// 한 번의 패스로 총 에러 수를 세고 요약을 빌드합니다.
let (total, summary) = errors.iter().fold(
    (0u32, String::new()),
    |(count, mut s), (name, errs)| {
        if *errs > 0 {
            s.push_str(&format!("{name}:{errs} "));
        }
        (count + errs, s)
    },
);
println!("총 에러: {total}, 상세: {summary}");
// 총 에러: 11, 상세: gpu0:3 gpu2:7 gpu3:1
```

#### `scan` — 상태를 유지하는 변환 (누적 합계, 델타 감지 등)

```rust
let readings = [100, 105, 103, 110, 108];

// 연속된 측정값 사이의 차이(델타)를 계산합니다.
let deltas: Vec<i32> = readings.iter()
    .scan(None::<i32>, |prev, &val| {
        let delta = prev.map(|p| val - p);
        *prev = Some(val);
        Some(delta)
    })
    .flatten()  // 초기 None 제거
    .collect();
println!("델타 목록: {deltas:?}"); // [5, -2, 7, -2]
```

#### 빠른 참조: C++ 루프 → Rust 반복자

| **C++ 패턴** | **Rust 반복자** | **예시** |
|----------------|------------------|------------|
| `for (int i = 0; i < v.size(); i++)` | `.enumerate()` | `v.iter().enumerate()` |
| 인덱스를 이용한 병렬 순회 | `.zip()` | `a.iter().zip(b.iter())` |
| 중첩 루프 → 평탄화된 결과 | `.flat_map()` | `vecs.iter().flat_map(\|v\| v.iter())` |
| 두 컨테이너 연결 | `.chain()` | `a.iter().chain(b.iter())` |
| 슬라이딩 윈도우 `v[i..i+n]` | `.windows(n)` | `v.windows(3)` |
| 고정 크기 그룹으로 처리 | `.chunks(n)` | `v.chunks(4)` |
| `std::accumulate` / 수동 누산기 | `.fold()` | `.fold(init, \|acc, x\| ...)` |
| 누적 합계 / 델타 추적 | `.scan()` | `.scan(state, \|s, x\| ...)` |
| `while (it != end && count < n) { ++it; ++count; }` | `.take(n)` | `.iter().take(5)` |
| `while (it != end && !pred(*it)) { ++it; }` | `.skip_while()` | `.skip_while(\|x\| x < &threshold)` |
| `std::any_of` | `.any()` | `.iter().any(\|x\| x > &limit)` |
| `std::all_of` | `.all()` | `.iter().all(\|x\| x.is_valid())` |
| `std::none_of` | `!.any()` | `!iter.any(\|x\| x.failed())` |
| `std::count_if` | `.filter().count()` | `.filter(\|x\| x > &0).count()` |
| `std::min_element` / `std::max_element` | `.min()` / `.max()` | `.iter().max()` → `Option<&T>` |
| `std::unique` | `.dedup()` (정렬된 상태에서) | `v.dedup()` (Vec에서 제자리 수행) |

### 연습 문제: 반복자 체인

센서 데이터가 `Vec<(String, f64)>` (이름, 온도)로 주어졌을 때, 다음을 수행하는 **단일 반복자 체인**을 작성하세요:
1. 온도가 80.0보다 높은 센서만 필터링합니다.
2. 온도가 높은 순서대로(내림차순) 정렬합니다.
3. 각각을 `"{name}: {temp}°C [ALARM]"` 형식으로 포맷팅합니다.
4. `Vec<String>`으로 수집합니다.

힌트: 정렬을 위해서는 `Vec`이 필요하므로, `.sort_by()` 전에 `.collect()`가 필요할 것입니다.

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
fn alarm_report(sensors: &[(String, f64)]) -> Vec<String> {
    let mut hot: Vec<_> = sensors.iter()
        .filter(|(_, temp)| *temp > 80.0)
        .collect();
    hot.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    hot.iter()
        .map(|(name, temp)| format!("{name}: {temp}°C [ALARM]"))
        .collect()
}

fn main() {
    let sensors = vec![
        ("gpu0".to_string(), 72.5),
        ("gpu1".to_string(), 85.3),
        ("gpu2".to_string(), 91.0),
        ("gpu3".to_string(), 78.0),
        ("gpu4".to_string(), 88.7),
    ];
    for line in alarm_report(&sensors) {
        println!("{line}");
    }
}
// 출력:
// gpu2: 91°C [ALARM]
// gpu4: 88.7°C [ALARM]
// gpu1: 85.3°C [ALARM]
```

</details>

----

# Rust 반복자
- ```Iterator``` 트레이트는 사용자 정의 타입에 대해 반복을 구현하는 데 사용됩니다 (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html 참조).
    - 예제에서는 1, 1, 2, ...로 시작하며 다음 숫자가 이전 두 숫자의 합인 피보나치 수열에 대한 반복자를 구현해 보겠습니다.
    - ```Iterator```의 ```연관 타입```(```type Item = u32;```)은 반복자가 출력하는 타입(```u32```)을 정의합니다.
    - ```next()``` 메서드는 단순히 반복자를 구현하는 로직을 포함합니다. 이 경우 모든 상태 정보는 ```Fibonacci``` 구조체에 저장됩니다.
    - 더 특수화된 반복자를 위해 ```into_iter()``` 메서드를 구현하는 ```IntoIterator```라는 또 다른 트레이트를 구현할 수도 있었습니다.
    - [▶ Rust Playground에서 시도해 보기](https://play.rust-lang.org/)
