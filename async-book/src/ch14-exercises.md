## 연습 문제

### 연습 문제 1: 비동기 에코 서버 (Async Echo Server)

여러 클라이언트를 동시에 처리할 수 있는 TCP 에코 서버를 구축하세요.

**요구 사항**:
- `127.0.0.1:8080` 포트에서 리슨(Listen)
- 연결을 수락하고 수신된 각 라인을 그대로 다시 전송(Echo)
- 클라이언트의 연결 종료를 우아하게 처리
- 클라이언트가 연결되거나 연결을 끊을 때 로그 출력

<details>
<summary>🔑 정답</summary>

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("에코 서버가 :8080 포트에서 실행 중입니다.");

    loop {
        let (socket, addr) = listener.accept().await?;
        println!("[{addr}] 연결됨");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.into_split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                match reader.read_line(&mut line).await {
                    Ok(0) => {
                        println!("[{addr}] 연결 종료됨");
                        break;
                    }
                    Ok(_) => {
                        print!("[{addr}] 에코: {line}");
                        if writer.write_all(line.as_bytes()).await.is_err() {
                            println!("[{addr}] 쓰기 에러, 연결을 종료합니다.");
                            break;
                        }
                    }
                    Err(e) => {
                        eprintln!("[{addr}] 읽기 에러: {e}");
                        break;
                    }
                }
            }
        });
    }
}
```

</details>

---

### 연습 문제 2: 속도 제한이 있는 동시 URL 페처 (Concurrent URL Fetcher)

URL 리스트를 동시에 페치하되, 최대 5개의 요청만 동시에 수행되도록 하세요.

<details>
<summary>🔑 정답</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

async fn fetch_urls(urls: Vec<String>) -> Vec<Result<String, String>> {
    // buffer_unordered(5)는 최대 5개의 퓨처가 동시에 폴링되도록 보장합니다.
    // 여기서는 별도의 세마포어(Semaphore)가 필요하지 않습니다.
    let results: Vec<_> = stream::iter(urls)
        .map(|url| {
            async move {
                println!("페치 중: {url}");

                match reqwest::get(&url).await {
                    Ok(resp) => match resp.text().await {
                        Ok(body) => Ok(body),
                        Err(e) => Err(format!("{url}: {e}")),
                    },
                    Err(e) => Err(format!("{url}: {e}")),
                }
            }
        })
        .buffer_unordered(5) // ← 이것만으로 동시성을 5로 제한합니다.
        .collect()
        .await;

    results
}

// 참고: 독립적으로 스폰된 태스크(tokio::spawn) 간의 동시성을 제한해야 할 때는
// Semaphore를 사용하세요. 스트림을 처리할 때는 buffer_unordered를 사용하세요.
// 동일한 제한을 위해 두 방식을 혼용하지 마세요.
```

</details>

---

### 연습 문제 3: 워커 풀을 이용한 우아한 종료

다음 기능을 갖춘 태스크 프로세서를 구축하세요:
- 채널 기반의 작업 큐
- 큐에서 작업을 가져와 소비하는 N개의 워커 태스크
- Ctrl+C 발생 시 우아한 종료: 새로운 작업 수락 중단, 진행 중인 작업 마무리

<details>
<summary>🔑 정답</summary>

