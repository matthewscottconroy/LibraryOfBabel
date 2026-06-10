use std::collections::HashMap;
use std::io::{self, BufRead, Write};

const BOLD: &str = "\x1b[1m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const YELLOW: &str = "\x1b[33m";
const RED: &str = "\x1b[31m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

fn bold(s: &str) -> String { format!("{BOLD}{s}{RESET}") }
fn cyan(s: &str) -> String { format!("{CYAN}{s}{RESET}") }
fn green(s: &str) -> String { format!("{GREEN}{s}{RESET}") }
fn yellow(s: &str) -> String { format!("{YELLOW}{s}{RESET}") }
fn red(s: &str) -> String { format!("{RED}{s}{RESET}") }
fn dim(s: &str) -> String { format!("{DIM}{s}{RESET}") }

// ── Encode-Decode method ──────────────────────────────────────────────────────
//
// The encode-decode method proves π₁(S¹) ≅ ℤ:
//
// 1. Define  code : S¹ → 𝒰  by:
//      code(base) = ℤ
// 2. Define  encode : base = x → code(x)  by:
//      encode(refl) = 0
//      encode(p · loop) = encode(p) + 1
//      encode(p · loop⁻¹) = encode(p) - 1
// 3. Define  decode : code(x) → base = x  by:
//      decode(n) = loop^n
// 4. Show encode and decode are inverse (η and ε)
//
// This pattern generalizes: to compute π₁(X), find a type family
// code : X → 𝒰 such that code(base) has the right group structure.

// ── S¹ path representation ────────────────────────────────────────────────────

fn encode_path(path: &str) -> Option<i64> {
    // Parse path expressions like: loop loop^-1 loop^3 refl
    let mut winding: i64 = 0;
    for token in path.split_whitespace() {
        match token {
            "refl" | "id" => {}
            "loop" => winding += 1,
            "loop^-1" | "loop_inv" => winding -= 1,
            t if t.starts_with("loop^") => {
                let n: i64 = t[5..].parse().ok()?;
                winding += n;
            }
            t if t.starts_with("loop^(") && t.ends_with(')') => {
                let n: i64 = t[6..t.len()-1].parse().ok()?;
                winding += n;
            }
            _ => return None,
        }
    }
    Some(winding)
}

fn decode_to_path(n: i64) -> String {
    match n {
        0 => "refl".into(),
        1 => "loop".into(),
        -1 => "loop^-1".into(),
        n if n > 0 => format!("loop^{n}"),
        n => format!("loop^({n})"),
    }
}

// ── ℕ encode-decode: code(n) = {m : ℕ | m = n} = Fin(n+1) at n ───────────────

fn encode_nat_path(from: u64, path: &str) -> Option<u64> {
    // In ℕ, (m = n) is Bool (either reflexivity or nothing)
    // For m = n: if m == n, refl is the only path, encode → true (1 proof)
    //            if m ≠ n, no path, encode → false (0 proofs) [Void]
    match path {
        "refl" => Some(from),
        _ => None,
    }
}

// ── Bool encode-decode: paths in Bool ────────────────────────────────────────

fn encode_bool_path(point: &str, path: &str) -> Option<String> {
    // In Bool: paths at true are just {refl_true}, paths at false are {refl_false}
    // Bool is a set — identity types have at most one element
    match (point, path) {
        ("true", "refl") => Some("true".into()),
        ("false", "refl") => Some("false".into()),
        _ => None,
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    named_paths: HashMap<String, i64>, // name -> winding number (S¹ paths)
    encode_results: Vec<(String, i64)>,
}

impl Sandbox {
    fn new() -> Self { Sandbox { named_paths: HashMap::new(), encode_results: vec![] } }

    fn print_help() {
        println!("{}", bold("── S¹ Encode-Decode ────────────────────────────────────────────────"));
        println!("  {}  loop loop^-1     — encode path to integer", cyan("encode"));
        println!("  {}  -3              — decode integer to path", cyan("decode"));
        println!("  {}  loop^5 loop^-3  — encode then decode (round-trip)", cyan("roundtrip"));
        println!("  {}  name = ...      — name a path expression", cyan("let"));
        println!("{}", bold("── The Method Explained ────────────────────────────────────────────"));
        println!("  {}          — explain code/encode/decode for S¹", cyan("explain"));
        println!("  {}          — show the proof that encode∘decode = id", cyan("eta"));
        println!("  {}          — show the proof that decode∘encode = id", cyan("epsilon"));
        println!("{}", bold("── Other Spaces ─────────────────────────────────────────────────────"));
        println!("  {}  true refl        — encode path in Bool", cyan("bool-encode"));
        println!("  {}  3 refl           — encode path in ℕ (only refl works)", cyan("nat-encode"));
        println!("  {}          — discuss πₙ(Sⁿ) general pattern", cyan("spheres"));
        println!("{}", bold("── Composition (in encode/ℤ space) ─────────────────────────────────"));
        println!("  {}  3 -2            — multiply in ℤ (= compose in π₁)", cyan("mul"));
        println!("  {}  5              — inverse in ℤ", cyan("inv"));
        println!("  {}  loop^3          — verify eta: encode(decode(encode(p))) = encode(p)", cyan("verify"));
    }

    fn handle(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() { return true; }
        if trimmed == "quit" || trimmed == "exit" || trimmed == "q" { return false; }
        if trimmed == "help" || trimmed == "?" { Self::print_help(); return true; }

        let parts: Vec<&str> = trimmed.splitn(2, ' ').collect();
        let cmd = parts[0];
        let rest = if parts.len() > 1 { parts[1].trim() } else { "" };

        match cmd {
            "encode" => {
                let expr = if let Some(n) = self.named_paths.get(rest) {
                    println!("  Using named path {} = loop^{}", rest, n);
                    decode_to_path(*n)
                } else { rest.to_string() };
                match encode_path(&expr) {
                    Some(n) => {
                        println!("  encode({}) = {}", cyan(&expr), bold(&cyan(&n.to_string())));
                        println!("  {} This is in {} = code(base)", dim("↓"), dim("ℤ"));
                        self.encode_results.push((expr, n));
                    }
                    None => println!("  {} Parse error in path: {expr}", red("✗")),
                }
            }
            "decode" => {
                let n: i64 = rest.parse().unwrap_or_else(|_| *self.named_paths.get(rest).unwrap_or(&0));
                let path = decode_to_path(n);
                println!("  decode({}) = {}", cyan(&n.to_string()), bold(&cyan(&path)));
                println!("  {} This loop has winding number {}", dim("↓"), n);
            }
            "roundtrip" => {
                match encode_path(rest) {
                    Some(n) => {
                        let decoded = decode_to_path(n);
                        let re_encoded = encode_path(&decoded).unwrap_or(n);
                        println!("  Original path: {}", cyan(rest));
                        println!("  encode: {}", bold(&n.to_string()));
                        println!("  decode(encode(...)): {}", bold(&cyan(&decoded)));
                        println!("  re-encode: {}", re_encoded);
                        if n == re_encoded { println!("  {} Round trip successful! encode ∘ decode ∘ encode = encode", green("✓")); }
                        else { println!("  {} Round trip failed (unexpected)", red("✗")); }
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "let" => {
                if let Some((name, expr)) = rest.split_once('=') {
                    let name = name.trim();
                    match encode_path(expr.trim()) {
                        Some(n) => {
                            println!("  {} = {} = loop^{}", cyan(name), expr.trim(), n);
                            self.named_paths.insert(name.to_string(), n);
                        }
                        None => println!("  {} Parse error", red("✗")),
                    }
                }
            }
            "eta" => {
                println!("{}", bold("── η : encode ∘ decode = id_ℤ ──────────────────────────────────────"));
                println!("  We need: encode(decode(n)) = n  for all n : ℤ");
                println!();
                println!("  decode(n) = loop^n");
                println!("  encode(loop^n) = n  (by definition of encode)");
                println!();
                println!("  Proof: encode(decode(n)) = encode(loop^n) = n  ∎");
                println!();
                println!("  Let's verify for a few values:");
                for n in [-3i64, -1, 0, 1, 3] {
                    let p = decode_to_path(n);
                    let e = encode_path(&p).unwrap_or(0);
                    let mark = if e == n { green("✓") } else { red("✗") };
                    println!("    encode(decode({})) = encode({}) = {}  {}", n, p, e, mark);
                }
            }
            "epsilon" => {
                println!("{}", bold("── ε : decode ∘ encode = id_{Ω S¹} ────────────────────────────────────"));
                println!("  We need: decode(encode(p)) = p  for all p : base = base");
                println!();
                println!("  This is the harder direction. We prove it by induction on p.");
                println!("  Key steps using the J eliminator:");
                println!("  • Base case (p = refl): decode(encode(refl)) = decode(0) = refl ✓");
                println!("  • Step (p · loop): if decode(encode(p)) = p,");
                println!("    then decode(encode(p·loop)) = decode(encode(p)+1) = p·loop ✓");
                println!("  • Step (p · loop⁻¹): similar, decrements winding number ✓");
                println!();
                println!("  This uses the recursive nature of paths in S¹ (universal property of ℤ).");
                println!();
                println!("  Demonstration:");
                let examples = [("refl", 0i64), ("loop", 1), ("loop^-1", -1), ("loop^3", 3)];
                for (p, expected_n) in &examples {
                    let n = encode_path(p).unwrap_or(0);
                    let recovered = decode_to_path(n);
                    let decoded_n = encode_path(&recovered).unwrap_or(0);
                    let check = if decoded_n == *expected_n { green("✓") } else { red("≠") };
                    println!("    {} → encode → {} → decode → {}  {}", p, n, recovered, check);
                }
            }
            "mul" => {
                let ws: Vec<i64> = rest.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if ws.len() < 2 { println!("  {} Use: mul n m", red("✗")); return true; }
                let (a, b) = (ws[0], ws[1]);
                println!("  In ℤ: {} + {} = {} (path concatenation in π₁(S¹))", a, b, a + b);
                println!("  decode: {} · {} = {}", decode_to_path(a), decode_to_path(b), decode_to_path(a + b));
            }
            "inv" => {
                let n: i64 = rest.parse().unwrap_or(0);
                println!("  In ℤ: -{n} = {}  (path inverse in π₁(S¹))", -n);
                println!("  decode: ({})⁻¹ = {}", decode_to_path(n), decode_to_path(-n));
            }
            "verify" => {
                match encode_path(rest) {
                    Some(n) => {
                        let decoded = decode_to_path(n);
                        let re_encoded = encode_path(&decoded).unwrap_or(999);
                        println!("  p = {}", cyan(rest));
                        println!("  encode(p) = {n}");
                        println!("  decode(encode(p)) = {}", cyan(&decoded));
                        println!("  encode(decode(encode(p))) = {re_encoded}");
                        if re_encoded == n { println!("  {} encode ∘ decode ∘ encode = encode (one direction of adjunction)", green("✓")); }
                    }
                    None => println!("  {} Parse error", red("✗")),
                }
            }
            "bool-encode" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: bool-encode point path", red("✗")); return true; }
                match encode_bool_path(ws[0], ws[1]) {
                    Some(v) => {
                        println!("  In Bool: code(base) = Bool (2 elements)");
                        println!("  encode({}) = {}", rest, cyan(&v));
                        println!("  {} Bool is a set: only refl is a path, encode gives back base point", dim("→"));
                    }
                    None => println!("  {} Not a valid path in Bool: {} at {}", red("✗"), ws[1], ws[0]),
                }
            }
            "nat-encode" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: nat-encode n path", red("✗")); return true; }
                let n: u64 = ws[0].parse().unwrap_or(0);
                match encode_nat_path(n, ws[1]) {
                    Some(_) => println!("  In ℕ: code({n}) = {{refl}} if m = {n} else ∅"),
                    None => println!("  {} No path exists: {} ≠ {} as types (discrete set)", red("✗"), ws[0], ws[0]),
                }
            }
            "spheres" => {
                println!("{}", bold("── π_n(Sⁿ) = ℤ : General Pattern ──────────────────────────────────"));
                println!("  n=1: S¹  code: S¹→𝒰, code(base)=ℤ,       encode via winding number");
                println!("  n=2: S²  code: S²→𝒰, code(base)=ℤ,       encode via Hopf invariant");
                println!("  n=n: Sⁿ  code: Sⁿ→𝒰, code(base)=ℤ,       (n-1)-connected + Freudenthal)");
                println!();
                println!("  The encode-decode method generalizes:");
                println!("  1. Find code : X → 𝒰 with code(base) = G  (target group)");
                println!("  2. encode : (base = x) → code(x)  via path induction on x");
                println!("  3. decode : code(x) → (base = x)  via the inverse construction");
                println!("  4. Prove η and ε — gives the equivalence (base = base) ≃ G");
                println!();
                println!("  For Sⁿ: the key tool is the Freudenthal suspension theorem");
                println!("  which says Ω(ΣX) ≃ X under connectivity conditions.");
                println!("  Together with π₁(S¹) = ℤ, this builds up πₙ(Sⁿ) = ℤ inductively.");
            }
            "explain" => {
                println!("{}", bold("── Encode-Decode for π₁(S¹) = ℤ ───────────────────────────────────"));
                println!("  Step 1: Define a type family over S¹");
                println!("    code : S¹ → 𝒰");
                println!("    code(base) := ℤ");
                println!("    code(loop) := ua(succ) : ℤ = ℤ   (univalence gives ua of succ)");
                println!();
                println!("  Step 2: encode : (base = x) → code(x)");
                println!("    encode(p) := transport^code(p, 0)");
                println!("    encode(refl)     = 0");
                println!("    encode(p · loop) = encode(p) + 1");
                println!("    encode(p · inv)  = encode(p) - 1");
                println!();
                println!("  Step 3: decode : code(x) → (base = x)");
                println!("    decode_{{base}}(n) := loop^n");
                println!("    decode_{{loop}}(n) := (by transport via loop)");
                println!();
                println!("  Step 4: Prove encode and decode are inverse");
                println!("    η : encode ∘ decode = id_ℤ   (easy, by definition)");
                println!("    ε : decode ∘ encode = id_{{Ω}}  (by path induction)");
                println!();
                println!("  Conclusion: (base = base) ≃ ℤ  as types, so π₁(S¹) = ℤ  ∎");
                println!();
                println!("  Try: encode loop loop^-1 loop    (= 1)");
                println!("       decode 5                    (= loop^5)");
                println!("       eta  and  epsilon           (formal proofs)");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Encode-Decode — π₁(S¹) = ℤ and Beyond                ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Explore the encode-decode method: the fundamental technique for");
    println!("  computing π₁(X) in HoTT. See how paths encode as integers and back.");
    println!("  Type {} for the method, {} for commands.\n", cyan("explain"), cyan("help"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}encode{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
