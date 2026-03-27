## 비동기 프로그래밍: C# Task vs Rust Future

> **학습 내용:** Rust의 지연 실행(lazy) `Future`와 C#의 즉시 실행(eager) `Task` 비교, 실행기(executor) 모델(tokio), 
> `Drop` + `select!`를 통한 취소 vs `CancellationToken`, 그리고 동시 요청을 위한 실전 패턴.
>
> **난이도:** 🔴 상급

C# 개발자들에게 `async`/`await`는 매우 익숙한 개념입니다. Rust도 동일한 키워드를 사용하지만, 실행 모델은 근본적으로 다릅니다.

### 실행기 모델 (The Executor Model)

```csharp
// C# — 런타임이 내장된 스레드 풀과 작업 스케줄러를 제공합니다.
// async/await는 별도의 설정 없이도 "그냥 작동"합니다.
public async Task<string> FetchDataAsync(string url)
{
    using var client = new HttpClient();
    return await client.GetStringAsync(url);  // .NET 스레드 풀에 의해 스케줄링됨
}
// .NET이 스레드 풀, 작업 스케줄링, 동기화 컨텍스트(synchronization context)를 관리합니다.
```

```rust
// Rust — 내장된 비동기 런타임이 없습니다. 직접 실행기(executor)를 선택해야 합니다.
// 가장 대중적인 것은 tokio입니다.
async fn fetch_data(url: &str) -> Result<String, reqwest::Error> {
    let body = reqwest::get(url).await?.text().await?;
    Ok(body)
}

// 비동기 코드를 실행하려면 반드시 런타임이 필요합니다:
#[tokio::main]  // 이 매크로는 tokio 런타임을 설정합니다.
async fn main() {
    let data = fetch_data("https://example.com").await.unwrap();
    println!("{}", &data[..100]);
}
```

### Future vs Task

| | C# `Task<T>` | Rust `Future<Output = T>` |
|---|---|---|
| **실행 방식** | 생성 즉시 시작됨 (Eager) | **지연 실행 (Lazy)** — `.await` 호출 전까지 아무것도 하지 않음 |
| **런타임** | 내장됨 (CLR 스레드 풀) | 외부 라이브러리 (tokio, async-std 등) |
| **취소** | `CancellationToken` 사용 | `Future`를 드롭(Drop)하거나 `tokio::select!` 사용 |
| **상태 머신** | 컴파일러가 생성 | 컴파일러가 생성 |
| **할당** | 힙(Heap) 할당 | Box로 감싸기 전까지는 스택(Stack) 할당 |

```rust
// 중요: Rust에서 Future는 지연 실행(lazy)됩니다!
async fn compute() -> i32 { println!("Computing!"); 42 }

let future = compute();  // 아무것도 출력되지 않습니다! Future가 아직 폴링(poll)되지 않았기 때문입니다.
let result = future.await; // 이제서야 "Computing!"이 출력됩니다.
```

```csharp
// C# Task는 즉시 시작됩니다!
var task = ComputeAsync();  // 즉시 "Computing!"이 출력됨
var result = await task;    // 완료될 때까지 기다릴 뿐입니다.
```

### 취소: CancellationToken vs Drop / select!

```csharp
// C# — CancellationToken을 사용한 협력적 취소(Cooperative cancellation)
public async Task ProcessAsync(CancellationToken ct)
{
    while (!ct.IsCancellationRequested)
    {
        await Task.Delay(1000, ct);  // 취소 시 예외 발생
        DoWork();
    }
}

var cts = new CancellationTokenSource(TimeSpan.FromSeconds(5));
await ProcessAsync(cts.Token);
```

```rust
// Rust — Future를 드롭하거나 tokio::select!를 사용하여 취소
use tokio::time::{sleep, Duration};

async fn process() {
    loop {
        sleep(Duration::from_secs(1)).await;
        do_work();
    }
}

// select!를 사용한 타임아웃 패턴
async fn run_with_timeout() {
    tokio::select! {
        _ = process() => { println!("Completed"); }
        _ = sleep(Duration::from_secs(5)) => { println!("Timed out!"); }
    }
    // select!가 타임아웃 분기를 선택하면, process() Future는 드롭(DROP)됩니다.
    // — 별도의 CancellationToken 없이도 자동 정리(cleanup)가 이루어집니다.
}
```

### 실전 패턴: 타임아웃이 포함된 동시 요청

```csharp
// C# — 타임아웃이 포함된 동시 HTTP 요청
public async Task<string[]> FetchAllAsync(string[] urls, CancellationToken ct)
{
    var tasks = urls.Select(url => httpClient.GetStringAsync(url, ct));
    return await Task.WhenAll(tasks);
}
```

```rust
// Rust — tokio::join! 또는 futures::join_all을 사용한 동시 요청
use futures::future::join_all;

async fn fetch_all(urls: &[&str]) -> Vec<Result<String, reqwest::Error>> {
    let futures = urls.iter().map(|url| reqwest::get(*url));
    let responses = join_all(futures).await;

    let mut results = Vec::new();
    for resp in responses {
        results.push(resp?.text().await);
    }
    results
}

// 타임아웃 포함:
async fn fetch_all_with_timeout(urls: &[&str]) -> Result<Vec<String>, &'static str> {
    tokio::time::timeout(
        Duration::from_secs(10),
        async {
            let futures: Vec<_> = urls.iter()
                .map(|url| async { reqwest::get(*url).await?.text().await })
                .collect();
            let results = join_all(futures).await;
            results.into_iter().collect::<Result<Vec<_>, _>>()
        }
    )
    .await
    .map_err(|_| "Request timed out")?
    .map_err(|_| "Request failed")
}
```

