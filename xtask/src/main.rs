use std::env;
use std::fs;
use std::io::{Read, Write};
use std::net::TcpListener;
use std::path::{Path, PathBuf};
use std::process::Command;

/// (슬러그, 제목, 설명, 카테고리)
const BOOKS: &[(&str, &str, &str, &str)] = &[
    (
        "c-cpp-book",
        "C/C++ 개발자를 위한 Rust",
        "이동 의미론(Move semantics), RAII, FFI, 임베디드, no_std",
        "bridge",
    ),
    (
        "csharp-book",
        "C# 개발자를 위한 Rust",
        "Swift / C# / Java 개발자에게 적합",
        "bridge",
    ),
    (
        "python-book",
        "Python 개발자를 위한 Rust",
        "동적 타이핑 → 정적 타이핑, GIL 없는 병렬 처리",
        "bridge",
    ),
    (
        "async-book",
        "비동기 Rust: Future부터 실전까지",
        "Tokio, 스트림, 취소 안전성(cancellation safety)",
        "deep-dive",
    ),
    (
        "rust-patterns-book",
        "Rust 패턴",
        "Pin, 할당자, 락 프리(lock-free) 구조체, unsafe",
        "advanced",
    ),
    (
        "type-driven-correctness-book",
        "타입 주도 설계와 정확성",
        "타입 상태(Type-state), 팬텀 타입(phantom types), 케이퍼빌리티 토큰",
        "expert",
    ),
    (
        "engineering-book",
        "Rust 엔지니어링 실무",
        "빌드 스크립트, 교차 컴파일, 커버리지, CI/CD",
        "practices",
    ),
];

fn project_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask는 워크스페이스 하위 디렉토리에 위치해야 합니다")
        .to_path_buf()
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    match args.first().map(|s| s.as_str()) {
        Some("build") => cmd_build(),
        Some("serve") => {
            cmd_build();
            cmd_serve();
        }
        Some("deploy") => cmd_deploy(),
        Some("clean") => cmd_clean(),
        Some("--help" | "-h" | "help") | None => print_usage(0),
        Some(other) => {
            eprintln!("알 수 없는 명령어: {other}\n");
            print_usage(1);
        }
    }
}

fn print_usage(code: i32) {
    let stream: &mut dyn Write = if code == 0 {
        &mut std::io::stdout()
    } else {
        &mut std::io::stderr()
    };
    let _ = writeln!(
        stream,
        "\
사용법: cargo xtask <명령어>

명령어:
  build    모든 도서를 site/ 디렉토리에 빌드 (로컬 미리보기용)
  serve    빌드 후 http://localhost:3000 에서 서비스 실행
  deploy   모든 도서를 docs/ 디렉토리에 빌드 (GitHub Pages용)
  clean    site/ 및 docs/ 디렉토리 삭제"
    );
    std::process::exit(code);
}

// ── build (빌드) ────────────────────────────────────────────────────────────

fn cmd_build() {
    build_to("site");
}

fn cmd_deploy() {
    build_to("docs");
    println!("\n배포하려면 docs/ 디렉토리를 커밋하고, GitHub Pages 설정에서 \"Deploy from a branch\" → /docs를 선택하세요.");
}

fn build_to(dir_name: &str) {
    let root = project_root();
    let out = root.join(dir_name);

    if out.exists() {
        fs::remove_dir_all(&out).expect("출력 디렉토리 청소 실패");
    }
    fs::create_dir_all(&out).expect("출력 디렉토리 생성 실패");

    println!("{dir_name}/ 디렉토리에 통합 사이트를 빌드 중...\n");

    let mut ok = 0u32;
    for &(slug, _, _, _) in BOOKS {
        let book_dir = root.join(slug);
        if !book_dir.is_dir() {
            eprintln!("  ✗ {slug}/ 디렉토리를 찾을 수 없어 건너뜁니다");
            continue;
        }
        let dest = out.join(slug);
        let status = Command::new("mdbook")
            .args(["build", "--dest-dir"])
            .arg(&dest)
            .current_dir(&book_dir)
            .status()
            .expect("mdbook 실행 실패 — 설치되어 있습니까?");

        if status.success() {
            println!("  ✓ {slug}");
            ok += 1;
        } else {
            eprintln!("  ✗ {slug} 빌드 실패");
        }
    }
    println!("\n  {ok}/{} 권의 도서가 빌드되었습니다", BOOKS.len());

    write_landing_page(&out);
    println!("\n완료! 출력물은 {dir_name}/ 에 있습니다.");
}

