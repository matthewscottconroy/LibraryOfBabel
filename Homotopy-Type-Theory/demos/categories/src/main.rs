// Category Builder — define objects, morphisms, composition; check axioms
use std::collections::HashMap;
use std::io::{self, BufRead, Write};

const B: &str = "\x1b[1m";  const R: &str = "\x1b[0m";
const G: &str = "\x1b[32m"; const C: &str = "\x1b[36m";
const Y: &str = "\x1b[33m"; const D: &str = "\x1b[2m";
const RE: &str = "\x1b[31m";

fn read_line(prompt: &str) -> String {
    print!("{B}{prompt}{R} ");
    io::stdout().flush().unwrap();
    let mut s = String::new();
    io::stdin().lock().read_line(&mut s).unwrap();
    s.trim().to_string()
}

// ── Category state ───────────────────────────────────────────────────────────

struct Category {
    name: String,
    objects: Vec<String>,
    morphisms: Vec<(String, String, String)>,   // (name, src, tgt)
    compose: HashMap<(String, String), String>, // (f, g) → g∘f  (f first, then g)
}

impl Category {
    fn new(name: &str) -> Self {
        Category { name: name.to_string(), objects: vec![], morphisms: vec![], compose: HashMap::new() }
    }

    fn has_obj(&self, x: &str) -> bool { self.objects.contains(&x.to_string()) }

    fn get_mor(&self, name: &str) -> Option<(&str, &str)> {
        self.morphisms.iter().find(|(n,_,_)| n == name).map(|(_,s,t)| (s.as_str(), t.as_str()))
    }

    fn add_obj(&mut self, name: &str) -> bool {
        if self.has_obj(name) { return false; }
        self.objects.push(name.to_string());
        // Add identity morphism
        let id_name = format!("id_{name}");
        self.morphisms.push((id_name.clone(), name.to_string(), name.to_string()));
        // id composed with anything = anything
        self.compose.insert((id_name.clone(), id_name.clone()), id_name);
        true
    }

    fn add_mor(&mut self, name: &str, src: &str, tgt: &str) -> Result<(), String> {
        if !self.has_obj(src) { return Err(format!("Unknown object '{src}'")); }
        if !self.has_obj(tgt) { return Err(format!("Unknown object '{tgt}'")); }
        if self.get_mor(name).is_some() { return Err(format!("Morphism '{name}' already exists")); }
        self.morphisms.push((name.to_string(), src.to_string(), tgt.to_string()));
        // Auto-compose with identities
        let id_src = format!("id_{src}");
        let id_tgt = format!("id_{tgt}");
        self.compose.insert((id_src, name.to_string()), name.to_string());
        self.compose.insert((name.to_string(), id_tgt), name.to_string());
        Ok(())
    }

    fn set_compose(&mut self, f: &str, g: &str, h: &str) -> Result<(), String> {
        // g ∘ f = h
        let (fs, ft) = self.get_mor(f).ok_or(format!("Unknown morphism '{f}'"))?.to_owned();
        let (gs, gt) = self.get_mor(g).ok_or(format!("Unknown morphism '{g}'"))?.to_owned();
        let (hs, ht) = self.get_mor(h).ok_or(format!("Unknown morphism '{h}'"))?.to_owned();
        if ft != gs {
            return Err(format!("Cannot compose: {f} ends at '{ft}' but {g} starts at '{gs}'"));
        }
        if hs != fs { return Err(format!("Target '{h}' should start at '{fs}'")); }
        if ht != gt { return Err(format!("Target '{h}' should end at '{gt}'")); }
        self.compose.insert((f.to_string(), g.to_string()), h.to_string());
        Ok(())
    }

    fn compose_mor(&self, f: &str, g: &str) -> Option<&str> {
        self.compose.get(&(f.to_string(), g.to_string())).map(|s| s.as_str())
    }

    fn check_identity(&self) -> Vec<String> {
        let mut failures = vec![];
        for (f, fs, ft) in &self.morphisms {
            let id_s = format!("id_{fs}");
            let id_t = format!("id_{ft}");
            // id_src ∘ f should be f
            match self.compose_mor(&id_s, f) {
                Some(r) if r == f => {}
                _ => failures.push(format!("id_{fs} ∘ {f} ≠ {f}")),
            }
            // f ∘ id_tgt should be f
            match self.compose_mor(f, &id_t) {
                Some(r) if r == f => {}
                _ => failures.push(format!("{f} ∘ id_{ft} ≠ {f}")),
            }
        }
        failures
    }

