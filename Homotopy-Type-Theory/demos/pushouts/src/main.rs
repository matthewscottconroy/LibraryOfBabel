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

// ── Pushout ───────────────────────────────────────────────────────────────────
//
// The pushout of  A ←f- C -g→ B  is the HIT:
//   Pushout(f, g) : Type with constructors
//     inl  : A → Pushout
//     inr  : B → Pushout
//     glue : ∀ c:C, inl(f(c)) = inr(g(c))
//
// Pushouts are the "colimit" construction.
// Special cases:
//   - C = ∅: coproduct  A + B
//   - C = A = B, f = id, g = id:  A ∥ A (two copies joined)
//   - A = B = 𝟙, C = S⁰: suspension ΣC
//   - circle: pushout of 𝟙 ←- 𝟙 -→ 𝟙 with C = S⁰ = {0,1}
//   - torus: pushout of S¹ and S¹ over S¹ ∨ S¹

// ── Finite pushout model ──────────────────────────────────────────────────────

struct PushoutBuilder {
    name: String,
    a_elems: Vec<String>,
    b_elems: Vec<String>,
    c_elems: Vec<String>,
    f_map: HashMap<String, String>, // C -> A
    g_map: HashMap<String, String>, // C -> B
}

impl PushoutBuilder {
    fn new(name: &str) -> Self {
        PushoutBuilder {
            name: name.to_string(),
            a_elems: vec![], b_elems: vec![], c_elems: vec![],
            f_map: HashMap::new(), g_map: HashMap::new(),
        }
    }

    fn add_a(&mut self, elems: Vec<String>) { self.a_elems.extend(elems); }
    fn add_b(&mut self, elems: Vec<String>) { self.b_elems.extend(elems); }
    fn add_c(&mut self, elems: Vec<String>) { self.c_elems.extend(elems); }

    fn set_f(&mut self, c: &str, a: &str) -> Result<(), String> {
        if !self.c_elems.contains(&c.to_string()) { return Err(format!("{c} not in C")); }
        if !self.a_elems.contains(&a.to_string()) { return Err(format!("{a} not in A")); }
        self.f_map.insert(c.to_string(), a.to_string());
        Ok(())
    }

    fn set_g(&mut self, c: &str, b: &str) -> Result<(), String> {
        if !self.c_elems.contains(&c.to_string()) { return Err(format!("{c} not in C")); }
        if !self.b_elems.contains(&b.to_string()) { return Err(format!("{b} not in B")); }
        self.g_map.insert(c.to_string(), b.to_string());
        Ok(())
    }

    fn compute_classes(&self) -> Vec<Vec<String>> {
        // Union-Find for equivalence classes
        let mut elements: Vec<String> = self.a_elems.iter().map(|a| format!("inl({a})")).collect();
        elements.extend(self.b_elems.iter().map(|b| format!("inr({b})")));
        let n = elements.len();
        let mut parent: Vec<usize> = (0..n).collect();

        let find = |parent: &mut Vec<usize>, mut x: usize| -> usize {
            while parent[x] != x { x = parent[x]; }
            x
        };

        // Apply glue relations
        for c in &self.c_elems {
            if let (Some(a), Some(b)) = (self.f_map.get(c), self.g_map.get(c)) {
                let ia = elements.iter().position(|e| e == &format!("inl({a})"));
                let ib = elements.iter().position(|e| e == &format!("inr({b})"));
                if let (Some(ia), Some(ib)) = (ia, ib) {
                    let ra = find(&mut parent, ia);
                    let rb = find(&mut parent, ib);
                    if ra != rb { parent[ra] = rb; }
                }
            }
        }

        // Collect classes
        let mut classes: HashMap<usize, Vec<String>> = HashMap::new();
        for (i, e) in elements.iter().enumerate() {
            let root = find(&mut parent, i);
            classes.entry(root).or_default().push(e.clone());
        }
        let mut result: Vec<Vec<String>> = classes.into_values().collect();
        result.sort_by_key(|c| c[0].clone());
        result
    }

    fn show(&self) {
        println!("  {} {}", bold("Pushout:"), cyan(&self.name));
        println!("  A = {{{}}}", self.a_elems.join(", "));
        println!("  B = {{{}}}", self.b_elems.join(", "));
        println!("  C = {{{}}}", self.c_elems.join(", "));
        println!("  f: {}", self.c_elems.iter().map(|c| format!("{c}↦{}", self.f_map.get(c).cloned().unwrap_or("?".into()))).collect::<Vec<_>>().join(", "));
        println!("  g: {}", self.c_elems.iter().map(|c| format!("{c}↦{}", self.g_map.get(c).cloned().unwrap_or("?".into()))).collect::<Vec<_>>().join(", "));
        let classes = self.compute_classes();
        println!("  Pushout has {} points (equivalence classes):", classes.len());
        for (i, cls) in classes.iter().enumerate() {
            if cls.len() == 1 { println!("  [{i}] = {}", cyan(&cls[0])); }
            else { println!("  [{i}] = {{{}}}  (glued by glue path)", cls.join(" = ")); }
        }
        let glues: Vec<String> = self.c_elems.iter()
            .filter(|c| self.f_map.contains_key(*c) && self.g_map.contains_key(*c))
            .map(|c| format!("glue({c}) : inl({}) = inr({})", self.f_map[c], self.g_map[c])).collect();
        if !glues.is_empty() { println!("  Glue paths: {}", glues.join(", ")); }
    }

