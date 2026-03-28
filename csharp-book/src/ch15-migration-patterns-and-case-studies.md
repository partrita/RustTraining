# 마이그레이션 패턴과 사례 연구: C#에서 Rust로

> **학습 목표:** C#에서 익숙하게 사용하던 설계 패턴들을 Rust에서는 어떻게 구현하는지 실전 예제를 통해 배웁니다. 저장소(Repository), 빌더(Builder), 의존성 주입(DI) 등 주요 패턴의 변환 과정을 살펴보고, 실제 마이그레이션 성공 사례를 통해 성능 이득을 확인합니다.

---

### 1. 주요 설계 패턴의 Rust식 변환

#### 저장소 패턴 (Repository Pattern)
C#의 `interface`는 Rust의 `trait`로 변환됩니다. 비동기 메서드가 필요한 경우 `#[async_trait]` 매크로를 사용합니다.

- **C#**: `Task<T> GetByIdAsync(int id)`
- **Rust**: `async fn get_by_id(&self, id: u64) -> Result<Option<T>, Error>` (널 대신 `Option`)

#### 빌더 패턴 (Builder Pattern)
복잡한 객체 생성 시 Rust에서는 구조체와 소유권을 활용한 빌더 패턴을 즐겨 씁니다. 마지막에 `build()`를 호출할 때 설정을 검증하고 실제 객체를 반환합니다.

```rust
let client = HttpClient::builder()
    .timeout(Duration::from_secs(30))
    .base_url("https://api.example.com")
    .build()?; // 설정 오류 시 Result 반환
```

#### 의존성 주입 (Dependency Injection)
번거로운 DI 컨테이너 없이도, 트레이트와 제네릭을 이용한 **생성자 주입**만으로도 충분히 유연한 설계를 할 수 있습니다.

---

### 2. 마이그레이션 사례 연구

#### 사례 1: 대용량 데이터 처리 (CSV 분석 도구)
- **배경**: C# 기반 툴이 500MB 파일 처리 시 메모리 4GB 점유 및 GC 스파이크 발생.
- **결과**: Rust로 교체 후 **메모리 12MB(스트리밍 처리), 속도 15배 향상**.
- **교훈**: Rust의 소유권 모델이 자연스럽게 메모리 효율적인 스트리밍 설계를 유도했습니다.

#### 사례 2: 마이크로서비스 (인증 게이트웨이)
- **배경**: 고부하 상황에서 ASP.NET Core 서비스의 p99 지연 시간이 200ms까지 튀는 현상 발생.
- **결과**: Rust(Axum)로 교체 후 **p99 지연 시간 4ms로 안정화, 메모리 95% 절감**.
- **교훈**: 가비지 컬렉터(GC)가 없으므로 꼬리 지연 시간(Tail Latency)을 매우 일정하게 유지할 수 있었습니다.

---

### 💡 실무 팁: '전부 교체'할 필요는 없습니다
성능이 정말로 중요한 **핫 패스(Hot path)**나, 메모리 안정성이 극도로 요구되는 모듈부터 부분적으로 마이그레이션하세요. C#과 Rust는 gRPC나 FFI를 통해 얼마든지 훌륭하게 협업할 수 있습니다.