    fn check_associativity(&self) -> Vec<String> {
        let mut failures = vec![];
        let mors: Vec<(&str, &str, &str)> = self.morphisms.iter()
            .map(|(n,s,t)| (n.as_str(), s.as_str(), t.as_str())).collect();
        for (f, _fs, ft) in &mors {
            for (g, gs, gt) in &mors {
                if ft != gs { continue; }
                for (h, hs, _ht) in &mors {
                    if gt != hs { continue; }
                    // Check (h∘g)∘f = h∘(g∘f)
                    let gf = self.compose_mor(f, g);
                    let hg = self.compose_mor(g, h);
                    let lhs = gf.and_then(|gf| self.compose_mor(gf, h));
                    let rhs = hg.and_then(|hg| self.compose_mor(f, hg));
                    match (lhs, rhs) {
                        (Some(l), Some(r)) if l == r => {}
                        (Some(l), Some(r)) => {
                            failures.push(format!("({h}∘{g})∘{f} = {l} ≠ {r} = {h}∘({g}∘{f})"));
                        }
                        _ => {} // composition not defined — skip
                    }
                }
            }
        }
        failures
    }

    fn hom(&self, src: &str, tgt: &str) -> Vec<&str> {
        self.morphisms.iter()
            .filter(|(_,s,t)| s == src && t == tgt)
            .map(|(n,_,_)| n.as_str())
            .collect()
    }

    fn is_iso(&self, f: &str) -> bool {
        let (fs, ft) = match self.get_mor(f) { Some(x) => x, None => return false };
        // Find g with g∘f = id_src and f∘g = id_tgt
        let id_s = format!("id_{fs}");
        let id_t = format!("id_{ft}");
        self.morphisms.iter().any(|(g,_,_)| {
            self.compose_mor(f, g).map(|r| r == id_t).unwrap_or(false) &&
            self.compose_mor(g, f).map(|r| r == id_s).unwrap_or(false)
        })
    }

    fn display_diagram(&self) {
        println!("{B}Category '{}{}':{R}", C, self.name);
        println!("  Objects: {}", self.objects.iter().map(|o| format!("{Y}{o}{R}")).collect::<Vec<_>>().join("  "));
        println!("  Morphisms:");
        for (name, src, tgt) in &self.morphisms {
            let is_id = name.starts_with("id_");
            let display = if is_id { format!("{D}{name}{R}") } else { format!("{C}{name}{R}") };
            println!("    {} : {Y}{src}{R} → {Y}{tgt}{R}", display);
        }
        if !self.compose.is_empty() {
            println!("  Composition (non-trivial):");
            for ((f, g), h) in &self.compose {
                if f.starts_with("id_") || g.starts_with("id_") { continue; }
                println!("    {C}{g}{R} ∘ {C}{f}{R} = {G}{h}{R}");
            }
        }
    }
}

// ── Sandbox ──────────────────────────────────────────────────────────────────

struct Sandbox {
    cats: HashMap<String, Category>,
    current: String,
}

impl Sandbox {
    fn new() -> Self {
        let mut s = Sandbox { cats: HashMap::new(), current: "C".to_string() };
        s.cats.insert("C".to_string(), Category::new("C"));
        s
    }

    fn cur(&self) -> &Category { self.cats.get(&self.current).unwrap() }
    fn cur_mut(&mut self) -> &mut Category { self.cats.get_mut(&self.current).unwrap() }

    fn handle(&mut self, tokens: &[&str]) {
        match tokens[0] {
            "cat" | "new"    => self.cmd_cat(tokens),
            "use" | "switch" => self.cmd_use(tokens),
            "obj"            => self.cmd_obj(tokens),
            "mor"            => self.cmd_mor(tokens),
            "compose" | "∘"  => self.cmd_compose(tokens),
            "check"          => self.cmd_check(),
            "hom"            => self.cmd_hom(tokens),
            "iso"            => self.cmd_iso(tokens),
            "show" | "diag"  => self.cmd_show(),
            "example"        => self.cmd_example(tokens),
            "help" | "h" | "?" => self.help(),
            _ => println!("{RE}Unknown command. Type 'help'.{R}"),
        }
    }

    fn cmd_cat(&mut self, tokens: &[&str]) {
        let name = tokens.get(1).copied().unwrap_or("C");
        self.cats.entry(name.to_string()).or_insert_with(|| Category::new(name));
        self.current = name.to_string();
        println!("{G}Category '{name}' ready.{R}");
    }