    fn pi1_comment(&self) {
        // van Kampen theorem comment
        println!("  {}", bold("van Kampen Theorem:"));
        println!("  π₁(A ∪_C B) = π₁(A) *_π₁(C) π₁(B)  (amalgamated free product)");
        println!("  (when A, B, C are path-connected)");
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    builders: HashMap<String, PushoutBuilder>,
    current: String,
}

impl Sandbox {
    fn new() -> Self { Sandbox { builders: HashMap::new(), current: String::new() } }

    fn cur(&self) -> Option<&PushoutBuilder> { self.builders.get(&self.current) }
    fn cur_mut(&mut self) -> Option<&mut PushoutBuilder> { self.builders.get_mut(&self.current) }

    fn print_help() {
        println!("{}", bold("── Pushout Builder ─────────────────────────────────────────────────"));
        println!("  {}  name              — create a pushout  A ←f- C -g→ B", cyan("new"));
        println!("  {}  a1 a2             — elements of A (left type)", cyan("A"));
        println!("  {}  b1 b2             — elements of B (right type)", cyan("B"));
        println!("  {}  c1 c2             — elements of C (span type)", cyan("C"));
        println!("  {}  c a               — set f(c) = a", cyan("f"));
        println!("  {}  c b               — set g(c) = b", cyan("g"));
        println!("  {}                    — compute and show the pushout", cyan("show"));
        println!("{}", bold("── Preset Spaces ────────────────────────────────────────────────────"));
        println!("  {}  circle|torus|sphere|rp2|wedge|coproduct", cyan("preset"));
        println!("{}", bold("── Theory ───────────────────────────────────────────────────────────"));
        println!("  {}         — van Kampen theorem for π₁", cyan("vanKampen"));
        println!("  {}         — explain pushout as HIT", cyan("explain"));
    }

    fn build_circle(&mut self) {
        // S¹ = pushout of 𝟙 ←- 2 -→ 𝟙 (two points glued to same point)
        let mut pb = PushoutBuilder::new("S¹");
        pb.add_a(vec!["base_l".into()]);
        pb.add_b(vec!["base_r".into()]);
        pb.add_c(vec!["n".into(), "s".into()]);
        let _ = pb.set_f("n", "base_l");
        let _ = pb.set_f("s", "base_l");
        let _ = pb.set_g("n", "base_r");
        let _ = pb.set_g("s", "base_r");
        self.builders.insert("S1".into(), pb);
        self.current = "S1".into();
    }

    fn build_wedge(&mut self) {
        // S¹ ∨ S¹ = pushout of S¹ ←- {*} -→ S¹ (sharing a point)
        let mut pb = PushoutBuilder::new("S¹∨S¹");
        pb.add_a(vec!["base1".into()]);
        pb.add_b(vec!["base2".into()]);
        pb.add_c(vec!["pt".into()]);
        let _ = pb.set_f("pt", "base1");
        let _ = pb.set_g("pt", "base2");
        self.builders.insert("wedge".into(), pb);
        self.current = "wedge".into();
    }

