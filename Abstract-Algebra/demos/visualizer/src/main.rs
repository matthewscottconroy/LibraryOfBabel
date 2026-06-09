/// Standalone visualizer CLI.
/// Usage: visualizer [--chapter N] [--cmd CMD] [--format svg|dot|tex|ascii|png|jpeg]
///                   [--output FILE] [--width W] [--height H]
///        visualizer list
///        visualizer help

use common::*;
use std::process;
use resvg::usvg;

fn main() {
    let raw: Vec<String> = std::env::args().collect();
    let args: Vec<&str> = raw.iter().map(|s| s.as_str()).collect();

    if args.len() < 2 || args[1] == "help" || args[1] == "--help" {
        print_usage();
        return;
    }

    if args[1] == "list" {
        print_chapter_list();
        return;
    }

    let chapter  = flag_val(&args, "--chapter").unwrap_or("0");
    let cmd      = flag_val(&args, "--cmd").unwrap_or("demo");
    let fmt      = flag_val(&args, "--format").unwrap_or("svg");
    let out_file = flag_val(&args, "--output");
    let width    = flag_val(&args, "--width").and_then(|s| s.parse::<f64>().ok()).unwrap_or(800.0);
    let height   = flag_val(&args, "--height").and_then(|s| s.parse::<f64>().ok()).unwrap_or(600.0);

    let ch_n: usize = match chapter.parse() {
        Ok(n) if n >= 1 && n <= 54 => n,
        _ => {
            eprintln!("error: --chapter must be 1–54");
            process::exit(1);
        }
    };

    let ch_bin = chapter_binary(ch_n);

    match fmt {
        "png" | "jpeg" | "jpg" => {
            // Render SVG first, then rasterize
            let svg_content = run_chapter_cmd(&ch_bin, cmd, "svg", &args[2..]);
            match rasterize(&svg_content, fmt, width as u32, height as u32) {
                Ok(bytes) => {
                    match out_file {
                        Some(f) => std::fs::write(f, &bytes).expect("write failed"),
                        None    => {
                            use std::io::Write;
                            std::io::stdout().write_all(&bytes).expect("write failed");
                        }
                    }
                }
                Err(e) => { eprintln!("rasterize error: {e}"); process::exit(1); }
            }
        }
        _ => {
            let content = run_chapter_cmd(&ch_bin, cmd, fmt, &args[2..]);
            match out_file {
                Some(f) => std::fs::write(f, &content).expect("write failed"),
                None    => print!("{content}"),
            }
        }
    }
}

fn flag_val<'a>(args: &[&'a str], flag: &str) -> Option<&'a str> {
    args.windows(2).find(|w| w[0] == flag).map(|w| w[1])
}

fn chapter_binary(n: usize) -> String {
    let names = [
        "ch01-logic", "ch02-sets", "ch03-cardinality", "ch04-vector-spaces",
        "ch05-bases-dimension", "ch06-linear-maps", "ch07-matrices", "ch08-determinants",
        "ch09-eigentheory", "ch10-canonical-forms", "ch11-inner-products", "ch12-multilinear",
        "ch13-groups", "ch14-cosets", "ch15-homomorphisms", "ch16-group-actions",
        "ch17-sylow", "ch18-group-structure", "ch19-abelian-groups", "ch20-rings",
        "ch21-ideals", "ch22-divisibility", "ch23-polynomials", "ch24-commutative-algebra",
        "ch25-modules", "ch26-projective-injective", "ch27-structure-theorem",
        "ch28-tensor-products", "ch29-field-extensions", "ch30-normal-separable",
        "ch31-galois-theory", "ch32-galois-applications", "ch33-categories", "ch34-yoneda",
        "ch35-adjoints", "ch36-limits-colimits", "ch37-abelian-categories",
        "ch38-chain-complexes", "ch39-resolutions", "ch40-ext-tor", "ch41-spectral-sequences",
        "ch42-representations", "ch43-complete-reducibility", "ch44-character-theory",
        "ch45-induced-representations", "ch46-lie-groups", "ch47-lie-algebras",
        "ch48-solvable-semisimple", "ch49-root-systems", "ch50-highest-weight",
        "ch51-modular", "ch52-geometric", "ch53-quantum-groups", "ch54-langlands",
    ];
    names[n - 1].to_string()
}

fn run_chapter_cmd(bin: &str, cmd: &str, fmt: &str, extra: &[&str]) -> String {
    let mut cmd_args = vec!["--run", cmd, "--format", fmt];
    cmd_args.extend_from_slice(extra);

    let output = process::Command::new(bin)
        .args(&cmd_args)
        .output()
        .unwrap_or_else(|_| {
            // Try cargo run as fallback
            let ch_dir = format!(
                "/home/matthewscott/Development/Abstract-Algebra/demos/{}",
                bin
            );
            process::Command::new("cargo")
                .args(["run", "--manifest-path",
                       &format!("{ch_dir}/Cargo.toml"), "--"])
                .args(&cmd_args)
                .output()
                .unwrap_or_else(|e| {
                    eprintln!("error running chapter: {e}");
                    process::exit(1);
                })
        });

    String::from_utf8_lossy(&output.stdout).into_owned()
}

