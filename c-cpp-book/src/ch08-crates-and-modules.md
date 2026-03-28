# Rust의 코드 구조화: 크레이트와 모듈

> **학습 목표:** Rust가 대규모 코드를 모듈과 크레이트 단위로 조직화하는 체계적인 방법을 익힙니다. 캡슐화의 핵심인 가시성 규칙(`pub`), 프로젝트 규모를 확장하는 워크스페이스(Workspaces), 그리고 강력한 외부 생태계인 `crates.io` 활용법을 다룹니다. 이는 C/C++의 헤더 파일, `#include`, 복잡한 CMake 의존성 관리를 완벽하게 대체하는 현대적인 솔루션입니다.

---

### 모듈 시스템의 기본 원칙
모듈은 크레이트 내부에서 코드를 논리적으로 구분하는 기초 단위입니다.

- **핵심 규칙**
    - **파일이 곧 모듈**: 각 소스 파일(`.rs`)은 그 자체로 하나의 모듈이 됩니다. 또한 `mod` 키워드를 사용해 파일 내부에 중첩된 하위 모듈을 만들 수도 있습니다.
    - **기본 비공개(Private by Default)**: 모듈 내 모든 요소는 기본적으로 외부에서 보이지 않습니다. 밖으로 노출시키려면 명시적으로 `pub` 키워드를 붙여야 합니다. (`pub(crate)` 등을 통해 공개 범위를 세밀하게 조정할 수도 있습니다.)
    - **명시적 임포트**: `pub`으로 공개된 타입이라도 `use` 키워드로 불러오지 않으면 다른 모듈에서 바로 쓸 수 없습니다. 부모 모듈의 요소를 참조할 때는 `use super::` 구문을 사용합니다.
    - **크레이트 루트 연결**: 새로운 파일(`.rs`)을 만들었다고 해서 자동으로 빌드에 포함되지는 않습니다. 반드시 `main.rs`(실행 파일용)나 `lib.rs`(라이브러리용)에 `mod` 선언으로 연결해 주어야 합니다.

---

### 📝 실습 연습: 모듈 정의와 함수 호출
함수는 `fn` 키워드로 정의하며, `->` 뒤에 반환 타입을 명시합니다. (반환 타입이 없으면 유닛 타입 `()`이 기본값입니다.)

**미션**: 아래의 미완성 구조를 완성하여 정상적으로 인사말이 출력되도록 하세요.
```rust
mod math {
    // TODO: 외부에서 호출 가능하도록 두 수의 합을 구하는 add 함수 구현
    pub fn add(a: u32, b: u32) -> u32 {
        a + b
    }
}

fn greet(name: &str) -> String {
    // TODO: "Hello, <이름>! 비밀 숫자는 <math::add(21,21) 결과>입니다." 문구 반환
    format!("Hello, {}! 비밀 숫자는 {}입니다.", name, math::add(21, 21))
}

fn main() {
    println!("{}", greet("Rustacean"));
}
```
> **팁**: `format!` 매크로는 `println!`과 사용법이 같지만 결과를 화면에 찍는 대신 `String` 객체로 반환해 줍니다.

---

### 워크스페이스(Workspaces)와 패키지 관리
프로젝트 규모가 커지면 여러 개의 크레이트를 하나의 단위로 묶어 관리해야 합니다. 이를 **워크스페이스**라고 부릅니다.

- **프로젝트 구조 예시**
```text
my_workspace/
|-- Cargo.toml      # 워크스페이스 전체 설정 (구성원 크레이트 명시)
|-- app_cli/        # 실행 파일 크레이트
|   |-- Cargo.toml  
|   `-- src/main.rs 
|-- core_lib/       # 공통 로직 라이브러리 크레이트
|   |-- Cargo.toml  
|   `-- src/lib.rs  
```

- **워크스페이스 설정 (루트 `Cargo.toml`)**
```toml
[workspace]
resolver = "2"
members = ["app_cli", "core_lib"]
```

---

### 📝 실습 연습: 워크스페이스 기반 의존성 설정
실제로 두 개의 패키지를 만들고 서로 참조하는 과정을 체험해 봅니다.

1.  **환경 구축** (터미널 명령)
    ```bash
    mkdir my_rust_project && cd my_rust_project
    # 루트 Cargo.toml 생성 후 [workspace] 설정 추가 (위 예시 참고)
    cargo new app_main        # 실행 파일 생성
    cargo new --lib core_util  # 공유 라이브러리 생성
    ```
2.  **의존성 연결** (`app_main/Cargo.toml`)
    ```toml
    [dependencies]
    core_util = { path = "../core_util" } # 로컬 파일 경로를 통해 연결
    ```
3.  **코드 구현 및 실행**
    - `core_util/src/lib.rs`에 `pub fn add(...)` 함수가 있는지 확인합니다.
    - `app_main/src/main.rs`에서 `core_util::add(21, 21)`을 호출해 봅니다.
    - 루트 디렉토리에서 `cargo run -p app_main` 명령으로 실행합니다.

---

# 외부 생태계 활용: crates.io