```rust
use tokio::sync::{mpsc, watch};
use tokio::time::{sleep, Duration};

struct WorkItem {
    id: u64,
    payload: String,
}

#[tokio::main]
async fn main() {
    let (work_tx, work_rx) = mpsc::channel::<WorkItem>(100);
    let (shutdown_tx, shutdown_rx) = watch::channel(false);

    // 4개의 워커 스폰
    let mut worker_handles = Vec::new();
    let work_rx = std::sync::Arc::new(tokio::sync::Mutex::new(work_rx));

    for id in 0..4 {
        let rx = work_rx.clone();
        let mut shutdown = shutdown_rx.clone();
        let handle = tokio::spawn(async move {
            loop {
                let item = {
                    let mut rx = rx.lock().await;
                    tokio::select! {
                        item = rx.recv() => item,
                        _ = shutdown.changed() => {
                            if *shutdown.borrow() { None } else { continue }
                        }
                    }
                };

                match item {
                    Some(work) => {
                        println!("워커 {id}: 아이템 {} 처리 중", work.id);
                        sleep(Duration::from_millis(200)).await; // 작업 시뮬레이션
                        println!("워커 {id}: 아이템 {} 처리 완료", work.id);
                    }
                    None => {
                        println!("워커 {id}: 채널 닫힘, 종료합니다.");
                        break;
                    }
                }
            }
        });
        worker_handles.push(handle);
    }

    // 생산자: 일부 작업 제출
    let producer = tokio::spawn(async move {
        for i in 0..20 {
            let _ = work_tx.send(WorkItem {
                id: i,
                payload: format!("task-{i}"),
            }).await;
            sleep(Duration::from_millis(50)).await;
        }
    });

    // Ctrl+C 대기
    tokio::signal::ctrl_c().await.unwrap();
    println!("\n종료 시그널 수신!");
    shutdown_tx.send(true).unwrap();
    producer.abort(); // 생산자 태스크 취소

    // 워커들이 끝날 때까지 대기
    for handle in worker_handles {
        let _ = handle.await;
    }
    println!("모든 워커가 종료되었습니다. 안녕히 가세요!");
}
```

</details>

---

### 연습 문제 4: 기초부터 시작하는 간단한 비동기 Mutex 구축

채널을 사용하여 비동기를 인식하는 뮤텍스를 구현하세요 (`tokio::sync::Mutex`를 사용하지 마세요).

*힌트*: 용량이 1인 `tokio::sync::mpsc` 채널을 세마포어처럼 사용하세요.

<details>
<summary>🔑 정답</summary>

```rust
use std::cell::UnsafeCell;
use std::sync::Arc;
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

pub struct SimpleAsyncMutex<T> {
    data: Arc<UnsafeCell<T>>,
    semaphore: Arc<Semaphore>,
}

// 안전성: T에 대한 접근은 세마포어에 의해 직렬화됩니다 (최대 1개 허가).
unsafe impl<T: Send> Send for SimpleAsyncMutex<T> {}
unsafe impl<T: Send> Sync for SimpleAsyncMutex<T> {}

pub struct SimpleGuard<T> {
    data: Arc<UnsafeCell<T>>,
    _permit: OwnedSemaphorePermit, // 가드가 드롭될 때 함께 드롭됨 → 락 해제
}

impl<T> SimpleAsyncMutex<T> {
    pub fn new(value: T) -> Self {
        SimpleAsyncMutex {
            data: Arc::new(UnsafeCell::new(value)),
            semaphore: Arc::new(Semaphore::new(1)),
        }
    }

    pub async fn lock(&self) -> SimpleGuard<T> {
        let permit = self.semaphore.clone().acquire_owned().await.unwrap();
        SimpleGuard {
            data: self.data.clone(),
            _permit: permit,
        }
    }
}

impl<T> std::ops::Deref for SimpleGuard<T> {
    type Target = T;
    fn deref(&self) -> &T {
        // 안전성: 우리는 유일한 세마포어 허가권을 가지고 있으므로,
        // 다른 SimpleGuard는 존재하지 않으며 배타적 접근이 보장됩니다.
        unsafe { &*self.data.get() }
    }
}

impl<T> std::ops::DerefMut for SimpleGuard<T> {
    fn deref_mut(&mut self) -> &mut T {
        // 안전성: 위와 동일한 이유 — 단일 허가권이 배타성을 보장합니다.
        unsafe { &mut *self.data.get() }
    }
}

// SimpleGuard가 드롭되면 _permit이 드롭되고,
// 이는 세마포어 허가권을 해제합니다 — 다른 lock() 호출이 진행될 수 있게 됩니다.

// 사용 예시:
// let mutex = SimpleAsyncMutex::new(vec![1, 2, 3]);
// {
//     let mut guard = mutex.lock().await;
//     guard.push(4);
// } // 여기서 허가권 해제
```