fn category_label(cat: &str) -> &str {
    match cat {
        "bridge" => "입문 (Bridge)",
        "deep-dive" => "심화 (Deep Dive)",
        "advanced" => "고급 (Advanced)",
        "expert" => "전문가 (Expert)",
        "practices" => "실무 (Practices)",
        _ => cat,
    }
}

fn write_landing_page(site: &Path) {
    let cards: String = BOOKS
        .iter()
        .map(|&(slug, title, desc, cat)| {
            let label = category_label(cat);
            format!(
                r#"    <a class="card cat-{cat}" href="{slug}/">
      <h2>{title} <span class="label">{label}</span></h2>
      <p>{desc}</p>
    </a>"#
            )
        })
        .collect::<Vec<_>>()
        .join("\n");

    let html = format!(
        r##"<!DOCTYPE html>
<html lang="ko">
<head>
  <meta charset="utf-8">
  <meta name="viewport" content="width=device-width, initial-scale=1">
  <title>Rust 교육 도서</title>
  <style>
    :root {{
      --bg: #1a1a2e;
      --card-bg: #16213e;
      --accent: #e94560;
      --text: #eee;
      --muted: #a8a8b3;
      --clr-bridge: #4ade80;
      --clr-deep-dive: #22d3ee;
      --clr-advanced: #fbbf24;
      --clr-expert: #c084fc;
      --clr-practices: #2dd4bf;
    }}
    * {{ margin: 0; padding: 0; box-sizing: border-box; }}
    body {{
      font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen, sans-serif;
      background: var(--bg);
      color: var(--text);
      min-height: 100vh;
      display: flex;
      flex-direction: column;
      align-items: center;
      padding: 3rem 1rem;
    }}
    h1 {{ font-size: 2.5rem; margin-bottom: 0.5rem; }}
    h1 span {{ color: var(--accent); }}
    .subtitle {{ color: var(--muted); font-size: 1.1rem; margin-bottom: 1.2rem; }}

    /* Legend (범례) */
    .legend {{
      display: flex; flex-wrap: wrap; gap: 0.6rem 1.4rem;
      justify-content: center; margin-bottom: 2.2rem;
      font-size: 0.8rem; color: var(--muted);
    }}
    .legend-item {{ display: flex; align-items: center; gap: 0.35rem; }}
    .legend-dot {{
      width: 10px; height: 10px; border-radius: 50%; flex-shrink: 0;
    }}

    /* Grid & Cards (그리드 및 카드) */
    .grid {{
      display: grid;
      grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
      gap: 1.5rem;
      max-width: 1000px;
      width: 100%;
    }}
    .card {{
      background: var(--card-bg);
      border-radius: 12px;
      padding: 1.5rem 1.5rem 1.5rem 1.25rem;
      text-decoration: none;
      color: var(--text);
      transition: transform 0.15s, box-shadow 0.15s;
      border: 1px solid rgba(255,255,255,0.05);
      border-left: 4px solid var(--stripe);
    }}
    .card:hover {{
      transform: translateY(-4px);
      box-shadow: 0 8px 25px color-mix(in srgb, var(--stripe) 30%, transparent);
      border-color: rgba(255,255,255,0.08);
      border-left-color: var(--stripe);
    }}
    .card h2 {{ font-size: 1.2rem; margin-bottom: 0.5rem; display: flex; align-items: center; gap: 0.6rem; flex-wrap: wrap; }}
    .card p  {{ color: var(--muted); font-size: 0.9rem; line-height: 1.4; }}

    /* Category colours (카테고리 색상) */
    .cat-bridge     {{ --stripe: var(--clr-bridge); }}
    .cat-deep-dive  {{ --stripe: var(--clr-deep-dive); }}
    .cat-advanced   {{ --stripe: var(--clr-advanced); }}
    .cat-expert     {{ --stripe: var(--clr-expert); }}
    .cat-practices  {{ --stripe: var(--clr-practices); }}

    /* Label pill (라벨 알약) */
    .label {{
      font-size: 0.55rem; font-weight: 700; letter-spacing: 0.08em;
      text-transform: uppercase; padding: 0.15em 0.55em;
      border-radius: 4px; white-space: nowrap; flex-shrink: 0;
      color: var(--bg); background: var(--stripe);
    }}

    footer {{ margin-top: 3rem; color: var(--muted); font-size: 0.85rem; }}
  </style>
</head>
<body>
  <h1>🦀 <span>Rust</span> 교육 도서</h1>
  <p class="subtitle">자신의 배경지식에 맞는 가이드를 선택하세요</p>

  <div class="legend">
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-bridge)"></span> Bridge &mdash; 다른 언어에서 Rust 배우기</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-deep-dive)"></span> Deep Dive &mdash; 심화 학습</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-advanced)"></span> Advanced &mdash; 고급 주제</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-expert)"></span> Expert &mdash; 전문가 수준</span>
    <span class="legend-item"><span class="legend-dot" style="background:var(--clr-practices)"></span> Practices &mdash; 실무 관행</span>
  </div>

  <div class="grid">
{cards}
  </div>
  <footer><a href="https://rust-lang.github.io/mdBook/" style="color:var(--accent)">mdBook</a>으로 제작되었습니다</footer>
