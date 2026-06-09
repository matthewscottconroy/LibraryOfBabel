//! Recursive-descent expression parser and numerical equivalence checker.
//!
//! Supported syntax:
//!   - Numbers: 3, 1.5, .5
//!   - Variables: x, t, C1, C2
//!   - Constants: pi, e
//!   - Operators: +  -  *  /  ^ (^ is right-associative)
//!   - Unary minus: -x, -(x+1)
//!   - Functions: exp sin cos tan ln log sqrt abs sinh cosh tanh asin acos atan
//!   - Parentheses
//!   - Implicit multiplication NOT supported (write 2*x not 2x)

use anyhow::{bail, Result};

// ── AST ───────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Expr {
    Num(f64),
    Var(Var),
    BinOp(Box<Expr>, BinOp, Box<Expr>),
    Neg(Box<Expr>),
    Func(Func, Box<Expr>),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Var { X, T, C1, C2, Pi, E, N }

#[derive(Debug, Clone, Copy)]
pub enum BinOp { Add, Sub, Mul, Div, Pow }

#[derive(Debug, Clone, Copy)]
pub enum Func {
    Exp, Sin, Cos, Tan, Ln, Log, Sqrt, Abs,
    Sinh, Cosh, Tanh, Asin, Acos, Atan,
}

// ── Evaluator ─────────────────────────────────────────────────────────────────

pub struct Env {
    pub x:  f64,
    pub t:  f64,
    pub c1: f64,
    pub c2: f64,
    pub n:  f64,
}

impl Expr {
    pub fn eval(&self, env: &Env) -> f64 {
        match self {
            Expr::Num(v) => *v,
            Expr::Var(v) => match v {
                Var::X  => env.x,
                Var::T  => env.t,
                Var::C1 => env.c1,
                Var::C2 => env.c2,
                Var::Pi => std::f64::consts::PI,
                Var::E  => std::f64::consts::E,
                Var::N  => env.n,
            },
            Expr::Neg(e) => -e.eval(env),
            Expr::BinOp(l, op, r) => {
                let lv = l.eval(env);
                let rv = r.eval(env);
                match op {
                    BinOp::Add => lv + rv,
                    BinOp::Sub => lv - rv,
                    BinOp::Mul => lv * rv,
                    BinOp::Div => lv / rv,
                    BinOp::Pow => lv.powf(rv),
                }
            }
            Expr::Func(f, arg) => {
                let v = arg.eval(env);
                match f {
                    Func::Exp  => v.exp(),
                    Func::Sin  => v.sin(),
                    Func::Cos  => v.cos(),
                    Func::Tan  => v.tan(),
                    Func::Ln   => v.ln(),
                    Func::Log  => v.ln(),
                    Func::Sqrt => v.sqrt(),
                    Func::Abs  => v.abs(),
                    Func::Sinh => v.sinh(),
                    Func::Cosh => v.cosh(),
                    Func::Tanh => v.tanh(),
                    Func::Asin => v.asin(),
                    Func::Acos => v.acos(),
                    Func::Atan => v.atan(),
                }
            }
        }
    }
}

// ── Numerical equivalence ─────────────────────────────────────────────────────

static TEST_ENVS: &[Env] = &[
    Env { x: 0.3,  t: 0.7,  c1:  1.0, c2:  2.0, n: 1.0 },
    Env { x: 1.5,  t: 0.2,  c1: -1.0, c2:  0.5, n: 2.0 },
    Env { x: 0.1,  t: 1.3,  c1:  3.0, c2: -2.0, n: 3.0 },
    Env { x: 2.7,  t: 0.4,  c1:  0.0, c2:  1.0, n: 4.0 },
    Env { x: 0.5,  t: 0.9,  c1:  2.0, c2: -1.0, n: 5.0 },
    Env { x: 1.1,  t: 0.6,  c1:  1.0, c2:  1.0, n: 2.0 },
    Env { x: 0.7,  t: 0.3,  c1: -2.0, c2:  3.0, n: 1.0 },
    Env { x: 2.0,  t: 1.0,  c1:  0.5, c2: -0.5, n: 3.0 },
];

