// Common utilities for Abstract Algebra chapter demos.
// Every chapter program imports this crate for display, math, and REPL infrastructure.

use std::fmt;
use std::io::{self, Write};
use std::collections::HashMap;

// ─── Terminal display ─────────────────────────────────────────────────────────

const BOLD: &str = "\x1b[1m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const YELLOW: &str = "\x1b[33m";
const GREEN: &str = "\x1b[32m";
const MAGENTA: &str = "\x1b[35m";
const RED: &str = "\x1b[31m";
const BLUE: &str = "\x1b[34m";
const BG_BLUE: &str = "\x1b[44m";

pub fn bold(s: &str) -> String { format!("{}{}{}", BOLD, s, RESET) }
pub fn cyan(s: &str) -> String { format!("{}{}{}", CYAN, s, RESET) }
pub fn yellow(s: &str) -> String { format!("{}{}{}", YELLOW, s, RESET) }
pub fn green(s: &str) -> String { format!("{}{}{}", GREEN, s, RESET) }
pub fn magenta(s: &str) -> String { format!("{}{}{}", MAGENTA, s, RESET) }
pub fn red(s: &str) -> String { format!("{}{}{}", RED, s, RESET) }
pub fn blue(s: &str) -> String { format!("{}{}{}", BLUE, s, RESET) }
pub fn dim(s: &str) -> String { format!("{}{}{}", DIM, s, RESET) }

pub fn print_banner(chapter: &str, title: &str, tagline: &str) {
    println!();
    let width = 70;
    let top = format!("╔{}╗", "═".repeat(width - 2));
    let bot = format!("╚{}╝", "═".repeat(width - 2));
    println!("{}", cyan(&top));
    let ch_line = format!("  {}{}{}", BOLD, chapter, RESET);
    println!("{}  {}{}{}{}",
        cyan("║"),
        BG_BLUE, BOLD, pad_center(title, width - 4), RESET);
    println!("{}  {}{}{}",
        cyan("║"),
        DIM, pad_center(tagline, width - 4), RESET);
    println!("{}  {}{}{}",
        cyan("║"),
        DIM, pad_center(ch_line.trim(), width - 4), RESET);
    println!("{}", cyan(&bot));
    println!();
}

fn pad_center(s: &str, width: usize) -> String {
    // strip ANSI for length calc
    let visible: String = {
        let mut out = String::new();
        let mut in_escape = false;
        for c in s.chars() {
            if c == '\x1b' { in_escape = true; }
            if !in_escape { out.push(c); }
            if in_escape && c == 'm' { in_escape = false; }
        }
        out
    };
    let len = visible.chars().count();
    if len >= width { return s.to_string(); }
    let pad = (width - len) / 2;
    format!("{}{}{}", " ".repeat(pad), s, " ".repeat(width - len - pad))
}

pub fn print_section(title: &str) {
    println!("\n  {}{}▸ {}{}", BOLD, YELLOW, title, RESET);
}

pub fn print_result(label: &str, value: &str) {
    println!("  {} {} {}", cyan(label), dim("="), green(value));
}

pub fn print_info(text: &str) {
    println!("  {}{}{}", DIM, text, RESET);
}

pub fn print_note(text: &str) {
    println!("  {} {}", yellow("◆"), text);
}

pub fn print_err(text: &str) {
    println!("  {} {}", red("✗"), text);
}

pub fn print_ok(text: &str) {
    println!("  {} {}", green("✓"), text);
}

pub fn print_sep() {
    println!("  {}", dim(&"─".repeat(60)));
}

pub fn print_help(commands: &[(&str, &str)]) {
    println!();
    println!("  {}", bold("Commands:"));
    let max = commands.iter().map(|(c,_)| c.len()).max().unwrap_or(0);
    for (cmd, desc) in commands {
        println!("    {:<width$}  {}", cyan(cmd), dim(desc), width = max + 2);
    }
    println!();
}

/// Build a help string from a commands table (same layout as print_help, ANSI-free).
pub fn help_string(commands: &[(&str, &str)]) -> String {
    let max = commands.iter().map(|(c, _)| c.len()).max().unwrap_or(0);
    let mut out = String::from("\nCommands:\n");
    for (cmd, desc) in commands {
        out.push_str(&format!("    {:<width$}  {}\n", cmd, desc, width = max + 2));
    }
    out
}

// ─── REPL runner ─────────────────────────────────────────────────────────────

/// Run an interactive read-eval-print loop.
/// `handler(cmd, args)` returns `false` to exit.
pub fn repl(prompt: &str, handler: &mut dyn FnMut(&str, &[&str]) -> bool) {
    loop {
        print!("\n  {}{}{} ", BOLD, CYAN, prompt);
        let _ = io::stdout().flush();
        let mut line = String::new();
        if io::stdin().read_line(&mut line).is_err() { break; }
        let line = line.trim();
        if line.is_empty() { continue; }
        let parts: Vec<&str> = line.split_whitespace().collect();
        let cmd = parts[0];
        let args = &parts[1..];
        if !handler(cmd, args) { break; }
    }
    println!("\n  {}", dim("Goodbye."));
}

/// Parse a single integer argument, printing an error if it fails.
pub fn parse_int(args: &[&str], idx: usize, name: &str) -> Option<i64> {
    args.get(idx).and_then(|s| s.parse().ok()).or_else(|| {
        print_err(&format!("Expected integer argument <{}>", name));
        None
    })
}

pub fn parse_uint(args: &[&str], idx: usize, name: &str) -> Option<u64> {
    args.get(idx).and_then(|s| s.parse().ok()).or_else(|| {
        print_err(&format!("Expected non-negative integer argument <{}>", name));
        None
    })
}

// ─── Number theory utilities ──────────────────────────────────────────────────

pub fn gcd(mut a: i64, mut b: i64) -> i64 {
    a = a.abs(); b = b.abs();
    while b != 0 { let t = b; b = a % b; a = t; }
    a
}

pub fn lcm(a: i64, b: i64) -> i64 {
    if a == 0 || b == 0 { 0 } else { (a / gcd(a, b)) * b }
}

/// Extended Euclidean: returns (g, s, t) with a*s + b*t = g = gcd(a,b)
pub fn ext_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if b == 0 { return (a, 1, 0); }
    let (g, s1, t1) = ext_gcd(b, a % b);
    (g, t1, s1 - (a / b) * t1)
}

pub fn is_prime(n: u64) -> bool {
    if n < 2 { return false; }
    if n == 2 { return true; }
    if n % 2 == 0 { return false; }
    let mut i = 3u64;
    while i * i <= n { if n % i == 0 { return false; } i += 2; }
    true
}

/// Returns (prime, exponent) pairs
pub fn factor(mut n: u64) -> Vec<(u64, u32)> {
    let mut out = Vec::new();
    let mut d = 2u64;
    while d * d <= n {
        if n % d == 0 {
            let mut e = 0u32;
            while n % d == 0 { e += 1; n /= d; }
            out.push((d, e));
        }
        d += 1;
    }
    if n > 1 { out.push((n, 1)); }
    out
}

/// All primes up to n (sieve)
pub fn primes_up_to(n: u64) -> Vec<u64> {
    if n < 2 { return vec![]; }
    let mut sieve = vec![true; (n + 1) as usize];
    sieve[0] = false; sieve[1] = false;
    let mut i = 2u64;
    while i * i <= n {
        if sieve[i as usize] {
            let mut j = i * i;
            while j <= n { sieve[j as usize] = false; j += i; }
        }
        i += 1;
    }
    (2..=n).filter(|&k| sieve[k as usize]).collect()
}

pub fn euler_totient(n: u64) -> u64 {
    factor(n).iter().fold(n, |acc, &(p, _)| acc / p * (p - 1))
}

pub fn mod_pow(mut base: i64, mut exp: u64, m: i64) -> i64 {
    let mut result = 1i64;
    base = base.rem_euclid(m);
    while exp > 0 {
        if exp & 1 == 1 { result = result * base % m; }
        base = base * base % m;
        exp >>= 1;
    }
    result
}

/// Multiplicative order of a mod m (returns None if gcd(a,m) != 1)
pub fn mult_order(a: i64, m: i64) -> Option<u64> {
    if gcd(a, m) != 1 { return None; }
    let phi = euler_totient(m as u64);
    let divs = divisors(phi);
    for &d in &divs {
        if mod_pow(a, d, m) == 1 { return Some(d); }
    }
    None
}

/// All divisors of n, sorted
pub fn divisors(n: u64) -> Vec<u64> {
    let mut divs: Vec<u64> = (1..=n).filter(|&d| n % d == 0).collect();
    divs.sort();
    divs
}

// ─── Integer matrix ───────────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct Mat {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<i64>,
}

impl Mat {
    pub fn new(rows: usize, cols: usize, data: Vec<i64>) -> Self {
        assert_eq!(data.len(), rows * cols);
        Mat { rows, cols, data }
    }

    pub fn zero(rows: usize, cols: usize) -> Self {
        Mat { rows, cols, data: vec![0; rows * cols] }
    }

    pub fn identity(n: usize) -> Self {
        let mut m = Self::zero(n, n);
        for i in 0..n { m[(i, i)] = 1; }
        m
    }