</body>
</html>
"##
    );

    let path = site.join("index.html");
    fs::write(&path, html).expect("index.html 쓰기 실패");
    println!("  ✓ index.html");
}

/// `request_target`(HTTP 요청 경로, 예: `/foo/bar?x=1`)을 `site_canon` 아래의 파일로 해석합니다.
/// 순회 시도, 누락된 파일 또는 `site_canon`을 벗어나는 경로(심볼릭 링크)에 대해서는 `None`을 반환합니다.
fn resolve_site_file(site_canon: &Path, request_target: &str) -> Option<PathBuf> {
    let path_only = request_target.split('?').next()?.split('#').next()?;
    let decoded = percent_decode_path(path_only);
    if decoded.as_bytes().contains(&0) {
        return None;
    }
    let rel = decoded.trim_start_matches('/');
    let mut file_path = site_canon.to_path_buf();
    if !rel.is_empty() {
        for seg in rel.split('/').filter(|s| !s.is_empty()) {
            if seg == ".." {
                return None;
            }
            file_path.push(seg);
        }
    }
    if file_path.is_dir() {
        file_path.push("index.html");
    }
    let real = fs::canonicalize(&file_path).ok()?;
    if !real.starts_with(site_canon) {
        return None;
    }
    real.is_file().then_some(real)
}

fn hex_val(c: u8) -> Option<u8> {
    match c {
        b'0'..=b'9' => Some(c - b'0'),
        b'a'..=b'f' => Some(c - b'a' + 10),
        b'A'..=b'F' => Some(c - b'A' + 10),
        _ => None,
    }
}

fn percent_decode_path(input: &str) -> String {
    let mut decoded = Vec::with_capacity(input.len());
    let b = input.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' && i + 2 < b.len() {
            if let (Some(hi), Some(lo)) = (hex_val(b[i + 1]), hex_val(b[i + 2])) {
                decoded.push(hi << 4 | lo);
                i += 3;
                continue;
            }
        }
        decoded.push(b[i]);
        i += 1;
    }
    String::from_utf8_lossy(&decoded).into_owned()
}

