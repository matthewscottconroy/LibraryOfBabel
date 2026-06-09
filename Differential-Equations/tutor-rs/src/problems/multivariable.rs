use rand::RngCore;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 1, generate: d1_partial },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 1, generate: d1_gradient_dir },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 2, generate: d2_chain_rule },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 2, generate: d2_double_integral },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 3, generate: d3_critical_points },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 3, generate: d3_lagrange },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 4, generate: d4_jacobian },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 4, generate: d4_divergence },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 5, generate: d5_stokes },
        GeneratorEntry { topic: Topic::Multivariable, difficulty: 5, generate: d5_implicit_fn },
    ]
}

fn d1_partial(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "partial_derivatives".into(),
        difficulty:  1,
        question:    "Let f(x,y) = x²y + 3y². Compute ∂f/∂x.".into(),
        answer:      Answer::Expr("2*x*y".into()),
        hint:        "Treat y as a constant and differentiate with respect to x.".into(),
        explanation: "∂f/∂x = 2xy + 0 = 2xy. The 3y² term vanishes since y is held constant.".into(),
        problem_type: ProblemType::Symbolic,
        choices:     None,
    }
}

fn d1_gradient_dir(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "gradient".into(),
        difficulty:  1,
        question:    "The gradient of f points in the direction of greatest increase of f. True or False?".into(),
        answer:      Answer::Bool(true),
        hint:        "Recall that the directional derivative D_u f = ∇f · u is maximized when u ∥ ∇f.".into(),
        explanation: "D_u f = ∇f · u = |∇f||u|cos θ is maximal when θ=0, i.e., u in the direction of ∇f.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d2_chain_rule(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "chain_rule".into(),
        difficulty:  2,
        question:    "Let w = x² + y², x = cos(t), y = sin(t). Compute dw/dt.".into(),
        answer:      Answer::Number(0.0),
        hint:        "dw/dt = (∂w/∂x)(dx/dt) + (∂w/∂y)(dy/dt). Note x² + y² = 1 for all t.".into(),
        explanation: "dw/dt = 2x(-sin t) + 2y(cos t) = -2cos(t)sin(t) + 2sin(t)cos(t) = 0.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d2_double_integral(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "double_integrals".into(),
        difficulty:  2,
        question:    "Compute ∬_R x dA where R = [0,1]×[0,1].".into(),
        answer:      Answer::Number(0.5),
        hint:        "Integrate x from 0 to 1, then integrate the result over y from 0 to 1.".into(),
        explanation: "∫₀¹ ∫₀¹ x dy dx = ∫₀¹ x[y]₀¹ dx = ∫₀¹ x dx = [x²/2]₀¹ = 1/2.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d3_critical_points(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "critical_points".into(),
        difficulty:  3,
        question:    "Find the critical point of f(x,y) = x² + y² - 2x - 4y + 5.".into(),
        answer:      Answer::Choice(1),
        hint:        "Set ∂f/∂x = 0 and ∂f/∂y = 0 simultaneously.".into(),
        explanation: "∂f/∂x = 2x-2 = 0 → x=1. ∂f/∂y = 2y-4 = 0 → y=2. Critical point at (1,2).".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "(2, 4)".into(),
            "(1, 2)".into(),
            "(0, 0)".into(),
            "(2, 1)".into(),
        ]),
    }
}

fn d3_lagrange(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "lagrange_multipliers".into(),
        difficulty:  3,
        question:    "Maximize f(x,y) = xy subject to g(x,y) = x + y - 10 = 0 using Lagrange multipliers. What is the maximum value?".into(),
        answer:      Answer::Number(25.0),
        hint:        "∇f = λ∇g gives (y, x) = λ(1,1), so y = x. Combine with x+y=10.".into(),
        explanation: "y=x and x+y=10 → x=y=5. Maximum f(5,5)=25.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d4_jacobian(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "jacobian".into(),
        difficulty:  4,
        question:    "For the polar coordinate map x = r cos θ, y = r sin θ, what is |J| (the absolute value of the Jacobian determinant)?".into(),
        answer:      Answer::Choice(2),
        hint:        "Compute det(∂(x,y)/∂(r,θ)) = det([[cos θ, -r sin θ],[sin θ, r cos θ]]).".into(),
        explanation: "J = cos θ · r cos θ - (-r sin θ) · sin θ = r(cos²θ + sin²θ) = r. So |J| = r.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec!["1".into(), "r²".into(), "r".into(), "2r".into()]),
    }
}

fn d4_divergence(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "divergence".into(),
        difficulty:  4,
        question:    "Compute div(F) for F = (x², xy, xz).".into(),
        answer:      Answer::Expr("4*x".into()),
        hint:        "div F = ∂F₁/∂x + ∂F₂/∂y + ∂F₃/∂z.".into(),
        explanation: "∂(x²)/∂x = 2x, ∂(xy)/∂y = x, ∂(xz)/∂z = x. div F = 2x + x + x = 4x.".into(),
        problem_type: ProblemType::Symbolic,
        choices:     None,
    }
}

fn d5_stokes(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "stokes_theorem".into(),
        difficulty:  5,
        question:    "Stokes' theorem relates a surface integral of curl F over S to a line integral of F over the boundary ∂S. This is a generalization of which classical theorem?".into(),
        answer:      Answer::Choice(2),
        hint:        "When S is a region in the plane and F = (P, Q, 0), Stokes reduces to...".into(),
        explanation: "In 2D, Stokes' theorem becomes ∮ P dx + Q dy = ∬(∂Q/∂x - ∂P/∂y) dA, which is Green's theorem.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Gauss's Divergence Theorem".into(),
            "Fundamental Theorem of Calculus".into(),
            "Green's Theorem".into(),
            "Cauchy's Integral Formula".into(),
        ]),
    }
}

fn d5_implicit_fn(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Multivariable,
        subtopic:    "implicit_function_theorem".into(),
        difficulty:  5,
        question:    "Let F(x,y) = x² + y² - 1. The Implicit Function Theorem guarantees a local function y = g(x) near (0,1) if ∂F/∂y ≠ 0 there. What is dy/dx at (0,1)?".into(),
        answer:      Answer::Number(0.0),
        hint:        "By implicit differentiation: dy/dx = -(∂F/∂x)/(∂F/∂y).".into(),
        explanation: "∂F/∂x = 2x, ∂F/∂y = 2y. At (0,1): dy/dx = -(2·0)/(2·1) = 0.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}