    pub fn from_rows(rows: Vec<Vec<i64>>) -> Self {
        let r = rows.len();
        let c = if r == 0 { 0 } else { rows[0].len() };
        let data = rows.into_iter().flatten().collect();
        Mat { rows: r, cols: c, data }
    }
}

impl std::ops::Index<(usize, usize)> for Mat {
    type Output = i64;
    fn index(&self, (r, c): (usize, usize)) -> &i64 { &self.data[r * self.cols + c] }
}

impl std::ops::IndexMut<(usize, usize)> for Mat {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut i64 {
        &mut self.data[r * self.cols + c]
    }
}

impl fmt::Display for Mat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // compute column widths
        let widths: Vec<usize> = (0..self.cols)
            .map(|c| (0..self.rows).map(|r| format!("{}", self[(r,c)]).len()).max().unwrap_or(1))
            .collect();
        for r in 0..self.rows {
            write!(f, "  │ ")?;
            for c in 0..self.cols {
                write!(f, "{:>width$}", self[(r,c)], width = widths[c])?;
                if c + 1 < self.cols { write!(f, "  ")?; }
            }
            writeln!(f, " │")?;
        }
        Ok(())
    }
}

impl Mat {
    pub fn mul(&self, other: &Mat) -> Mat {
        assert_eq!(self.cols, other.rows);
        let mut out = Mat::zero(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    out[(i,j)] += self[(i,k)] * other[(k,j)];
                }
            }
        }
        out
    }

    pub fn transpose(&self) -> Mat {
        let mut out = Mat::zero(self.cols, self.rows);
        for i in 0..self.rows {
            for j in 0..self.cols { out[(j,i)] = self[(i,j)]; }
        }
        out
    }

    pub fn det(&self) -> i64 {
        assert_eq!(self.rows, self.cols);
        let n = self.rows;
        if n == 1 { return self[(0,0)]; }
        if n == 2 { return self[(0,0)]*self[(1,1)] - self[(0,1)]*self[(1,0)]; }
        // Laplace expansion along first row
        let mut result = 0i64;
        for j in 0..n {
            let minor = self.minor(0, j);
            let sign = if j % 2 == 0 { 1 } else { -1 };
            result += sign * self[(0,j)] * minor.det();
        }
        result
    }

    pub fn minor(&self, skip_r: usize, skip_c: usize) -> Mat {
        let data: Vec<i64> = (0..self.rows)
            .filter(|&r| r != skip_r)
            .flat_map(|r| (0..self.cols).filter(|&c| c != skip_c).map(move |c| (r, c)))
            .map(|(r,c)| self[(r,c)])
            .collect();
        Mat::new(self.rows - 1, self.cols - 1, data)
    }

    /// Row reduce over ℚ (returns steps log); mutates self
    pub fn row_reduce_steps(&mut self) -> Vec<String> {
        let mut steps = Vec::new();
        let mut pivot_row = 0;
        for col in 0..self.cols {
            // find pivot
            let mut found = None;
            for r in pivot_row..self.rows {
                if self[(r, col)] != 0 { found = Some(r); break; }
            }
            let pr = match found { None => continue, Some(r) => r };
            if pr != pivot_row {
                for c in 0..self.cols {
                    let tmp = self[(pr, c)];
                    self[(pr, c)] = self[(pivot_row, c)];
                    self[(pivot_row, c)] = tmp;
                }
                steps.push(format!("R{} ↔ R{}", pivot_row+1, pr+1));
            }
            // eliminate below
            for r in (pivot_row+1)..self.rows {
                if self[(r, col)] == 0 { continue; }
                let g = gcd(self[(pivot_row, col)].abs(), self[(r, col)].abs());
                let a = self[(pivot_row, col)] / g;
                let b = self[(r, col)] / g;
                for c in 0..self.cols {
                    let val = a * self[(r, c)] - b * self[(pivot_row, c)];
                    self[(r, c)] = val;
                }
                steps.push(format!("R{} ← {}·R{} − {}·R{}",
                    r+1, a, r+1, b, pivot_row+1));
            }
            pivot_row += 1;
        }
        steps
    }

    /// Smith normal form over ℤ: returns D (diagonal), and logs the operations
    pub fn smith_normal_form(&self) -> (Mat, Vec<String>) {
        let mut d = self.clone();
        let mut ops = Vec::new();
        let min_dim = d.rows.min(d.cols);
        for k in 0..min_dim {
            // find smallest nonzero entry in submatrix [k.., k..]
            loop {
                let mut changed = false;
                // clear off-diagonal in row k and col k
                for c in (k+1)..d.cols {
                    if d[(k,c)] == 0 { continue; }
                    let g = gcd(d[(k,k)].abs(), d[(k,c)].abs());
                    if g == 0 { continue; }
                    if d[(k,k)] == 0 {
                        // swap columns
                        for r in 0..d.rows {
                            let tmp = d[(r,k)]; d[(r,k)] = d[(r,c)]; d[(r,c)] = tmp;
                        }
                        ops.push(format!("C{} ↔ C{}", k+1, c+1));
                        changed = true;
                    } else {
                        let q = d[(k,c)] / d[(k,k)];
                        for r in 0..d.rows { d[(r,c)] -= q * d[(r,k)]; }
                        ops.push(format!("C{} ← C{} − {}·C{}", c+1, c+1, q, k+1));
                        changed = true;
                    }
                }
                for r in (k+1)..d.rows {
                    if d[(r,k)] == 0 { continue; }
                    let g = gcd(d[(k,k)].abs(), d[(r,k)].abs());
                    if g == 0 { continue; }
                    if d[(k,k)] == 0 {
                        for c in 0..d.cols {
                            let tmp = d[(k,c)]; d[(k,c)] = d[(r,c)]; d[(r,c)] = tmp;
                        }
                        ops.push(format!("R{} ↔ R{}", k+1, r+1));
                        changed = true;
                    } else {
                        let q = d[(r,k)] / d[(k,k)];
                        for c in 0..d.cols { d[(r,c)] -= q * d[(k,c)]; }
                        ops.push(format!("R{} ← R{} − {}·R{}", r+1, r+1, q, k+1));
                        changed = true;
                    }
                }
                if !changed { break; }
            }
            // make diagonal entry positive
            if d[(k,k)] < 0 {
                for c in 0..d.cols { d[(k,c)] = -d[(k,c)]; }
            }
        }
        (d, ops)
    }
}

// ─── Float matrix (for eigenvalues) ──────────────────────────────────────────

#[derive(Clone, Debug)]
pub struct FMat {
    pub rows: usize,
    pub cols: usize,
    pub data: Vec<f64>,
}

impl FMat {
    pub fn new(rows: usize, cols: usize, data: Vec<f64>) -> Self {
        FMat { rows, cols, data }
    }
    pub fn zero(rows: usize, cols: usize) -> Self {
        FMat { rows, cols, data: vec![0.0; rows * cols] }
    }
    pub fn identity(n: usize) -> Self {
        let mut m = Self::zero(n, n);
        for i in 0..n { m.data[i * n + i] = 1.0; }
        m
    }
}

impl std::ops::Index<(usize, usize)> for FMat {
    type Output = f64;
    fn index(&self, (r, c): (usize, usize)) -> &f64 { &self.data[r * self.cols + c] }
}
impl std::ops::IndexMut<(usize, usize)> for FMat {
    fn index_mut(&mut self, (r, c): (usize, usize)) -> &mut f64 {
        &mut self.data[r * self.cols + c]
    }
}

impl FMat {
    pub fn mul(&self, other: &FMat) -> FMat {
        let mut out = FMat::zero(self.rows, other.cols);
        for i in 0..self.rows {
            for j in 0..other.cols {
                for k in 0..self.cols {
                    out.data[i * out.cols + j] += self[(i,k)] * other[(k,j)];
                }
            }
        }
        out
    }

    pub fn power_iteration(&self, iters: usize) -> (f64, Vec<f64>) {
        let n = self.rows;
        let mut v: Vec<f64> = vec![1.0; n];
        let mut eigenval = 0.0;
        for _ in 0..iters {
            // w = A*v
            let mut w = vec![0.0; n];
            for i in 0..n {
                for j in 0..n { w[i] += self[(i,j)] * v[j]; }
            }
            // find largest component
            let norm = w.iter().cloned().fold(0.0f64, f64::max);
            if norm.abs() < 1e-15 { break; }
            eigenval = norm;
            v = w.iter().map(|x| x / norm).collect();
        }
        (eigenval, v)
    }

    pub fn display(&self) -> String {
        let mut s = String::new();
        for r in 0..self.rows {
            s.push_str("  │ ");
            for c in 0..self.cols {
                s.push_str(&format!("{:8.4}", self[(r,c)]));
                if c + 1 < self.cols { s.push_str("  "); }
            }
            s.push_str(" │\n");
        }
        s
    }
}

// ─── Polynomial over ℤ ────────────────────────────────────────────────────────

#[derive(Clone, Debug, PartialEq)]
pub struct Poly {
    /// coeffs[i] = coefficient of x^i
    pub coeffs: Vec<i64>,
}

impl Poly {
    pub fn new(coeffs: Vec<i64>) -> Self {
        let mut p = Poly { coeffs };
        p.normalize();
        p
    }

