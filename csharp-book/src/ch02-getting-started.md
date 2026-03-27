## 설치 및 설정

> **학습 목표:** Rust를 설치하고 IDE를 설정하는 방법을 배웁니다. Cargo 빌드 시스템과 MSBuild/NuGet의 차이점을 알아보고, C#과 비교하여 첫 번째 Rust 프로그램을 작성하며 커맨드 라인 입력을 읽는 방법을 익힙니다.
>
> **난이도:** 🟢 초급

### Rust 설치하기
```bash
# Rust 설치 (Windows, macOS, Linux 공통)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Windows 사용자는 다음 주소에서 다운로드할 수도 있습니다: https://rustup.rs/
```

### Rust 도구 vs C# 도구
| C# 도구 | Rust 대응 도구 | 용도 |
|---------|----------------|---------|
| `dotnet new` | `cargo new` | 새 프로젝트 생성 |
| `dotnet build` | `cargo build` | 프로젝트 컴파일 |
| `dotnet run` | `cargo run` | 프로젝트 실행 |
| `dotnet test` | `cargo test` | 테스트 실행 |
| NuGet | Crates.io | 패키지 저장소 |
| MSBuild | Cargo | 빌드 시스템 |
| Visual Studio | VS Code + rust-analyzer | IDE |

### IDE 설정
1. **VS Code** (입문자 권장)
   - "rust-analyzer" 확장 설치
   - 디버깅을 위해 "CodeLLDB" 확장 설치

2. **Visual Studio** (Windows)
   - Rust 지원 확장 설치

3. **JetBrains RustRover** (전용 IDE)
   - C#의 Rider와 유사한 환경 제공

***

## 첫 번째 Rust 프로그램

### C# Hello World
```csharp
// Program.cs
using System;

namespace HelloWorld
{
    class Program
    {
        static void Main(string[] args)
        {
            Console.WriteLine("Hello, World!");
        }
    }
}
```

### Rust Hello World
```rust
// main.rs
fn main() {
    println!("Hello, World!");
}
```

### C# 개발자를 위한 주요 차이점
1. **클래스가 필수가 아님** - 함수는 최상위 레벨에 존재할 수 있습니다.
2. **네임스페이스 없음** - 대신 모듈(Module) 시스템을 사용합니다.
3. **`println!`은 매크로임** - 끝에 붙은 `!`를 확인하세요.
4. **println! 뒤에 세미콜론이 없음** - 표현식(Expression)과 문(Statement)의 차이입니다.
5. **명시적인 반환 타입 없음** - `main`은 유닛 타입인 `()`를 반환합니다.

### 첫 번째 프로젝트 만들기
```bash
# 새 프로젝트 생성 ('dotnet new console'과 유사)
cargo new hello_rust
cd hello_rust

# 생성된 프로젝트 구조:
# hello_rust/
# ├── Cargo.toml      (.csproj 파일과 유사)
# └── src/
#     └── main.rs     (Program.cs와 유사)

# 프로젝트 실행 ('dotnet run'과 유사)
cargo run
```

***

## Cargo vs NuGet/MSBuild

### 프로젝트 구성

**C# (.csproj)**
```xml
<Project Sdk="Microsoft.NET.Sdk">
  <PropertyGroup>
    <OutputType>Exe</OutputType>
    <TargetFramework>net8.0</TargetFramework>
  </PropertyGroup>
  
  <PackageReference Include="Newtonsoft.Json" Version="13.0.3" />
  <PackageReference Include="Serilog" Version="3.0.1" />
</Project>
```

**Rust (Cargo.toml)**
```toml
[package]
name = "hello_rust"
version = "0.1.0"
edition = "2021"

[dependencies]
serde_json = "1.0"    # Newtonsoft.Json에 해당
log = "0.4"           # Serilog에 해당
```

### 주요 Cargo 명령어
```bash
# 새 프로젝트 생성
cargo new my_project
cargo new my_project --lib  # 라이브러리 프로젝트 생성

# 빌드 및 실행
cargo build          # 'dotnet build'와 유사
cargo run            # 'dotnet run'와 유사
cargo test           # 'dotnet test'와 유사

# 패키지 관리
cargo add serde      # 의존성 추가 ('dotnet add package'와 유사)
cargo update         # 의존성 업데이트

# 릴리스 빌드
cargo build --release  # 최적화된 빌드
cargo run --release    # 최적화된 버전 실행

# 문서화
cargo doc --open     # 문서를 생성하고 브라우저로 엽니다
```

### 작업 공간(Workspace) vs 솔루션(Solution)

**C# 솔루션 (.sln)**
```text
MySolution/
├── MySolution.sln
├── WebApi/
│   └── WebApi.csproj
├── Business/
│   └── Business.csproj
└── Tests/
    └── Tests.csproj
```

**Rust 작업 공간 (Cargo.toml)**
```toml
[workspace]
members = [
    "web_api",
    "business", 
    "tests"
]
```

***

## 입력 읽기 및 CLI 인자 처리

모든 C# 개발자는 `Console.ReadLine()`을 알고 있습니다. Rust에서 사용자 입력, 환경 변수, 커맨드 라인 인자를 처리하는 방법은 다음과 같습니다.