    fn cmd_use(&mut self, tokens: &[&str]) {
        let name = tokens.get(1).copied().unwrap_or("C");
        if self.cats.contains_key(name) {
            self.current = name.to_string();
            println!("{G}Switched to category '{name}'.{R}");
        } else {
            println!("{RE}Unknown category '{name}'. Use 'cat {name}' to create it.{R}");
        }
    }

    fn cmd_obj(&mut self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: obj <A> [B] [C] ...{R}"); return; }
        for name in &tokens[1..] {
            if self.cur_mut().add_obj(name) {
                println!("{G}Object {Y}{name}{R}{G} added (id_{name} auto-created){R}");
            } else {
                println!("{D}Object '{name}' already exists.{R}");
            }
        }
    }

    fn cmd_mor(&mut self, tokens: &[&str]) {
        // mor <name> : <src> -> <tgt>   or   mor <name> <src> <tgt>
        if tokens.len() < 4 { println!("{RE}Usage: mor <name> : <A> -> <B>   or   mor <name> <A> <B>{R}"); return; }
        let (name, src, tgt) = if tokens[2] == ":" {
            let src = tokens[3];
            let tgt = tokens.get(5).copied().unwrap_or(tokens.get(4).copied().unwrap_or("?"));
            (tokens[1], src, tgt)
        } else {
            (tokens[1], tokens[2], tokens[3])
        };
        match self.cur_mut().add_mor(name, src, tgt) {
            Ok(()) => println!("{G}Morphism {C}{name}{R}{G} : {Y}{src}{R}{G} → {Y}{tgt}{R}{G} added.{R}"),
            Err(e) => println!("{RE}{e}{R}"),
        }
    }

    fn cmd_compose(&mut self, tokens: &[&str]) {
        // compose f g = h  (computes g ∘ f = h)
        if tokens.len() < 4 { println!("{RE}Usage: compose <f> <g> = <h>  — sets g∘f = h{R}"); return; }
        let (f, g) = (tokens[1], tokens[2]);
        // Check if this is a query or definition
        if tokens.len() == 3 || (tokens.len() >= 4 && tokens[3] != "=") {
            // Query: what is g∘f?
            match self.cur().compose_mor(f, g) {
                Some(h) => println!("{C}{g}{R} ∘ {C}{f}{R} = {G}{h}{R}"),
                None => println!("{Y}Composition {C}{g}{R} ∘ {C}{f}{R} not yet defined. Use: compose {f} {g} = <result>{R}"),
            }
            return;
        }
        let h = tokens.get(4).copied().unwrap_or_default();
        match self.cur_mut().set_compose(f, g, h) {
            Ok(()) => println!("{C}{g}{R} ∘ {C}{f}{R} = {G}{h}{R}  {D}(defined){R}"),
            Err(e) => println!("{RE}{e}{R}"),
        }
    }

    fn cmd_check(&self) {
        let cat = self.cur();
        println!("{B}Checking category axioms for '{}{}':{R}", C, cat.name);

        // All objects present
        println!("  {} Objects: {}", tick(true), cat.objects.join(", "));

        // Identities exist
        let id_ok = cat.objects.iter().all(|o| cat.get_mor(&format!("id_{o}")).is_some());
        println!("  {} Identities: {}", tick(id_ok),
            if id_ok { format!("{G}all objects have identity morphisms{R}") }
            else { format!("{RE}some objects missing identity!{R}") });

        // Identity laws
        let id_laws = cat.check_identity();
        println!("  {} Identity laws: {}", tick(id_laws.is_empty()),
            if id_laws.is_empty() { format!("{G}f∘id = f and id∘f = f for all f{R}") }
            else { format!("{RE}{} failures: {}{R}", id_laws.len(), id_laws[0]) });

        // Associativity
        let assoc = cat.check_associativity();
        println!("  {} Associativity: {}", tick(assoc.is_empty()),
            if assoc.is_empty() { format!("{G}(h∘g)∘f = h∘(g∘f) for all composable triples{R}") }
            else { format!("{RE}{} failures: {}{R}", assoc.len(), assoc[0]) });

        if id_ok && id_laws.is_empty() && assoc.is_empty() {
            println!("\n  {G}✓ Valid category!{R}");
        }
    }

    fn cmd_hom(&self, tokens: &[&str]) {
        if tokens.len() < 3 { println!("{RE}Usage: hom <A> <B>{R}"); return; }
        let (src, tgt) = (tokens[1], tokens[2]);
        let hom = self.cur().hom(src, tgt);
        if hom.is_empty() {
            println!("{Y}Hom({src},{tgt}) = ∅  (no morphisms from {src} to {tgt}){R}");
        } else {
            println!("{B}Hom({Y}{src}{R},{Y}{tgt}{R}) = {{ {} }}", hom.iter().map(|f| format!("{C}{f}{R}")).collect::<Vec<_>>().join(", "));
        }
    }

