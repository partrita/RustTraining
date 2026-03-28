# 14. 실전 연습 문제 🟡

> **학습 목표:**
> - 지금까지 배운 비동기 Rust의 핵심 개념들을 실무적인 코드로 구현하며 복습합니다.
> - 에코 서버, 동시성 제어, 우아한 종료 등 필수 패턴을 직접 짜봅니다.
> - 퓨처와 스트림의 동작 원리를 응용한 심화 과제에 도전합니다.

---

### [연습 1] 비동기 에코 서버 (Echo Server)
여러 클라이언트의 연결을 동시에 처리할 수 있는 TCP 에코 서버를 구축하세요.

**요구 사항**:
- `127.0.0.1:8080` 포트에서 대기합니다.
- 클라이언트가 보낸 메시지(라인 단위)를 그대로 다시 보냅니다.
- 클라이언트의 연결과 종료를 로그로 출력합니다.

<details>
<summary>🔑 정답 및 힌트 보기</summary>

```rust
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("에코 서버 시작 (Port: 8080)");

    loop {
        let (mut socket, addr) = listener.accept().await?;
        println!("[{addr}] 연결됨");

        tokio::spawn(async move {
            let (reader, mut writer) = socket.split();
            let mut reader = BufReader::new(reader);
            let mut line = String::new();

            loop {
                line.clear();
                // 한 줄씩 읽어서 그대로 다시 쓰기
                if reader.read_line(&mut line).await.unwrap_or(0) == 0 { break; }
                writer.write_all(line.as_bytes()).await.unwrap();
            }
            println!("[{addr}] 연결 종료");
        });
    }
}
```
</details>

---

### [연습 2] 동시성 제한 URL 페처
리스트에 담긴 수십 개의 URL을 페치하되, 서버 부하를 방지하기 위해 **동시에 실행되는 요청은 최대 5개**로 제한하세요.

<details>
<summary>🔑 정답 및 힌트 보기</summary>

```rust
use futures::stream::{self, StreamExt};

async fn fetch_all(urls: Vec<String>) -> Vec<String> {
    stream::iter(urls)
        .map(|url| async move {
            // 실제 HTTP 요청 로직 (reqwest 등 사용)
            fetch_one(url).await
        })
        .buffer_unordered(5) // 핵심: 동시 실행 수를 5개로 제한
        .collect()
        .await
}
```
</details>

---

### [연습 3] 우아한 종료가 포함된 워커 풀
작업 큐(Channel)를 감시하는 4개의 워커 태스크를 만들고, `Ctrl+C` 입력 시 **현재 처리 중인 작업까지만 마무리하고 종료**되도록 구현하세요.

<details>
<summary>🔑 정답 및 힌트 보기</summary>
`tokio::sync::watch` 채널을 사용해 종료 신호를 모든 워커에게 전파하고, `tokio::select!`로 작업 수신과 종료 신호를 동시에 감시하도록 설계합니다. (자세한 코드는 13장 운영 패턴 참고)
</details>

---

### [연습 4] 나만의 비동기 Mutex 만들기
`tokio::sync::Mutex`를 사용하지 않고, 채널이나 세마포어를 활용해 간단한 비동기 뮤텍스를 직접 구현해 보세요. (Deref, DerefMut 트레이트 활용 권장)

---

### [연습 5] 스트림 파이프라인 구축
1부터 100까지의 숫자 스트림을 필터링(`x % 2 == 0`)하고, 제곱 연산을 수행한 뒤, 결과를 10개씩 묶어서 출력하는 파이프라인을 구축하세요.

---

### 📌 요약
- **`tokio::spawn`**은 동시성 구현의 기본입니다.
- **`buffer_unordered`**는 스트림 처리의 핵심 효율 도구입니다.
- **`select!`**와 **`watch`** 채널의 조합은 운영 환경 필수 패턴입니다.
- 직접 퓨처를 제어해보며 `poll`과 `Pending`의 감각을 익히는 것이 중요합니다.