Rust는 표준 라이브러리를 작고 핵심적인 기능 위주로 유지하는 대신, 고도화된 기능은 커뮤니티 저장소인 [crates.io](https://crates.io/)에 위임하는 철학을 가지고 있습니다.

- **유의적 버전 (SemVer) 규칙**
    - 모든 크레이트는 `주 버전.부 버전.패치`(예: `1.2.3`) 형식을 따릅니다.
    - **주 버전(Major)**이 같으면 하위 호환성이 유지되는 것을 원칙으로 합니다.
- **의존성 선언 방식 (`Cargo.toml`)**
    - `rand = "0.8.5"`: 0.8.5 이상, 0.9.0 미만의 최신 버전을 자동으로 선택 (권장)
    - `rand = "=0.8.5"`: 정확히 0.8.5 버전만 고정 사용
    - `rand = "*"`: 무조건 최신 버전 사용 (호환성 문제로 지양함)

---

### 📝 실습 연습: 난수 생성 라이브러리(`rand`) 사용하기
1.  터미널에서 `cargo add rand`를 입력하여 프로젝트에 의존성을 추가합니다.
2.  아래 코드를 완성하여 다양한 난수를 생성해 보세요.

```rust
use rand::Rng; // 난수 생성을 위한 트레이트 가져오기

fn main() {
    let mut rng = rand::thread_rng();

    // 1. 1부터 100 사이의 u32 난수
    let n: u32 = rng.gen_range(1..=100);
    println!("행운의 숫자: {n}");

    // 2. 임의의 불리언(T/F) 값
    let is_lucky: bool = rng.gen();
    println!("오늘의 운세는? {}", if is_lucky { "대박" } else { "평범" });

    // 3. 0.0 ~ 1.0 사이의 실수
    let prob: f64 = rng.gen();
    println!("성공 확률: {:.2}%", prob * 100.0);
}
```

---

### `Cargo.toml` vs `Cargo.lock`
- **`Cargo.toml`**: 개발자가 직접 작성하는 **설계도**입니다. "어떤 라이브러리의 어떤 버전 범위가 필요한지"를 적습니다.
- **`Cargo.lock`**: 시스템이 자동으로 관리하는 **스냅샷**입니다. 실제로 빌드 시점에 어떤 구체적인 버전이 다운로드되었는지 기록하여, 다른 환경에서도 팀원 모두가 동일한 결과를 얻도록 보장합니다. (Git 저장소에 반드시 포함해야 합니다.)

---

# Cargo의 강력한 부가 기능

1.  **`cargo clippy`**: Rust의 깐깐한 코드 리뷰어입니다. 더 효율적이고 Rust스러운(Idiomatic) 코드 작성 방향을 제시합니다.
2.  **`cargo fmt`**: 소스 코드를 표준 스타일 가이드에 맞춰 자동 정렬합니다. 팀원 간의 스타일 논쟁을 마침표 찍어주는 훌륭한 도구입니다.
3.  **`cargo doc`**: 코드 내의 주석(`///`)을 분석하여 멋진 웹 문서로 만들어 줍니다. `cargo doc --open` 명령으로 확인해 보세요.

---

### 빌드 프로필 (Optimization Control)
C/C++의 `-O2`, `-O3`와 같은 최적화 옵션을 `Cargo.toml`에서 직접 제어합니다.

```toml
[profile.dev]
opt-level = 0    # 개발용: 빠른 컴파일 우선 (-O0 수준)

[profile.release]
opt-level = 3    # 제품용: 최대 최적화 적용 (-O3 수준)
lto = "fat"      # 전체 프로젝트 단위 링크 타임 최적화 (LTO) 적용
strip = true     # 불필요한 디버그 심볼 제거로 바이너리 크기 최소화
```

### 🌉 빌드 스크립트와 C 라이브러리 연동
기존 C 프로젝트를 Rust로 전환하거나 함께 사용해야 할 때 `build.rs`를 활용합니다.

```rust
// build.rs: 컴파일 전 실행되는 로직
fn main() {
    // 1. 시스템 라이브러리 링크 (-l 옵션과 유사)
    println!("cargo:rustc-link-lib=sqlite3");

    // 2. 라이브러리 검색 경로 추가 (-L 옵션과 유사)
    println!("cargo:rustc-link-search=native=/usr/local/lib");
}
```
또한 `cc` 크레이트를 사용하면 Rust 빌드 과정 중에 C 소스 파일을 직접 컴파일하여 라이브러리 형상으로 포함시킬 수도 있습니다.

---

### 교차 컴파일 (Cross-Compilation)
별도의 복잡한 툴체인 설정 없이 `rustup`과 `cargo` 명령어만으로 다양한 타겟(ARM, RISC-V 등)을 위한 바이너리를 빌드할 수 있습니다.

```bash
# 1. ARM 64비트 리눅스용 타겟 추가
rustup target add aarch64-unknown-linux-gnu

# 2. 해당 타겟으로 빌드
cargo build --target aarch64-unknown-linux-gnu --release
```

---

### 기능 플래그 (Feature Flags)
C의 전처리기(`#ifdef`) 기능을 훨씬 깔끔하고 체계적으로 구현합니다. 필요한 기능만 선택해서 빌드할 수 있어 가벼운 실행 파일을 만드는 데 유용합니다.

```toml
# Cargo.toml 설정
[features]
default = ["json"]
json = ["dep:serde_json"] # json 기능을 켜면 관련 라이브러리도 함께 의존성 추가
gpu = []                 # 하드웨어 가속 플래그
```

```rust
#[cfg(feature = "gpu")]
fn process_raw_data() {
    // GPU 구동 시에만 컴파일되는 로직
}
```
