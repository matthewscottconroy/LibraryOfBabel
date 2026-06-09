use rand::RngCore;
use rand::Rng;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::Odes, difficulty: 1, generate: d1_separable },
        GeneratorEntry { topic: Topic::Odes, difficulty: 1, generate: d1_order },
        GeneratorEntry { topic: Topic::Odes, difficulty: 2, generate: d2_linear_first },
        GeneratorEntry { topic: Topic::Odes, difficulty: 2, generate: d2_exact },
        GeneratorEntry { topic: Topic::Odes, difficulty: 3, generate: d3_second_order_const },
        GeneratorEntry { topic: Topic::Odes, difficulty: 3, generate: d3_wronskian },
        GeneratorEntry { topic: Topic::Odes, difficulty: 4, generate: d4_variation_params },
        GeneratorEntry { topic: Topic::Odes, difficulty: 4, generate: d4_laplace },
        GeneratorEntry { topic: Topic::Odes, difficulty: 5, generate: d5_series_sol },
        GeneratorEntry { topic: Topic::Odes, difficulty: 5, generate: d5_systems },
    ]
}

fn d1_separable(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "separable".into(),
        difficulty:  1,
        question:    "Solve the separable ODE: dy/dx = y, with y(0) = 1.".into(),
        answer:      Answer::Expr("exp(x)".into()),
        hint:        "Separate: dy/y = dx, then integrate both sides.".into(),
        explanation: "ln|y| = x + C → y = Ae^x. Initial condition y(0)=1 gives A=1. Solution: y = e^x.".into(),
        problem_type: ProblemType::Symbolic,
        choices:     None,
    }
}

fn d1_order(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "classification".into(),
        difficulty:  1,
        question:    "What is the order of the ODE: y''' + 3y' - y = sin(x)?".into(),
        answer:      Answer::Number(3.0),
        hint:        "The order is the highest derivative present.".into(),
        explanation: "The highest derivative is y''' (third derivative), so the order is 3.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d2_linear_first(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "linear_first_order".into(),
        difficulty:  2,
        question:    "Solve dy/dx + y = e^x using an integrating factor.".into(),
        answer:      Answer::Expr("(exp(x) + C1) * exp(-x)".into()),
        hint:        "Integrating factor μ = e^(∫1 dx) = e^x. Multiply through and integrate.".into(),
        explanation: "μ = e^x. d/dx[e^x y] = e^(2x). Integrate: e^x y = e^(2x)/2 + C. So y = e^x/2 + Ce^(-x).".into(),
        problem_type: ProblemType::Symbolic,
        choices:     None,
    }
}

fn d2_exact(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "exact_equations".into(),
        difficulty:  2,
        question:    "Is the ODE (2xy)dx + (x² + 1)dy = 0 exact?".into(),
        answer:      Answer::Bool(true),
        hint:        "Check ∂M/∂y = ∂N/∂x where M = 2xy and N = x²+1.".into(),
        explanation: "∂M/∂y = 2x and ∂N/∂x = 2x. Since they are equal, the equation is exact.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d3_second_order_const(rng: &mut dyn RngCore) -> Problem {
    let cases = [
        ("y'' - y = 0", "C1*exp(x) + C2*exp(-x)", "Characteristic equation r²-1=0, roots r=±1."),
        ("y'' + y = 0", "C1*cos(x) + C2*sin(x)", "Characteristic equation r²+1=0, roots r=±i. General solution uses cos and sin."),
        ("y'' - 4*y = 0", "C1*exp(2*x) + C2*exp(-2*x)", "Characteristic equation r²-4=0, roots r=±2."),
    ];
    let (q, ans, exp) = cases[rng.gen_range(0..3usize)];
    Problem {
        topic:       Topic::Odes,
        subtopic:    "second_order_const_coeff".into(),
        difficulty:  3,
        question:    format!("Find the general solution of: {}", q),
        answer:      Answer::Expr(ans.into()),
        hint:        "Substitute y = e^(rx) and solve the characteristic equation.".into(),
        explanation: exp.into(),
        problem_type: ProblemType::Symbolic,
        choices:     None,
    }
}

