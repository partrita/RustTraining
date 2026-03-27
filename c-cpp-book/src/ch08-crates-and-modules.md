# Rust 크레이트(Crates) 및 모듈(Modules)

> **학습 내용:** Rust가 코드를 모듈과 크레이트로 조직화하는 방법을 배웁니다 — 기본적으로 비공개인 가시성, `pub` 수식어, 워크스페이스(workspaces), 그리고 `crates.io` 에코시스템을 다룹니다. C/C++의 헤더 파일, `#include`, CMake 의존성 관리를 대체하는 개념입니다.

- 모듈은 크레이트 내에서 코드를 조직화하는 기본 단위입니다.
    - 각 소스 파일(.rs)은 그 자체로 하나의 모듈이며, ```mod``` 키워드를 사용하여 중첩된 모듈을 생성할 수 있습니다.
    - (하위) 모듈의 모든 타입은 기본적으로 **비공개(private)**이며, 명시적으로 ```pub```(public)으로 표시하지 않는 한 동일한 크레이트 내에서도 외부에서 보이지 않습니다. ```pub```의 범위는 ```pub(crate)``` 등으로 더욱 제한될 수 있습니다.
    - 타입이 공개되어 있더라도 ```use``` 키워드를 사용하여 임포트하지 않으면 다른 모듈의 범위 내에서 자동으로 보이지 않습니다. 자식 하위 모듈은 ```use super::```를 사용하여 부모 범위의 타입을 참조할 수 있습니다.
    - 소스 파일(.rs)은 ```main.rs```(실행 파일) 또는 ```lib.rs```(라이브러리)에 명시적으로 나열되지 않으면 크레이트에 자동으로 포함되지 않습니다.

# 연습 문제: 모듈 및 함수
- [hello world](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=522d86dbb8c4af71ff2ec081fb76aee7) 예제를 수정하여 다른 함수를 호출해 보겠습니다.
    - 앞서 언급했듯이 함수는 ```fn``` 키워드로 정의됩니다. ```->``` 키워드는 함수가 ```u32```(부호 없는 32비트 정수) 타입의 값을 반환함을 선언합니다(기본값은 void입니다).
    - 함수는 모듈별로 범위가 지정되므로, 두 모듈에 이름이 똑같은 함수가 있어도 이름 충돌이 발생하지 않습니다.
        - 모듈 범위 지정은 모든 타입에 적용됩니다 (예를 들어, ```mod a { struct foo; }```의 ```struct foo```는 ```mod b { struct foo; }```의 ```struct foo```와는 구별되는 별개의 타입(```a::foo``` vs ```b::foo```)입니다).

