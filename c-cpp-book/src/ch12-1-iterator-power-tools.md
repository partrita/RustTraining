## 반복자(Iterator) 강력한 도구들 참조

> **학습 내용:** `filter`/`map`/`collect`를 넘어서는 고급 반복자 어댑터인 `enumerate`, `zip`, `chain`, `flat_map`, `scan`, `windows`, `chunks` 등을 배웁니다. C 스타일의 인덱스 기반 `for` 루프를 안전하고 표현력이 뛰어난 Rust 반복자로 대체하는 데 필수적인 도구들입니다.

기본적인 `filter`/`map`/`collect` 체인도 많은 경우를 해결하지만, Rust의 반복자 라이브러리는 훨씬 더 풍부합니다. 이 섹션에서는 인덱스를 수동으로 추적하거나, 결과를 누적하거나, 고정된 크기의 덩어리로 데이터를 처리하는 C 스타일 루프를 변환할 때 매일 사용하게 될 도구들을 다룹니다.

### 빠른 참조 테이블

| 메서드 | C 대응 개념 | 역할 | 반환 타입 |
|--------|-------------|-------------|---------|
| `enumerate()` | `for (int i=0; ...)` | 각 요소를 해당 인덱스와 쌍으로 묶음 | `(usize, T)` |
| `zip(other)` | 동일한 인덱스를 가진 병렬 배열 | 두 반복자의 요소를 쌍으로 묶음 | `(A, B)` |
| `chain(other)` | 배열1 처리 후 배열2 처리 | 두 반복자를 연결함 | `T` |
| `flat_map(f)` | 중첩 루프 | 매핑 후 한 수준 평탄화함 | `U` |
| `windows(n)` | `for (int i=0; i<len-n+1; i++) &arr[i..i+n]` | 크기 `n`인 중첩되는 슬라이스 생성 | `&[T]` |
| `chunks(n)` | 한 번에 `n`개씩 요소 처리 | 크기 `n`인 중첩되지 않는 슬라이스 생성 | `&[T]` |
| `fold(init, f)` | `int acc = init; for (...) acc = f(acc, x);` | 단일 값으로 축약 | `Acc` |
| `scan(init, f)` | 출력이 있는 중간 누산기 | `fold`와 비슷하지만 중간 결과를 산출함 | `Option<B>` |
| `take(n)` / `skip(n)` | 오프셋/제한에서 루프 시작 | 처음 `n`개 선택 / 처음 `n`개 건너뜀 | `T` |
| `take_while(f)` / `skip_while(f)` | `while (pred) {...}` | 조건이 참인 동안 선택/건너뜀 | `T` |
| `peekable()` | `arr[i+1]`을 이용한 미리보기 | 소비하지 않고 `.peek()` 가능하게 함 | `T` |
| `step_by(n)` | `for (i=0; i<len; i+=n)` | 매 n번째 요소마다 선택 | `T` |
| `unzip()` | 병렬 배열 분리 | 쌍(pair)을 두 개의 컬렉션으로 분리하여 수집 | `(A, B)` |
| `sum()` / `product()` | 합계/곱셈 누적 | `+` 또는 `*`를 사용하여 축약 | `T` |
| `min()` / `max()` | 극값 찾기 | 최솟값/최댓값 반환 | `Option<T>` |
| `any(f)` / `all(f)` | `bool found = false; for (...) ...` | 단락 평가(short-circuit) 불리언 검색 | `bool` |
| `position(f)` | `for (i=0; ...) if (pred) return i;` | 첫 번째 일치하는 요소의 인덱스 | `Option<usize>` |

### `enumerate` — 인덱스 + 값 (C 인덱스 루프 대체)

```rust
fn main() {
    let sensors = ["GPU_TEMP", "CPU_TEMP", "FAN_RPM", "PSU_WATT"];

    // C 스타일: for (int i = 0; i < 4; i++) printf("[%d] %s\n", i, sensors[i]);
    for (i, name) in sensors.iter().enumerate() {
        println!("[{i}] {name}");
    }

    // 특정 센서의 인덱스 찾기
    let gpu_idx = sensors.iter().position(|&s| s == "GPU_TEMP");
    println!("GPU 센서 인덱스: {gpu_idx:?}");  // Some(0)
}
```

### `zip` — 병렬 반복 (병렬 배열 루프 대체)

```rust
fn main() {
    let names = ["accel_diag", "nic_diag", "cpu_diag"];
    let statuses = [true, false, true];
    let durations_ms = [1200, 850, 3400];

    // C: for (int i=0; i<3; i++) printf("%s: %s (%d ms)\n", names[i], ...);
    for ((name, passed), ms) in names.iter().zip(&statuses).zip(&durations_ms) {
        let status = if *passed { "PASS" } else { "FAIL" };
        println!("{name}: {status} ({ms} ms)");
    }
}
```

### `chain` — 반복자 연결

```rust
fn main() {
    let critical = vec!["ECC error", "Thermal shutdown"];
    let warnings = vec!["Link degraded", "Fan slow"];

    // 모든 이벤트를 우선순위 순서대로 처리
    let all_events: Vec<_> = critical.iter().chain(warnings.iter()).collect();
    println!("{all_events:?}");
    // ["ECC error", "Thermal shutdown", "Link degraded", "Fan slow"]
}
```

### `flat_map` — 중첩된 결과 평탄화