    pub fn zero() -> Self { Poly { coeffs: vec![] } }
    pub fn one() -> Self { Poly { coeffs: vec![1] } }
    pub fn x() -> Self { Poly { coeffs: vec![0, 1] } }

    fn normalize(&mut self) {
        while self.coeffs.last() == Some(&0) { self.coeffs.pop(); }
    }

    pub fn degree(&self) -> Option<usize> {
        if self.coeffs.is_empty() { None } else { Some(self.coeffs.len() - 1) }
    }

    pub fn leading(&self) -> i64 {
        self.coeffs.last().copied().unwrap_or(0)
    }

    pub fn eval(&self, x: i64) -> i64 {
        self.coeffs.iter().enumerate().map(|(i, &c)| c * x.pow(i as u32)).sum()
    }

    pub fn eval_mod(&self, x: i64, m: i64) -> i64 {
        self.coeffs.iter().enumerate()
            .map(|(i, &c)| c * mod_pow(x, i as u64, m) % m)
            .fold(0, |a, b| (a + b).rem_euclid(m))
    }

    pub fn add(&self, other: &Poly) -> Poly {
        let len = self.coeffs.len().max(other.coeffs.len());
        let coeffs = (0..len).map(|i| {
            self.coeffs.get(i).copied().unwrap_or(0)
            + other.coeffs.get(i).copied().unwrap_or(0)
        }).collect();
        Poly::new(coeffs)
    }

    pub fn sub(&self, other: &Poly) -> Poly {
        let len = self.coeffs.len().max(other.coeffs.len());
        let coeffs = (0..len).map(|i| {
            self.coeffs.get(i).copied().unwrap_or(0)
            - other.coeffs.get(i).copied().unwrap_or(0)
        }).collect();
        Poly::new(coeffs)
    }

    pub fn mul(&self, other: &Poly) -> Poly {
        if self.coeffs.is_empty() || other.coeffs.is_empty() { return Poly::zero(); }
        let mut coeffs = vec![0i64; self.coeffs.len() + other.coeffs.len() - 1];
        for (i, &a) in self.coeffs.iter().enumerate() {
            for (j, &b) in other.coeffs.iter().enumerate() {
                coeffs[i + j] += a * b;
            }
        }
        Poly::new(coeffs)
    }

    pub fn scalar_mul(&self, s: i64) -> Poly {
        Poly::new(self.coeffs.iter().map(|&c| c * s).collect())
    }

    /// Polynomial division over ℤ (returns None if exact division fails)
    pub fn div(&self, other: &Poly) -> Option<Poly> {
        let (q, r) = self.div_rem(other);
        if r.coeffs.is_empty() { Some(q) } else { None }
    }

    pub fn div_rem(&self, other: &Poly) -> (Poly, Poly) {
        if other.coeffs.is_empty() { panic!("Division by zero polynomial"); }
        let mut rem = self.clone();
        let mut quot_coeffs = vec![];
        while rem.degree() >= other.degree() && !rem.coeffs.is_empty() {
            let rd = rem.degree().unwrap();
            let od = other.degree().unwrap();
            let lc_r = rem.leading();
            let lc_o = other.leading();
            if lc_r % lc_o != 0 { break; }
            let c = lc_r / lc_o;
            let deg = rd - od;
            if quot_coeffs.len() <= deg { quot_coeffs.resize(deg + 1, 0); }
            quot_coeffs[deg] = c;
            // subtract c * x^deg * other from rem
            let mut sub_coeffs = vec![0i64; deg + other.coeffs.len()];
            for (i, &v) in other.coeffs.iter().enumerate() {
                sub_coeffs[i + deg] += c * v;
            }
            let sub = Poly::new(sub_coeffs);
            rem = rem.sub(&sub);
        }
        (Poly::new(quot_coeffs), rem)
    }

    pub fn gcd_poly(a: &Poly, b: &Poly) -> Poly {
        if b.coeffs.is_empty() { return a.clone(); }
        let (_, r) = a.div_rem(b);
        Self::gcd_poly(b, &r)
    }

    /// Check irreducibility over 𝔽_p using the standard test:
    /// f is irreducible iff gcd(f, x^(p^k) − x mod f) = 1 for all k = 1..⌊deg/2⌋.
    pub fn is_irreducible_mod(&self, p: u64) -> bool {
        let deg = match self.degree() { None => return false, Some(0) => return false, Some(d) => d };
        if deg == 1 { return true; }
        let pm = p as i64;
        // Normalise to monic mod p
        let monic = self.reduce_mod(pm);
        // For each k = 1..deg/2, compute h = x^(p^k) mod f(x) over F_p,
        // then gcd(f, h - x). If that gcd is non-trivial, f is reducible.
        let x_poly = Poly::new(vec![0, 1]); // x
        let mut h = x_poly.clone();         // starts as x = x^(p^0 * p)
        for _ in 1..=(deg / 2) {
            // h ← h^p mod f mod p  (repeated squaring, p times via fast exp)
            h = poly_mod_pow_mod(&h, p, &monic, pm);
            // gcd(f, h − x)
            let diff = h.sub(&x_poly).reduce_mod(pm);
            let g = poly_gcd_mod(&monic, &diff, pm);
            if g.degree().unwrap_or(0) >= 1 { return false; }
        }
        true
    }

    /// Reduce all coefficients modulo `m` (into [0, m))
    pub fn reduce_mod(&self, m: i64) -> Poly {
        Poly::new(self.coeffs.iter().map(|&c| c.rem_euclid(m)).collect())
    }

    /// Factor self over 𝔽_p; returns `(irreducible_factor, multiplicity)` pairs.
    /// Uses trial division by all monic polynomials of degree 1..deg/2 over 𝔽_p.
    /// Practical for small p (≤ 13) and degree ≤ 8.
    pub fn factor_mod(&self, p: u64) -> Vec<(Poly, usize)> {
        let pm = p as i64;
        let lc = self.leading().rem_euclid(pm);
        if lc == 0 { return vec![]; }
        // make monic
        let inv_lc = mod_inv_pos(lc, pm);
        let mut rem = self.scalar_mul(inv_lc).reduce_mod(pm);
        let mut result: Vec<(Poly, usize)> = Vec::new();

        // Trial division: degree 1 factors first (roots)
        for root in 0..p {
            if rem.eval_mod(root as i64, pm) == 0 {
                let factor = Poly::new(vec![pm - root as i64, 1]).reduce_mod(pm); // x - root
                let mut mult = 0;
                while rem.eval_mod(root as i64, pm) == 0 {
                    let (q, r) = rem.div_rem_mod(&factor, pm);
                    if r.coeffs.iter().all(|&c| c.rem_euclid(pm) == 0) {
                        rem = q.reduce_mod(pm);
                        mult += 1;
                    } else { break; }
                }
                if mult > 0 { result.push((factor, mult)); }
            }
        }

        // Trial division by monic irreducible polynomials of degree 2..deg/2
        let max_d = rem.degree().unwrap_or(0) / 2;
        for d in 2..=max_d {
            let mut candidates = monic_polys_mod(d, pm);
            for cand in candidates.drain(..) {
                if !cand.is_irreducible_mod(p) { continue; }
                let mut mult = 0;
                loop {
                    let (q, r) = rem.div_rem_mod(&cand, pm);
                    if r.reduce_mod(pm).coeffs.iter().all(|&c| c == 0) {
                        rem = q.reduce_mod(pm);
                        mult += 1;
                    } else { break; }
                }
                if mult > 0 { result.push((cand, mult)); }
                if rem.degree().map_or(true, |d| d == 0) { break; }
            }
        }

        // remaining factor (should be irreducible)
        if rem.degree().unwrap_or(0) >= 1 {
            result.push((rem, 1));
        }
        result
    }

    /// Polynomial division mod p: returns (quotient, remainder) with coefficients in [0, p)
    pub fn div_rem_mod(&self, other: &Poly, p: i64) -> (Poly, Poly) {
        if other.coeffs.is_empty() { panic!("division by zero polynomial"); }
        let mut rem: Vec<i64> = self.coeffs.iter().map(|&c| c.rem_euclid(p)).collect();
        let od = other.coeffs.len() - 1;
        let lc_inv = mod_inv_pos(other.coeffs[od].rem_euclid(p), p);
        let mut quot = vec![0i64; rem.len().saturating_sub(od)];
        while rem.len() > od {
            let rd = rem.len() - 1;
            let c = (rem[rd] * lc_inv).rem_euclid(p);
            let deg = rd - od;
            quot[deg] = c;
            for (i, &v) in other.coeffs.iter().enumerate() {
                rem[i + deg] = (rem[i + deg] - c * v).rem_euclid(p);
            }
            while rem.last() == Some(&0) { rem.pop(); }
        }
        (Poly::new(quot).reduce_mod(p), Poly::new(rem).reduce_mod(p))
    }
}

impl fmt::Display for Poly {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.coeffs.is_empty() { return write!(f, "0"); }
        let mut first = true;
        for (i, &c) in self.coeffs.iter().enumerate().rev() {
            if c == 0 { continue; }
            if !first {
                write!(f, " {} ", if c < 0 { "−" } else { "+" })?;
            } else if c < 0 {
                write!(f, "−")?;
            }
            let ac = c.abs();
            match i {
                0 => write!(f, "{}", ac)?,
                1 => if ac == 1 { write!(f, "x")? } else { write!(f, "{}x", ac)? },
                _ => if ac == 1 { write!(f, "x^{}", i)? } else { write!(f, "{}x^{}", ac, i)? },
            }
            first = false;
        }
        Ok(())
    }
}

