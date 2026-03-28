# 시작하기: 설치와 환경 설정

> **학습 목표:** Rust 개발 환경을 구축하고 첫 번째 프로그램을 작성해 봅니다. C#의 `dotnet` 도구 체인에 대응하는 `cargo` 빌드 시스템을 익히고, 간단한 콘솔 입력과 커맨드 라인 인자를 처리하는 방법을 배웁니다.

---

### 1. 개발 환경 구축
Rust는 `rustup`이라는 올인원 설치 도구를 사용합니다.

- **설치**: [rustup.rs](https://rustup.rs/)에서 OS에 맞는 인스톨러를 실행하세요. (macOS/Linux는 한 줄의 터미널 명령어로 설치 가능)
- **IDE 추천**:
  - **VS Code**: `rust-analyzer` 확장이 사실상의 표준입니다. 디버깅을 위해 `CodeLLDB` 확장을 함께 설치하세요.
  - **RustRover**: JetBrains의 전용 IDE로, Rider를 쓰던 개발자에게 익숙한 환경입니다.

| **기능** | **C# (.NET)** | **Rust** |
| :--- | :--- | :--- |
| **빌드/실행 도구** | `dotnet` (build, run, test) | `cargo` (build, run, test) |
| **패키지 관리자** | NuGet | Crates.io |
| **프로젝트 설정** | `.csproj` (XML) | `Cargo.toml` (TOML) |
| **종속성 잠금** | `packages.lock.json` | `Cargo.lock` |

---

### 2. 첫 번째 프로그램: Hello World
C#의 `Program.cs`와 Rust의 `main.rs`를 비교해 봅시다.

```rust
// src/main.rs: 클래스 없이 함수만으로 시작 가능
fn main() {
    // println!은 함수가 아닌 '매크로'입니다.
    // 타입 안전한 포맷팅을 컴파일 타임에 검사합니다.
    println!("Hello, Rust from C# developer!");
}
```

- **프로젝트 생성**: `cargo new my_project` 명령으로 새 프로젝트 폴더를 만듭니다.
- **실행**: `cargo run` 명령으로 빌드와 실행을 한 번에 수행합니다.

---

### 3. 입력 처리와 CLI 인자
C#의 `Console.ReadLine()`과 `args[]`가 Rust에서는 어떻게 바뀌는지 살펴봅니다.

| **사용 사례** | **C# 스타일** | **Rust 스타일** |
| :--- | :--- | :--- |
| **콘솔 읽기** | `Console.ReadLine()` | `io::stdin().read_line(&mut buf)` |
| **문자열 파싱** | `int.Parse(s)` | `s.trim().parse::<i32>()` |
| **CLI 인자** | `string[] args` | `std::env::args().collect::<Vec<_>>()` |
| **환경 변수** | `Environment.GetVar()` | `std::env::var("KEY")` |

#### 💡 실무에서는 `clap`을 쓰세요
C#에서 복잡한 CLI 인자를 위해 `CommandLineParser`를 쓰듯, Rust에서는 **`clap`** 크레이트가 표준입니다. 구조체 선언만으로 `--help` 메시지와 타입 검증이 자동으로 생성됩니다.

```rust
// clap을 활용한 타입 안전한 인자 파싱 (예시)
#[derive(Parser)]
struct Args {
    #[arg(short, long)]
    name: String,
    #[arg(short, long, default_value_t = 1)]
    count: u8,
}
```

---

### 📝 실습 연습: 에코(Echo) 프로그램 만들기

🟢 **초급 과정** — 아래 기능을 구현해 보세요.
1. 사용자의 이름을 입력받습니다.
2. 입력받은 이름이 비어있다면 "이름을 입력하지 않으셨습니다."를 출력합니다.
3. 이름이 있다면 "안녕하세요, [이름]님!"을 출력합니다.

```rust
// [힌트]
// 1. io::stdin().read_line() 사용
// 2. .trim()으로 입력 끝의 줄바꿈 제거
// 3. .is_empty()로 빈 문자열 확인
```