**시작 코드** — 함수를 완성하세요:
```rust
mod math {
    // TODO: pub fn add(a: u32, b: u32) -> u32 구현
}

fn greet(name: &str) -> String {
    // TODO: "Hello, <name>! The secret number is <math::add(21,21)>" 반환
    todo!()
}

fn main() {
    println!("{}", greet("Rustacean"));
}
```

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
mod math {
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

fn greet(name: &str) -> String {
    format!("Hello, {}! The secret number is {}", name, math::add(21, 21))
}

fn main() {
    println!("{}", greet("Rustacean"));
}
// 출력: Hello, Rustacean! The secret number is 42
```

</details>
## 워크스페이스(Workspaces) 및 크레이트(패키지)

- 규모가 있는 Rust 프로젝트는 워크스페이스를 사용하여 구성 요소 크레이트들을 조직화해야 합니다.
    - 워크스페이스는 단순히 타겟 바이너리를 빌드하는 데 사용될 로컬 크레이트들의 모음입니다. 워크스페이스 루트의 `Cargo.toml`에는 구성 패키지(크레이트)들에 대한 포인터가 있어야 합니다.

```toml
[workspace]
resolver = "2"
members = ["package1", "package2"]
```

```text
workspace_root/
|-- Cargo.toml      # 워크스페이스 설정
|-- package1/
|   |-- Cargo.toml  # 패키지 1 설정
|   `-- src/
|       `-- lib.rs  # 패키지 1 소스 코드
|-- package2/
|   |-- Cargo.toml  # 패키지 2 설정
|   `-- src/
|       `-- main.rs # 패키지 2 소스 코드
```

---
## 연습 문제: 워크스페이스 및 패키지 의존성 사용하기
- 간단한 패키지를 만들고 우리의 ```hello world``` 프로그램에서 사용해 보겠습니다.
- 워크스페이스 디렉토리 생성
```bash
mkdir workspace
cd workspace
```
- Cargo.toml 파일을 생성하고 다음 내용을 추가합니다. 이렇게 하면 빈 워크스페이스가 생성됩니다.
```toml
[workspace]
resolver = "2"
members = []
```
- 패키지 추가 (```cargo new --lib```는 실행 파일 대신 라이브러리를 생성하도록 지정합니다)
```bash
cargo new hello
cargo new --lib hellolib
```

## 연습 문제: 워크스페이스 및 패키지 의존성 사용하기
- ```hello```와 ```hellolib```에 생성된 Cargo.toml을 살펴보세요. 두 패키지 모두 상위 레벨의 ```Cargo.toml```에 추가된 것을 확인할 수 있습니다.
- ```hellolib```에 ```lib.rs```가 있다는 것은 라이브러리 패키지임을 의미합니다 (커스터마이징 옵션은 https://doc.rust-lang.org/cargo/reference/cargo-targets.html 참조).
- ```hello```의 ```Cargo.toml```에 ```hellolib```에 대한 의존성을 추가합니다.
```toml
[dependencies]
hellolib = {path = "../hellolib"}
```
- ```hellolib```의 ```add()``` 함수 사용하기
```rust
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
```

<details><summary>풀이 (클릭하여 확장)</summary>

전체 워크스페이스 설정:

```bash
# 터미널 명령
mkdir workspace && cd workspace

# 워크스페이스 Cargo.toml 생성
cat > Cargo.toml << 'EOF'
[workspace]
resolver = "2"
members = ["hello", "hellolib"]
EOF

cargo new hello
cargo new --lib hellolib
```

```toml
# hello/Cargo.toml — 의존성 추가
[dependencies]
hellolib = {path = "../hellolib"}
```

```rust
// hellolib/src/lib.rs — cargo new --lib에 의해 이미 add() 함수가 생성되어 있습니다.
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
```

```rust,ignore
// hello/src/main.rs
fn main() {
    println!("Hello, world! {}", hellolib::add(21, 21));
}
// 출력: Hello, world! 42
```

</details>

# crates.io의 커뮤니티 크레이트 사용하기
- Rust는 활발한 커뮤니티 크레이트 에코시스템을 가지고 있습니다 (https://crates.io/ 참조).
    - Rust의 철학은 표준 라이브러리를 작게 유지하고 기능을 커뮤니티 크레이트에 아웃소싱하는 것입니다.
    - 커뮤니티 크레이트 사용에 대한 엄격한 규칙은 없으나, 크레이트의 성숙도(버전 번호로 표시됨)와 활발히 유지보수되고 있는지를 확인하는 것이 좋습니다. 의문이 생기면 내부 소스에 문의하세요.
- ```crates.io```에 게시된 모든 크레이트는 주 버전(major)과 부 버전(minor)을 가집니다.
    - 크레이트들은 여기에 정의된 주 버전 및 부 버전 ```SemVer```(유의적 버전) 가이드라인을 따를 것으로 기대됩니다: https://doc.rust-lang.org/cargo/reference/semver.html
    - 요약하자면, 동일한 부 버전 내에서는 하위 호환성을 깨는 변경(breaking changes)이 없어야 합니다. 예를 들어, v0.11은 v0.15와 호환되어야 합니다 (하지만 v0.20에서는 호환성이 깨질 수 있습니다).

# 크레이트 의존성 및 SemVer
- 크레이트는 특정 버전, 특정 부 버전 또는 주 버전, 혹은 버전에 상관없이 의존성을 정의할 수 있습니다. 다음 예시들은 ```rand``` 크레이트에 대한 의존성을 선언하는 ```Cargo.toml``` 항목들을 보여줍니다.
- 최소 ```0.10.0``` 이상, ```0.11.0``` 미만인 버전
```toml
[dependencies]
rand = { version = "0.10.0"}
```
- 오직 ```0.10.0``` 버전만 허용
```toml
[dependencies]
rand = { version = "=0.10.0"}
```
- 버전 상관없음; ```cargo```가 최신 버전을 선택함
```toml
[dependencies]
rand = { version = "*"}
```
- 참고: https://doc.rust-lang.org/cargo/reference/specifying-dependencies.html
----
# 연습 문제: rand 크레이트 사용하기
- ```helloworld``` 예제를 수정하여 난수를 출력해 보세요.
- ```cargo add rand```를 사용하여 의존성을 추가합니다.
- API에 대한 참조로 ```https://docs.rs/rand/latest/rand/```를 사용하세요.

**시작 코드** — `cargo add rand` 실행 후 `main.rs`에 추가하세요:
```rust,ignore
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    // TODO: 1..=100 사이의 난수 u32를 생성하고 출력하세요.
    // TODO: 임의의 불리언 값을 생성하고 출력하세요.
    // TODO: 임의의 f64 값을 생성하고 출력하세요.
}
```

<details><summary>풀이 (클릭하여 확장)</summary>

```rust
use rand::RngExt;

fn main() {
    let mut rng = rand::rng();
    let n: u32 = rng.random_range(1..=100);
    println!("난수 (1-100): {n}");

    // 임의의 불리언 값 생성
    let b: bool = rng.random();
    println!("임의의 불리언: {b}");

    // 0.0과 1.0 사이의 임의의 실수 생성
    let f: f64 = rng.random();
    println!("임의의 실수: {f:.4}");
}
```

</details>

# Cargo.toml 및 Cargo.lock
- 앞서 언급했듯이, Cargo.lock은 Cargo.toml로부터 자동으로 생성됩니다.
    - Cargo.lock의 주요 목적은 재현 가능한 빌드(reproducible builds)를 보장하는 것입니다. 예를 들어, ```Cargo.toml```에 ```0.10.0``` 버전을 지정했다면, cargo는 ```0.11.0``` 미만의 어떤 버전이든 자유롭게 선택할 수 있습니다.
    - Cargo.lock은 빌드 중에 사용된 rand 크레이트의 *구체적인* 버전을 기록합니다.
    - 재현 가능한 빌드를 위해 ```Cargo.lock```을 git 저장소에 포함하는 것을 권장합니다.

## Cargo 테스트 기능
- Rust 단위 테스트는 관례적으로 동일한 소스 파일 내에 위치하며, 보통 별도의 모듈로 그룹화됩니다.
    - 테스트 코드는 실제 바이너리에는 포함되지 않습니다. 이는 ```cfg```(구성) 기능 덕분에 가능합니다. 구성(Configurations)은 예를 들어 플랫폼별 코드(```Linux``` vs. ```Windows```)를 작성하는 데 유용합니다.
    - 테스트는 ```cargo test```로 실행할 수 있습니다. 참고: https://doc.rust-lang.org/reference/conditional-compilation.html

```rust
pub fn add(left: u64, right: u64) -> u64 {
    left + right
}
// 테스트 중에만 포함됨
#[cfg(test)]
mod tests {
    use super::*; // 부모 범위의 모든 타입을 보이게 함
    #[test]
    fn it_works() {
        let result = add(2, 2); // 또는 super::add(2, 2);
        assert_eq!(result, 4);
    }
}
```

# 기타 Cargo 기능
- ```cargo```에는 다음과 같은 유용한 기능들이 더 있습니다:
    - ```cargo clippy```는 Rust 코드를 린팅(linting)하는 훌륭한 방법입니다. 일반적으로 경고는 수정되어야 합니다 (정말로 정당한 사유가 있는 경우에만 드물게 억제합니다).
    - ```cargo format```은 ```rustfmt``` 도구를 실행하여 소스 코드를 포맷팅합니다. 이 도구를 사용하면 체크인되는 코드의 표준 포맷팅을 보장하고 스타일 논쟁을 끝낼 수 있습니다.
    - ```cargo doc```은 ```///``` 스타일 주석으로부터 문서를 생성하는 데 사용될 수 있습니다. ```crates.io```의 모든 크레이트 문서는 이 방식으로 생성되었습니다.

### 빌드 프로필: 최적화 제어하기

C에서는 `gcc`/`clang`에 `-O0`, `-O2`, `-Os`, `-flto`를 전달합니다. Rust에서는 `Cargo.toml`에서 빌드 프로필을 설정합니다.

```toml
# Cargo.toml — 빌드 프로필 설정

[profile.dev]
opt-level = 0          # 최적화 없음 (빠른 컴파일, -O0와 유사)
debug = true           # 전체 디버그 심볼 포함 (-g와 유사)

[profile.release]
opt-level = 3          # 최대 최적화 (-O3와 유사)
lto = "fat"            # 링크 타임 최적화 (Link-Time Optimization, -flto와 유사)
strip = true           # 심볼 제거 (strip 명령과 유사)
codegen-units = 1      # 단일 코드 생성 단위 — 느린 컴파일, 더 나은 최적화
panic = "abort"        # 언와인드(unwind) 테이블 없음 (바이너리 크기 감소)
```

| C/GCC 플래그 | Cargo.toml 키 | 값 |
|------------|---------------|--------|
| `-O0` / `-O2` / `-O3` | `opt-level` | `0`, `1`, `2`, `3`, `"s"`, `"z"` |
| `-flto` | `lto` | `false`, `"thin"`, `"fat"` |
| `-g` / `-g` 없음 | `debug` | `true`, `false`, `"line-tables-only"` |
| `strip` 명령 | `strip` | `"none"`, `"debuginfo"`, `"symbols"`, `true`/`false` |
| — | `codegen-units` | `1` = 최적의 최적화, 가장 느린 컴파일 |

```bash
cargo build              # [profile.dev] 사용
cargo build --release    # [profile.release] 사용
```

### 빌드 스크립트 (`build.rs`): C 라이브러리 링크하기

C에서는 라이브러리를 링크하고 코드를 생성하기 위해 Makefile이나 CMake를 사용합니다. Rust는 크레이트 루트에 있는 `build.rs` 파일을 사용합니다.

```rust
// build.rs — 크레이트 컴파일 전에 실행됨

fn main() {
    // 시스템 C 라이브러리 링크 (gcc의 -lbmc_ipmi와 유사)
    println!("cargo::rustc-link-lib=bmc_ipmi");

    // 라이브러리를 찾을 경로 (gcc의 -L/usr/lib/bmc와 유사)
    println!("cargo::rustc-link-search=/usr/lib/bmc");

    // C 헤더가 변경되면 다시 실행
    println!("cargo::rerun-if-changed=wrapper.h");
}
```

Rust 크레이트에서 직접 C 소스 파일을 컴파일할 수도 있습니다.

```toml
# Cargo.toml
[build-dependencies]
cc = "1"  # C 컴파일러 통합
```

```rust
// build.rs
fn main() {
    cc::Build::new()
        .file("src/c_helpers/ipmi_raw.c")
        .include("/usr/include/bmc")
        .compile("ipmi_raw");   // libipmi_raw.a를 생성하고 자동으로 링크됨
    println!("cargo::rerun-if-changed=src/c_helpers/ipmi_raw.c");
}
```

| C / Make / CMake | Rust `build.rs` |
|-----------------|-----------------|
| `-lfoo` | `println!("cargo::rustc-link-lib=foo")` |
| `-L/경로` | `println!("cargo::rustc-link-search=/경로")` |
| C 소스 컴파일 | `cc::Build::new().file("foo.c").compile("foo")` |
| 코드 생성 | `$OUT_DIR`에 파일 쓰기 후 `include!()` |

### 교차 컴파일(Cross-Compilation)

C에서 교차 컴파일은 별도의 툴체인(`arm-linux-gnueabihf-gcc`)을 설치하고 Make/CMake를 설정해야 합니다. Rust에서는 다음과 같습니다.

```bash
# 교차 컴파일 타겟 설치
rustup target add aarch64-unknown-linux-gnu

# 교차 컴파일 수행
cargo build --target aarch64-unknown-linux-gnu --release
```

`.cargo/config.toml`에서 링커를 지정합니다.

```toml
[target.aarch64-unknown-linux-gnu]
linker = "aarch64-linux-gnu-gcc"
```

| C 교차 컴파일 | Rust 대응 방식 |
|-----------------|-----------------|
| `apt install gcc-aarch64-linux-gnu` | `rustup target add aarch64-unknown-linux-gnu` + 링커 설치 |
| `CC=aarch64-linux-gnu-gcc make` | `.cargo/config.toml`의 `[target.X] linker = "..."` |
| `#ifdef __aarch64__` | `#[cfg(target_arch = "aarch64")]` |
| 별도의 Makefile 타겟 | `cargo build --target ...` |

### 기능 플래그(Feature Flags): 조건부 컴파일

C는 조건부 컴파일을 위해 `#ifdef`와 `-DFOO`를 사용합니다. Rust는 `Cargo.toml`에 정의된 기능 플래그를 사용합니다.

```toml
# Cargo.toml
[features]
default = ["json"]         # 기본적으로 활성화됨
json = ["dep:serde_json"]  # 선택적 의존성
verbose = []               # 의존성 없는 플래그
gpu = ["dep:cuda-sys"]     # 선택적 GPU 지원
```

```rust
// 기능에 따라 제한된 코드:
#[cfg(feature = "json")]
pub fn parse_config(data: &str) -> Result<Config, Error> {
    serde_json::from_str(data).map_err(Error::from)
}

#[cfg(feature = "verbose")]
macro_rules! verbose {
    ($($arg:tt)*) => { eprintln!("[VERBOSE] {}", format!($($arg)*)); }
}
#[cfg(not(feature = "verbose"))]
macro_rules! verbose {
    ($($arg:tt)*) => {}; // 아무것도 생성하지 않음
}
```

| C 전처리기 | Rust 기능 플래그 |
|---------------|-------------------|
| `gcc -DDEBUG` | `cargo build --features verbose` |
| `#ifdef DEBUG` | `#[cfg(feature = "verbose")]` |
| `#define MAX 100` | `const MAX: u32 = 100;` |
| `#ifdef __linux__` | `#[cfg(target_os = "linux")]` |

### 통합 테스트 vs 단위 테스트

단위 테스트는 `#[cfg(test)]`와 함께 코드 옆에 위치합니다. **통합 테스트**는 `tests/` 디렉토리에 위치하며 크레이트의 **공개 API만**을 테스트합니다.

```rust
// tests/smoke_test.rs — #[cfg(test)] 필요 없음
use my_crate::parse_config;

#[test]
fn parse_valid_config() {
    let config = parse_config("test_data/valid.json").unwrap();
    assert_eq!(config.max_retries, 5);
}
```

| 구분 | 단위 테스트 (`#[cfg(test)]`) | 통합 테스트 (`tests/`) |
|--------|----------------------------|------------------------------|
| 위치 | 코드와 동일한 파일 | 별도의 `tests/` 디렉토리 |
| 접근 범위 | 비공개 + 공개 항목 모두 | **공개 API만 가능** |
| 실행 명령 | `cargo test` | `cargo test --test smoke_test` |


### 테스트 패턴 및 전략

C 펌웨어 팀은 일반적으로 CUnit, CMocka 또는 많은 상용구가 포함된 커스텀 프레임워크에서 테스트를 작성합니다. Rust의 내장 테스트 하네스는 훨씬 더 강력합니다. 이 섹션에서는 프로덕션 코드에 필요한 패턴을 다룹니다.

#### `#[should_panic]` — 예상된 실패 테스트하기

```rust
// 특정 조건이 패닉을 일으키는지 테스트 (C의 assert 실패와 유사)
#[test]
#[should_panic(expected = "index out of bounds")]
fn test_bounds_check() {
    let v = vec![1, 2, 3];
    let _ = v[10];  // 패닉 발생 예정
}

#[test]
#[should_panic(expected = "temperature exceeds safe limit")]
fn test_thermal_shutdown() {
    fn check_temperature(celsius: f64) {
        if celsius > 105.0 {
            panic!("온도가 안전 한계를 초과했습니다: {celsius}°C");
        }
    }
    check_temperature(110.0);
}
```

#### `#[ignore]` — 느리거나 하드웨어 의존적인 테스트

```rust
// 특수한 조건이 필요한 테스트 표시 (C의 #ifdef HARDWARE_TEST와 유사)
#[test]
#[ignore = "GPU 하드웨어가 필요함"]
fn test_gpu_ecc_scrub() {
    // 이 테스트는 GPU가 있는 머신에서만 실행됨
    // 실행 명령: cargo test -- --ignored
    // 실행 명령: cargo test -- --include-ignored (모든 테스트 실행)
}
```

#### Result를 반환하는 테스트 (`unwrap` 체인 대체)

```rust
// 실제 실패 원인을 숨기는 수많은 unwrap() 호출 대신:
#[test]
fn test_config_parsing() -> Result<(), Box<dyn std::error::Error>> {
    let json = r#"{"hostname": "node-01", "port": 8080}"#;
    let config: ServerConfig = serde_json::from_str(json)?;  // unwrap() 대신 ? 사용
    assert_eq!(config.hostname, "node-01");
    assert_eq!(config.port, 8080);
    Ok(())  // 에러 없이 여기까지 도달하면 테스트 통과
}
```

#### 빌더 함수를 이용한 테스트 픽스처(Test Fixtures)

C는 `setUp()`/`tearDown()` 함수를 사용합니다. Rust는 헬퍼 함수와 `Drop`을 사용합니다.

```rust
struct TestFixture {
    temp_dir: std::path::PathBuf,
    config: Config,
}

impl TestFixture {
    fn new() -> Self {
        let temp_dir = std::env::temp_dir().join(format!("test_{}", std::process::id()));
        std::fs::create_dir_all(&temp_dir).unwrap();
        let config = Config {
            log_dir: temp_dir.clone(),
            max_retries: 3,
            ..Default::default()
        };
        Self { temp_dir, config }
    }
}

impl Drop for TestFixture {
    fn drop(&mut self) {
        // 자동 정리 — C의 tearDown()과 유사하지만 잊어버릴 염려가 없음
        let _ = std::fs::remove_dir_all(&self.temp_dir);
    }
}

#[test]
fn test_with_fixture() {
    let fixture = TestFixture::new();
    // fixture.config, fixture.temp_dir 사용...
    assert!(fixture.temp_dir.exists());
    // fixture가 여기서 자동으로 드롭됨 → 정리 작업 실행
}
```

#### 하드웨어 인터페이스를 위한 트레이트 모킹(Mocking)

C에서 하드웨어를 모킹하려면 전처리기 트릭이나 함수 포인터 교체가 필요합니다. Rust에서는 트레이트를 사용하여 자연스럽게 수행할 수 있습니다.

```rust
// IPMI 통신을 위한 프로덕션 트레이트
trait IpmiTransport {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String>;
}

// 실제 구현 (프로덕션용)
struct RealIpmi { /* BMC 연결 상세 정보 */ }
impl IpmiTransport for RealIpmi {
    fn send_command(&self, cmd: u8, data: &[u8]) -> Result<Vec<u8>, String> {
        // 실제 BMC 하드웨어와 통신
        todo!("실제 IPMI 호출")
    }
}

// 모킹된 구현 (테스트용)
struct MockIpmi {
    responses: std::collections::HashMap<u8, Vec<u8>>,
}
impl IpmiTransport for MockIpmi {
    fn send_command(&self, cmd: u8, _data: &[u8]) -> Result<Vec<u8>, String> {
        self.responses.get(&cmd)
            .cloned()
            .ok_or_else(|| format!("명령 0x{cmd:02x}에 대한 모킹된 응답이 없음"))
    }
}

// 실제와 모킹 모두에서 작동하는 제네릭 함수
fn read_sensor_temperature(transport: &dyn IpmiTransport) -> Result<f64, String> {
    let response = transport.send_command(0x2D, &[])?;
    if response.len() < 2 {
        return Err("응답이 너무 짧음".into());
    }
    Ok(response[0] as f64 + (response[1] as f64 / 256.0))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_temperature_reading() {
        let mut mock = MockIpmi { responses: std::collections::HashMap::new() };
        mock.responses.insert(0x2D, vec![72, 128]); // 72.5°C

        let temp = read_sensor_temperature(&mock).unwrap();
        assert!((temp - 72.5).abs() < 0.01);
    }

    #[test]
    fn test_short_response() {
        let mock = MockIpmi { responses: std::collections::HashMap::new() };
        // 응답이 설정되지 않음 → 에러 발생
        assert!(read_sensor_temperature(&mock).is_err());
    }
}
```

#### `proptest`를 이용한 속성 기반 테스트(Property-Based Testing)

특정 값을 테스트하는 대신, 항상 유지되어야 하는 **속성**을 테스트합니다.

```rust
// Cargo.toml: [dev-dependencies] proptest = "1"
use proptest::prelude::*;

fn parse_sensor_id(s: &str) -> Option<u32> {
    s.strip_prefix("sensor_")?.parse().ok()
}

fn format_sensor_id(id: u32) -> String {
    format!("sensor_{id}")
}

proptest! {
    #[test]
    fn roundtrip_sensor_id(id in 0u32..10000) {
        // 속성: 포맷 후 파싱하면 원본이 돌아와야 함
        let formatted = format_sensor_id(id);
        let parsed = parse_sensor_id(&formatted);
        prop_assert_eq!(parsed, Some(id));
    }

    #[test]
    fn parse_rejects_garbage(s in "[^s].*") {
        // 속성: 's'로 시작하지 않는 문자열은 절대 파싱되지 않아야 함
        let result = parse_sensor_id(&s);
        prop_assert!(result.is_none());
    }
}
```

#### C vs Rust 테스트 비교

| C 테스트 | Rust 대응 방식 |
|-----------|----------------|
| `CUnit`, `CMocka`, 커스텀 프레임워크 | 내장 `#[test]` + `cargo test` |
| `setUp()` / `tearDown()` | 빌더 함수 + `Drop` 트레이트 |
| `#ifdef TEST` 모킹 함수 | 트레이트 기반 의존성 주입 |
| `assert(x == y)` | 자동 diff 출력을 제공하는 `assert_eq!(x, y)` |
| 별도의 테스트 실행 파일 | 동일 바이너리, `#[cfg(test)]`를 이용한 조건부 컴파일 |
| `valgrind --leak-check=full ./test` | `cargo test` (기본적으로 메모리 안전) + `cargo miri test` |
| 코드 커버리지: `gcov` / `lcov` | `cargo tarpaulin` 또는 `cargo llvm-cov` |
| 테스트 탐색: 수동 등록 | 자동 — 모든 `#[test]` 함수를 탐색함 |