// ─── Finite group utilities ───────────────────────────────────────────────────

/// Cayley table for ℤ/nℤ under addition
pub fn zn_add_table(n: usize) -> Vec<Vec<usize>> {
    (0..n).map(|i| (0..n).map(|j| (i + j) % n).collect()).collect()
}

/// Cayley table for (ℤ/nℤ)* under multiplication
pub fn zn_mul_table(n: usize) -> (Vec<usize>, Vec<Vec<usize>>) {
    let units: Vec<usize> = (1..n).filter(|&x| gcd(x as i64, n as i64) == 1).collect();
    let m = units.len();
    let table = (0..m).map(|i|
        (0..m).map(|j| {
            let prod = (units[i] * units[j]) % n;
            units.iter().position(|&u| u == prod).unwrap()
        }).collect()
    ).collect();
    (units, table)
}

/// Order of element a in ℤ/nℤ under addition
pub fn additive_order(a: usize, n: usize) -> usize {
    if a == 0 { return 1; }
    let g = gcd(a as i64, n as i64) as usize;
    n / g
}

/// Print a Cayley table with colored formatting
pub fn print_cayley(elements: &[String], table: &[Vec<usize>]) {
    let n = elements.len();
    let w = elements.iter().map(|s| s.len()).max().unwrap_or(1) + 1;
    // header
    print!("  {:>width$} │", "·", width = w);
    for e in elements { print!(" {:>width$}", cyan(e), width = w); }
    println!();
    print!("  {}─┼", "─".repeat(w));
    for _ in 0..n { print!("{}", "─".repeat(w + 1)); }
    println!();
    for (i, row) in table.iter().enumerate() {
        print!("  {:>width$} │", green(&elements[i]), width = w);
        for &j in row { print!(" {:>width$}", elements[j], width = w); }
        println!();
    }
}

/// Subgroups of ℤ/nℤ (divisors of n, each gives subgroup of that order)
pub fn subgroups_zn(n: usize) -> Vec<Vec<usize>> {
    divisors(n as u64).iter().map(|&d| {
        let step = n / d as usize;
        (0..n).step_by(step).collect()
    }).collect()
}

// ─── Poly helper free functions ──────────────────────────────────────────────

/// Modular inverse of a mod m (m prime); panics if a ≡ 0.
pub fn mod_inv_pos(a: i64, m: i64) -> i64 {
    let a = a.rem_euclid(m);
    mod_pow(a, (m - 2) as u64, m) // Fermat's little theorem (m prime)
}

/// Compute base^exp mod modulus over the polynomial ring F_p[x] / f(x).
pub fn poly_mod_pow_mod(base: &Poly, exp: u64, modulus: &Poly, p: i64) -> Poly {
    let mut result = Poly::new(vec![1]);
    let mut b = base.clone();
    let mut e = exp;
    while e > 0 {
        if e & 1 == 1 {
            result = poly_mul_mod(&result, &b, modulus, p);
        }
        b = poly_mul_mod(&b, &b, modulus, p);
        e >>= 1;
    }
    result
}

fn poly_mul_mod(a: &Poly, b: &Poly, modulus: &Poly, p: i64) -> Poly {
    let prod = a.mul(b).reduce_mod(p);
    let (_, r) = prod.div_rem_mod(modulus, p);
    r
}

/// GCD of two polynomials over F_p.
pub fn poly_gcd_mod(a: &Poly, b: &Poly, p: i64) -> Poly {
    let a = a.reduce_mod(p);
    let b = b.reduce_mod(p);
    if b.coeffs.is_empty() { return a; }
    let (_, r) = a.div_rem_mod(&b, p);
    poly_gcd_mod(&b, &r, p)
}

/// All monic polynomials of a given degree over F_p.
pub fn monic_polys_mod(deg: usize, p: i64) -> Vec<Poly> {
    let count = (p as usize).pow(deg as u32);
    (0..count).map(|i| {
        let mut coeffs = vec![0i64; deg + 1];
        coeffs[deg] = 1; // monic
        let mut v = i;
        for k in 0..deg {
            coeffs[k] = (v % p as usize) as i64;
            v /= p as usize;
        }
        Poly::new(coeffs)
    }).collect()
}

// ─── Number theory additions ─────────────────────────────────────────────────

/// Legendre symbol (a | p) for odd prime p.  Returns -1, 0, or 1.
pub fn legendre(a: i64, p: i64) -> i64 {
    let a = a.rem_euclid(p);
    if a == 0 { return 0; }
    let v = mod_pow(a, (p as u64 - 1) / 2, p);
    if v == p - 1 { -1 } else { v }
}

/// Jacobi symbol (a | n) for odd positive n.
pub fn jacobi(mut a: i64, mut n: i64) -> i64 {
    assert!(n > 0 && n % 2 == 1);
    let mut result = 1i64;
    a = a.rem_euclid(n);
    while a != 0 {
        while a % 2 == 0 {
            a /= 2;
            let r = n % 8;
            if r == 3 || r == 5 { result = -result; }
        }
        std::mem::swap(&mut a, &mut n);
        if a % 4 == 3 && n % 4 == 3 { result = -result; }
        a %= n;
    }
    if n == 1 { result } else { 0 }
}

/// All quadratic residues mod p (odd prime).
pub fn quadratic_residues(p: u64) -> Vec<u64> {
    (1..p).filter(|&a| legendre(a as i64, p as i64) == 1).collect()
}

/// Möbius function μ(n).
pub fn mobius(n: u64) -> i64 {
    let f = factor(n);
    if f.iter().any(|&(_, e)| e > 1) { return 0; }
    if f.len() % 2 == 0 { 1 } else { -1 }
}

// ─── Divisor Hasse diagram ────────────────────────────────────────────────────

/// Returns the cover relations of the divisibility poset of n.
/// (a, b) means a | b and there is no c with a | c | b, c ≠ a,b.
pub fn divisor_hasse_edges(n: u64) -> Vec<(u64, u64)> {
    let divs = divisors(n);
    let mut edges = Vec::new();
    for &a in &divs {
        for &b in &divs {
            if b % a != 0 || a == b { continue; }
            // cover: no c strictly between a and b
            let covered = divs.iter().any(|&c| c != a && c != b && b % c == 0 && c % a == 0);
            if !covered { edges.push((a, b)); }
        }
    }
    edges
}

/// Render a divisor Hasse diagram as an SvgCanvas.
pub fn svg_divisor_hasse(n: u64, title: &str) -> SvgCanvas {
    let divs = divisors(n);
    let edges = divisor_hasse_edges(n);
    let nodes: Vec<(u64, String)> = divs.iter().map(|&d| (d, d.to_string())).collect();
    // Layout: layer by log2(d) (number of prime factors with multiplicity)
    fn omega(d: u64) -> u32 { factor(d).iter().map(|&(_, e)| e).sum() }
    let max_layer = divs.iter().map(|&d| omega(d)).max().unwrap_or(0);
    let h = 500.0f64;
    let w = 600.0f64;
    let mut c = SvgCanvas::new(w, h);
    c.title(title);
    // assign positions
    let layers: Vec<Vec<u64>> = (0..=max_layer).map(|k|
        divs.iter().filter(|&&d| omega(d) == k).copied().collect()
    ).collect();
    let mut pos: std::collections::HashMap<u64, (f64, f64)> = std::collections::HashMap::new();
    for (k, layer) in layers.iter().enumerate() {
        let y = h - 60.0 - k as f64 * (h - 120.0) / (max_layer.max(1) as f64);
        for (i, &d) in layer.iter().enumerate() {
            let x = w * (i + 1) as f64 / (layer.len() + 1) as f64;
            pos.insert(d, (x, y));
        }
    }
    for (a, b) in &edges {
        if let (Some(&(x1,y1)), Some(&(x2,y2))) = (pos.get(a), pos.get(b)) {
            c.line(x1, y1, x2, y2, colors::GREY, 1.0);
        }
    }
    for &d in &divs {
        if let Some(&(x, y)) = pos.get(&d) {
            c.node_circle(x, y, &d.to_string(), colors::LIGHT, 18.0, 12.0);
        }
    }
    c
}

// ─── Matrix rank ─────────────────────────────────────────────────────────────

impl Mat {
    /// Rank via integer row reduction (counts pivot rows).
    pub fn rank(&self) -> usize {
        let mut m = self.clone();
        m.row_reduce_steps();
        (0..m.rows).filter(|&r| (0..m.cols).any(|c| m[(r, c)] != 0)).count()
    }

    /// LU decomposition over ℝ (returns L, U as FMat; no partial pivoting warning if singular).
    pub fn lu(&self) -> (FMat, FMat) {
        assert_eq!(self.rows, self.cols, "LU requires square matrix");
        let n = self.rows;
        let mut l = FMat::identity(n);
        let mut u = FMat::zero(n, n);
        for i in 0..n {
            for j in 0..n { u.data[i * n + j] = self[(i, j)] as f64; }
        }
        for k in 0..n {
            for i in (k+1)..n {
                if u.data[k * n + k].abs() < 1e-15 { continue; }
                let factor = u.data[i * n + k] / u.data[k * n + k];
                l.data[i * n + k] = factor;
                for j in k..n {
                    let sub = factor * u.data[k * n + j];
                    u.data[i * n + j] -= sub;
                }
            }
        }
        (l, u)
    }
}

