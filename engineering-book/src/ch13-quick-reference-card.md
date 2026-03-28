# 빠른 참조 카드 (Quick Reference Card)

### 명령어 요약: 한눈에 보는 가이드

```bash
# ─── 빌드 스크립트 (Build Scripts) ───
cargo build                          # build.rs를 먼저 컴파일한 후 크레이트 빌드
cargo build -vv                      # 상세 출력 — build.rs의 모든 출력을 표시

# ─── 교차 컴파일 (Cross-Compilation) ───
rustup target add x86_64-unknown-linux-musl
cargo build --release --target x86_64-unknown-linux-musl
cargo zigbuild --release --target x86_64-unknown-linux-gnu.2.17
cross build --release --target aarch64-unknown-linux-gnu

# ─── 벤치마킹 (Benchmarking) ───
cargo bench                          # 모든 벤치마크 실행
cargo bench -- parse                 # "parse"가 포함된 벤치마크만 실행
cargo flamegraph -- --args           # 바이너리에서 플레임그래프 생성
perf record -g ./target/release/bin  # perf 데이터 기록
perf report                          # 대화형으로 perf 결과 확인

# ─── 커버리지 (Coverage) ───
cargo llvm-cov --html                # HTML 보고서 생성
cargo llvm-cov --lcov --output-path lcov.info
cargo llvm-cov --workspace --fail-under-lines 80
cargo tarpaulin --out Html           # 대안 도구 (Linux용)

# ─── 안전성 검증 (Safety Verification) ───
cargo +nightly miri test             # Miri에서 테스트 실행 (UB 감지)
valgrind --leak-check=full ./target/debug/binary  # 메모리 누수 확인
RUSTFLAGS="-Zsanitizer=address" cargo +nightly test -Zbuild-std --target x86_64-unknown-linux-gnu

# ─── 감사 및 공급망 (Audit & Supply Chain) ───
cargo audit                          # 알려진 취약점 스캔
cargo audit --deny warnings          # 취약점 발견 시 빌드 실패
cargo deny check                     # 라이선스 + 보안 + 소스 검증
cargo vet                            # 공급망 신뢰성 검증
cargo outdated --workspace           # 오래된 의존성 찾기
cargo semver-checks                  # 하위 호환성 깨짐 감지
cargo geiger                         # 의존성 트리의 unsafe 코드 개수 확인

# ─── 바이너리 최적화 (Binary Optimization) ───
cargo bloat --release --crates       # 크레이트별 바이너리 크기 기여도 확인
cargo bloat --release -n 20          # 가장 큰 함수 20개 확인
cargo +nightly udeps --workspace     # 사용되지 않는 의존성 찾기
cargo machete                        # 빠른 사용 중단 의존성 감지
cargo expand --lib module::name      # 매크로 확장 결과 확인
cargo msrv find                      # 최소 지원 Rust 버전(MSRV) 확인
cargo clippy --fix --workspace --allow-dirty  # 린트 경고 자동 수정

# ─── 컴파일 타임 최적화 (Compile-Time) ───
export RUSTC_WRAPPER=sccache         # 공용 컴파일 캐시 설정
sccache --show-stats                 # 캐시 적중 통계 표시
cargo nextest run                    # 더 빠른 병렬 테스트 실행 도구

# ─── 플랫폼 엔지니어링 (Platform Engineering) ───
cargo check --target thumbv7em-none-eabihf   # no_std 빌드 확인
cargo build --target x86_64-pc-windows-gnu   # Linux에서 Windows용 빌드
cargo xwin build --target x86_64-pc-windows-msvc  # MSVC ABI 기반 교차 빌드

# ─── 릴리스 (Release) ───
cargo release patch --dry-run        # 릴리스 예행 연습
cargo release patch --execute        # 버전 업, 커밋, 태그, 게시 실행
cargo dist plan                      # 배포용 아티팩트(바이너리) 생성 계획 확인
```

---

### 의사결정 테이블: 언제 어떤 도구를 사용할 것인가?

| 목표 | 도구 | 사용 시점 |
|------|------|-------------|
| git 해시 / 빌드 정보 포함 | `build.rs` | 바이너리에 추적성이 필요할 때 |
| Rust와 C 코드 함께 컴파일 | `cc` 크레이트 | 소규모 C 라이브러리와 연결할 때 |
| 스키마에서 코드 생성 | `prost-build` | Protobuf, gRPC 스텁 생성이 필요할 때 |
| 시스템 라이브러리 링크 | `pkg-config` | OpenSSL, libpci, systemd와 연결할 때 |
| 정적 Linux 바이너리 | `musl` 타겟 | 컨테이너나 클라우드 배포 시 |
| 고령화된 glibc 타겟 | `cargo-zigbuild` | CentOS 7, RHEL 7 호환성이 필요할 때 |
| ARM 서버용 빌드 | `cross` / `zig` | Graviton, Ampere 서버용 배포 시 |
| 통계적 벤치마킹 | Criterion.rs | 정밀한 성능 회귀 감지가 필요할 때 |
| 빠른 성능 확인 | Divan | 개발 중 프로파일링 전 가볍게 확인 시 |
| 핫스팟 식별 | `cargo flamegraph` | 어느 함수가 느린지 시각화할 때 |
| 라인/브랜치 커버리지 | `cargo-llvm-cov` | CI 게이트 설정 및 사각지대 분석 시 |
| Rust UB 감지 | Miri | 순수 Rust `unsafe` 코드를 검증할 때 |
| C FFI 메모리 안전성 | Valgrind | Rust와 C가 혼합된 코드베이스 검증 시 |
| 데이터 레이스 감지 | TSan / Miri | 동시성 기반 코드의 안전성을 확인할 때 |
| 로컬 자동화 | `cargo make` | 다단계 로컬 검증 과정을 하나로 묶을 때 |
| 자동 릴리스 | `cargo-dist` | 멀티 플랫폼 바이너리 배포를 자동화할 때 |
| 의존성 취약점 점검 | `cargo-audit` | 보안 감사가 필수인 프로젝트일 때 |
| 라이선스 준수 | `cargo-deny` | 상용 프로젝트의 라이선스 정책을 관리할 때 |
| 바이너리 크기 분석 | `cargo-bloat` | 임베디드나 용량 제한 환경용 빌드 시 |
| 하위 호환성 체크 | `cargo-semver` | 라이브러리 퍼블리싱 전 브레이킹 체인지 확인 시 |

