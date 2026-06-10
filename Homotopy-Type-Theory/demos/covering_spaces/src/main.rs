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

// ── Covering Spaces in HoTT ───────────────────────────────────────────────────
//
// A covering space of X is a type family F : X → Set.
// The Galois correspondence:
//   Covering spaces of X ↔ Sets with π₁(X)-action
//
// For S¹: covering spaces correspond to ℤ-sets (sets with a bijection)
// The universal cover is ℝ (or ℤ in the discrete version).
//
// In HoTT: a covering space is F : X → Set (set-valued type family)
// Transport along paths gives the monodromy action.

// ── Covering space model ──────────────────────────────────────────────────────

struct CoveringSpace {
    name: String,
    base: String,
    // fiber over the base point (discrete set)
    fiber: Vec<String>,
    // monodromy: how the fiber transforms when we go around the generator loop
    monodromy: Vec<usize>, // permutation of fiber indices
}

impl CoveringSpace {
    fn apply_monodromy(&self, fiber_idx: usize, n: i64) -> usize {
        if self.fiber.is_empty() { return 0; }
        let k = self.fiber.len();
        // Apply monodromy n times (positive = forward, negative = backward)
        let mut cur = fiber_idx;
        if n >= 0 {
            for _ in 0..n { cur = self.monodromy[cur % k]; }
        } else {
            // inverse monodromy
            let mut inv = vec![0usize; k];
            for (i, &j) in self.monodromy.iter().enumerate() { inv[j] = i; }
            for _ in 0..(-n) { cur = inv[cur % k]; }
        }
        cur
    }

    fn lift_loop(&self, start: usize, winding: i64) -> usize {
        self.apply_monodromy(start, winding)
    }

    fn orbit_of(&self, idx: usize) -> Vec<usize> {
        let mut orbit = vec![idx];
        let mut cur = self.monodromy[idx % self.fiber.len()];
        while !orbit.contains(&cur) { orbit.push(cur); cur = self.monodromy[cur % self.fiber.len()]; }
        orbit
    }

    fn is_connected(&self) -> bool {
        if self.fiber.is_empty() { return true; }
        let orbit = self.orbit_of(0);
        orbit.len() == self.fiber.len()
    }

    fn deck_transformations(&self) -> Vec<Vec<usize>> {
        // A deck transformation is a fiber automorphism that commutes with monodromy
        // For a regular covering, these form a group = π₁(base)/π₁(total)
        let n = self.fiber.len();
        let mut decks = vec![];
        // Check all permutations (for small fibers)
        if n <= 6 {
            let mut all_perms: Vec<Vec<usize>> = vec![];
            gen_perms(&(0..n).collect::<Vec<_>>(), &mut vec![], &mut all_perms);
            for perm in all_perms {
                // Check if perm commutes with monodromy
                let commutes = (0..n).all(|i| {
                    let perm_then_mono = self.monodromy[perm[i]];
                    let mono_then_perm = perm[self.monodromy[i]];
                    perm_then_mono == mono_then_perm
                });
                if commutes { decks.push(perm); }
            }
        }
        decks
    }

    fn show(&self) {
        println!("  {} over {}", bold(&cyan(&self.name)), self.base);
        println!("  Fiber = {{{}}}", self.fiber.join(", "));
        let mono_str: Vec<String> = self.fiber.iter().enumerate()
            .map(|(i, x)| format!("{x}↦{}", self.fiber.get(self.monodromy[i]).cloned().unwrap_or("?".into()))).collect();
        println!("  Monodromy (loop action): {}", mono_str.join(", "));
        let conn = if self.is_connected() { green("connected") } else { red("disconnected") };
        println!("  {}", conn);
        let decks = self.deck_transformations();
        println!("  Deck transformations: {} ({} total)", if decks.len() <= 6 { format!("{}", decks.len()) } else { "many".into() }, decks.len());
    }
}

fn gen_perms(remaining: &[usize], current: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
    if remaining.is_empty() { out.push(current.clone()); return; }
    for i in 0..remaining.len() {
        let mut r2 = remaining.to_vec(); let x = r2.remove(i);
        current.push(x); gen_perms(&r2, current, out); current.pop();
    }
}

// ── Sandbox ───────────────────────────────────────────────────────────────────

