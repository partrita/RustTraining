# 12-1. 반복자의 파워 툴: 고급 어댑터 활용 🟡

> **학습 목표:**
> - `filter`, `map`, `collect`를 넘어선 고차원 반복자 어댑터들을 마스터합니다.
> - 인덱스 추적, 병렬 배열 순회, 중첩 루프 평탄화, 슬라이딩 윈도우 분석 등 복잡한 루프를 루프 대신 반복자 체인으로 대체하는 방법을 배웁니다.
> - C 스타일의 복식 루프나 복잡한 알고리즘을 안전하고 표현력 넘치는 코드로 변환합니다.

---

### 🛠️ 핵심 반복자 도구함 (Quick Reference)

| **메서드** | **C/C++ 대응 개념** | **주요 역할** |
| :--- | :--- | :--- |
| **`.enumerate()`** | `for (int i=0; ...)` | 인덱스와 값을 쌍으로 묶음 `(usize, T)` |
| **`.zip(other)`** | 병렬 배열(Parallel Arrays) | 두 반복자의 요소를 1:1로 묶음 |
| **`.chain(other)`** | 순차적 처리 (A 후 B) | 두 반복자를 하나로 이어 붙임 |
| **`.flat_map(f)`** | 중첩 루프 (Nested Loops) | 매핑 후 중첩된 구조를 평탄하게 폄 |
| **`.windows(n)`** | `arr[i..i+n]` (슬라이딩) | 중첩되는 `n`개 크기의 슬라이스 생성 |
| **`.chunks(n)`** | 고정 크기 블록 처리 | 중첩되지 않는 `n`개 단위 덩어리 생성 |
| **`.fold(init, f)`** | `std::accumulate` / 누산기 | 하나의 최종 결과값으로 응축 |
| **`.scan(init, f)`** | 중간 상태를 가진 변환 | 누적 합계 등 중간 과정을 포함한 결과 산출 |
| **`.peekable()`** | `arr[i+1]` (미리보기) | 다음 요소를 소비하지 않고 미리 확인(`peek`) |

---

### 1. 인덱스와 병렬 순회 (`enumerate`, `zip`)
수동으로 인덱스 변수를 관리하거나 여러 배열의 길이를 맞추는 번거로움을 덜어줍니다.

```rust
fn main() {
    let tasks = ["센서 확인", "데이터 파싱", "업로드"];
    let status = [true, true, false];

    // zip으로 묶고 enumerate로 번호를 매깁니다.
    for (i, (task, done)) in tasks.iter().zip(status.iter()).enumerate() {
        let result = if *done { "완료" } else { "대기" };
        println!("[작업 {i}] {task}: {result}");
    }
}
```

---

### 2. 슬라이딩 윈도우와 덩어리 처리 (`windows`, `chunks`)
데이터의 흐름(Trend)을 분석하거나 데이터를 고정된 크기의 패킷으로 나눌 때 매우 유용합니다.

```rust
fn main() {
    let temps = [60, 62, 65, 64, 68, 72, 70];

    // 3개들이 슬라이딩 윈도우: 3일 연속 온도 상승 여부 확인
    let is_rising = temps.windows(3).any(|w| w[0] < w[1] && w[1] < w[2]);
    println!("상승 추세 감지: {is_rising}");

    // 데이터를 2개씩 덩어리로 묶어서 처리 (중첩되지 않음)
    for pair in temps.chunks(2) {
        println!("데이터 쌍: {pair:?}");
    }
}
```

---

### 3. 복잡한 누계 연산 (`fold`, `scan`)
루프를 돌며 외부 가변 변수를 수정하는 대신, 상태를 안전하게 전달하며 결과를 도출합니다.

```rust
fn main() {
    let values = [1, 2, 3, 4, 5];

    // fold: 모든 요소를 곱한 최종 결과 (초기값 1)
    let total_product = values.iter().fold(1, |acc, &x| acc * x);
    println!("최종 곱: {total_product}"); // 120

    // scan: 누적 합계를 계산하며 중간 과정까지 벡터로 수집
    let running_sum: Vec<i32> = values.iter()
        .scan(0, |sum, &x| {
            *sum += x;
            Some(*sum)
        })
        .collect();
    println!("단계별 합계: {running_sum:?}"); // [1, 3, 6, 10, 15]
}
```

---

### 💡 전문가를 위한 팁: `peekable()` 활용하기
루프를 도는 중에 "다음 값에 따라 현재 처리를 결정"해야 하는 상황이 있습니다 (예: LL 파서 구현). 이때 `.peekable()`을 사용하면 현재 아이템을 소비하지 않고 다음 아이템을 미리 엿볼 수 있습니다.

```rust
let mut iter = vec![1, 2, 3].into_iter().peekable();

if let Some(&next) = iter.peek() {
    println!("다음 값은 {next}입니다. 하지만 아직 꺼내지 않았습니다.");
}
let first = iter.next(); // 여기서 비로소 1이 추출됩니다.
```

---

### 📌 요약
- **`enumerate()`**와 **`zip()`**은 인덱스와 병렬 처리를 안전하게 만듭니다.
- **`windows()`**와 **`chunks()`**는 시계열 분석이나 패킷 처리에 필수적입니다.
- **`fold()`**와 **`scan()`**은 함수형 스타일의 누적 연산을 가능케 합니다.
- 복잡한 루프를 만났을 때 바로 `for`를 쓰기보다 적절한 반복자 어댑터가 없는지 먼저 고민해 보세요.

