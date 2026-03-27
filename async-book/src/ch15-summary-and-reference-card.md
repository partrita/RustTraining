# 요약 및 참조 카드 (Summary and Reference Card)

## 빠른 참조 카드 (Quick Reference Card)

### 비동기 멘탈 모델

```text
┌─────────────────────────────────────────────────────┐
│  async fn → 상태 머신 (열거형) → impl Future        │
│  .await   → 내부 퓨처를 poll() 함                   │
│  실행기   → loop { poll(); 깨어날 때까지 잠듦; }    │
│  웨이커   → "이봐 실행기, 나를 다시 폴링해줘"       │
│  Pin      → "메모리에서 이동하지 않겠다고 약속함"   │
└─────────────────────────────────────────────────────┘
```

### 흔히 사용되는 패턴 치트 시트

| 목표 | 방법 |
|------|-----|
| 두 퓨처를 동시에 실행 | `tokio::join!(a, b)` |
| 두 퓨처 경합 처리 | `tokio::select! { ... }` |
| 백그라운드 태스크 스폰 | `tokio::spawn(async { ... })` |
| 비동기에서 블로킹 코드 실행 | `tokio::task::spawn_blocking(\\|\\| { ... })` |
| 동시성 제한 | `Semaphore::new(N)` |
| 수많은 태스크 결과 수집 | `JoinSet` |
| 태스크 간 상태 공유 | `Arc<Mutex<T>>` 또는 채널 |
| 우아한 종료 | `watch::channel` + `select!` |
| 스트림을 N개씩 동시에 처리 | `.buffer_unordered(N)` |
| 퓨처에 타임아웃 적용 | `tokio::time::timeout(dur, fut)` |
| 백오프와 함께 재시도 | 커스텀 결합기 (13장 참조) |

### 피닝(Pinning) 빠른 참조

| 상황 | 방법 |
|-----------|-----|
| 퓨처를 힙에 고정 | `Box::pin(fut)` |
| 퓨처를 스택에 고정 | `tokio::pin!(fut)` |
| `Unpin` 타입을 고정 | `Pin::new(&mut val)` — 안전하고 비용 없음 |
| 고정된 트레이트 객체 반환 | `-> Pin<Box<dyn Future<Output = T> + Send>>` |

### 채널 선택 가이드

| 채널 | 생산자 | 소비자 | 값 | 사용 시점 |
|---------|-----------|-----------|--------|----------|
| `mpsc` | N | 1 | 스트림 | 작업 큐, 이벤트 버스 |
| `oneshot` | 1 | 1 | 단일 값 | 요청/응답, 완료 알림 |
| `broadcast` | N | N | 모두 수신 | 알림 전파(Fan-out), 종료 시그널 |
| `watch` | 1 | N | 최신 값만 | 설정 업데이트, 상태 확인 |

### 뮤텍스(Mutex) 선택 가이드

| 뮤텍스 | 사용 시점 |
|-------|----------|
| `std::sync::Mutex` | 짧게 유지되는 락, `.await`를 가로지르지 않을 때 |
| `tokio::sync::Mutex` | `.await` 지점을 가로질러 락을 유지해야 할 때 |
| `parking_lot::Mutex` | 경합이 심하고 `.await`가 없으며 성능이 중요할 때 |
| `tokio::sync::RwLock` | 읽기 주체가 많고 쓰기가 적으며, 락이 `.await`를 가로지를 때 |

### 의사 결정 빠른 참조

```text
동시성(concurrency)이 필요한가요?
├── I/O 바운드 → async/await
├── CPU 바운드 → rayon / std::thread
└── 혼합됨 → CPU 부분에 spawn_blocking 사용

런타임을 선택하시나요?
├── 서버 앱 → tokio
├── 라이브러리 → 런타임 중립적 (futures 크레이트)
├── 임베디드 → embassy
└── 최소 기능 → smol

동시 실행 퓨처가 필요한가요?
├── 'static + Send 가능 → tokio::spawn
├── 'static + !Send 가능 → LocalSet
├── 'static 불가능 → FuturesUnordered
└── 추적/중단 필요 → JoinSet
```

### 흔한 에러 메시지 및 해결 방법

| 에러 | 원인 | 해결 방법 |
|-------|-------|-----|
| `future is not Send` | `.await`를 가로질러 `!Send` 타입 보유 | 해당 값을 `.await` 전에 드롭되도록 범위를 좁히거나, `current_thread` 런타임 사용 |
| `borrowed value does not live long enough` (spawn 시) | `tokio::spawn`은 `'static`을 요구함 | `Arc`, `clone()`을 사용하거나 `FuturesUnordered` 고려 |
| `the trait Future is not implemented for ()` | `.await` 누락 | 비동기 호출에 `.await` 추가 |
| `cannot borrow as mutable` (poll 내에서) | 자기 참조 빌려오기 | `Pin<&mut Self>`를 올바르게 사용 (4장 참조) |
| 프로그램이 조용히 멈춤 | `waker.wake()` 호출 잊음 | 모든 `Pending` 경로에서 웨이커를 등록하고 트리거하는지 확인 |

### 더 읽을거리

| 리소스 | 이유 |
|----------|-----|
| [Tokio 튜토리얼](https://tokio.rs/tokio/tutorial) | 공식 실습 가이드 — 첫 프로젝트에 최적 |
| [비동기 북 (공식)](https://rust-lang.github.io/async-book/) | 언어 수준에서의 `Future`, `Pin`, `Stream` 설명 |
| [Jon Gjengset — Crust of Rust: async/await](https://www.youtube.com/watch?v=ThjvMReOXYM) | 라이브 코딩과 함께하는 2시간짜리 내부 구조 심층 분석 |
| [Alice Ryhl — Actors with Tokio](https://ryhl.io/blog/actors-with-tokio/) | 상태 저장 서비스를 위한 운영 아키텍처 패턴 |
| [Without Boats — Pin, Unpin, and why Rust needs them](https://without.boats/blog/pin/) | 언어 설계자가 밝히는 원래의 동기 |
| [Tokio mini-Redis](https://github.com/tokio-rs/mini-redis) | 완전한 비동기 Rust 프로젝트 — 학습하기 좋은 운영 수준 코드 |
| [Tower 문서](https://docs.rs/tower) | axum, tonic, hyper에서 사용되는 미들웨어/서비스 아키텍처 |

***

*비동기 Rust 교육 가이드 끝*