**핵심 요약**: 비동기 뮤텍스는 보통 세마포어 위에 구축됩니다. 세마포어는 비동기 대기 메커니즘을 제공합니다. 락이 걸려 있을 때 `acquire()`는 허가권이 해제될 때까지 태스크를 중단시킵니다. 이것이 바로 `tokio::sync::Mutex`가 내부적으로 작동하는 방식입니다.

> **왜 `std::sync::Mutex`가 아닌 `UnsafeCell`인가요?** 이 연습 문제의 이전 버전에서는
> `Deref`/`DerefMut` 내에서 `.lock().unwrap()`을 호출하는 `Arc<Mutex<T>>`를 사용했습니다.
> 하지만 이는 컴파일되지 않습니다. 반환되는 `&T`가 즉시 드롭되는 임시 `MutexGuard`로부터
> 빌려오기 때문입니다. `UnsafeCell`은 중간 가드를 피하게 해주며, 세마포어 기반의 직렬화 덕분에
> `unsafe` 사용이 안전(sound)해집니다.

</details>

---

### 연습 문제 5: 스트림 파이프라인 (Stream Pipeline)

스트림을 사용하여 다음 데이터 처리 파이프라인을 구축하세요:
1. 1..=100 숫자 생성
2. 짝수만 필터링
3. 각 숫자를 제곱으로 변환
4. 한 번에 10개씩 동시에 처리 (sleep으로 시뮬레이션)
5. 결과 수집

<details>
<summary>🔑 정답</summary>

```rust
use futures::stream::{self, StreamExt};
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let results: Vec<u64> = stream::iter(1u64..=100)
        // 단계 2: 짝수 필터링
        .filter(|x| futures::future::ready(x % 2 == 0))
        // 단계 3: 각각 제곱
        .map(|x| x * x)
        // 단계 4: 동시에 처리 (비동기 작업 시뮬레이션)
        .map(|x| async move {
            sleep(Duration::from_millis(50)).await;
            println!("처리 완료: {x}");
            x
        })
        .buffer_unordered(10) // 10개 동시 처리
        // 단계 5: 수집
        .collect()
        .await;

    println!("총 {}개의 결과를 얻었습니다.", results.len());
    println!("합계: {}", results.iter().sum::<u64>());
}
```

</details>

---

### 연습 문제 6: 타임아웃이 있는 Select 구현하기

`tokio::select!`나 `tokio::time::timeout`을 사용하지 않고, 퓨처를 데드라인과 경합시켜 완료 시 `Either::Left(result)`를, 타임아웃 시 `Either::Right(())`를 반환하는 함수를 구현하세요.

*힌트*: 6장에서 만든 `Select` 결합기와 같은 장의 `TimerFuture`를 활용하세요.

<details>
<summary>🔑 정답</summary>

```rust,ignore
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::Duration;

pub enum Either<A, B> {
    Left(A),
    Right(B),
}

pub struct Timeout<F> {
    future: F,
    timer: TimerFuture, // 6장에서 구현한 것
}

impl<F: Future + Unpin> Timeout<F> {
    pub fn new(future: F, duration: Duration) -> Self {
        Timeout {
            future,
            timer: TimerFuture::new(duration),
        }
    }
}

impl<F: Future + Unpin> Future for Timeout<F> {
    type Output = Either<F::Output, ()>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // 메인 퓨처가 완료되었는지 확인
        if let Poll::Ready(val) = Pin::new(&mut self.future).poll(cx) {
            return Poll::Ready(Either::Left(val));
        }

        // 타이머가 만료되었는지 확인
        if let Poll::Ready(()) = Pin::new(&mut self.timer).poll(cx) {
            return Poll::Ready(Either::Right(()));
        }

        Poll::Pending
    }
}

// 사용 예시:
// match Timeout::new(fetch_data(), Duration::from_secs(5)).await {
//     Either::Left(data) => println!("데이터 수신: {data}"),
//     Either::Right(()) => println!("시간 초과!"),
// }
```

**핵심 요약**: `select`/`timeout`은 단지 두 개의 퓨처를 폴링하며 어느 것이 먼저 완료되는지 확인하는 것입니다. 전체 비동기 생태계는 poll, Pending/Ready, Waker라는 단순한 기본 요소로부터 구축됩니다.

</details>

***