### 콘솔 입력
```csharp
// C# — 사용자 입력 읽기
Console.Write("이름을 입력하세요: ");
string? name = Console.ReadLine();  // .NET 6+에서는 string? 반환
Console.WriteLine($"안녕하세요, {name}님!");

// 입력 파싱
Console.Write("숫자를 입력하세요: ");
if (int.TryParse(Console.ReadLine(), out int number))
{
    Console.WriteLine($"입력한 숫자: {number}");
}
else
{
    Console.WriteLine("유효한 숫자가 아닙니다.");
}
```

```rust
use std::io::{self, Write};

fn main() {
    // 입력 한 줄 읽기
    print!("이름을 입력하세요: ");
    io::stdout().flush().unwrap(); // print!는 자동 플러시되지 않으므로 수동 플러시 필요

    let mut name = String::new();
    io::stdin().read_line(&mut name).expect("줄을 읽지 못했습니다");
    let name = name.trim(); // 줄바꿈 문자 제거
    println!("안녕하세요, {name}님!");

    // 입력 파싱
    print!("숫자를 입력하세요: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("읽지 못했습니다");
    match input.trim().parse::<i32>() {
        Ok(number) => println!("입력한 숫자: {number}"),
        Err(_)     => println!("유효한 숫자가 아닙니다."),
    }
}
```

### 커맨드 라인 인자
```csharp
// C# — CLI 인자 읽기
static void Main(string[] args)
{
    if (args.Length < 1)
    {
        Console.WriteLine("사용법: program <파일명>");
        return;
    }
    string filename = args[0];
    Console.WriteLine($"처리 중: {filename}");
}
```

```rust
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    //  args[0] = 프로그램 이름 (C#의 Assembly 이름과 유사)
    //  args[1..] = 실제 인자들

    if args.len() < 2 {
        eprintln!("사용법: {} <파일명>", args[0]); // eprintln! → 표준 에러(stderr)로 출력
        std::process::exit(1);
    }
    let filename = &args[1];
    println!("처리 중: {filename}");
}
```

### 환경 변수
```csharp
// C#
string dbUrl = Environment.GetEnvironmentVariable("DATABASE_URL") ?? "localhost";
```

```rust
use std::env;

let db_url = env::var("DATABASE_URL").unwrap_or_else(|_| "localhost".to_string());
// env::var는 Result<String, VarError>를 반환합니다 — null이 없습니다!
```

### `clap`을 이용한 실무형 CLI 앱

단순한 인자 파싱을 넘어선 작업에는 **`clap`** 크레이트를 사용하세요. 이는 C#의 `System.CommandLine`이나 `CommandLineParser` 같은 라이브러리에 대응하는 Rust의 표준적인 도구입니다.

```toml
# Cargo.toml
[dependencies]
clap = { version = "4", features = ["derive"] }
```

```rust
use clap::Parser;

/// 단순한 파일 처리기 — 이 문서 주석은 도움말 텍스트가 됩니다.
#[derive(Parser, Debug)]
#[command(name = "processor", version, about)]
struct Args {
    /// 처리할 입력 파일
    #[arg(short, long)]
    input: String,

    /// 출력 파일 (기본값: 표준 출력)
    #[arg(short, long)]
    output: Option<String>,

    /// 상세 로그 활성화
    #[arg(short, long, default_value_t = false)]
    verbose: bool,

    /// 작업자 스레드 수
    #[arg(short = 'j', long, default_value_t = 4)]
    threads: usize,
}

fn main() {
    let args = Args::parse(); // 자동 파싱, 검증, --help 생성

    if args.verbose {
        println!("입력:   {}", args.input);
        println!("출력:  {:?}", args.output);
        println!("스레드: {}", args.threads);
    }

    // args.input, args.output 등을 사용합니다.
}
```

```bash
# 자동 생성된 도움말:
$ processor --help
단순한 파일 처리기

사용법: processor [OPTIONS] --input <INPUT>

옵션:
  -i, --input <INPUT>      처리할 입력 파일
  -o, --output <OUTPUT>    출력 파일 (기본값: 표준 출력)
  -v, --verbose            상세 로그 활성화
  -j, --threads <THREADS>  작업자 스레드 수 [기본값: 4]
  -h, --help               도움말 출력
  -V, --version            버전 출력
```

```csharp
// System.CommandLine을 사용한 C# 코드 (더 많은 보일러플레이트 필요):
var inputOption = new Option<string>("--input", "입력 파일") { IsRequired = true };
var verboseOption = new Option<bool>("--verbose", "상세 로그 활성화");
var rootCommand = new RootCommand("단순한 파일 처리기");
rootCommand.AddOption(inputOption);
rootCommand.AddOption(verboseOption);
rootCommand.SetHandler((input, verbose) => { /* ... */ }, inputOption, verboseOption);
await rootCommand.InvokeAsync(args);
// clap의 derive 매크로 방식이 훨씬 간결하고 타입 안전합니다.
```

| C# | Rust | 비고 |
|----|------|-------|
| `Console.ReadLine()` | `io::stdin().read_line(&mut buf)` | 버퍼를 제공해야 하며 `Result`를 반환함 |
| `int.TryParse(s, out n)` | `s.parse::<i32>()` | `Result<i32, ParseIntError>`를 반환함 |
| `args[0]` | `env::args().nth(1)` | Rust의 args[0]은 프로그램 이름임 |
| `Environment.GetEnvironmentVariable` | `env::var("KEY")` | null이 아닌 `Result`를 반환함 |
| `System.CommandLine` | `clap` | Derive 기반, 자동 도움말 생성 |

***