struct Sandbox {
    spaces: HashMap<String, CoveringSpace>,
    current: String,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { spaces: HashMap::new(), current: String::new() };
        // Pre-load: universal cover of S¹ (ℤ-sheeted)
        let fiber: Vec<String> = (-3i32..=3).map(|i| i.to_string()).collect();
        let n = fiber.len();
        // Monodromy: shift by 1 (loop takes k to k+1)
        let mono: Vec<usize> = (0..n).map(|i| (i + 1) % n).collect();
        let univ = CoveringSpace { name: "ℝ̃ (universal cover of S¹, truncated)".into(), base: "S¹".into(), fiber, monodromy: mono };
        sb.spaces.insert("universal".into(), univ);
        sb.current = "universal".into();
        sb
    }

    fn print_help() {
        println!("{}", bold("── Covering Space Construction ─────────────────────────────────────"));
        println!("  {}  name base         — create a covering space", cyan("cover"));
        println!("  {}  a b c d          — set fiber elements", cyan("fiber"));
        println!("  {}  0 1  1 2  2 0    — set monodromy (pairs i->j)", cyan("mono"));
        println!("  {}                   — display current covering space", cyan("show"));
        println!("  {}  name             — switch to covering space", cyan("use"));
        println!("{}", bold("── Path Lifting ────────────────────────────────────────────────────"));
        println!("  {}  e n              — lift loop^n starting at fiber element e", cyan("lift"));
        println!("  {}  e               — find orbit of fiber element e", cyan("orbit"));
        println!("  {}          — find all deck transformations", cyan("decks"));
        println!("{}", bold("── Galois Correspondence ───────────────────────────────────────────"));
        println!("  {}          — explain covering/HoTT correspondence", cyan("galois"));
        println!("  {}  n              — n-sheeted cover of S¹ (ℤ/nℤ)", cyan("ncover"));
        println!("  {}          — show universal cover of S¹", cyan("universal"));
        println!("  {}          — show preloaded covers", cyan("presets"));
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
            "cover" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.is_empty() { println!("  {} Use: cover name [base]", red("✗")); return true; }
                let base = ws.get(1).copied().unwrap_or("X");
                let cs = CoveringSpace { name: ws[0].to_string(), base: base.to_string(), fiber: vec![], monodromy: vec![] };
                self.spaces.insert(ws[0].to_string(), cs);
                self.current = ws[0].to_string();
                println!("  Created covering {} → {}", cyan(ws[0]), base);
            }
            "use" | "switch" => {
                if self.spaces.contains_key(rest) { self.current = rest.to_string(); println!("  Using {}", cyan(rest)); }
                else { println!("  {} Covering {rest} not found", red("✗")); }
            }
            "fiber" => {
                let elems: Vec<String> = rest.split_whitespace().map(|s| s.to_string()).collect();
                if let Some(cs) = self.spaces.get_mut(&self.current) {
                    let n = elems.len();
                    cs.fiber = elems.clone();
                    cs.monodromy = (0..n).collect(); // default: identity
                    println!("  Fiber = {{{}}}  (monodromy = identity, set with 'mono')", elems.join(", "));
                } else { println!("  {} No current cover", red("✗")); }
            }
            "mono" => {
                // mono: pairs i->j defining permutation
                let nums: Vec<usize> = rest.split_whitespace().filter_map(|s| s.parse().ok()).collect();
                if let Some(cs) = self.spaces.get_mut(&self.current) {
                    let n = cs.fiber.len();
                    if nums.len() >= n {
                        cs.monodromy = nums[..n].to_vec();
                        let mono_str: Vec<String> = cs.fiber.iter().enumerate()
                            .map(|(i, x)| format!("{x}→{}", cs.fiber.get(cs.monodromy[i]).cloned().unwrap_or("?".into()))).collect();
                        println!("  Monodromy: {}", mono_str.join(", "));
                    } else { println!("  {} Need exactly {} indices", red("✗"), n); }
                } else { println!("  {} No current cover", red("✗")); }
            }
            "show" => {
                let cur = self.current.clone();
                if let Some(cs) = self.spaces.get(&cur) { cs.show(); }
                else { println!("  {} No current covering space", red("✗")); }
            }
            "lift" => {
                let ws: Vec<&str> = rest.split_whitespace().collect();
                if ws.len() < 2 { println!("  {} Use: lift e n", red("✗")); return true; }
                let n: i64 = ws[1].parse().unwrap_or(1);
                let cur = self.current.clone();
                if let Some(cs) = self.spaces.get(&cur) {
                    let start = cs.fiber.iter().position(|x| x == ws[0]);
                    match start {
                        Some(idx) => {
                            let end_idx = cs.lift_loop(idx, n);
                            let end = cs.fiber.get(end_idx).cloned().unwrap_or("?".into());
                            println!("  Lift loop^{n} starting at {}:", cyan(ws[0]));
                            println!("  {} → (monodromy^{n}) → {}", cyan(ws[0]), cyan(&end));
                            if end == *ws[0] { println!("  {} Path closes! (loop^{n} is trivial in this cover)", green("✓")); }
                            else { println!("  {} Path does NOT close (loop^{n} is non-trivial)", yellow("ℹ")); }
                        }
                        None => println!("  {} {} not in fiber", red("✗"), ws[0]),
                    }
                } else { println!("  {} No current covering space", red("✗")); }
            }
            "orbit" => {
                let cur = self.current.clone();
                if let Some(cs) = self.spaces.get(&cur) {
                    let idx = cs.fiber.iter().position(|x| x == rest);
                    match idx {
                        Some(i) => {
                            let orbit = cs.orbit_of(i);
                            let names: Vec<String> = orbit.iter().filter_map(|&j| cs.fiber.get(j).cloned()).collect();
                            println!("  Orbit of {}: {{{}}}", cyan(rest), names.join(", "));
                            if orbit.len() == cs.fiber.len() { println!("  {} Transitive action — connected cover", green("✓")); }
                        }
                        None => println!("  {} {} not in fiber", red("✗"), rest),
                    }
                } else { println!("  {} No current covering space", red("✗")); }
            }
            "decks" => {
                let cur = self.current.clone();
                if let Some(cs) = self.spaces.get(&cur) {
                    let decks = cs.deck_transformations();
                    println!("  Deck transformations of {}:", cyan(&cs.name));
                    for (k, d) in decks.iter().enumerate() {
                        let map: Vec<String> = cs.fiber.iter().zip(d.iter()).map(|(x, &j)| format!("{x}↦{}", cs.fiber.get(j).cloned().unwrap_or("?".into()))).collect();
                        println!("  δ{k}: {}", map.join(", "));
                    }
                    println!("  |Deck(cover)| = {}  = π₁(base)/π₁(total)", decks.len());
                } else { println!("  {} No current covering space", red("✗")); }
            }
            "ncover" => {
                let n: usize = rest.parse().unwrap_or(2);
                let fiber: Vec<String> = (0..n).map(|i| i.to_string()).collect();
                let mono: Vec<usize> = (0..n).map(|i| (i + 1) % n).collect();
                let cs = CoveringSpace {
                    name: format!("ℤ/{n}ℤ-cover of S¹"),
                    base: "S¹".into(),
                    fiber, monodromy: mono,
                };
                println!("  {}-sheeted cover of S¹:", n);
                cs.show();
                println!("  This corresponds to the ℤ/{}ℤ action on ℤ.", n);
                println!("  Deck transformations = ℤ/{}ℤ (regular cover).", n);
                self.spaces.insert(format!("Z{n}"), cs);
                self.current = format!("Z{n}");
            }
            "universal" => {
                self.current = "universal".into();
                if let Some(cs) = self.spaces.get("universal") { cs.show(); }
                println!("  This is the truncated universal cover of S¹ (infinite = ℤ).");
                println!("  The full universal cover has fiber ℤ = {{..., -2, -1, 0, 1, 2, ...}}.");
                println!("  Monodromy: n ↦ n+1  (shift by 1 when we go around loop).");
                println!("  Deck transformations = ℤ (all integer shifts).");
            }
            "galois" => {
                println!("{}", bold("── Galois Correspondence for Covering Spaces ───────────────────────"));
                println!("  Classical topology:");
                println!("  {{ connected covers of X }} ↔ {{ transitive π₁(X)-sets }}");
                println!("  (Subgroups H ≤ π₁(X) correspond to X_H = universal cover / H)");
                println!();
                println!("  In HoTT:");
                println!("  {{ covering spaces of X }} = {{ type families F : X → Set }}");
                println!("  (a covering = a set-valued family, i.e. h-level-0 family)");
                println!();
                println!("  The monodromy action: for p : x = y in X,");
                println!("  transport^F(p) : F(x) → F(y)  is the bijection on fibers.");
                println!();
                println!("  For S¹: F : S¹ → Set");
                println!("  F determined by: F(base) = S  (some set)");
                println!("                   F(loop) = ua(σ) : S = S  for some bijection σ");
                println!("  So: covers of S¹ = sets with a bijection = ℤ-sets");
                println!("  This recovers the classical ℤ = π₁(S¹) action.");
            }
            "presets" => {
                println!("{}", bold("── Preloaded Covering Spaces ───────────────────────────────────────"));
                println!("  {}  — ℤ-action (fiber = {{-3..3}}, monodromy = shift)", cyan("universal"));
                println!("  Build more with: ncover 2, ncover 3, ncover 6");
                println!("  Or manually with: cover, fiber, mono");
            }
            _ => println!("  {} Unknown command: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Covering Spaces — Galois Correspondence Sandbox       ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build covering spaces as set-valued type families.");
    println!("  Lift paths, compute orbits, find deck transformations.");
    println!("  See the Galois correspondence between covers and π₁-actions.");
    println!("  Type {} for commands, {} for the correspondence.\n", cyan("help"), cyan("galois"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}cover{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
