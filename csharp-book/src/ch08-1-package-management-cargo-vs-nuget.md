# 패키지 관리: Cargo vs NuGet

> **학습 목표:** Rust의 `Cargo.toml`과 C#의 `.csproj`를 비교하며 의존성을 관리하는 방법을 배웁니다. 버전 표기법의 차이, 재현 가능한 빌드를 위한 `Cargo.lock` 활용법, 그리고 Rust만의 독특한 기능인 **피처(Features)** 기반의 조건부 컴파일을 익힙니다.

---

### 1. 의존성 선언: `Cargo.toml` vs `.csproj`

| **특징** | **C# (.csproj / XML)** | **Rust (Cargo.toml / TOML)** |
| :--- | :--- | :--- |
| **선언 방식** | `<PackageReference ... />` | `[dependencies]` 아래에 작성 |
| **패키지 소스** | NuGet.org | Crates.io |
| **로컬 프로젝트** | `<ProjectReference ... />` | `my_lib = { path = "../my_lib" }` |
| **개발용 의존성** | 별도 구분 없음 (보통 `PrivateAssets`) | `[dev-dependencies]` (테스트/벤치 전용) |

```toml
# [Rust 의존성 예시]
[dependencies]
serde = { version = "1.0", features = ["derive"] } # 피처 활성화
tokio = { version = "1.0", features = ["full"] }   # 비동기 런타임
```

---

### 2. 버전 관리와 잠금 파일
Rust는 **시맨틱 버저닝(SemVer)**을 엄격히 따릅니다.

- **`Cargo.toml`**: "어떤 버전 범위와 호환되는가?"를 정의합니다. (예: `1.0`은 1.x.x 대와 호환)
- **`Cargo.lock`**: "현재 빌드에 사용된 정확한 버전은 무엇인가?"를 기록합니다. C#의 `packages.lock.json`과 같은 역할을 하며, 협업 시 빌드 결과의 일관성을 보장합니다.

---

### 3. 피처(Features): 필요한 기능만 골라 쓰기
Rust 패키지 관리의 백미입니다. 라이브러리가 제공하는 수많은 기능 중 **필요한 것만 컴파일**하도록 선택할 수 있어, 바이너리 크기를 줄이고 빌드 속도를 높일 수 있습니다.

```rust
// [조건부 컴파일 예시]
#[cfg(feature = "json")]
fn process() { /* JSON 처리 로직 */ }

#[cfg(not(feature = "json"))]
fn process() { /* 기본 처리 로직 */ }
```

---

### 4. 주요 명령어 비교

| **작업** | **C# (`dotnet`)** | **Rust (`cargo`)** |
| :--- | :--- | :--- |
| **패키지 추가** | `dotnet add package ...` | `cargo add ...` |
| **빌드 및 실행** | `dotnet run` | `cargo run` |
| **테스트 실행** | `dotnet test` | `cargo test` |
| **의존성 트리** | `dotnet list package` | `cargo tree` |
| **사용하지 않는 파일 정리** | `dotnet clean` | `cargo clean` |

---

### 💡 실무 팁: `cargo-edit`과 `cargo-audit`
- **`cargo add`**: 예전에는 직접 TOML을 고쳐야 했지만, 이제는 `dotnet add`처럼 명령어로 패키지를 추가할 수 있습니다.
- **`cargo audit`**: 프로젝트에 포함된 의존성 중 보안 취약점이 있는 패키지를 검사해 주는 도구입니다. 운영 환경 배포 전 필수 코스입니다.