    fn build_coproduct(&mut self) {
        // A + B = pushout over empty span (C = ∅)
        let mut pb = PushoutBuilder::new("A+B");
        pb.add_a(vec!["a1".into(), "a2".into()]);
        pb.add_b(vec!["b1".into(), "b2".into()]);
        // No C, no glue — pure coproduct
        self.builders.insert("coproduct".into(), pb);
        self.current = "coproduct".into();
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
            "new" => {
                if rest.is_empty() { println!("  {} Use: new name", red("✗")); return true; }
                self.builders.insert(rest.to_string(), PushoutBuilder::new(rest));
                self.current = rest.to_string();
                println!("  Created pushout {}", cyan(rest));
            }
            "use" | "switch" => {
                if self.builders.contains_key(rest) { self.current = rest.to_string(); println!("  Using {}", cyan(rest)); }
                else { println!("  {} Pushout {rest} not found", red("✗")); }
            }
            "A" => {
                let elems: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
                if let Some(cur) = self.cur_mut() { cur.add_a(elems.clone()); println!("  A = {{...+{}}}", elems.join(", ")); }
                else { println!("  {} Create a pushout first with 'new'", red("✗")); }
            }
            "B" => {
                let elems: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
                if let Some(cur) = self.cur_mut() { cur.add_b(elems.clone()); println!("  B = {{...+{}}}", elems.join(", ")); }
                else { println!("  {} Create a pushout first", red("✗")); }
            }
            "C" => {
                let elems: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
                if let Some(cur) = self.cur_mut() { cur.add_c(elems.clone()); println!("  C = {{...+{}}}", elems.join(", ")); }
                else { println!("  {} Create a pushout first", red("✗")); }
            }
            "f" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: f c a", red("✗")); return true; }
                if let Some(cur) = self.cur_mut() {
                    match cur.set_f(ws[0], ws[1]) {
                        Ok(()) => println!("  f({}) = {}", ws[0], ws[1]),
                        Err(e) => println!("  {} {}", red("✗"), e),
                    }
                } else { println!("  {} Create a pushout first", red("✗")); }
            }
            "g" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: g c b", red("✗")); return true; }
                if let Some(cur) = self.cur_mut() {
                    match cur.set_g(ws[0], ws[1]) {
                        Ok(()) => println!("  g({}) = {}", ws[0], ws[1]),
                        Err(e) => println!("  {} {}", red("✗"), e),
                    }
                } else { println!("  {} Create a pushout first", red("✗")); }
            }
            "show" => {
                if let Some(cur) = self.cur() { let cur = cur; cur.show(); }
                else { println!("  {} No pushout selected", red("✗")); }
            }
            "preset" => {
                match rest {
                    "circle" | "S1" => { self.build_circle(); self.cur().unwrap().show(); }
                    "wedge" | "S1vS1" => { self.build_wedge(); self.cur().unwrap().show(); }
                    "coproduct" | "A+B" => { self.build_coproduct(); self.cur().unwrap().show(); }
                    "torus" => {
                        println!("  {} T² = pushout of S¹ and S¹ over S¹∨S¹", bold("Torus"));
                        println!("  A = S¹, B = S¹, C = S¹∨S¹");
                        println!("  f sends the wedge to the meridian circle");
                        println!("  g sends the wedge to the longitude circle");
                        println!("  π₁(T²) = ℤ × ℤ (by van Kampen on S¹ ∗_{{*}} S¹)");
                        println!("  The pushout glue creates both a and b loops that commute.");
                    }
                    "sphere" | "S2" => {
                        println!("  {} S² = suspension of S¹ = pushout of 𝟙 ←- S¹ -→ 𝟙", bold("S²"));
                        println!("  Or: S² = pushout of D² and D² over S¹ (gluing two disks)");
                        println!("  π₁(S²) = 0 (simply connected)");
                        println!("  π₂(S²) = ℤ (the Hopf map is a generator)");
                    }
                    "rp2" => {
                        println!("  {} RP² = pushout of D² over S¹ with antipodal map", bold("RP²"));
                        println!("  A = D² (disk), C = S¹, B = 𝟙");
                        println!("  f = boundary inclusion S¹ → D²");
                        println!("  g = collapse S¹ → 𝟙 (identifies antipodal points)");
                        println!("  π₁(RP²) = ℤ/2ℤ");
                        println!("  χ(RP²) = 1 (non-orientable)");
                    }
                    _ => println!("  {} Presets: circle, wedge, coproduct, torus, sphere, rp2", red("✗")),
                }
            }
            "vanKampen" => {
                println!("{}", bold("── van Kampen Theorem ──────────────────────────────────────────────"));
                println!("  If X = A ∪_C B (pushout), A,B,C path-connected, then:");
                println!("  π₁(X) = π₁(A) *_π₁(C) π₁(B)");
                println!("  (amalgamated free product: π₁(A) and π₁(B) merged over π₁(C))");
                println!();
                println!("  Examples:");
                println!("  S¹ = 𝟙 ∪_{{0,1}} 𝟙:  π₁ = 1 *_1 1 = ℤ  (free on 1 generator)");
                println!("  S¹∨S¹: π₁ = ℤ * ℤ  (free product — two independent loops)");
                println!("  T²:    π₁ = ℤ × ℤ  (both loops commute by torus relation)");
                println!("  RP²:   π₁ = ℤ/2ℤ  (meridian loop squares to identity)");
                println!("  S²:    π₁ = 0     (no loops — simply connected)");
                println!();
                println!("  In HoTT, van Kampen is proved using encode-decode + pushout recursion.");
            }
            "explain" => {
                println!("{}", bold("── Pushout as a Higher Inductive Type ──────────────────────────────"));
                println!("  A ←f- C -g→ B  (a cospan/span)");
                println!();
                println!("  Pushout(f, g) : Type with:");
                println!("    inl  : A → Pushout(f, g)");
                println!("    inr  : B → Pushout(f, g)");
                println!("    glue : (c : C) → inl(f(c)) = inr(g(c))");
                println!();
                println!("  Recursion principle: to define h : Pushout → P, give:");
                println!("    hl : A → P");
                println!("    hr : B → P");
                println!("    hg : (c:C) → hl(f(c)) = hr(g(c))");
                println!();
                println!("  This is the HoTT generalization of colimits.");
                println!("  glue gives paths that identify images of C in A and B.");
                println!("  van Kampen then follows from the universal property.");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Pushouts — HIT Colimits and van Kampen Sandbox        ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build pushouts by specifying spans A ←f- C -g→ B.");
    println!("  See how glue paths identify elements. Load preset spaces.");
    println!("  Explore van Kampen's theorem for fundamental groups.");
    println!("  Type {} for commands, {} for presets.\n", cyan("help"), cyan("preset"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}pushout{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