// ─── Permutation group ────────────────────────────────────────────────────────

/// Compose two permutations: result[i] = p[q[i]] (apply q first, then p).
pub fn perm_compose(p: &[usize], q: &[usize]) -> Vec<usize> {
    q.iter().map(|&i| p[i]).collect()
}

/// Inverse of a permutation.
pub fn perm_inverse(p: &[usize]) -> Vec<usize> {
    let mut inv = vec![0usize; p.len()];
    for (i, &v) in p.iter().enumerate() { inv[v] = i; }
    inv
}

/// Order of a permutation (LCM of cycle lengths).
pub fn perm_order(p: &[usize]) -> usize {
    let mut visited = vec![false; p.len()];
    let mut order = 1usize;
    for start in 0..p.len() {
        if visited[start] { continue; }
        let mut len = 0;
        let mut cur = start;
        loop {
            visited[cur] = true;
            cur = p[cur];
            len += 1;
            if cur == start { break; }
        }
        order = lcm(order as i64, len as i64) as usize;
    }
    order
}

/// Disjoint cycle notation string, e.g. "(0 2 4)(1 3)".
pub fn perm_cycle_notation(p: &[usize]) -> String {
    let mut visited = vec![false; p.len()];
    let mut cycles = Vec::new();
    for start in 0..p.len() {
        if visited[start] || p[start] == start { visited[start] = true; continue; }
        let mut cycle = vec![start];
        let mut cur = p[start];
        while cur != start {
            visited[cur] = true;
            cycle.push(cur);
            cur = p[cur];
        }
        visited[start] = true;
        cycles.push(format!("({})", cycle.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(" ")));
    }
    if cycles.is_empty() { "e".to_string() } else { cycles.join("") }
}

/// Sign of a permutation: +1 (even) or -1 (odd).
pub fn perm_sign(p: &[usize]) -> i64 {
    let mut visited = vec![false; p.len()];
    let mut sign = 1i64;
    for start in 0..p.len() {
        if visited[start] { continue; }
        let mut len = 0;
        let mut cur = start;
        loop { visited[cur] = true; cur = p[cur]; len += 1; if cur == start { break; } }
        if len % 2 == 0 { sign *= -1; }
    }
    sign
}

/// Generate all permutations of {0..n-1} (the symmetric group S_n).
pub fn symmetric_group(n: usize) -> Vec<Vec<usize>> {
    let mut base: Vec<usize> = (0..n).collect();
    let mut result = Vec::new();
    permutations_heap(&mut base, n, &mut result);
    result
}

fn permutations_heap(arr: &mut Vec<usize>, k: usize, out: &mut Vec<Vec<usize>>) {
    if k == 1 { out.push(arr.clone()); return; }
    for i in 0..k {
        permutations_heap(arr, k - 1, out);
        if k % 2 == 0 { arr.swap(i, k - 1); } else { arr.swap(0, k - 1); }
    }
}

/// Generate a subgroup from a list of generators (BFS closure).
pub fn group_from_generators(degree: usize, gens: &[Vec<usize>]) -> Vec<Vec<usize>> {
    let id: Vec<usize> = (0..degree).collect();
    let mut group = std::collections::HashSet::new();
    group.insert(id.clone());
    let mut queue = vec![id];
    while let Some(elem) = queue.pop() {
        for g in gens {
            let prod = perm_compose(g, &elem);
            if group.insert(prod.clone()) { queue.push(prod); }
            let inv = perm_inverse(&elem);
            let prod2 = perm_compose(g, &inv);
            if group.insert(prod2.clone()) { queue.push(prod2); }
        }
    }
    let mut v: Vec<Vec<usize>> = group.into_iter().collect();
    v.sort();
    v
}

// ─── Convenience re-exports ───────────────────────────────────────────────────

pub fn read_line() -> String {
    let mut s = String::new();
    let _ = io::stdin().read_line(&mut s);
    s.trim().to_string()
}

// ─── CLI argument parsing ────────────────────────────────────────────────────

/// How the program was invoked.
#[derive(Debug, Clone)]
pub enum AppMode {
    /// Default: interactive REPL
    Interactive,
    /// `--run <cmd> [args…]`  — execute one command, print output, exit
    Run { cmd: String, args: Vec<String> },
}

/// Output format requested via `--format <fmt>`.
#[derive(Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Text,   // default terminal output
    Svg,    // --format svg
    Dot,    // --format dot
    Tex,    // --format tex
    Ascii,  // --format ascii
    Json,   // --format json
    Toml,   // --format toml
}

/// Parsed command-line arguments.
#[derive(Debug, Clone)]
pub struct AppArgs {
    pub mode:        AppMode,
    pub format:      OutputFormat,
    pub output_file: Option<String>,  // --output <path>
    pub load_file:   Option<String>,  // --load   <path>
    pub save_file:   Option<String>,  // --save   <path>
}

impl AppArgs {
    /// Parse from `std::env::args()`.
    pub fn parse() -> Self {
        let argv: Vec<String> = std::env::args().skip(1).collect();
        let mut mode = AppMode::Interactive;
        let mut format = OutputFormat::Text;
        let mut output_file = None;
        let mut load_file   = None;
        let mut save_file   = None;

        let mut i = 0;
        while i < argv.len() {
            match argv[i].as_str() {
                "--run" | "-r" => {
                    i += 1;
                    if i >= argv.len() { break; }
                    let cmd = argv[i].clone();
                    i += 1;
                    let mut args = Vec::new();
                    while i < argv.len() && !argv[i].starts_with("--") {
                        args.push(argv[i].clone());
                        i += 1;
                    }
                    mode = AppMode::Run { cmd, args };
                }
                "--format" | "-f" => {
                    i += 1;
                    if i < argv.len() {
                        format = match argv[i].as_str() {
                            "svg"          => OutputFormat::Svg,
                            "dot"          => OutputFormat::Dot,
                            "tex" | "latex"=> OutputFormat::Tex,
                            "ascii"        => OutputFormat::Ascii,
                            "json"         => OutputFormat::Json,
                            "toml"         => OutputFormat::Toml,
                            _              => OutputFormat::Text,
                        };
                        i += 1;
                    }
                }
                "--output" | "-o" => {
                    i += 1;
                    if i < argv.len() { output_file = Some(argv[i].clone()); i += 1; }
                }
                "--load" | "-l" => {
                    i += 1;
                    if i < argv.len() { load_file = Some(argv[i].clone()); i += 1; }
                }
                "--save" | "-s" => {
                    i += 1;
                    if i < argv.len() { save_file = Some(argv[i].clone()); i += 1; }
                }
                _ => { i += 1; }
            }
        }
        AppArgs { mode, format, output_file, load_file, save_file }
    }

    pub fn is_interactive(&self) -> bool { matches!(self.mode, AppMode::Interactive) }

    /// Emit content to --output file or stdout.
    pub fn emit(&self, content: &str) {
        if let Some(ref path) = self.output_file {
            if let Err(e) = std::fs::write(path, content) {
                eprintln!("Output error: {e}");
            }
        } else {
            print!("{content}");
        }
    }
}

// ─── Persistent state (TOML-backed key/value store) ──────────────────────────

/// Chapter state: a flat key→TOML-value map.
/// Save with `save_state(path, &state)`, load with `load_state(path)`.
pub type StateMap = HashMap<String, toml::Value>;

pub fn state_new() -> StateMap { HashMap::new() }

pub fn state_set_str(m: &mut StateMap, k: &str, v: &str) {
    m.insert(k.into(), toml::Value::String(v.into()));
}
pub fn state_get_str<'a>(m: &'a StateMap, k: &str) -> Option<&'a str> {
    m.get(k)?.as_str()
}

pub fn state_set_int(m: &mut StateMap, k: &str, v: i64) {
    m.insert(k.into(), toml::Value::Integer(v));
}
pub fn state_get_int(m: &StateMap, k: &str) -> Option<i64> {
    m.get(k)?.as_integer()
}

pub fn state_set_float(m: &mut StateMap, k: &str, v: f64) {
    m.insert(k.into(), toml::Value::Float(v));
}
pub fn state_get_float(m: &StateMap, k: &str) -> Option<f64> {
    m.get(k)?.as_float()
}

pub fn state_set_ints(m: &mut StateMap, k: &str, v: &[i64]) {
    m.insert(k.into(), toml::Value::Array(
        v.iter().map(|&x| toml::Value::Integer(x)).collect()
    ));
}
pub fn state_get_ints(m: &StateMap, k: &str) -> Option<Vec<i64>> {
    m.get(k)?.as_array()?.iter().map(|v| v.as_integer()).collect()
}

pub fn state_set_strings(m: &mut StateMap, k: &str, v: &[String]) {
    m.insert(k.into(), toml::Value::Array(
        v.iter().map(|s| toml::Value::String(s.clone())).collect()
    ));
}
pub fn state_get_strings(m: &StateMap, k: &str) -> Option<Vec<String>> {
    m.get(k)?.as_array()?.iter()
        .map(|v| v.as_str().map(|s| s.to_owned())).collect()
}

