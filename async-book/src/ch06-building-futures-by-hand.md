# 6. 수동으로 Future 구현하기 🟡

> **학습 목표:**
> - 스레드를 활용한 깨움(Waking) 로직을 포함한 **`TimerFuture`**를 직접 구현해 봅니다.
> - 두 퓨처를 동시에 실행하는 **`Join`** 결합기(Combinator)의 원리를 이해합니다.
> - 먼저 끝나는 작업을 선택하는 **`Select`** 결합기를 구축합니다.
> - 여러 퓨처를 조합하여 더 복잡한 비동기 흐름을 설계하는 방식을 익힙니다.

---

### 타이머 퓨처 (Timer Future) 만들기
이론을 넘어, 실제로 유용한 퓨처를 밑바닥부터 만들어 보겠습니다. 이를 통해 퓨처와 실행기의 계약 관계를 확실히 이해할 수 있습니다.

#### 실습: `TimerFuture` 구현
지정된 시간이 지나면 완료되는 단순한 타이머입니다.

```rust
pub struct TimerFuture {
    shared_state: Arc<Mutex<SharedState>>,
}

struct SharedState {
    completed: bool,
    waker: Option<Waker>,
}

impl TimerFuture {
    pub fn new(duration: Duration) -> Self {
        let shared_state = Arc::new(Mutex::new(SharedState {
            completed: false,
            waker: None,
        }));

        let thread_shared_state = Arc::clone(&shared_state);
        // 백그라운드 스레드에서 타이머를 돌립니다.
        thread::spawn(move || {
            thread::sleep(duration);
            let mut state = thread_shared_state.lock().unwrap();
            state.completed = true;
            // 중요: 작업이 끝났음을 실행기에 알립니다!
            if let Some(waker) = state.waker.take() {
                waker.wake();
            }
        });

        TimerFuture { shared_state }
    }
}
```
*실전에서는 타이머마다 스레드를 만드는 대신, 효율적인 타이머 휠(Timer Wheel)을 사용하는 `tokio::time::sleep`을 씁니다.*

---

### Join: 두 작업을 동시에!
`Join`은 감싸고 있는 모든 퓨처가 완료될 때까지 기다리는 결합기입니다. `tokio::join!`의 동작 원리를 엿볼 수 있습니다.

```rust
impl<A, B> Future for Join<A, B> {
    type Output = (A::Output, B::Output);

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // A와 B를 차례로 폴링하여 결과가 나왔는지 확인합니다.
        // 둘 다 Ready가 되면 Poll::Ready((a_res, b_res))를 반환합니다.
        // 하나라도 Pending이면 Poll::Pending을 반환합니다.
    }
}
```
**핵심**: `Join`은 별도의 스레드를 만들지 않습니다. 한 번의 `poll` 호출 안에서 여러 퓨처를 번갈아 확인하는 **협력적 동시성**의 정수를 보여줍니다.

---

### Select: 먼저 끝나는게 임자
`Select`는 여러 작업 중 하나라도 완료되면 즉시 결과를 반환하고 나머지는 취소(Drop)합니다.

#### 💡 사용 예시: 타임아웃 처리
```rust
// 요청이 5초 안에 안 오면 "Timeout" 출력
match select(http_get(url), timer(5sec)).await {
    Either::Left(res) => println!("성공: {res}"),
    Either::Right(_) => println!("타임아웃 발생!"),
}
```

---

### 💡 실무 팁: 결합기(Combinator)의 위력
Rust의 비동기는 작은 퓨처들을 조립하여 큰 퓨처를 만드는 '레고 블록'과 같습니다. 직접 `Future` 트레이트를 구현하기보다는, `join`, `select`, `then` 같은 기존 결합기를 조합해 사용하는 것이 훨씬 안전하고 유지보수하기 좋습니다.

---

### 🏋️ 연습 문제: RetryFuture 설계하기
**도전 과제:** 특정 작업을 최대 N번까지 재시도하는 `RetryFuture`를 설계해 보세요. 실패할 경우 지정된 횟수만큼 다시 퓨처를 생성하고 실행해야 합니다.

<details>
<summary>🔑 정답 및 힌트 보기</summary>
재시도 횟수를 저장하는 카운터와 현재 실행 중인 퓨처를 들고 있는 상태 머신이 필요합니다. `poll` 내부에서 결과가 `Err`일 때 카운터를 줄이고 새로운 퓨처를 만들어 `poll`을 다시 시도하는 로직을 구현하면 됩니다. 
이처럼 퓨처 내부에 다른 퓨처를 동적으로 갈아 끼우는 것이 결합기 설계의 핵심입니다.
</details>

---

### 📌 요약
- 퓨처 구현에는 **상태 관리**, **`poll` 로직**, **웨이커 등록**이 필수입니다.
- `Join`과 `Select`는 여러 작업을 제어하는 가장 기본적인 결합기입니다.
- 비동기 Rust는 "퓨처들의 거대한 조립체"로 이해할 수 있습니다.

