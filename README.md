<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**라이선스** 이 프로젝트는 [MIT 라이선스](LICENSE)와 [크리에이티브 커먼즈 저작자표시 4.0 국제(CC-BY-4.0)](LICENSE-DOCS)에 따라 이중 라이선스가 부여됩니다.

</div>

<div style="background-color: #d9d9d9; padding: 16px; border-radius: 6px; color: #000000;">

**상표** 이 프로젝트에는 프로젝트, 제품 또는 서비스에 대한 상표 또는 로고가 포함될 수 있습니다. Microsoft 상표 또는 로고의 공인된 사용은 [Microsoft의 상표 및 브랜드 가이드라인](https://www.microsoft.com/en-us/legal/intellectualproperty/trademarks/usage/general)을 준수해야 합니다. 이 프로젝트의 수정된 버전에서 Microsoft 상표 또는 로고를 사용하는 것이 혼동을 일으키거나 Microsoft의 후원을 암시해서는 안 됩니다. 타사 상표 또는 로고의 사용은 해당 타사의 정책을 따릅니다.

</div>

# Rust 교육 도서 (Rust Training Books)

다양한 프로그래밍 배경을 가진 분들을 위한 7개의 Rust 교육 과정과 함께, 비동기(async), 고급 패턴 및 엔지니어링 실무에 대한 심층 분석을 제공합니다.

본 자료는 독창적인 콘텐츠와 Rust 생태계의 우수한 리소스에서 영감을 얻은 아이디어 및 예시를 결합한 것입니다. 목표는 책, 블로그, 컨퍼런스 발표, 비디오 시리즈 등에 흩어져 있는 지식을 교육적으로 구조화된 하나의 응집력 있는 경험으로 엮어 심층적이고 기술적으로 정확한 커리큘럼을 제공하는 것입니다.

> **면책 조항:** 이 도서들은 교육용 자료이며 공식적인 기술 참조서가 아닙니다. 정확성을 기하기 위해 노력하고 있으나, 중요한 세부 사항은 항상 [공식 Rust 문서](https://doc.rust-lang.org/) 및 [Rust 참조서(Rust Reference)](https://doc.rust-lang.org/reference/)를 통해 확인하시기 바랍니다.

### 영감 및 감사의 글 (Inspirations & Acknowledgments)

- [**The Rust Programming Language**](https://doc.rust-lang.org/book/) — 모든 것의 기초가 되는 도서
- [**Jon Gjengset**](https://www.youtube.com/c/JonGjengset) — 고급 Rust 내부 구조에 대한 심층 분석 스트림, `Crust of Rust` 시리즈
- [**withoutboats**](https://without.boats/blog/) — 비동기 설계, `Pin`, 그리고 퓨처(futures) 모델
- [**fasterthanlime (Amos)**](https://fasterthanli.me/) — 기초 원리부터 시작하는 시스템 프로그래밍, 긴 호흡의 탐구형 글
- [**Mara Bos**](https://marabos.nl/) — *Rust Atomics and Locks*, 동시성 기본 요소(concurrency primitives)
- [**Aleksey Kladov (matklad)**](https://matklad.github.io/) — Rust analyzer 통찰, API 설계, 에러 처리 패턴
- [**Niko Matsakis**](https://smallcultfollowing.com/babysteps/) — 언어 설계, 빌려오기 검사기(borrow checker) 내부 구조, Polonius
- [**Rust by Example**](https://doc.rust-lang.org/rust-by-example/) 및 [**Rustonomicon**](https://doc.rust-lang.org/nomicon/) — 실용적인 패턴 및 unsafe 심층 분석
- [**This Week in Rust**](https://this-week-in-rust.org/) — 많은 예시의 토대가 된 커뮤니티의 발견들
- …그리고 이 자료를 만드는 데 정보를 제공해 주신 블로그 포스트, 컨퍼런스 발표, RFC, 포럼 토론 등 **전체 Rust 커뮤니티**의 수많은 분들께 깊은 감사를 드립니다.

## 📖 학습 시작하기

본인의 배경에 맞는 도서를 선택하세요. 도서들은 난이도별로 그룹화되어 있어 학습 경로를 계획할 수 있습니다:

| 수준 | 설명 |
|-------|-------------|
| 🟢 **Bridge** | 다른 언어에서 Rust로 전환하는 분들을 위한 입문 단계 |
| 🔵 **Deep Dive** | 주요 Rust 하위 시스템에 대한 집중 탐구 |
| 🟡 **Advanced** | 숙련된 Rust 개발자를 위한 패턴 및 기법 |
| 🟣 **Expert** | 최첨단 타입 수준(type-level) 및 정확성 기법 |
| 🟤 **Practices** | 엔지니어링, 도구 사용 및 운영 환경 준비 |

| 도서 | 수준 | 대상 독자 |
|------|-------|-------------|
| [**C/C++ 개발자를 위한 Rust**](c-cpp-book/src/SUMMARY.md) | 🟢 Bridge | 이동 의미론(Move semantics), RAII, FFI, 임베디드, no_std |
| [**C# 개발자를 위한 Rust**](csharp-book/src/SUMMARY.md) | 🟢 Bridge | Swift / C# / Java 사용 → 소유권(ownership) 및 타입 시스템 |
| [**Python 개발자를 위한 Rust**](python-book/src/SUMMARY.md) | 🟢 Bridge | 동적 타입 → 정적 타입, GIL 없는 동시성 |
| [**비동기 Rust (Async Rust)**](async-book/src/SUMMARY.md) | 🔵 Deep Dive | Tokio, 스트림(streams), 취소 안전성(cancellation safety) |
| [**Rust 패턴 (Rust Patterns)**](rust-patterns-book/src/SUMMARY.md) | 🟡 Advanced | Pin, 할당자(allocators), 락-프리(lock-free) 구조, unsafe |
| [**타입 기반 정확성 (Type-Driven Correctness)**](type-driven-correctness-book/src/SUMMARY.md) | 🟣 Expert | 타입-상태(Type-state), 팬텀 타입(phantom types), 기능 토큰(capability tokens) |
| [**Rust 엔지니어링 실무**](engineering-book/src/SUMMARY.md) | 🟤 Practices | 빌드 스크립트, 교차 컴파일(cross-compilation), CI/CD, Miri |

각 도서는 Mermaid 다이어그램, 편집 가능한 Rust 플레이그라운드, 연습 문제, 전체 텍스트 검색을 포함한 15~16개의 장으로 구성되어 있습니다.

> **팁:** GitHub에서 직접 마크다운 소스를 읽거나, 사이드바 탐색과 검색 기능이 있는 [GitHub Pages 사이트](https://microsoft.github.io/RustTraining/)에서 렌더링된 콘텐츠를 찾아보실 수 있습니다.
>
> **로컬 서버 실행:** 최상의 읽기 환경(장 사이의 키보드 탐색, 즉시 검색, 오프라인 액세스)을 위해 저장소를 클론하고 다음 명령을 실행하세요:
> ```bash
> # 아직 Rust가 설치되지 않았다면 rustup을 통해 설치하세요:
> # https://rustup.rs/
>
> cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
> cargo xtask serve          # 모든 도서를 빌드하고 로컬 서버를 엽니다
> ```

---

## 🔧 관리자용 안내 (For Maintainers)

<details>
<summary>로컬에서 도서 빌드, 서버 실행 및 편집하기</summary>

### 사전 요구 사항

아직 설치하지 않았다면 [**rustup**을 통해 Rust를 설치](https://rustup.rs/)한 후, 다음 명령을 실행하세요:

```bash
cargo install mdbook@0.4.52 mdbook-mermaid@0.14.0
```

### 빌드 및 로컬 서버 실행

```bash
cargo xtask build               # 모든 도서를 site/ 디렉토리에 빌드 (로컬 미리보기)
cargo xtask serve               # http://localhost:3000 에서 빌드 및 서버 실행
cargo xtask deploy              # GitHub Pages용으로 모든 도서를 docs/ 에 빌드
cargo xtask clean               # site/ 및 docs/ 디렉토리 삭제
```

개별 도서만 빌드하거나 서버를 실행하려면:

```bash
cd c-cpp-book && mdbook serve --open    # http://localhost:3000
```

### 배포

사이트는 `.github/workflows/pages.yml`을 통해 `master` 브랜치에 푸시될 때 GitHub Pages로 자동 배포됩니다. 수동 작업은 필요하지 않습니다.

</details>
