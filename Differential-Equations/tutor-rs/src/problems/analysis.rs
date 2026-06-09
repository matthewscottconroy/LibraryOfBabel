use rand::RngCore;
use rand::Rng;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::Analysis, difficulty: 1, generate: d1_limit },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 1, generate: d1_continuity },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 2, generate: d2_epsilon_delta },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 2, generate: d2_derivative_def },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 3, generate: d3_mean_value },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 3, generate: d3_uniform_continuity },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 4, generate: d4_series_conv },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 4, generate: d4_taylor },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 5, generate: d5_riemann },
        GeneratorEntry { topic: Topic::Analysis, difficulty: 5, generate: d5_cauchy_seq },
    ]
}

fn d1_limit(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "limits".into(),
        difficulty:  1,
        question:    "Evaluate the limit: lim_{x→2} (x^2 - 4) / (x - 2)".into(),
        answer:      Answer::Number(4.0),
        hint:        "Factor the numerator as (x-2)(x+2) and cancel.".into(),
        explanation: "(x^2-4)/(x-2) = (x+2) for x≠2, so the limit is 2+2 = 4.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d1_continuity(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "continuity".into(),
        difficulty:  1,
        question:    "Is f(x) = |x| continuous at x = 0?".into(),
        answer:      Answer::Bool(true),
        hint:        "Check: lim_{x→0} |x| = |0| = 0.".into(),
        explanation: "|x| satisfies lim_{x→0⁻} |x| = 0 = lim_{x→0⁺} |x| = f(0), so it is continuous.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d2_epsilon_delta(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "epsilon_delta".into(),
        difficulty:  2,
        question:    "For f(x) = 3x, given ε > 0, what choice of δ proves lim_{x→1} 3x = 3 via ε-δ?".into(),
        answer:      Answer::Choice(0),
        hint:        "|f(x) - 3| = |3x - 3| = 3|x - 1|. Need 3|x-1| < ε.".into(),
        explanation: "|3x-3| = 3|x-1| < 3δ. Set δ = ε/3 so 3δ = ε, giving |3x-3| < ε.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec!["δ = ε/3".into(), "δ = ε".into(), "δ = 3ε".into(), "δ = ε²".into()]),
    }
}

fn d2_derivative_def(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "derivative_definition".into(),
        difficulty:  2,
        question:    "Using the limit definition of the derivative, find f'(x) for f(x) = x^2.".into(),
        answer:      Answer::Expr("2*x".into()),
        hint:        "f'(x) = lim_{h→0} [(x+h)^2 - x^2] / h. Expand and simplify.".into(),
        explanation: "[(x+h)^2 - x^2]/h = [2xh + h^2]/h = 2x + h → 2x as h→0.".into(),
        problem_type: ProblemType::Symbolic,
        choices:     None,
    }
}

fn d3_mean_value(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "mean_value_theorem".into(),
        difficulty:  3,
        question:    "For f(x) = x^2 on [0, 2], the Mean Value Theorem guarantees a c ∈ (0,2) where f'(c) equals the average rate of change. What is c?".into(),
        answer:      Answer::Number(1.0),
        hint:        "Average rate = (f(2)-f(0))/(2-0). Set f'(c) = 2c equal to this.".into(),
        explanation: "Average rate = (4-0)/2 = 2. f'(c) = 2c = 2 → c = 1 ∈ (0,2). ✓".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d3_uniform_continuity(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "uniform_continuity".into(),
        difficulty:  3,
        question:    "Is f(x) = x^2 uniformly continuous on all of ℝ?".into(),
        answer:      Answer::Bool(false),
        hint:        "Try x_n = n, y_n = n + 1/n. Compute |f(x_n) - f(y_n)|.".into(),
        explanation: "|f(n) - f(n+1/n)| = |n^2 - (n+1/n)^2| = |2 + 1/n^2| → 2 ≠ 0, so no δ works for ε < 2.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d4_series_conv(rng: &mut dyn RngCore) -> Problem {
    let p_vals = [2u32, 3, 4];
    let p = p_vals[rng.gen_range(0..3usize)];
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "series_convergence".into(),
        difficulty:  4,
        question:    format!("Does the p-series Σ 1/n^{p} converge?", p = p),
        answer:      Answer::Bool(true),
        hint:        "The p-series Σ 1/n^p converges if and only if p > 1.".into(),
        explanation: format!("p = {} > 1, so the p-series converges by the integral test.", p),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d4_taylor(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "taylor_series".into(),
        difficulty:  4,
        question:    "Write the first three nonzero terms of the Maclaurin series for sin(x).".into(),
        answer:      Answer::Choice(1),
        hint:        "Recall sin(x) = x - x^3/3! + x^5/5! - ...".into(),
        explanation: "Differentiating sin at 0: sin(0)=0, sin'(0)=1, sin''(0)=0, sin'''(0)=-1, ... giving x - x^3/6 + x^5/120.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "1 - x^2/2 + x^4/24".into(),
            "x - x^3/6 + x^5/120".into(),
            "x + x^3/6 + x^5/120".into(),
            "x - x^2/2 + x^3/6".into(),
        ]),
    }
}

fn d5_riemann(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "riemann_integral".into(),
        difficulty:  5,
        question:    "A function f:[0,1]→ℝ is Riemann integrable if and only if it is bounded and continuous almost everywhere (Lebesgue's criterion). Is the Dirichlet function f(x) = 1 if x∈ℚ, 0 otherwise, Riemann integrable on [0,1]?".into(),
        answer:      Answer::Bool(false),
        hint:        "Consider the set of discontinuities of the Dirichlet function.".into(),
        explanation: "The Dirichlet function is discontinuous everywhere on [0,1], so its discontinuity set has positive measure, violating Lebesgue's criterion. It is not Riemann integrable.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d5_cauchy_seq(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Analysis,
        subtopic:    "cauchy_sequences".into(),
        difficulty:  5,
        question:    "In a complete metric space, every Cauchy sequence converges. Is the sequence a_n = 1/n a Cauchy sequence in (ℝ, |·|)?".into(),
        answer:      Answer::Bool(true),
        hint:        "For m,n > N, |1/m - 1/n| ≤ 1/m + 1/n < 2/N. Choose N > 2/ε.".into(),
        explanation: "Given ε>0, pick N > 2/ε. For m,n > N: |1/m - 1/n| ≤ 1/m + 1/n < 2/N < ε. So (1/n) is Cauchy, and it converges to 0 in ℝ.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}
