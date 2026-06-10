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

// ── Higher Category Theory ────────────────────────────────────────────────────
//
// An (∞,1)-category: objects, 1-morphisms, 2-morphisms, …
//   all k-morphisms for k≥2 are invertible
//
// Key models: quasi-categories, complete Segal spaces, ∞-groupoids
//
// HoTT relationship:
//   Every type in HoTT is an ∞-groupoid.
//   Universes are (∞,1)-categories.
//   The internal language of (∞,1)-topoi is HoTT.

#[derive(Clone, Debug)]
struct Cell {
    source: String,
    target: String,
    label: String,
    dim: usize,
}

#[derive(Clone, Debug)]
struct HigherCat {
    name: String,
    objects: Vec<String>,
    cells: Vec<Cell>,
    is_groupoid: bool,
}

impl HigherCat {
    fn new(name: &str, groupoid: bool) -> Self {
        HigherCat { name: name.into(), objects: Vec::new(), cells: Vec::new(), is_groupoid: groupoid }
    }

    fn add_obj(&mut self, o: &str) { self.objects.push(o.into()); }

    fn add_cell(&mut self, src: &str, tgt: &str, label: &str, dim: usize) {
        self.cells.push(Cell { source: src.into(), target: tgt.into(), label: label.into(), dim });
    }

    fn cells_of_dim(&self, d: usize) -> Vec<&Cell> {
        self.cells.iter().filter(|c| c.dim == d).collect()
    }

    fn display(&self) {
        let kind = if self.is_groupoid { "∞-groupoid" } else { "(∞,1)-category" };
        println!("  {} «{}»", cyan(kind), self.name);
        println!("  Objects (0-cells): {}", self.objects.join(", "));
        for d in 1..=3 {
            let cs = self.cells_of_dim(d);
            if !cs.is_empty() {
                let strs: Vec<String> = cs.iter().map(|c|
                    format!("{}: {}→{}", c.label, c.source, c.target)).collect();
                println!("  {d}-cells: {}", strs.join("; "));
            }
        }
        if self.is_groupoid {
            println!("  {} All cells are invertible (it's a groupoid).", green("✓"));
        }
    }
}

struct Sandbox {
    cats: Vec<HigherCat>,
    current: usize,
}

impl Sandbox {
    fn new() -> Self {
        let mut sb = Sandbox { cats: Vec::new(), current: 0 };
        let mut g = HigherCat::new("path-groupoid", true);
        g.add_obj("x");
        g.add_obj("y");
        g.add_cell("x", "y", "p", 1);
        g.add_cell("x", "y", "q", 1);
        g.add_cell("p", "q", "α", 2);
        sb.cats.push(g);
        sb
    }