// ── serve (서비스) ───────────────────────────────────────────────────────────

fn cmd_serve() {
    let site = project_root().join("site");
    let site_canon = fs::canonicalize(&site).expect(
        "site/ 폴더를 찾을 수 없습니다 — `cargo xtask build`를 먼저 실행하세요 (예: `cargo xtask serve`는 빌드를 자동으로 실행합니다)",
    );
    let addr = "127.0.0.1:3000";
    let listener = TcpListener::bind(addr).expect("3000번 포트 바인딩 실패");

    // cargo가 에러를 보고하지 않도록 Ctrl+C를 우아하게 처리합니다.
    ctrlc_exit();

    println!("\nhttp://{addr} 에서 서비스 중 (중단하려면 Ctrl+C)");

    for stream in listener.incoming() {
        let Ok(mut stream) = stream else { continue };
        let mut buf = [0u8; 4096];
        let n = stream.read(&mut buf).unwrap_or(0);
        let request = String::from_utf8_lossy(&buf[..n]);

        let path = request
            .lines()
            .next()
            .and_then(|line| line.split_whitespace().nth(1))
            .unwrap_or("/");

        if let Some(file_path) = resolve_site_file(&site_canon, path) {
            let body = fs::read(&file_path).unwrap_or_default();
            let mime = guess_mime(&file_path);
            let header = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: {mime}\r\nContent-Length: {}\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(&body);
        } else {
            let body = b"404 Not Found";
            let header = format!(
                "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n",
                body.len()
            );
            let _ = stream.write_all(header.as_bytes());
            let _ = stream.write_all(body);
        }
    }
}

/// OS가 STATUS_CONTROL_C_EXIT와 함께 종료되지 않도록
/// 깔끔하게 종료(코드 0)하는 Ctrl+C 핸들러를 설치합니다.
fn ctrlc_exit() {
    unsafe {
        libc_set_handler();
    }
}

#[cfg(windows)]
unsafe fn libc_set_handler() {
    // Windows API를 통해 SetConsoleCtrlHandler 설정
    extern "system" {
        fn SetConsoleCtrlHandler(
            handler: Option<unsafe extern "system" fn(u32) -> i32>,
            add: i32,
        ) -> i32;
    }
    unsafe extern "system" fn handler(_ctrl_type: u32) -> i32 {
        std::process::exit(0);
    }
    unsafe {
        SetConsoleCtrlHandler(Some(handler), 1);
    }
}

#[cfg(not(windows))]
unsafe fn libc_set_handler() {
    // Unix에서 libc를 통해 SIGINT 등록
    extern "C" {
        fn signal(sig: i32, handler: extern "C" fn(i32)) -> usize;
    }
    extern "C" fn handler(_sig: i32) {
        std::process::exit(0);
    }
    unsafe {
        signal(2 /* SIGINT */, handler);
    }
}

fn guess_mime(path: &Path) -> &'static str {
    match path.extension().and_then(|e| e.to_str()) {
        Some("html") => "text/html; charset=utf-8",
        Some("css") => "text/css",
        Some("js") => "application/javascript",
        Some("svg") => "image/svg+xml",
        Some("png") => "image/png",
        Some("jpg" | "jpeg") => "image/jpeg",
        Some("woff2") => "font/woff2",
        Some("woff") => "font/woff",
        Some("json") => "application/json",
        _ => "application/octet-stream",
    }
}

// ── clean (청소) ────────────────────────────────────────────────────────────

fn cmd_clean() {
    let root = project_root();
    for dir_name in ["site", "docs"] {
        let dir = root.join(dir_name);
        if dir.exists() {
            fs::remove_dir_all(&dir).expect("디렉토리 삭제 실패");
            println!("{dir_name}/ 디렉토리를 삭제했습니다.");
        }
    }
}