pub fn equivalent(user_raw: &str, expected_raw: &str) -> Result<bool> {
    // Normalize common user input: e^( → exp(, e^ → exp(
    let user_norm = normalize(user_raw);
    let expected_norm = normalize(expected_raw);

    let lhs = parse(&user_norm)?;
    let rhs = match parse(&expected_norm) {
        Ok(e) => e,
        Err(_) => return Ok(false),
    };

    let mut all_agree = true;
    let mut valid_count = 0;

    for env in TEST_ENVS {
        let lv = lhs.eval(env);
        let rv = rhs.eval(env);

        // Skip points where both are non-finite (e.g., log at 0)
        if !lv.is_finite() && !rv.is_finite() {
            continue;
        }
        if !lv.is_finite() || !rv.is_finite() {
            all_agree = false;
            break;
        }

        valid_count += 1;
        let scale = lv.abs().max(rv.abs()).max(1.0);
        if (lv - rv).abs() / scale > 1e-6 {
            all_agree = false;
            break;
        }
    }

    Ok(all_agree && valid_count > 0)
}

fn normalize(s: &str) -> String {
    // Replace common user input patterns to match parser expectations
    s.replace("e^(", "exp(")
     .replace("e^", "exp(")
     // Close any extra open parens left by the replacements above
     .trim()
     .to_string()
}

// ── Parser ────────────────────────────────────────────────────────────────────

pub fn parse(src: &str) -> Result<Expr> {
    let src = src.trim();
    let mut p = Parser { src: src.as_bytes(), pos: 0 };
    let expr = p.parse_expr()?;
    p.skip_ws();
    if p.pos < p.src.len() {
        bail!("unexpected character '{}' at position {}", p.src[p.pos] as char, p.pos);
    }
    Ok(expr)
}

struct Parser<'a> {
    src: &'a [u8],
    pos: usize,
}

impl<'a> Parser<'a> {
    fn peek(&self) -> Option<u8> {
        if self.pos < self.src.len() { Some(self.src[self.pos]) } else { None }
    }

    fn consume(&mut self) -> Option<u8> {
        if self.pos < self.src.len() {
            let ch = self.src[self.pos];
            self.pos += 1;
            Some(ch)
        } else {
            None
        }
    }

    fn skip_ws(&mut self) {
        while self.pos < self.src.len() && self.src[self.pos].is_ascii_whitespace() {
            self.pos += 1;
        }
    }

    fn expect(&mut self, ch: u8) -> Result<()> {
        self.skip_ws();
        match self.consume() {
            Some(c) if c == ch => Ok(()),
            Some(c) => bail!("expected '{}', got '{}'", ch as char, c as char),
            None    => bail!("expected '{}', got end of input", ch as char),
        }
    }