/// Store a Mat as `[rows, cols, data…]`.
pub fn state_set_mat(m: &mut StateMap, k: &str, mat: &Mat) {
    let mut v = vec![mat.rows as i64, mat.cols as i64];
    v.extend(&mat.data);
    state_set_ints(m, k, &v);
}
pub fn state_get_mat(m: &StateMap, k: &str) -> Option<Mat> {
    let v = state_get_ints(m, k)?;
    if v.len() < 2 { return None; }
    let rows = v[0] as usize;
    let cols = v[1] as usize;
    if v.len() != 2 + rows * cols { return None; }
    Some(Mat::new(rows, cols, v[2..].to_vec()))
}

/// Serialise state to a TOML file.
pub fn save_state(path: &str, state: &StateMap) -> Result<(), String> {
    let mut tbl = toml::map::Map::new();
    for (k, v) in state { tbl.insert(k.clone(), v.clone()); }
    let content = toml::to_string_pretty(&toml::Value::Table(tbl))
        .map_err(|e| e.to_string())?;
    std::fs::write(path, content).map_err(|e| e.to_string())
}

/// Load a TOML file into a StateMap.
pub fn load_state(path: &str) -> Result<StateMap, String> {
    let content = std::fs::read_to_string(path).map_err(|e| e.to_string())?;
    let val: toml::Value = toml::from_str(&content).map_err(|e| e.to_string())?;
    let tbl = val.as_table().ok_or("Not a TOML table")?;
    Ok(tbl.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
}

/// Load a raw text file.
pub fn load_file(path: &str) -> Result<String, String> {
    std::fs::read_to_string(path).map_err(|e| e.to_string())
}
/// Save a raw text file.
pub fn save_file(path: &str, content: &str) -> Result<(), String> {
    std::fs::write(path, content).map_err(|e| e.to_string())
}

// ─── SVG canvas ──────────────────────────────────────────────────────────────

/// CSS color palette mirroring the terminal theme.
pub mod colors {
    pub const CYAN:        &str = "#00bcd4";
    pub const YELLOW:      &str = "#ffd54f";
    pub const GREEN:       &str = "#66bb6a";
    pub const RED:         &str = "#ef5350";
    pub const BLUE:        &str = "#42a5f5";
    pub const MAGENTA:     &str = "#ab47bc";
    pub const ORANGE:      &str = "#ffa726";
    pub const GREY:        &str = "#9e9e9e";
    pub const DARK:        &str = "#212121";
    pub const LIGHT:       &str = "#f5f5f5";
    pub const WHITE:       &str = "#ffffff";
    pub const NONE:        &str = "none";
    pub const HEADER_FILL: &str = "#b2ebf2";
    pub const ROW_ALT:     &str = "#e8f5e9";
    pub const ROW_NORM:    &str = "#fafafa";
}

fn xml_escape(s: &str) -> String {
    s.replace('&', "&amp;").replace('<', "&lt;").replace('>', "&gt;").replace('"', "&quot;")
}

/// Build SVG diagrams without external crates.
pub struct SvgCanvas {
    pub width:  f64,
    pub height: f64,
    elems: Vec<String>,
    defs:  Vec<String>,
    arrow_added: bool,
}

impl SvgCanvas {
    pub fn new(width: f64, height: f64) -> Self {
        SvgCanvas { width, height, elems: Vec::new(), defs: Vec::new(), arrow_added: false }
    }

    fn add_arrowhead(&mut self) {
        if !self.arrow_added {
            self.defs.push(
                r##"<marker id="ah" markerWidth="10" markerHeight="7" refX="9" refY="3.5" orient="auto"><polygon points="0 0,10 3.5,0 7" fill="#444"/></marker>"##
                .into()
            );
            self.arrow_added = true;
        }
    }

    // ── Primitives ────────────────────────────────────────────────────────────

    pub fn rect(&mut self, x: f64, y: f64, w: f64, h: f64,
                fill: &str, stroke: &str, sw: f64) {
        self.elems.push(format!(
            r#"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" fill="{fill}" stroke="{stroke}" stroke-width="{sw:.1}"/>"#
        ));
    }

    pub fn rrect(&mut self, x: f64, y: f64, w: f64, h: f64, rx: f64,
                 fill: &str, stroke: &str, sw: f64) {
        self.elems.push(format!(
            r#"<rect x="{x:.1}" y="{y:.1}" width="{w:.1}" height="{h:.1}" rx="{rx:.1}" fill="{fill}" stroke="{stroke}" stroke-width="{sw:.1}"/>"#
        ));
    }

    pub fn circle(&mut self, cx: f64, cy: f64, r: f64,
                  fill: &str, stroke: &str, sw: f64) {
        self.elems.push(format!(
            r#"<circle cx="{cx:.1}" cy="{cy:.1}" r="{r:.1}" fill="{fill}" stroke="{stroke}" stroke-width="{sw:.1}"/>"#
        ));
    }

    pub fn ellipse(&mut self, cx: f64, cy: f64, rx: f64, ry: f64,
                   fill: &str, stroke: &str, sw: f64) {
        self.elems.push(format!(
            r#"<ellipse cx="{cx:.1}" cy="{cy:.1}" rx="{rx:.1}" ry="{ry:.1}" fill="{fill}" stroke="{stroke}" stroke-width="{sw:.1}"/>"#
        ));
    }

    pub fn line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, sw: f64) {
        self.elems.push(format!(
            r#"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="{stroke}" stroke-width="{sw:.1}"/>"#
        ));
    }

    pub fn dashed_line(&mut self, x1: f64, y1: f64, x2: f64, y2: f64,
                       stroke: &str, sw: f64, dash: &str) {
        self.elems.push(format!(
            r#"<line x1="{x1:.1}" y1="{y1:.1}" x2="{x2:.1}" y2="{y2:.1}" stroke="{stroke}" stroke-width="{sw:.1}" stroke-dasharray="{dash}"/>"#
        ));
    }

    pub fn arrow(&mut self, x1: f64, y1: f64, x2: f64, y2: f64, stroke: &str, sw: f64) {
        self.add_arrowhead();
        let dx = x2 - x1; let dy = y2 - y1;
        let len = (dx*dx + dy*dy).sqrt();
        let (ex, ey) = if len > 14.0 {
            (x2 - dx/len * 10.0, y2 - dy/len * 10.0)
        } else { (x2, y2) };
        self.elems.push(format!(
            r#"<line x1="{x1:.1}" y1="{y1:.1}" x2="{ex:.1}" y2="{ey:.1}" stroke="{stroke}" stroke-width="{sw:.1}" marker-end="url(#ah)"/>"#
        ));
    }

    pub fn path(&mut self, d: &str, fill: &str, stroke: &str, sw: f64) {
        self.elems.push(format!(
            r#"<path d="{d}" fill="{fill}" stroke="{stroke}" stroke-width="{sw:.1}"/>"#
        ));
    }

    pub fn polygon(&mut self, pts: &[(f64,f64)], fill: &str, stroke: &str, sw: f64) {
        let p = pts.iter().map(|(x,y)| format!("{x:.1},{y:.1}")).collect::<Vec<_>>().join(" ");
        self.elems.push(format!(
            r#"<polygon points="{p}" fill="{fill}" stroke="{stroke}" stroke-width="{sw:.1}"/>"#
        ));
    }

    // ── Text ──────────────────────────────────────────────────────────────────

    pub fn text(&mut self, x: f64, y: f64, s: &str, size: f64, color: &str, anchor: &str) {
        self.elems.push(format!(
            r#"<text x="{x:.1}" y="{y:.1}" font-size="{size:.0}" fill="{color}" text-anchor="{anchor}" dominant-baseline="central" font-family="monospace">{}</text>"#,
            xml_escape(s)
        ));
    }

    pub fn text_bold(&mut self, x: f64, y: f64, s: &str, size: f64, color: &str, anchor: &str) {
        self.elems.push(format!(
            r#"<text x="{x:.1}" y="{y:.1}" font-size="{size:.0}" fill="{color}" text-anchor="{anchor}" dominant-baseline="central" font-family="monospace" font-weight="bold">{}</text>"#,
            xml_escape(s)
        ));
    }

    pub fn text_italic(&mut self, x: f64, y: f64, s: &str, size: f64, color: &str, anchor: &str) {
        self.elems.push(format!(
            r#"<text x="{x:.1}" y="{y:.1}" font-size="{size:.0}" fill="{color}" text-anchor="{anchor}" dominant-baseline="central" font-family="serif" font-style="italic">{}</text>"#,
            xml_escape(s)
        ));
    }

    pub fn title(&mut self, s: &str) {
        self.text_bold(self.width / 2.0, 22.0, s, 17.0, colors::DARK, "middle");
    }

    pub fn subtitle(&mut self, s: &str, y: f64) {
        self.text(self.width / 2.0, y, s, 12.0, colors::GREY, "middle");
    }

    pub fn arrow_labeled(&mut self, x1: f64, y1: f64, x2: f64, y2: f64,
                         label: &str, stroke: &str, sw: f64) {
        self.arrow(x1, y1, x2, y2, stroke, sw);
        let mx = (x1+x2)/2.0;
        let my = (y1+y2)/2.0 - 9.0;
        self.text(mx, my, label, 11.0, stroke, "middle");
    }

    // ── Composed elements ─────────────────────────────────────────────────────

    /// Render a table of strings. First row and first column get header styling.
    /// Returns (table_width, table_height).
    pub fn table(&mut self, x0: f64, y0: f64,
                 cells: &[Vec<String>],
                 cell_w: f64, cell_h: f64,
                 header_fill: &str, cell_fill: &str,
                 border: &str) -> (f64, f64) {
        if cells.is_empty() { return (0.0, 0.0); }
        let nr = cells.len();
        let nc = cells[0].len();
        for (ri, row) in cells.iter().enumerate() {
            for ci in 0..nc.min(row.len()) {
                let x = x0 + ci as f64 * cell_w;
                let y = y0 + ri as f64 * cell_h;
                let fill = if ri == 0 || ci == 0 { header_fill } else { cell_fill };
                self.rect(x, y, cell_w, cell_h, fill, border, 0.5);
                let cx = x + cell_w / 2.0;
                let cy = y + cell_h / 2.0;
                let fsz = if cell_h < 24.0 { 10.0 } else { 12.0 };
                if ri == 0 || ci == 0 {
                    self.text_bold(cx, cy, &row[ci], fsz, colors::DARK, "middle");
                } else {
                    self.text(cx, cy, &row[ci], fsz, colors::DARK, "middle");
                }
            }
        }
        (nc as f64 * cell_w, nr as f64 * cell_h)
    }

    /// Draw an integer matrix with bracket notation.
    pub fn matrix_display(&mut self, x0: f64, y0: f64, data: &[Vec<i64>], cell: f64) {
        if data.is_empty() { return; }
        let nr = data.len() as f64;
        let nc = data[0].len() as f64;
        let w = nc * cell;
        let h = nr * cell;
        // Left bracket
        let lx = x0 - 6.0;
        self.path(&format!("M{lx:.1},{y0:.1} Q{:.1},{y0:.1} {:.1},{:.1} Q{:.1},{:.1} {lx:.1},{:.1}",
            lx-8.0, lx-8.0, y0+h/2.0, lx-8.0, y0+h, y0+h),
            colors::NONE, colors::DARK, 1.5);
        // Right bracket
        let rx = x0 + w + 6.0;
        self.path(&format!("M{rx:.1},{y0:.1} Q{:.1},{y0:.1} {:.1},{:.1} Q{:.1},{:.1} {rx:.1},{:.1}",
            rx+8.0, rx+8.0, y0+h/2.0, rx+8.0, y0+h, y0+h),
            colors::NONE, colors::DARK, 1.5);
        // Values
        for (ri, row) in data.iter().enumerate() {
            for (ci, &v) in row.iter().enumerate() {
                let cx = x0 + ci as f64 * cell + cell/2.0;
                let cy = y0 + ri as f64 * cell + cell/2.0;
                self.text(cx, cy, &v.to_string(), 13.0, colors::DARK, "middle");
            }
        }
    }

    /// Draw a Hasse / graph node.
    pub fn node_circle(&mut self, cx: f64, cy: f64, label: &str,
                       fill: &str, r: f64, font_size: f64) {
        self.circle(cx, cy, r, fill, colors::DARK, 1.2);
        self.text(cx, cy, label, font_size, colors::DARK, "middle");
    }

    /// Draw a float matrix display.
    pub fn fmatrix_display(&mut self, x0: f64, y0: f64, data: &[Vec<f64>], cell: f64) {
        let idata: Vec<Vec<String>> = data.iter().map(|row|
            row.iter().map(|&v| if v == v.round() && v.abs() < 1e6 {
                format!("{}", v as i64)
            } else { format!("{:.3}", v) }).collect()
        ).collect();
        if idata.is_empty() { return; }
        let nr = idata.len() as f64;
        let nc = idata[0].len() as f64;
        let w = nc * cell;
        let h = nr * cell;
        let lx = x0 - 6.0;
        self.path(&format!("M{lx:.1},{y0:.1} Q{:.1},{y0:.1} {:.1},{:.1} Q{:.1},{:.1} {lx:.1},{:.1}",
            lx-8.0, lx-8.0, y0+h/2.0, lx-8.0, y0+h, y0+h),
            colors::NONE, colors::DARK, 1.5);
        let rx = x0 + w + 6.0;
        self.path(&format!("M{rx:.1},{y0:.1} Q{:.1},{y0:.1} {:.1},{:.1} Q{:.1},{:.1} {rx:.1},{:.1}",
            rx+8.0, rx+8.0, y0+h/2.0, rx+8.0, y0+h, y0+h),
            colors::NONE, colors::DARK, 1.5);
        for (ri, row) in idata.iter().enumerate() {
            for (ci, s) in row.iter().enumerate() {
                let cx = x0 + ci as f64 * cell + cell/2.0;
                let cy = y0 + ri as f64 * cell + cell/2.0;
                self.text(cx, cy, s, 12.0, colors::DARK, "middle");
            }
        }
    }

    /// Inject a raw SVG snippet.
    pub fn raw(&mut self, svg: &str) { self.elems.push(svg.to_string()); }

    // ── Output ────────────────────────────────────────────────────────────────

    pub fn build(&self) -> String {
        let mut out = format!(
            "<?xml version=\"1.0\" encoding=\"UTF-8\"?>\n\
             <svg xmlns=\"http://www.w3.org/2000/svg\" \
                  width=\"{w:.0}\" height=\"{h:.0}\" \
                  viewBox=\"0 0 {w:.0} {h:.0}\">\n\
               <rect width=\"{w:.0}\" height=\"{h:.0}\" fill=\"{bg}\"/>\n",
            w = self.width, h = self.height, bg = colors::WHITE
        );
        if !self.defs.is_empty() {
            out += "  <defs>\n";
            for d in &self.defs { out += &format!("    {d}\n"); }
            out += "  </defs>\n";
        }
        for e in &self.elems { out += &format!("  {e}\n"); }
        out += "</svg>\n";
        out
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.build())
    }
}