    fn cmd_iso(&self, tokens: &[&str]) {
        if tokens.len() < 2 { println!("{RE}Usage: iso <f>{R}"); return; }
        let f = tokens[1];
        if self.cur().get_mor(f).is_none() { println!("{RE}Unknown morphism '{f}'{R}"); return; }
        if self.cur().is_iso(f) {
            println!("{G}{f} is an isomorphism (invertible){R}");
        } else {
            println!("{Y}{f} is not an isomorphism (no inverse found){R}");
            println!("{D}To make it an iso, define its inverse and set composition to identities.{R}");
        }
    }

    fn cmd_show(&self) { self.cur().display_diagram(); }

    fn cmd_example(&mut self, tokens: &[&str]) {
        let kind = tokens.get(1).copied().unwrap_or("2");
        match kind {
            "1" | "terminal" => {
                println!("{D}Building the terminal category 1 (one object, one morphism)...{R}");
                self.cats.insert("1".to_string(), Category::new("1"));
                self.current = "1".to_string();
                let cat = self.cur_mut();
                cat.add_obj("*");
                println!("{G}Category '1' created: one object *, one morphism id_*.{R}");
            }
            "2" | "arrow" => {
                println!("{D}Building the walking arrow category 2 (0 → 1)...{R}");
                self.cats.insert("2".to_string(), Category::new("2"));
                self.current = "2".to_string();
                let cat = self.cur_mut();
                cat.add_obj("0");
                cat.add_obj("1");
                let _ = cat.add_mor("f", "0", "1");
                println!("{G}Category '2' created. Objects: 0,1. Morphism f:0→1.{R}");
            }
            "3" | "chain" => {
                println!("{D}Building the chain category 0 → 1 → 2...{R}");
                self.cats.insert("chain".to_string(), Category::new("chain"));
                self.current = "chain".to_string();
                let cat = self.cur_mut();
                cat.add_obj("0"); cat.add_obj("1"); cat.add_obj("2");
                let _ = cat.add_mor("f", "0", "1");
                let _ = cat.add_mor("g", "1", "2");
                let _ = cat.add_mor("gf", "0", "2");
                let _ = cat.set_compose("f", "g", "gf");
                println!("{G}Chain category created. Try: compose f g → check → iso f{R}");
            }
            _ => println!("{RE}Examples: example 1  |  example 2  |  example 3{R}"),
        }
    }

    fn help(&self) {
        println!("{B}Category Builder{R} — define categories, check axioms\n");
        println!("{B}Setup:{R}");
        println!("  {C}cat <name>{R}              create/switch to a category");
        println!("  {C}example 1{R}               load a preset (1, 2, 3/chain)");
        println!();
        println!("{B}Build:{R}");
        println!("  {C}obj A B C{R}               add objects (identity morphisms auto-created)");
        println!("  {C}mor f : A -> B{R}           add morphism f : A → B");
        println!("  {C}compose f g = h{R}          define g ∘ f = h");
        println!("  {C}compose f g{R}              query: what is g ∘ f?");
        println!();
        println!("{B}Verify:{R}");
        println!("  {C}check{R}                   verify all category axioms");
        println!("  {C}hom A B{R}                 list morphisms from A to B");
        println!("  {C}iso f{R}                   check if f is an isomorphism");
        println!("  {C}show{R}                    display the category diagram");
        println!();
        println!("{D}Try: obj A B C → mor f : A -> B → mor g : B -> C → mor gf : A -> C → compose f g = gf → check{R}");
    }
}

fn tick(b: bool) -> &'static str { if b { "\x1b[32m✓\x1b[0m" } else { "\x1b[31m✗\x1b[0m" } }

fn main() {
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{B}  Category Builder{R}");
    println!("{D}  Define objects, morphisms, composition; check axioms{R}");
    println!("{B}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━{R}");
    println!("{D}Type 'help'. Try: example 3 → show → check{R}\n");

    let mut sb = Sandbox::new();
    loop {
        let line = read_line("cat>");
        if line.is_empty() { continue; }
        let tokens: Vec<&str> = line.split_whitespace().collect();
        match tokens[0] {
            "quit" | "q" | "exit" => { println!("{D}Goodbye.{R}"); break; }
            _ => sb.handle(&tokens),
        }
    }
}
