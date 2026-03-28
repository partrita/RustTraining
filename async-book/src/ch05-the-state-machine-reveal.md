# 5. 상태 머신의 실체: 컴파일러가 만드는 마법 🟢

> **학습 목표:**
> - 컴파일러가 순차적인 `async fn` 코드를 어떻게 **열거형(Enum) 기반의 상태 머신**으로 변환하는지 배웁니다.
> - 비동기 함수 내의 큰 변수 할당이 퓨처의 전체 크기에 미치는 영향을 이해합니다.
> - 상태 전이 과정에서 발생하는 **드롭(Drop) 최적화** 원리를 파악합니다.

---

### 컴파일러가 실제로 생성하는 것
우리가 작성한 `async fn`은 컴파일 타임에 복잡한 상태 머신 구조체로 재탄생합니다. 이 과정을 이해하면 비동기 Rust의 성능과 메모리 특성을 정확히 파악할 수 있습니다.

#### 코드 대조: `async fn` vs 상태 머신

```rust
// [우리가 작성한 코드]
async fn fetch_two_pages() -> String {
    let page1 = http_get("url_a").await;
    let page2 = http_get("url_b").await;
    format!("{page1}\n{page2}")
}
```

```rust
// [컴파일러가 개념적으로 생성하는 열거형]
enum FetchStateMachine {
    Start,
    
    // 첫 번째 페이지 응답을 기다리는 상태
    WaitingPage1 { 
        fut1: HttpGetFuture 
    },
    
    // 첫 번째 페이지를 받았고, 두 번째를 기다리는 상태
    WaitingPage2 { 
        page1: String, 
        fut2: HttpGetFuture 
    },
    
    Complete,
}
```

각 `.await` 지점은 상태 머신의 **중단 지점(Yield Point)**이 되며, 열거형의 새로운 변형(Variant)을 생성합니다.

---

### 성능과 메모리에 중요한 이유

#### ① 제로 비용 (Zero-cost)
이 상태 머신은 기본적으로 **스택**에 할당되는 열거형입니다. 별도의 힙 할당이나 가비지 컬렉터 없이, 일반적인 구조체와 똑같은 방식으로 메모리가 관리됩니다.

#### ② 퓨처의 크기 (Size)
열거형의 크기는 모든 상태 중 **가장 큰 상태**의 크기에 따라 결정됩니다.

```rust
async fn dangerous() {
    let buffer = [0u8; 1_000_000]; // 1MB 크기의 버퍼를 스택에 할당
    some_io().await; // 중단 지점 발생!
    process(buffer);
}
```
위와 같이 비동기 함수 내부에서 큰 배열을 스택에 할당하면, 해당 퓨처 객체 자체가 1MB가 넘는 거구가 됩니다. 이는 스택 오버플로의 원인이 될 수 있으므로, 큰 데이터는 **`Vec`**이나 **`Box`**를 써서 힙에 할당하는 것이 상책입니다.

#### ③ 드롭 최적화 (Drop Optimization)
상태가 전이될 때, 더 이상 필요 없는 데이터는 즉시 메모리에서 해제됩니다. 예를 들어 `WaitingPage2`로 넘어가면, 이미 완료된 `fut1`은 즉시 드롭되어 메모리를 효율적으로 사용합니다.

---

### 💡 실무 팁: 복잡한 퓨처는 `Box::pin` 하세요
만약 비동기 함수의 결과물(Future)이 너무 커서 전달하기 부담스럽다면, `Box::pin()`을 사용해 힙으로 옮기세요. 스택 공간을 절약하고 메모리 레이아웃을 더 안정적으로 관리할 수 있습니다.

---

### 🏋️ 연습 문제: 상태 머신 예측하기
**도전 과제:** 다음 함수에서 컴파일러가 만들어낼 상태는 총 몇 개일까요? 각 상태에는 어떤 값이 담길까요?

```rust
async fn pipeline(url: &str) -> Result<usize, Error> {
    let response = fetch(url).await?;
    let body = response.text().await?;
    let len = parse(body).await?;
    Ok(len)
}
```

<details>
<summary>🔑 정답 및 해설 보기</summary>
**정답:** 총 4가지 주요 상태가 생성됩니다.
1. **Start**: 초기 상태
2. **WaitingFetch**: `fetch` 결과를 기다림 (url 저장)
3. **WaitingText**: `text()` 결과를 기다림 (response 저장)
4. **WaitingParse**: `parse()` 결과를 기다림 (body 저장)

`.await`가 나타날 때마다 이전 상태의 결과물과 다음 작업을 위한 퓨처를 보관해야 하므로 새로운 상태가 추가됩니다.
</details>

---

### 📌 요약
- `async fn`은 각 `.await` 지점을 경계로 하는 **열거형 상태 머신**으로 변환됩니다.
- 퓨처의 크기는 내부에서 들고 있는 변수 중 가장 큰 것에 맞춰집니다. (큰 버퍼 주의!)
- 상태가 변할 때마다 컴파일러가 자동으로 메모리 해제(Drop) 코드를 삽입합니다.