    fn print_help() {
        println!("{}", bold("── Higher Category Builder ──────────────────────────────────────────"));
        println!("  {}  <name>     — create new (∞,1)-category", cyan("cat"));
        println!("  {}  <name>     — create new ∞-groupoid", cyan("groupoid"));
        println!("  {}  <x>        — add object", cyan("obj"));
        println!("  {}  <s> <t> <f>  — add 1-cell s→t labelled f", cyan("cell1"));
        println!("  {}  <s> <t> <α>  — add 2-cell (homotopy) s⇒t", cyan("cell2"));
        println!("  {}  <s> <t> <Γ>  — add 3-cell", cyan("cell3"));
        println!("  {}            — display current structure", cyan("show"));
        println!("  {}  <n>        — switch to structure #n", cyan("use"));
        println!("{}", bold("── Theory ──────────────────────────────────────────────────────────"));
        println!("  {}  <n> <k>    — show (n,k)-category concept", cyan("nk"));
        println!("  {}      — (∞,1)-topoi and HoTT", cyan("topos"));
        println!("  {}  — how types are ∞-groupoids", cyan("types-as-groupoids"));
        println!("  {}      — Grothendieck's homotopy hypothesis", cyan("hypothesis"));
        println!("  {}    — globular sets, simplicial, etc.", cyan("models"));
        println!("  {}  — Kan complexes and quasi-categories", cyan("quasi-cat"));
        println!("  {}  — adjoints, limits, colimits in ∞-cats", cyan("adjoint"));
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
            "cat" => {
                let name = if rest.is_empty() { "C" } else { rest };
                let c = HigherCat::new(name, false);
                self.current = self.cats.len();
                self.cats.push(c);
                println!("  Created (∞,1)-category «{}» (#{}).", cyan(name), self.current);
                println!("  Add objects with {}, 1-cells with {}.", cyan("obj"), cyan("cell1"));
            }
            "groupoid" => {
                let name = if rest.is_empty() { "G" } else { rest };
                let c = HigherCat::new(name, true);
                self.current = self.cats.len();
                self.cats.push(c);
                println!("  Created ∞-groupoid «{}» (#{}).", cyan(name), self.current);
            }
            "obj" => {
                if self.cats.is_empty() { println!("  No structure. Use {} first.", cyan("cat")); return true; }
                let o = if rest.is_empty() { "x" } else { rest };
                self.cats[self.current].add_obj(o);
                println!("  Added object {} to «{}».", cyan(o), self.cats[self.current].name);
            }
            "cell1" => {
                if self.cats.is_empty() { println!("  No structure. Use {} first.", cyan("cat")); return true; }
                let args: Vec<&str> = rest.splitn(3, ' ').collect();
                if args.len() < 3 { println!("  Usage: cell1 <source> <target> <label>"); return true; }
                self.cats[self.current].add_cell(args[0], args[1], args[2], 1);
                println!("  Added 1-cell {} : {} → {}.", cyan(args[2]), args[0], args[1]);
            }
            "cell2" => {
                if self.cats.is_empty() { println!("  No structure. Use {} first.", cyan("cat")); return true; }
                let args: Vec<&str> = rest.splitn(3, ' ').collect();
                if args.len() < 3 { println!("  Usage: cell2 <source-1cell> <target-1cell> <label>"); return true; }
                self.cats[self.current].add_cell(args[0], args[1], args[2], 2);
                println!("  Added 2-cell {} : {} ⇒ {}.", cyan(args[2]), args[0], args[1]);
                if self.cats[self.current].is_groupoid {
                    println!("  {} As a groupoid, this 2-cell has an inverse.", dim("Note:"));
                }
            }
            "cell3" => {
                if self.cats.is_empty() { println!("  No structure. Use {} first.", cyan("cat")); return true; }
                let args: Vec<&str> = rest.splitn(3, ' ').collect();
                if args.len() < 3 { println!("  Usage: cell3 <source-2cell> <target-2cell> <label>"); return true; }
                self.cats[self.current].add_cell(args[0], args[1], args[2], 3);
                println!("  Added 3-cell {} : {} ⟹ {}.", cyan(args[2]), args[0], args[1]);
            }
            "show" => {
                if self.cats.is_empty() { println!("  No structures yet."); return true; }
                self.cats[self.current].display();
            }
            "use" => {
                let n: usize = rest.parse().unwrap_or(0);
                if n < self.cats.len() {
                    self.current = n;
                    println!("  Switched to «{}» (#{})", self.cats[n].name, n);
                } else {
                    println!("  {} Index out of range. {} structures exist.",
                        red("✗"), self.cats.len());
                }
            }
            "nk" => {
                let args: Vec<&str> = rest.split_whitespace().collect();
                let n: i64 = args.first().and_then(|s| s.parse().ok()).unwrap_or(1);
                let k: i64 = args.get(1).and_then(|s| s.parse().ok()).unwrap_or(1);
                println!("  ({n},{k})-category: has n-cells, k-invertible (i.e., cells above dim k are invertible)");
                match (n, k) {
                    (1, 0) => println!("  (1,0)-category = ordinary category with all morphisms invertible = groupoid"),
                    (1, 1) => println!("  (1,1)-category = ordinary 1-category"),
                    (2, 1) => println!("  (2,1)-category = 2-category where 2-cells are invertible"),
                    (2, 2) => println!("  (2,2)-category = bicategory (weak 2-category)"),
                    (n, 0) if n >= 0 => println!("  ({n},0)-category = ∞-groupoid (all cells invertible up to dim {n})"),
                    (i64::MAX, 0) | (-1, _) => println!("  (∞,0)-category = ∞-groupoid (Grothendieck's homotopy hypothesis)"),
                    (i64::MAX, 1) => println!("  (∞,1)-category = the main protagonist of modern homotopy theory"),
                    _ => println!("  ({n},{k})-category: cells of dim ≤{k} not necessarily invertible, >{k} are."),
                }
            }
            "topos" => {
                println!("{}", bold("── (∞,1)-Topoi and HoTT ────────────────────────────────────────────"));
                println!("  An (∞,1)-topos is an (∞,1)-category that:");
                println!("    • has all finite limits and colimits");
                println!("    • satisfies descent (Giraud's axioms for ∞-cats)");
                println!("    • has an object classifier (analogous to subobject classifier)");
                println!();
                println!("  Key example: 𝒮 = (∞,1)-category of spaces (Kan complexes)");
                println!("  Other examples: Sh_∞(X) = ∞-sheaves on a site X");
                println!();
                println!("  Shulman's theorem (2019): Every (∞,1)-topos has an internal language");
                println!("  which is a form of HoTT (homotopy type theory).");
                println!();
                println!("  Concretely: working internally to an (∞,1)-topos = working in HoTT");
                println!("  with the universe 𝒰 interpreted as the object classifier.");
                println!();
                println!("  This makes HoTT the correct internal logic for (∞,1)-topoi.");
            }
            "types-as-groupoids" => {
                println!("{}", bold("── Types as ∞-Groupoids ────────────────────────────────────────────"));
                println!("  Every type A in HoTT is an ∞-groupoid:");
                println!("  • Objects: terms a : A");
                println!("  • 1-morphisms: paths p : a = b");
                println!("  • 2-morphisms: paths between paths α : p ={{a=b}} q");
                println!("  • k-morphisms: iterated identity types");
                println!();
                println!("  The groupoid laws hold up to higher homotopy:");
                println!("  • Composition: path concatenation");
                println!("  • Associativity: holds up to 2-morphism (not definitionally)");
                println!("  • Units: refl is left/right unit, up to 2-morphism");
                println!("  • Inverses: path reversal");
                println!();
                println!("  h-levels restrict this:");
                println!("  • Propositions (-1): trivial groupoid (contractible hom-sets)");
                println!("  • Sets (0): discrete groupoid (no non-trivial paths)");
                println!("  • Groupoids (1): genuine 1-groupoids");
                println!("  • 2-groupoids (2): genuine 2-groupoids");
                println!("  • ∞-groupoids: general types");
            }
            "hypothesis" => {
                println!("{}", bold("── Grothendieck's Homotopy Hypothesis ─────────────────────────────"));
                println!("  Grothendieck (1983, Pursuing Stacks):");
                println!("    (∞,0)-categories = ∞-groupoids");
                println!("    ∞-groupoids ≃ homotopy types (spaces up to weak homotopy equiv.)");
                println!();
                println!("  HoTT incarnation:");
                println!("    Types in HoTT = ∞-groupoids = homotopy types");
                println!("    The univalent universe 𝒰 classifies small ∞-groupoids.");
                println!();
                println!("  Formal proof: the (∞,1)-category of types in HoTT");
                println!("  is equivalent (as an (∞,1)-category) to the (∞,1)-category of spaces.");
                println!();
                println!("  Status: Proved for Kan complexes (Quillen model structure on sSet).");
                println!("  The hypothesis is now a theorem for the right definitions.");
            }
            "models" => {
                println!("{}", bold("── Models of Higher Categories ─────────────────────────────────────"));
                println!("  For ∞-groupoids:");
                println!("    • Kan complexes (simplicial sets satisfying Kan condition)");
                println!("    • CW complexes (topological spaces)");
                println!("    • Cubical sets with connections");
                println!("    • Globular sets (Street's ω-groupoids)");
                println!();
                println!("  For (∞,1)-categories:");
                println!("    • Quasi-categories (Joyal, Lurie): simplicial sets where inner horns fill");
                println!("    • Complete Segal spaces (Rezk): bisimplicial sets");
                println!("    • Segal categories");
                println!("    • 1-complicial sets (Verity)");
                println!();
                println!("  All models are equivalent via the model structure on each.");
                println!("  Lurie's HTT and HA develop the theory using quasi-categories.");
            }
            "quasi-cat" => {
                println!("{}", bold("── Quasi-Categories ────────────────────────────────────────────────"));
                println!("  A quasi-category (Joyal 2002) = simplicial set K where:");
                println!("  every inner horn Λⁿᵢ → K (0 < i < n) has a filler Δⁿ → K.");
                println!();
                println!("  Intuition:");
                println!("  • 0-simplices = objects");
                println!("  • 1-simplices = morphisms");
                println!("  • 2-simplices = witnesses that two composites are homotopic");
                println!("  • Higher simplices = higher coherences");
                println!();
                println!("  Kan complexes = quasi-categories where ALL horns fill (not just inner)");
                println!("  These correspond to ∞-groupoids.");
                println!();
                println!("  Lurie's higher topos theory is entirely formulated in quasi-categories.");
                println!("  Key book: HTT (Higher Topos Theory, Lurie 2009).");
            }
            "adjoint" => {
                println!("{}", bold("── Adjoints in (∞,1)-Categories ───────────────────────────────────"));
                println!("  An adjunction F ⊣ G between (∞,1)-cats C, D means:");
                println!("    Maps_D(F(x), y) ≃ Maps_C(x, G(y))  (natural ≃, not just bijection)");
                println!();
                println!("  The unit η: id → G∘F and counit ε: F∘G → id satisfy");
                println!("  triangle identities up to coherent homotopy.");
                println!();
                println!("  Limits and colimits are defined as adjoints to diagonal functors.");
                println!("  In HoTT: Π corresponds to right adjoint, Σ to left adjoint.");
                println!();
                println!("  Example: Σ ⊣ Ω  (suspension ⊣ loop space) is an adjunction");
                println!("  in the (∞,1)-category of pointed spaces.");
            }
            _ => println!("  {} Unknown: {cmd}. Type {} for help.", red("✗"), cyan("help")),
        }
        true
    }
}

fn main() {
    println!("{}", bold("\n  ╔══════════════════════════════════════════════════════════╗"));
    println!("{}", bold("  ║    Higher Category Theory — Interactive Sandbox           ║"));
    println!("{}", bold("  ╚══════════════════════════════════════════════════════════╝\n"));
    println!("  Build (∞,1)-categories and ∞-groupoids interactively.");
    println!("  Explore how types in HoTT are ∞-groupoids.");
    println!("  Type {} or {} to get started.\n",
        cyan("types-as-groupoids"), cyan("hypothesis"));

    let stdin = io::stdin();
    let mut sb = Sandbox::new();
    loop {
        print!("  {}∞-cat{} > ", CYAN, RESET);
        io::stdout().flush().unwrap();
        let mut line = String::new();
        match stdin.lock().read_line(&mut line) {
            Ok(0) | Err(_) => break,
            Ok(_) => { if !sb.handle(&line) { break; } }
        }
    }
    println!("  Goodbye.");
}