fn d3_wronskian(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "wronskian".into(),
        difficulty:  3,
        question:    "Compute the Wronskian W(e^x, e^(-x)).".into(),
        answer:      Answer::Number(-2.0),
        hint:        "W(f,g) = fg' - f'g. Compute derivatives and evaluate the determinant.".into(),
        explanation: "W = e^x·(-e^(-x)) - e^x·e^(-x) = -1 - 1 = -2.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d4_variation_params(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "variation_of_parameters".into(),
        difficulty:  4,
        question:    "The method of variation of parameters finds a particular solution y_p to y'' + p(x)y' + q(x)y = g(x) by assuming y_p = u₁y₁ + u₂y₂. What condition is imposed on u₁' and u₂'?".into(),
        answer:      Answer::Choice(1),
        hint:        "We add an extra constraint to simplify the system; the constraint eliminates terms with second derivatives of u.".into(),
        explanation: "The condition u₁'y₁ + u₂'y₂ = 0 is imposed, together with u₁'y₁' + u₂'y₂' = g(x). This gives a 2×2 linear system solvable by Cramer's rule.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "u₁'y₁' + u₂'y₂' = 0".into(),
            "u₁'y₁ + u₂'y₂ = 0".into(),
            "u₁'' + u₂'' = 0".into(),
            "u₁'y₁ - u₂'y₂ = g(x)".into(),
        ]),
    }
}

fn d4_laplace(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "laplace_transform".into(),
        difficulty:  4,
        question:    "Using Laplace transforms, what is L{e^(at)}(s)?".into(),
        answer:      Answer::Choice(2),
        hint:        "L{e^(at)} = ∫₀^∞ e^(at)e^(-st) dt = ∫₀^∞ e^(-(s-a)t) dt, valid for s > a.".into(),
        explanation: "∫₀^∞ e^(-(s-a)t) dt = 1/(s-a) for s > a. This is the fundamental Laplace transform of the exponential.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "s/(s² + a²)".into(),
            "a/(s² + a²)".into(),
            "1/(s - a)".into(),
            "1/(s + a)".into(),
        ]),
    }
}

fn d5_series_sol(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "power_series".into(),
        difficulty:  5,
        question:    "For the ODE y'' - xy' - y = 0 with power series solution y = Σ aₙxⁿ, the recurrence relation is aₙ₊₂ = aₙ(n+1)/((n+2)(n+1)). True or False?".into(),
        answer:      Answer::Bool(false),
        hint:        "Substitute y = Σ aₙxⁿ into y'' - xy' - y = 0. Collect powers of x.".into(),
        explanation: "y'' = Σ (n+2)(n+1)aₙ₊₂xⁿ, xy' = Σ n·aₙxⁿ, y = Σ aₙxⁿ. Equation gives (n+2)(n+1)aₙ₊₂ - n·aₙ - aₙ = 0, so aₙ₊₂ = (n+1)aₙ/((n+2)(n+1)) = aₙ/(n+2). The recurrence is aₙ₊₂ = aₙ/(n+2), not aₙ(n+1)/((n+2)(n+1)).".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d5_systems(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Odes,
        subtopic:    "systems_of_odes".into(),
        difficulty:  5,
        question:    "The system x' = Ax where A = [[0,1],[-1,0]] has eigenvalues λ = ±i. The general real solution is:".into(),
        answer:      Answer::Choice(0),
        hint:        "Complex eigenvalues λ = α ± βi give solutions e^(αt)(c₁cos(βt) + c₂sin(βt)).".into(),
        explanation: "λ = ±i → α=0, β=1. Solution: x(t) = C₁(cos t, -sin t) + C₂(sin t, cos t). This represents rotation in the phase plane.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "x = C₁(cos t, -sin t) + C₂(sin t, cos t)".into(),
            "x = C₁e^t(1, i) + C₂e^(-t)(1, -i)".into(),
            "x = (C₁t + C₂)(1, 0)".into(),
            "x = C₁e^t + C₂e^(-t)".into(),
        ]),
    }
}