<details>
<summary><strong>🏋️ 연습 문제: 비동기 타임아웃 패턴</strong> (클릭하여 펼치기)</summary>

**도전 과제**: 두 개의 URL에서 동시에 데이터를 가져와서, 먼저 응답하는 쪽을 반환하고 다른 쪽은 취소하는 비동기 함수를 작성해 보세요. (C#의 `Task.WhenAny`와 유사합니다.)

<details>
<summary>🔑 정답</summary>

```rust
use tokio::time::{sleep, Duration};

// 시뮬레이션된 비동기 페치(fetch) 함수
async fn fetch(url: &str, delay_ms: u64) -> String {
    sleep(Duration::from_millis(delay_ms)).await;
    format!("Response from {url}")
}

async fn fetch_first(url1: &str, url2: &str) -> String {
    tokio::select! {
        result = fetch(url1, 200) => {
            println!("URL 1 won");
            result
        }
        result = fetch(url2, 500) => {
            println!("URL 2 won");
            result
        }
    }
    // 선택되지 않은 분기의 Future는 자동으로 드롭(취소)됩니다.
}

#[tokio::main]
async fn main() {
    let result = fetch_first("https://fast.api", "https://slow.api").await;
    println!("{result}");
}
```

**핵심 요점**: `tokio::select!`는 C#의 `Task.WhenAny`에 해당하는 Rust의 기능입니다. 여러 Future를 동시에 실행하여 가장 먼저 완료되는 것을 취하고 나머지는 드롭(취소)합니다.

</details>
</details>

### `tokio::spawn`을 사용하여 독립적인 작업 실행하기

C#에서 `Task.Run`은 호출자와 독립적으로 실행되는 작업을 시작합니다. Rust에서 이에 해당하는 것은 `tokio::spawn`입니다.

```rust
use tokio::task;

async fn background_work() {
    // 독립적으로 실행됩니다 — 호출자의 Future가 드롭되어도 계속 실행됩니다.
    let handle = task::spawn(async {
        tokio::time::sleep(Duration::from_secs(2)).await;
        42
    });

    // 백그라운드 작업이 실행되는 동안 다른 작업을 수행합니다...
    println!("Doing other work");

    // 결과가 필요할 때 await합니다.
    let result = handle.await.unwrap(); // 42
}
```

```csharp
// C#의 해당 코드
var task = Task.Run(async () => {
    await Task.Delay(2000);
    return 42;
});
// 다른 작업 수행...
var result = await task;
```

**핵심 차이점**: 일반적인 `async {}` 블록은 지연 실행(lazy)되므로 await하기 전까지 아무것도 하지 않습니다. 반면 `tokio::spawn`은 C#의 `Task.Run`처럼 즉시 런타임에서 실행을 시작합니다.

### Pin: C#에는 없는 Rust 비동기의 개념

C# 개발자들은 `Pin`을 접할 일이 없습니다. CLR의 가비지 컬렉터(GC)가 객체를 자유롭게 이동시키고 모든 참조를 자동으로 업데이트하기 때문입니다. 하지만 Rust에는 GC가 없습니다. 컴파일러가 `async fn`을 상태 머신으로 변환할 때, 해당 구조체는 자신의 필드를 가리키는 내부 포인터를 포함할 수 있습니다. 이때 구조체가 메모리에서 이동하면 해당 포인터들은 무효화됩니다.

`Pin<T>`는 **"이 값은 메모리에서 이동하지 않는다"**는 것을 보장하는 래퍼(wrapper)입니다.

```rust
// 다음과 같은 상황에서 Pin을 보게 될 것입니다:
trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
    //           ^^^^^^^^^^^^^^ 핀 고정됨 — 내부 참조가 유효하게 유지됨
}

// 트레이트에서 박싱된(boxed) Future를 반환할 때:
fn make_future() -> Pin<Box<dyn Future<Output = i32> + Send>> {
    Box::pin(async { 42 })
}
```

**실무에서는 직접 `Pin`을 작성하는 경우가 거의 없습니다.** `async fn`과 `.await` 문법이 이를 내부적으로 처리해 주기 때문입니다. 다음과 같은 경우에만 접하게 됩니다:
- 컴파일러 에러 메시지 (제시된 해결책을 따르면 됩니다)
- `tokio::select!` 사용 시 (`pin!()` 매크로 사용)
- `dyn Future`를 반환하는 트레이트 메서드 (`Box::pin(async { ... })` 사용)

> **더 자세히 알고 싶으신가요?** [Async Rust 트레이닝](../../async-book/src/ch04-pin-and-unpin.md)에서 Pin, Unpin, 자기 참조 구조체(self-referential structs), 그리고 구조적 핀 고정(structural pinning)에 대해 상세히 다룹니다.

***
