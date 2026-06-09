use rand::RngCore;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::Vectors, difficulty: 1, generate: d1_cross_product },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 1, generate: d1_line_integral },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 2, generate: d2_curl },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 2, generate: d2_flux },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 3, generate: d3_greens },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 3, generate: d3_conservative },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 4, generate: d4_divergence_thm },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 4, generate: d4_potential },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 5, generate: d5_helmholtz },
        GeneratorEntry { topic: Topic::Vectors, difficulty: 5, generate: d5_stokes_apply },
    ]
}

fn d1_cross_product(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "cross_product".into(),
        difficulty:  1,
        question:    "Compute ||a × b|| where a = (1,0,0) and b = (0,1,0).".into(),
        answer:      Answer::Number(1.0),
        hint:        "i × j = k, so |i × j| = |k| = 1.".into(),
        explanation: "a × b = (0·0-0·1, 0·0-1·0, 1·1-0·0) = (0,0,1) = k. ||k|| = 1.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d1_line_integral(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "line_integrals".into(),
        difficulty:  1,
        question:    "Compute ∫_C f ds where f(x,y) = 1 and C is the unit circle. (Arc length of unit circle.)".into(),
        answer:      Answer::Number(std::f64::consts::TAU),
        hint:        "∫_C 1 ds = arc length of C = 2πr with r=1.".into(),
        explanation: "The line integral of f=1 is the arc length. For the unit circle, arc length = 2π.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d2_curl(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "curl".into(),
        difficulty:  2,
        question:    "Compute curl(F) for F = (y, -x, 0).".into(),
        answer:      Answer::Choice(2),
        hint:        "curl F = (∂F₃/∂y - ∂F₂/∂z, ∂F₁/∂z - ∂F₃/∂x, ∂F₂/∂x - ∂F₁/∂y).".into(),
        explanation: "∂F₃/∂y=0, ∂F₂/∂z=0, ∂F₁/∂z=0, ∂F₃/∂x=0, ∂F₂/∂x=-1, ∂F₁/∂y=1. curl F = (0,0,-1-1) = (0,0,-2).".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "(0, 0, 0)".into(),
            "(0, 0, 2)".into(),
            "(0, 0, -2)".into(),
            "(-2, 0, 0)".into(),
        ]),
    }
}

fn d2_flux(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "flux".into(),
        difficulty:  2,
        question:    "The flux of F = (1,0,0) through the unit square in the yz-plane (x=0) with outward normal n = (1,0,0) is:".into(),
        answer:      Answer::Number(1.0),
        hint:        "Flux = ∬_S F·n dS. F·n = (1,0,0)·(1,0,0) = 1 everywhere on S.".into(),
        explanation: "Flux = ∬_S 1 dS = area of S = 1×1 = 1.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d3_greens(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "greens_theorem".into(),
        difficulty:  3,
        question:    "Use Green's theorem to compute ∮_C y dx - x dy where C is the unit circle traversed counterclockwise.".into(),
        answer:      Answer::Number(-std::f64::consts::TAU),
        hint:        "Green's theorem: ∮ P dx + Q dy = ∬(∂Q/∂x - ∂P/∂y) dA. Here P=y, Q=-x.".into(),
        explanation: "∂Q/∂x - ∂P/∂y = -1 - 1 = -2. ∬_D -2 dA = -2·π(1)² = -2π.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d3_conservative(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "conservative_fields".into(),
        difficulty:  3,
        question:    "Is F = (2xy, x² + 3y²) a conservative vector field on ℝ²?".into(),
        answer:      Answer::Bool(true),
        hint:        "Check if curl F = 0, i.e., ∂F₂/∂x = ∂F₁/∂y.".into(),
        explanation: "∂F₂/∂x = ∂(x²+3y²)/∂x = 2x. ∂F₁/∂y = ∂(2xy)/∂y = 2x. Equal, so F is conservative.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d4_divergence_thm(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "divergence_theorem".into(),
        difficulty:  4,
        question:    "Compute the flux of F = (x, y, z) out of the unit ball B using the Divergence Theorem.".into(),
        answer:      Answer::Choice(2),
        hint:        "div F = ∂x/∂x + ∂y/∂y + ∂z/∂z = 3. Volume of unit ball = 4π/3.".into(),
        explanation: "Flux = ∭_B div F dV = 3 · (4π/3) = 4π.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "2π".into(),
            "3π".into(),
            "4π".into(),
            "8π".into(),
        ]),
    }
}

fn d4_potential(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "potential_function".into(),
        difficulty:  4,
        question:    "Find the potential function φ for F = (2x, 2y) (so ∇φ = F).".into(),
        answer:      Answer::Expr("x^2 + y^2".into()),
        hint:        "Integrate ∂φ/∂x = 2x to get φ = x² + g(y), then use ∂φ/∂y = 2y.".into(),
        explanation: "∂φ/∂x = 2x → φ = x² + g(y). ∂φ/∂y = g'(y) = 2y → g(y) = y². So φ = x² + y².".into(),
        problem_type: ProblemType::Symbolic,
        choices:     None,
    }
}

fn d5_helmholtz(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "helmholtz_decomposition".into(),
        difficulty:  5,
        question:    "The Helmholtz decomposition states any smooth vector field F on ℝ³ can be written as F = ∇φ + ∇×A. What are these two components called?".into(),
        answer:      Answer::Choice(0),
        hint:        "∇φ is irrotational (curl-free); ∇×A is solenoidal (divergence-free).".into(),
        explanation: "F = (irrotational part) + (solenoidal part). ∇φ has curl zero; ∇×A has divergence zero. Together they span all smooth vector fields.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Irrotational and solenoidal".into(),
            "Conservative and incompressible".into(),
            "Potential and stream".into(),
            "Gradient and rotational".into(),
        ]),
    }
}

fn d5_stokes_apply(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Vectors,
        subtopic:    "stokes_theorem".into(),
        difficulty:  5,
        question:    "Use Stokes' theorem to compute ∮_C F·dr where F = (-y², x, 0) and C is the circle x²+y²=4 in the plane z=0, oriented counterclockwise.".into(),
        answer:      Answer::Number(4.0 * std::f64::consts::PI),
        hint:        "curl F = (0,0, ∂x/∂x - ∂(-y²)/∂y) = (0,0, 1+2y). On z=0, n=(0,0,1).".into(),
        explanation: "curl F · n = 1+2y. ∬_{x²+y²≤4} (1+2y) dA = ∬ 1 dA + ∬ 2y dA = π(2²) + 0 = 4π. The ∬ 2y term vanishes by odd symmetry.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}