// ─── DOT (Graphviz) graph builder ────────────────────────────────────────────

pub struct DotGraph {
    directed: bool,
    name: String,
    graph_attrs: Vec<String>,
    node_defaults: Vec<String>,
    edge_defaults: Vec<String>,
    nodes: Vec<String>,
    edges: Vec<String>,
    subgraphs: Vec<String>,
}

fn dot_attrs(attrs: &[(&str, &str)]) -> String {
    if attrs.is_empty() { return String::new(); }
    let inner = attrs.iter().map(|(k,v)| format!("{k}=\"{v}\"")).collect::<Vec<_>>().join(", ");
    format!(" [{inner}]")
}

impl DotGraph {
    pub fn digraph(name: &str) -> Self {
        DotGraph { directed: true, name: name.into(), graph_attrs: vec![],
                   node_defaults: vec![], edge_defaults: vec![],
                   nodes: vec![], edges: vec![], subgraphs: vec![] }
    }
    pub fn graph(name: &str) -> Self {
        DotGraph { directed: false, name: name.into(), graph_attrs: vec![],
                   node_defaults: vec![], edge_defaults: vec![],
                   nodes: vec![], edges: vec![], subgraphs: vec![] }
    }

    pub fn graph_attr(&mut self, k: &str, v: &str) -> &mut Self {
        self.graph_attrs.push(format!("  {k}={v:?};")); self
    }
    pub fn node_default(&mut self, k: &str, v: &str) -> &mut Self {
        self.node_defaults.push(format!("{k}={v:?}")); self
    }
    pub fn edge_default(&mut self, k: &str, v: &str) -> &mut Self {
        self.edge_defaults.push(format!("{k}={v:?}")); self
    }
    pub fn node(&mut self, id: &str, attrs: &[(&str, &str)]) -> &mut Self {
        self.nodes.push(format!("  \"{id}\"{};", dot_attrs(attrs))); self
    }
    pub fn edge(&mut self, from: &str, to: &str, attrs: &[(&str, &str)]) -> &mut Self {
        let arrow = if self.directed { "->" } else { "--" };
        self.edges.push(format!("  \"{from}\" {arrow} \"{to}\"{};", dot_attrs(attrs))); self
    }
    pub fn subgraph_cluster(&mut self, label: &str, node_ids: &[&str], fill: &str) -> &mut Self {
        let mut s = format!("  subgraph cluster_{} {{\n    label={label:?};\n    style=filled; fillcolor={fill:?};\n", label.replace(' ', "_"));
        for n in node_ids { s += &format!("    \"{n}\";\n"); }
        s += "  }";
        self.subgraphs.push(s); self
    }

    pub fn build(&self) -> String {
        let kw = if self.directed { "digraph" } else { "graph" };
        let mut out = format!("{kw} \"{}\" {{\n  rankdir=LR;\n", self.name);
        for a in &self.graph_attrs { out += &format!("{a}\n"); }
        if !self.node_defaults.is_empty() {
            out += &format!("  node [{}];\n", self.node_defaults.join(", "));
        }
        if !self.edge_defaults.is_empty() {
            out += &format!("  edge [{}];\n", self.edge_defaults.join(", "));
        }
        for s in &self.subgraphs { out += &format!("{s}\n"); }
        for n in &self.nodes { out += &format!("{n}\n"); }
        for e in &self.edges { out += &format!("{e}\n"); }
        out += "}\n";
        out
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.build())
    }
}