```rust
fn main() {
    let lines = vec!["gpu:42:ok", "nic:99:fail", "cpu:7:ok"];

    // 콜론(:)으로 구분된 라인에서 모든 숫자 값 추출
    let numbers: Vec<u32> = lines.iter()
        .flat_map(|line| line.split(':'))
        .filter_map(|token| token.parse::<u32>().ok())
        .collect();
    println!("{numbers:?}");  // [42, 99, 7]
}
```

### `windows` 및 `chunks` — 슬라이딩 및 고정 크기 그룹

```rust
fn main() {
    let temps = [65, 68, 72, 71, 75, 80, 78, 76];

    // windows(3): 중첩되는 3개씩의 그룹 (이동 평균과 유사)
    // C: for (int i = 0; i <= len-3; i++) avg(arr[i], arr[i+1], arr[i+2]);
    let moving_avg: Vec<f64> = temps.windows(3)
        .map(|w| w.iter().sum::<i32>() as f64 / 3.0)
        .collect();
    println!("이동 평균: {moving_avg:.1?}");

    // chunks(2): 중첩되지 않는 2개씩의 그룹
    // C: for (int i = 0; i < len; i += 2) process(arr[i], arr[i+1]);
    for pair in temps.chunks(2) {
        println!("덩어리(Chunk): {pair:?}");
    }

    // chunks_exact(2): 동일하지만 나머지가 있으면 패닉 발생
    // 또한: .remainder()는 남은 요소들을 제공함
}
```

### `fold` 및 `scan` — 누적 작업

```rust
fn main() {
    let values = [10, 20, 30, 40, 50];

    // fold: 단일 최종 결과 (C의 누산기 루프와 유사)
    let sum = values.iter().fold(0, |acc, &x| acc + x);
    println!("합계: {sum}");  // 150

    // fold를 사용하여 문자열 빌드
    let csv = values.iter()
        .fold(String::new(), |acc, x| {
            if acc.is_empty() { format!("{x}") }
            else { format!("{acc},{x}") }
        });
    println!("CSV: {csv}");  // "10,20,30,40,50"

    // scan: fold와 비슷하지만 중간 결과를 산출함
    let running_sum: Vec<i32> = values.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect();
    println!("누적 합계: {running_sum:?}");  // [10, 30, 60, 100, 150]
}
```

### 연습 문제: 센서 데이터 파이프라인

원시 센서 데이터(한 줄당 하나, 형식 `"센서이름:값:단위"`)가 주어졌을 때, 다음을 수행하는 반복자 파이프라인을 작성하세요:
1. 각 줄을 `(이름, f64, 단위)`로 파싱합니다.
2. 특정 임계값(threshold) 미만의 값은 필터링합니다.
3. `fold`를 사용하여 센서 이름별로 `HashMap`에 그룹화합니다.
4. 센서당 평균 값을 출력합니다.

```rust
// 시작 코드
fn main() {
    let raw_data = vec![
        "gpu_temp:72.5:C",
        "cpu_temp:65.0:C",
        "gpu_temp:74.2:C",
        "fan_rpm:1200.0:RPM",
        "cpu_temp:63.8:C",
        "gpu_temp:80.1:C",
        "fan_rpm:1150.0:RPM",
    ];
    let threshold = 70.0;
    // TODO: 파싱, 임계값 이상의 값 필터링, 이름별 그룹화, 평균 계산
}
```

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
use std::collections::HashMap;

fn main() {
    let raw_data = vec![
        "gpu_temp:72.5:C",
        "cpu_temp:65.0:C",
        "gpu_temp:74.2:C",
        "fan_rpm:1200.0:RPM",
        "cpu_temp:63.8:C",
        "gpu_temp:80.1:C",
        "fan_rpm:1150.0:RPM",
    ];
    let threshold = 70.0;

    // 파싱 → 필터링 → 그룹화 → 평균 계산
    let grouped = raw_data.iter()
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(3, ':').collect();
            if parts.len() == 3 {
                let value: f64 = parts[1].parse().ok()?;
                Some((parts[0], value, parts[2]))
            } else {
                None
            }
        })
        .filter(|(_, value, _)| *value >= threshold)
        .fold(HashMap::<&str, Vec<f64>>::new(), |mut acc, (name, value, _)| {
            acc.entry(name).or_default().push(value);
            acc
        });

    for (name, values) in &grouped {
        let avg = values.iter().sum::<f64>() / values.len() as f64;
        println!("{name}: 평균={avg:.1} ({}개의 측정값)", values.len());
    }
}
// 출력 (순서는 다를 수 있음):
// gpu_temp: 평균=75.6 (3개의 측정값)
// fan_rpm: 평균=1175.0 (2개의 측정값)
```

</details>


# Rust 반복자
- ```Iterator``` 트레이트는 사용자 정의 타입에 대해 반복을 구현하는 데 사용됩니다 (https://doc.rust-lang.org/std/iter/trait.IntoIterator.html 참조).
    - 예제에서는 1, 1, 2, ...로 시작하며 다음 숫자가 이전 두 숫자의 합인 피보나치 수열에 대한 반복자를 구현해 보겠습니다.
    - ```Iterator```의 ```연관 타입```(```type Item = u32;```)은 반복자가 출력하는 타입(```u32```)을 정의합니다.
    - ```next()``` 메서드는 단순히 반복자를 구현하는 로직을 포함합니다. 이 경우 모든 상태 정보는 ```Fibonacci``` 구조체에 저장됩니다.
    - 더 특수화된 반복자를 위해 ```into_iter()``` 메서드를 구현하는 ```IntoIterator```라는 또 다른 트레이트를 구현할 수도 있었습니다.
    - [▶ Rust Playground에서 시도해 보기](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=ab367dc2611e1b5a0bf98f1185b38f3f)
