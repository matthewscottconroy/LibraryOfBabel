use rand::RngCore;

// ── Topics ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Topic {
    Analysis,
    Linalg,
    Multivariable,
    Vectors,
    Odes,
    Fourier,
    Pdes,
    ComplexAn,
}

impl Topic {
    pub fn all() -> &'static [Topic] {
        &[
            Topic::Analysis, Topic::Linalg, Topic::Multivariable,
            Topic::Vectors, Topic::Odes, Topic::Fourier,
            Topic::Pdes, Topic::ComplexAn,
        ]
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Topic::Analysis     => "analysis",
            Topic::Linalg       => "linalg",
            Topic::Multivariable => "multivariable",
            Topic::Vectors      => "vectors",
            Topic::Odes         => "odes",
            Topic::Fourier      => "fourier",
            Topic::Pdes         => "pdes",
            Topic::ComplexAn    => "complex_an",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            Topic::Analysis     => "Real Analysis",
            Topic::Linalg       => "Linear Algebra",
            Topic::Multivariable => "Multivariable Calculus",
            Topic::Vectors      => "Vector Calculus",
            Topic::Odes         => "Ordinary Differential Equations",
            Topic::Fourier      => "Fourier Analysis",
            Topic::Pdes         => "Partial Differential Equations",
            Topic::ComplexAn    => "Complex Analysis",
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "analysis"      => Some(Topic::Analysis),
            "linalg"        => Some(Topic::Linalg),
            "multivariable" => Some(Topic::Multivariable),
            "vectors"       => Some(Topic::Vectors),
            "odes"          => Some(Topic::Odes),
            "fourier"       => Some(Topic::Fourier),
            "pdes"          => Some(Topic::Pdes),
            "complex_an"    => Some(Topic::ComplexAn),
            _ => None,
        }
    }

    pub fn prerequisites(&self) -> &[Topic] {
        match self {
            Topic::Multivariable => &[Topic::Analysis, Topic::Linalg],
            Topic::Vectors       => &[Topic::Multivariable],
            Topic::Odes          => &[Topic::Analysis, Topic::Linalg],
            Topic::Fourier       => &[Topic::Odes],
            Topic::Pdes          => &[Topic::Fourier, Topic::Vectors],
            Topic::ComplexAn     => &[Topic::Multivariable],
            _                    => &[],
        }
    }
}

// ── Problem type ──────────────────────────────────────────────────────────────

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProblemType {
    Symbolic,
    Numeric,
    TrueFalse,
    MultipleChoice,
}

impl ProblemType {
    pub fn elo_offset(self) -> f64 {
        match self {
            ProblemType::Symbolic       =>   0.0,
            ProblemType::Numeric        => -30.0,
            ProblemType::TrueFalse      => -50.0,
            ProblemType::MultipleChoice => -70.0,
        }
    }

    pub fn as_str(self) -> &'static str {
        match self {
            ProblemType::Symbolic       => "symbolic",
            ProblemType::Numeric        => "numeric",
            ProblemType::TrueFalse      => "true_false",
            ProblemType::MultipleChoice => "multiple_choice",
        }
    }
}

// ── Answer ────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub enum Answer {
    Expr(String),   // symbolic: stored as parseable expression string
    Number(f64),    // numeric
    Bool(bool),     // true/false
    Choice(usize),  // multiple choice: index into choices vec
}

// ── Problem ───────────────────────────────────────────────────────────────────

#[derive(Debug, Clone)]
pub struct Problem {
    pub topic:        Topic,
    pub subtopic:     String,
    pub difficulty:   u8,
    pub question:     String,
    pub answer:       Answer,
    pub hint:         String,
    pub explanation:  String,
    pub problem_type: ProblemType,
    pub choices:      Option<Vec<String>>,
}

impl Problem {
    pub fn answer_display(&self) -> String {
        match &self.answer {
            Answer::Expr(s)   => s.clone(),
            Answer::Number(n) => format!("{}", n),
            Answer::Bool(b)   => if *b { "True".into() } else { "False".into() },
            Answer::Choice(i) => {
                if let Some(cs) = &self.choices {
                    let letter = (b'A' + *i as u8) as char;
                    format!("{}. {}", letter, cs[*i])
                } else {
                    i.to_string()
                }
            }
        }
    }

    pub fn check_answer(&self, raw: &str) -> (bool, Option<String>) {
        let raw = raw.trim();
        if raw.is_empty() {
            return (false, Some("Please enter an answer.".into()));
        }
        match &self.answer {
            Answer::Expr(expected) => {
                match crate::expr::equivalent(raw, expected) {
                    Ok(eq) => (eq, None),
                    Err(e) => (false, Some(format!("Could not parse '{}': {}", raw, e))),
                }
            }
            Answer::Number(expected) => {
                match raw.parse::<f64>() {
                    Ok(v) => {
                        let tol = (expected.abs() * 1e-4).max(1e-9);
                        ((v - expected).abs() <= tol, None)
                    }
                    Err(_) => (false, Some(format!("Could not parse '{}' as a number.", raw))),
                }
            }
            Answer::Bool(expected) => {
                let sl = raw.to_lowercase();
                let user = match sl.as_str() {
                    "t" | "true"  | "yes" | "1" | "y" => Some(true),
                    "f" | "false" | "no"  | "0" | "n" => Some(false),
                    _ => None,
                };
                match user {
                    Some(v) => (v == *expected, None),
                    None => (false, Some(format!("Enter True or False (got '{}')", raw))),
                }
            }
            Answer::Choice(correct_idx) => {
                let sl = raw.to_lowercase();
                // Accept letter: a, b, c, ...
                if sl.len() == 1 && sl.chars().next().unwrap().is_alphabetic() {
                    let idx = (sl.chars().next().unwrap() as u8 - b'a') as usize;
                    return (idx == *correct_idx, None);
                }
                // Accept full text
                if let Some(cs) = &self.choices {
                    let ok = cs.get(*correct_idx)
                        .map(|c| c.to_lowercase() == sl)
                        .unwrap_or(false);
                    return (ok, None);
                }
                (false, None)
            }
        }
    }
}

// ── Generator ─────────────────────────────────────────────────────────────────

pub struct GeneratorEntry {
    pub topic:      Topic,
    pub difficulty: u8,
    pub generate:   fn(&mut dyn RngCore) -> Problem,
}