// ─── LaTeX / TikZ document builder ───────────────────────────────────────────

pub struct TikzDoc {
    standalone: bool,
    preamble_extra: Vec<String>,
    body: Vec<String>,
}

impl TikzDoc {
    /// Full LaTeX document (for compilation with pdflatex).
    pub fn standalone() -> Self {
        TikzDoc { standalone: true, preamble_extra: vec![], body: vec![] }
    }
    /// Bare TikZ picture body only (for inclusion in a larger document).
    pub fn snippet() -> Self {
        TikzDoc { standalone: false, preamble_extra: vec![], body: vec![] }
    }

    pub fn use_library(&mut self, lib: &str) -> &mut Self {
        self.preamble_extra.push(format!("\\usetikzlibrary{{{lib}}}"));
        self
    }

    pub fn raw(&mut self, latex: &str) -> &mut Self {
        self.body.push(latex.to_string()); self
    }

    /// Add a TikZ node.
    pub fn node(&mut self, id: &str, x: f64, y: f64, label: &str, style: &str) -> &mut Self {
        self.body.push(format!(
            "  \\node[{style}] ({id}) at ({x:.2},{y:.2}) {{{}}};",
            label.replace('$', "\\$").replace('_', "\\_")
        )); self
    }

    /// Add a raw TikZ node (label with math).
    pub fn node_math(&mut self, id: &str, x: f64, y: f64, math: &str, style: &str) -> &mut Self {
        self.body.push(format!(
            "  \\node[{style}] ({id}) at ({x:.2},{y:.2}) {{${math}$}};"
        )); self
    }

    /// Draw an edge between nodes.
    pub fn edge(&mut self, from: &str, to: &str, label: &str, style: &str) -> &mut Self {
        let lab = if label.is_empty() { String::new() }
                  else { format!(" node[midway,above,font=\\small] {{${label}$}}") };
        self.body.push(format!("  \\draw[{style}] ({from}) -- ({to}){lab};"));
        self
    }

    /// Draw an arrow.
    pub fn arrow(&mut self, from: &str, to: &str, label: &str, style: &str) -> &mut Self {
        let lab = if label.is_empty() { String::new() }
                  else { format!(" node[midway,above,font=\\small] {{${label}$}}") };
        self.body.push(format!("  \\draw[-stealth,{style}] ({from}) -- ({to}){lab};"));
        self
    }

    /// Render a matrix of strings (uses TikZ matrix library).
    pub fn matrix_table(&mut self, rows: &[Vec<String>], x: f64, y: f64) -> &mut Self {
        self.use_library("matrix");
        let mut s = format!("  \\matrix[matrix of nodes, nodes={{draw,minimum width=1.2cm,minimum height=0.7cm}}, ampersand replacement=\\&] at ({x:.1},{y:.1}) {{\n");
        for row in rows {
            s += "    ";
            s += &row.iter().map(|c| c.as_str()).collect::<Vec<_>>().join(" \\& ");
            s += " \\\\\n";
        }
        s += "  };";
        self.body.push(s); self
    }

    pub fn build(&self) -> String {
        if self.standalone {
            let mut out = String::from(
                "\\documentclass[tikz,border=4pt]{standalone}\n\
                 \\usepackage{tikz}\n\
                 \\usepackage{amsmath,amssymb}\n"
            );
            for p in &self.preamble_extra { out += &format!("{p}\n"); }
            out += "\\begin{document}\n\\begin{tikzpicture}\n";
            for b in &self.body { out += &format!("{b}\n"); }
            out += "\\end{tikzpicture}\n\\end{document}\n";
            out
        } else {
            let mut out = String::new();
            for p in &self.preamble_extra { out += &format!("{p}\n"); }
            out += "\\begin{tikzpicture}\n";
            for b in &self.body { out += &format!("{b}\n"); }
            out += "\\end{tikzpicture}\n";
            out
        }
    }

    pub fn save(&self, path: &str) -> std::io::Result<()> {
        std::fs::write(path, self.build())
    }
}

// ─── ASCII art canvas ─────────────────────────────────────────────────────────

/// Simple grid-based ASCII canvas for terminal diagram output.
pub struct AsciiCanvas {
    pub width:  usize,
    pub height: usize,
    grid: Vec<Vec<char>>,
}

impl AsciiCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        AsciiCanvas { width, height, grid: vec![vec![' '; width]; height] }
    }

    fn set(&mut self, x: i32, y: i32, c: char) {
        if x >= 0 && y >= 0 && (x as usize) < self.width && (y as usize) < self.height {
            self.grid[y as usize][x as usize] = c;
        }
    }

    pub fn hline(&mut self, x: i32, y: i32, len: i32, c: char) {
        for dx in 0..len { self.set(x+dx, y, c); }
    }
    pub fn vline(&mut self, x: i32, y: i32, len: i32, c: char) {
        for dy in 0..len { self.set(x, y+dy, c); }
    }

    pub fn text_at(&mut self, x: i32, y: i32, s: &str) {
        for (i, c) in s.chars().enumerate() { self.set(x + i as i32, y, c); }
    }

    pub fn box_at(&mut self, x: i32, y: i32, w: i32, h: i32) {
        self.hline(x+1, y, w-2, '─');
        self.hline(x+1, y+h-1, w-2, '─');
        self.vline(x, y+1, h-2, '│');
        self.vline(x+w-1, y+1, h-2, '│');
        self.set(x,     y,     '┌'); self.set(x+w-1, y,     '┐');
        self.set(x,     y+h-1, '└'); self.set(x+w-1, y+h-1, '┘');
    }

    pub fn arrow_right(&mut self, x: i32, y: i32, len: i32) {
        self.hline(x, y, len-1, '─');
        self.set(x+len-1, y, '→');
    }
    pub fn arrow_down(&mut self, x: i32, y: i32, len: i32) {
        self.vline(x, y, len-1, '│');
        self.set(x, y+len-1, '↓');
    }

    pub fn render(&self) -> String {
        self.grid.iter().map(|row| row.iter().collect::<String>()).collect::<Vec<_>>().join("\n")
    }
}

// ─── Higher-level diagram helpers ─────────────────────────────────────────────

/// Draw a Hasse diagram (nodes + edges) as SVG.
/// `nodes`: (id, x, y, label)
/// `edges`: (from_id, to_id, label)
pub fn svg_hasse(nodes: &[(usize, f64, f64, &str)],
                 edges: &[(usize, usize, &str)],
                 width: f64, height: f64,
                 title: &str) -> SvgCanvas {
    let mut svg = SvgCanvas::new(width, height);
    svg.title(title);
    for &(fi, ti, lab) in edges {
        let (_, fx, fy, _) = nodes[fi];
        let (_, tx, ty, _) = nodes[ti];
        svg.arrow_labeled(fx, fy, tx, ty, lab, colors::GREY, 1.0);
    }
    for &(_, cx, cy, label) in nodes {
        svg.circle(cx, cy, 18.0, colors::LIGHT, colors::CYAN, 1.5);
        svg.text(cx, cy, label, 11.0, colors::DARK, "middle");
    }
    svg
}

/// Cayley table as SVG.
pub fn svg_cayley_table(n: usize, title: &str) -> SvgCanvas {
    let table = zn_add_table(n);
    let names: Vec<String> = (0..n).map(|i| i.to_string()).collect();
    let mut cells = vec![{
        let mut h = vec!["⊕".to_string()];
        h.extend(names.clone()); h
    }];
    for (i, row) in table.iter().enumerate() {
        let mut r = vec![names[i].clone()];
        r.extend(row.iter().map(|&j| names[j].clone()));
        cells.push(r);
    }
    let cw = 36.0; let ch = 28.0;
    let w = (n+1) as f64 * cw + 60.0;
    let h = (n+1) as f64 * ch + 60.0;
    let mut svg = SvgCanvas::new(w, h);
    svg.title(title);
    svg.table(30.0, 44.0, &cells, cw, ch,
              colors::HEADER_FILL, colors::ROW_NORM, colors::GREY);
    svg
}

/// Commutative diagram as SVG (objects at positions, arrows between them).
pub fn svg_comm_diagram(objects: &[(&str, f64, f64)],
                        arrows: &[(&str, &str, &str)],  // (from_label, to_label, arrow_label)
                        width: f64, height: f64,
                        title: &str) -> SvgCanvas {
    let mut svg = SvgCanvas::new(width, height);
    svg.title(title);
    let pos: HashMap<&str, (f64, f64)> = objects.iter().map(|&(l,x,y)| (l,(x,y))).collect();
    for &(from, to, label) in arrows {
        if let (Some(&(x1,y1)), Some(&(x2,y2))) = (pos.get(from), pos.get(to)) {
            svg.arrow_labeled(x1, y1, x2, y2, label, colors::BLUE, 1.5);
        }
    }
    for &(label, cx, cy) in objects {
        svg.rrect(cx-25.0, cy-14.0, 50.0, 28.0, 5.0,
                  colors::LIGHT, colors::CYAN, 1.5);
        svg.text_italic(cx, cy, label, 14.0, colors::DARK, "middle");
    }
    svg
}