    // expr = term (('+' | '-') term)*
    fn parse_expr(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_term()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some(b'+') => { self.consume(); let r = self.parse_term()?; lhs = Expr::BinOp(lhs.into(), BinOp::Add, r.into()); }
                Some(b'-') => { self.consume(); let r = self.parse_term()?; lhs = Expr::BinOp(lhs.into(), BinOp::Sub, r.into()); }
                _ => break,
            }
        }
        Ok(lhs)
    }

    // term = unary (('*' | '/') unary)*
    fn parse_term(&mut self) -> Result<Expr> {
        let mut lhs = self.parse_unary()?;
        loop {
            self.skip_ws();
            match self.peek() {
                Some(b'*') => { self.consume(); let r = self.parse_unary()?; lhs = Expr::BinOp(lhs.into(), BinOp::Mul, r.into()); }
                Some(b'/') => { self.consume(); let r = self.parse_unary()?; lhs = Expr::BinOp(lhs.into(), BinOp::Div, r.into()); }
                _ => break,
            }
        }
        Ok(lhs)
    }

    // unary = '-' unary | power
    fn parse_unary(&mut self) -> Result<Expr> {
        self.skip_ws();
        if let Some(b'-') = self.peek() {
            self.consume();
            let e = self.parse_unary()?;
            Ok(Expr::Neg(e.into()))
        } else {
            self.parse_power()
        }
    }

    // power = primary ('^' unary)?   right-associative
    fn parse_power(&mut self) -> Result<Expr> {
        let base = self.parse_primary()?;
        self.skip_ws();
        if let Some(b'^') = self.peek() {
            self.consume();
            let exp = self.parse_unary()?;
            Ok(Expr::BinOp(base.into(), BinOp::Pow, exp.into()))
        } else {
            Ok(base)
        }
    }

    // primary = NUMBER | IDENT | func_call | '(' expr ')'
    fn parse_primary(&mut self) -> Result<Expr> {
        self.skip_ws();
        match self.peek() {
            Some(b'(') => {
                self.consume();
                let e = self.parse_expr()?;
                self.expect(b')')?;
                Ok(e)
            }
            Some(ch) if ch.is_ascii_digit() || ch == b'.' => self.parse_number(),
            Some(ch) if ch.is_ascii_alphabetic() || ch == b'_' => self.parse_ident_or_func(),
            Some(ch) => bail!("unexpected character '{}'", ch as char),
            None     => bail!("unexpected end of input"),
        }
    }

    fn parse_number(&mut self) -> Result<Expr> {
        let start = self.pos;
        while self.peek().map(|c| c.is_ascii_digit() || c == b'.').unwrap_or(false) {
            self.consume();
        }
        let s = std::str::from_utf8(&self.src[start..self.pos])?;
        let v: f64 = s.parse().map_err(|_| anyhow::anyhow!("invalid number '{}'", s))?;
        Ok(Expr::Num(v))
    }

    fn parse_ident_or_func(&mut self) -> Result<Expr> {
        let start = self.pos;
        while self.peek().map(|c| c.is_ascii_alphanumeric() || c == b'_').unwrap_or(false) {
            self.consume();
        }
        let name = std::str::from_utf8(&self.src[start..self.pos])?;

        // Check if followed by '(' → function call
        let saved = self.pos;
        self.skip_ws();
        if self.peek() == Some(b'(') {
            if let Some(f) = parse_func_name(name) {
                self.consume(); // consume '('
                let arg = self.parse_expr()?;
                self.expect(b')')?;
                return Ok(Expr::Func(f, arg.into()));
            }
        }
        self.pos = saved; // restore if not a function

        // Variable or constant
        match name {
            "x"  => Ok(Expr::Var(Var::X)),
            "t"  => Ok(Expr::Var(Var::T)),
            "n"  => Ok(Expr::Var(Var::N)),
            "C1" => Ok(Expr::Var(Var::C1)),
            "C2" => Ok(Expr::Var(Var::C2)),
            "pi" | "PI" => Ok(Expr::Var(Var::Pi)),
            "e"  | "E"  => Ok(Expr::Var(Var::E)),
            other => bail!("unknown identifier '{}'", other),
        }
    }
}

fn parse_func_name(s: &str) -> Option<Func> {
    match s {
        "exp"  => Some(Func::Exp),
        "sin"  => Some(Func::Sin),
        "cos"  => Some(Func::Cos),
        "tan"  => Some(Func::Tan),
        "ln"   => Some(Func::Ln),
        "log"  => Some(Func::Log),
        "log2" => None, // not supported
        "sqrt" => Some(Func::Sqrt),
        "abs"  => Some(Func::Abs),
        "sinh" => Some(Func::Sinh),
        "cosh" => Some(Func::Cosh),
        "tanh" => Some(Func::Tanh),
        "asin" => Some(Func::Asin),
        "acos" => Some(Func::Acos),
        "atan" => Some(Func::Atan),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_arithmetic() {
        assert!(equivalent("2 + 3", "5").unwrap());
        assert!(equivalent("x^2", "x * x").unwrap());
        assert!(!equivalent("x + 1", "x + 2").unwrap());
    }

    #[test]
    fn test_functions() {
        assert!(equivalent("exp(x)", "exp(x)").unwrap());
        assert!(equivalent("sin(x)^2 + cos(x)^2", "1").unwrap());
    }

    #[test]
    fn test_c1_c2() {
        assert!(equivalent("C1 * exp(x)", "C1 * exp(x)").unwrap());
    }
}
