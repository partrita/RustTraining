# 백문이 불여일견: 코드로 이해하는 Rust

> **학습 목표:** 여러분의 첫 번째 Rust 프로그램을 작성해 봅니다. `fn main()`, `println!()`의 기본 사용법과 함께, Rust의 매크로가 C/C++ 전처리기 매크로와 근본적으로 어떻게 다른지 살펴봅니다. 이 장을 마치면 직접 Rust 프로그램을 작성하고 컴파일하여 실행할 수 있게 됩니다.

```rust
fn main() {
    println!("Rust의 세계에 오신 것을 환영합니다!");
}
```
위의 코드는 C 시리즈 언어(C, C++, Java 등)에 익숙한 분이라면 매우 친숙하게 느껴질 것입니다.

- **기본 문법의 특징**
    - Rust의 모든 함수 선언은 `fn` 키워드로 시작합니다.
    - 실행 파일의 진입점(Entry point)은 언제나 `main()` 함수입니다.
    - `println!`은 함수처럼 보이지만, 실제로는 **매크로**입니다. Rust의 매크로는 C/C++의 단순 텍스트 치환 방식이 아닌, 구문 트리(Syntax tree) 단위에서 작동하는 '위생적(Hygienic)'이고 타입 안전한 시스템입니다.
- **Rust 코드를 빠르게 테스트하는 방법**
    - **온라인 환경**: [Rust Playground](https://play.rust-lang.org/)를 이용하면 별도의 설치 없이도 브라우저에서 바로 코드를 실행하고 결과를 공유할 수 있습니다.
    - **로컬 대화형 환경 (REPL)**: Python의 IDLE처럼 Rust 코드를 한 줄씩 실행해 볼 수 있는 [`evcxr_repl`](https://github.com/evcxr/evcxr)을 설치해 보세요.
    ```bash
    cargo install --locked evcxr_repl
    evcxr   # REPL 프로그램을 시작합니다.
    ```

### 로컬 환경에 Rust 설치하기
Rust는 `rustup`이라는 툴체인 관리자를 통해 매우 쉽게 설치하고 업데이트할 수 있습니다.

- **OS별 설치 방법**
    - **Windows**: [rustup-init.exe](https://static.rust-lang.org/rustup/dist/x86_64-pc-windows-msvc/rustup-init.exe) 파일을 다운로드하여 실행하세요.
    - **Linux / WSL / macOS**: 터미널에서 다음 명령어를 입력하세요.
      ```bash
      curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
      ```
- **Rust 생태계 구성 요소**
    - `rustc`: Rust 컴파일러 본체입니다. 하지만 개발자가 직접 호출하는 경우는 드뭅니다.
    - **`cargo`**: Rust의 '맥가이버 칼'입니다. 패키지 관리, 빌드, 테스트, 포맷팅, 린팅 등 모든 작업을 담당하는 핵심 도구입니다.
    - **툴체인 채널**: 실무에서는 가장 안정적인 `stable` 채널을 사용합니다. 6주마다 출시되는 최신 버전을 적용하려면 `rustup update` 명령어만 입력하면 됩니다.
- **추천 개발 환경**: VSCode를 사용한다면 필수 확장 프로그램인 **`rust-analyzer`**를 반드시 설치하시기 바랍니다.

---

# Rust의 패키지 단위: 크레이트(Crates)

Rust에서 실행 파일이나 라이브러리를 만드는 기초 단위는 '패키지'이며, 우리는 이를 **크레이트(Crate)**라고 부릅니다.

- **크레이트의 특징**
    - 독립적으로 존재하거나 다른 크레이트를 참조(의존)할 수 있습니다.
    - 외부 라이브러리는 주로 중앙 패키지 저장소인 [crates.io](https://crates.io/)에서 공유됩니다.
- **Cargo의 역할**
    - 하려고 하는 작업에 필요한 외부 라이브러리를 자동으로 다운로드하고 관리합니다. 이는 C 프로젝트에서 라이브러리를 수동으로 링크하는 과정과 개념적으로 비슷하지만 훨씬 편리합니다.
    - 의존성 정보와 프로젝트 설정은 **`Cargo.toml`** 파일에 명시합니다. 이 파일에서는 실행 파일, 정적 라이브러리, 동적 라이브러리 등 결과물의 형태(Target type)도 정의합니다.

## Cargo vs 전통적인 C 빌드 시스템 비교

### 의존성 관리의 혁신

```mermaid
graph TD
    subgraph "전통적인 C 빌드 및 관리 방식"
        CC["C 소스 파일<br/>(.c, .h)"]
        CM["수동 Makefile<br/>또는 CMake 구성"]
        CL["링커 (Linker)"]
        CB["최종 바이너리"]
        
        CC --> CM
        CM --> CL
        CL --> CB
        
        CDep["수동 의존성 관리의 고충"]
        CLib1["libcurl-dev<br/>(패키지 매니저 설치)"]
        CLib2["libjson-dev<br/>(수동 설치)"]
        CInc["수동 헤더 경로 지정<br/>-I/usr/include/curl"]
        CLink["수동 라이브러리 링크<br/>-lcurl -ljson"]
        
        CDep --> CLib1
        CDep --> CLib2
        CLib1 --> CInc
        CLib2 --> CInc
        CInc --> CM
        CLink --> CL
        
        C_ISSUES["[에러] 버전 간 충돌<br/>[에러] 플랫폼별 환경 차이<br/>[에러] 의존성 누락 문제<br/>[에러] 복잡한 링크 순서<br/>[에러] 자동 업데이트 불가"]
    end
    
    subgraph "Rust의 현대적인 Cargo 방식"
        RS["Rust 소스 파일<br/>(.rs)"]
        CT["Cargo.toml 설정<br/>reqwest = '0.11'<br/>serde_json = '1.0'"]
        CRG["Cargo 통합 빌드 시스템"]
        RB["최종 바이너리 결과물"]
        
        RS --> CRG
        CT --> CRG
        CRG --> RB
        
        CRATES["crates.io<br/>(공식 패키지 저장소)"]
        DEPS["자동 의존성 분석 및 해결"]
        LOCK["Cargo.lock<br/>(버전 고정 및 재현성 보장)"]
        
        CRATES --> DEPS
        DEPS --> CRG
        CRG --> LOCK
        
        R_BENEFITS["[OK] 유의적 버전(SemVer) 체계<br/>[OK] 원클릭 자동 설치<br/>[OK] 완벽한 크로스 플랫폼 지원<br/>[OK] 하위 의존성까지 자동 관리<br/>[OK] 동일한 빌드 결과 보장"]
    end
    
    style C_ISSUES fill:#ff6b6b,color:#000
    style R_BENEFITS fill:#91e5a3,color:#000
    style CM fill:#ffa07a,color:#000
    style CDep fill:#ffa07a,color:#000
    style CT fill:#91e5a3,color:#000
    style CRG fill:#91e5a3,color:#000
    style DEPS fill:#91e5a3,color:#000
    style CRATES fill:#91e5a3,color:#000
```

### 표준 Cargo 프로젝트 구조
Cargo는 일관된 프로젝트 구조를 지향하여 협업 효율을 높입니다.

```text
my_project/
|-- Cargo.toml          # 프로젝트의 '설명서' (의존성 및 설정 등)
|-- Cargo.lock          # 의존성 버전의 '스냅샷' (시스템이 자동 관리)
|-- src/
|   |-- main.rs         # 실행 파일의 메인 진입점
|   |-- lib.rs          # 라이브러리 개발 시 루트 파일
|   `-- bin/            # 추가 실행 파일이 필요할 때 활용
|-- tests/              # 외부 통합 테스트 코드
|-- examples/           # 라이브러리 사용법 예제
|-- benches/            # 성능 측정을 위한 벤치마크
`-- target/             # 빌드 결과물이 저장되는 폴더
    |-- debug/          # 디버그 빌드 (빠른 컴파일, 디버깅 용이)
    `-- release/        # 릴리스 빌드 (최적화 적용, 빠른 실행 속도)
```

### 자주 사용하는 Cargo 명령어

```mermaid
graph LR
    subgraph "프로젝트 핵심 생애주기"
        NEW["cargo new [프로젝트명]<br/>새로운 프로젝트 생성"]
        CHECK["cargo check<br/>빠른 문법 및 타입 검사"]
        BUILD["cargo build<br/>전체 프로젝트 컴파일"]
        RUN["cargo run<br/>빌드 후 즉시 실행"]
        TEST["cargo test<br/>단위/통합 테스트 통합 실행"]
        
        NEW --> CHECK
        CHECK --> BUILD
        BUILD --> RUN
        BUILD --> TEST
    end
    
    subgraph "생산성 향상 도구"
        UPDATE["cargo update<br/>의존성 라이브러리 업데이트"]
        FORMAT["cargo fmt<br/>표준 코딩 스타일로 자동 정렬"]
        LINT["cargo clippy<br/>코드 개선 제안 및 린팅"]
        DOC["cargo doc<br/>의존성 포함 API 문서 생성"]
        PUBLISH["cargo publish<br/>패키지를 저장소에 배포"]
    end
    
    subgraph "빌드 시나리오 선택"
        DEBUG["cargo build<br/>(디버그 버전)<br/>빠른 컴파일 속도<br/>디버깅 정보 포함"]
        RELEASE["cargo build --release<br/>(릴리스 버전)<br/>강력한 코드 최적화<br/>상용 서비스용 성능"]
    end
    
    style NEW fill:#a3d5ff,color:#000
    style CHECK fill:#91e5a3,color:#000
    style BUILD fill:#ffa07a,color:#000
    style RUN fill:#ffcc5c,color:#000
    style TEST fill:#c084fc,color:#000
    style DEBUG fill:#94a3b8,color:#000
    style RELEASE fill:#ef4444,color:#000
```

---

# 실습 가이드: Cargo와 크레이트 체험하기
1.  새로운 프로젝트를 생성해 보겠습니다. 터미널에서 다음 명령어를 입력하세요.
    ```bash
    cargo new helloworld
    cd helloworld
    ls -p  # 생성된 파일 구조 확인
    cat Cargo.toml  # 설정 파일 내용 보기
    ```
2.  프로젝트를 실행해 봅니다.
    - 기본 명령인 `cargo run`은 개발용(`debug`) 버전을 만들고 바로 실행합니다.
    - 상용 환경처럼 최적화된 성능을 원한다면 `cargo run --release`를 사용하세요.
3.  빌드 결과물은 `target` 폴더 내의 각 빌드 프로필 폴더(`debug` 또는 `release`)에 생성됩니다.
4.  프로젝트 루트에 생성된 **`Cargo.lock`** 파일은 프로젝트가 사용하는 모든 라이브러리의 정확한 버전을 기록한 '스냅샷'입니다. 이 파일은 시스템이 관리하므로 수동으로 고칠 필요는 없으며, 나중에 상세히 다루게 될 핵심적인 파일 중 하나입니다.