fn rasterize(svg_src: &str, fmt: &str, w: u32, h: u32) -> Result<Vec<u8>, String> {
    use image::ImageEncoder;
    use image::codecs::{png::PngEncoder, jpeg::JpegEncoder};

    // Parse and render via resvg
    let options = usvg::Options::default();
    let tree = usvg::Tree::from_str(svg_src, &options)
        .map_err(|e| format!("SVG parse error: {e}"))?;

    let scale_x = w as f32 / tree.size().width();
    let scale_y = h as f32 / tree.size().height();
    let scale   = scale_x.min(scale_y);

    let mut pixmap = resvg::tiny_skia::Pixmap::new(w, h)
        .ok_or("failed to allocate pixmap")?;
    pixmap.fill(resvg::tiny_skia::Color::WHITE);

    resvg::render(
        &tree,
        resvg::tiny_skia::Transform::from_scale(scale, scale),
        &mut pixmap.as_mut(),
    );

    // Convert RGBA pixmap to RGB image
    let rgba = pixmap.data();
    let rgb: Vec<u8> = rgba.chunks(4).flat_map(|p| [p[0], p[1], p[2]]).collect();

    let mut buf = Vec::new();
    match fmt {
        "png" => {
            PngEncoder::new(&mut buf)
                .write_image(&rgb, w, h, image::ColorType::Rgb8.into())
                .map_err(|e| e.to_string())?;
        }
        _ => {
            JpegEncoder::new_with_quality(&mut buf, 90)
                .encode(&rgb, w, h, image::ColorType::Rgb8.into())
                .map_err(|e| e.to_string())?;
        }
    }
    Ok(buf)
}

fn print_usage() {
    println!("visualizer — Abstract Algebra demo renderer");
    println!();
    println!("USAGE:");
    println!("  visualizer list");
    println!("  visualizer --chapter N [--cmd CMD] [--format FMT] [--output FILE]");
    println!("             [--width W] [--height H]");
    println!();
    println!("FORMATS:  svg (default)  dot  tex  ascii  png  jpeg");
    println!("COMMANDS: demo  (chapter-specific commands vary — run the chapter REPL for help)");
    println!();
    println!("EXAMPLES:");
    println!("  visualizer --chapter 13 --cmd cayley --format svg --output cayley.svg");
    println!("  visualizer --chapter 31 --cmd galois --format dot | dot -Tpdf -o galois.pdf");
    println!("  visualizer --chapter 7  --cmd matrix --format tex --output matrix.tex");
    println!("  visualizer --chapter 1  --format png --output preview.png");
}

fn print_chapter_list() {
    let chapters = [
        ( 1, "Logic & Proofs"),
        ( 2, "Sets & Functions"),
        ( 3, "Cardinality"),
        ( 4, "Vector Spaces"),
        ( 5, "Bases & Dimension"),
        ( 6, "Linear Maps"),
        ( 7, "Matrices"),
        ( 8, "Determinants"),
        ( 9, "Eigentheory"),
        (10, "Canonical Forms"),
        (11, "Inner Products"),
        (12, "Multilinear Algebra"),
        (13, "Groups"),
        (14, "Cosets & Lagrange"),
        (15, "Homomorphisms"),
        (16, "Group Actions"),
        (17, "Sylow Theorems"),
        (18, "Group Structure"),
        (19, "Abelian Groups"),
        (20, "Rings"),
        (21, "Ideals"),
        (22, "Divisibility"),
        (23, "Polynomials"),
        (24, "Commutative Algebra"),
        (25, "Modules"),
        (26, "Projective & Injective"),
        (27, "Structure Theorem"),
        (28, "Tensor Products"),
        (29, "Field Extensions"),
        (30, "Normal & Separable"),
        (31, "Galois Theory"),
        (32, "Galois Applications"),
        (33, "Categories"),
        (34, "Yoneda Lemma"),
        (35, "Adjoints"),
        (36, "Limits & Colimits"),
        (37, "Abelian Categories"),
        (38, "Chain Complexes"),
        (39, "Resolutions"),
        (40, "Ext & Tor"),
        (41, "Spectral Sequences"),
        (42, "Representations"),
        (43, "Complete Reducibility"),
        (44, "Character Theory"),
        (45, "Induced Representations"),
        (46, "Lie Groups"),
        (47, "Lie Algebras"),
        (48, "Solvable & Semisimple"),
        (49, "Root Systems"),
        (50, "Highest Weight"),
        (51, "Modular Representation"),
        (52, "Geometric Representation"),
        (53, "Quantum Groups"),
        (54, "Langlands Program"),
    ];
    println!("Chapter  Title");
    println!("{}", "-".repeat(50));
    for (n, title) in &chapters {
        println!("  {:>2}     {}", n, title);
    }
}
