use rand::RngCore;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 1, generate: d1_modulus },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 1, generate: d1_analytic },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 2, generate: d2_cauchy_riemann },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 2, generate: d2_contour },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 3, generate: d3_cauchy_integral },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 3, generate: d3_residue },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 4, generate: d4_laurent },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 4, generate: d4_singularities },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 5, generate: d5_residue_integral },
        GeneratorEntry { topic: Topic::ComplexAn, difficulty: 5, generate: d5_conformal },
    ]
}

fn d1_modulus(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "complex_arithmetic".into(),
        difficulty:  1,
        question:    "Compute |3 + 4i|.".into(),
        answer:      Answer::Number(5.0),
        hint:        "|a + bi| = √(a² + b²).".into(),
        explanation: "|3 + 4i| = √(9 + 16) = √25 = 5.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d1_analytic(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "analyticity".into(),
        difficulty:  1,
        question:    "Is f(z) = |z|² analytic on ℂ?".into(),
        answer:      Answer::Bool(false),
        hint:        "Write |z|² = x² + y² and check the Cauchy-Riemann equations.".into(),
        explanation: "f = x² + y² + 0i, so u = x²+y², v = 0. ∂u/∂x = 2x ≠ 0 = ∂v/∂y (except at origin). C-R fails everywhere except z=0, so f is not analytic.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d2_cauchy_riemann(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "cauchy_riemann".into(),
        difficulty:  2,
        question:    "Verify that f(z) = e^z = e^x cos y + i e^x sin y satisfies the Cauchy-Riemann equations. What is ∂u/∂x?".into(),
        answer:      Answer::Choice(2),
        hint:        "u = e^x cos y. Differentiate with respect to x.".into(),
        explanation: "∂u/∂x = e^x cos y. By C-R: ∂u/∂x = ∂v/∂y = e^x cos y. ✓ Also ∂u/∂y = -e^x sin y = -∂v/∂x = -e^x sin y. ✓ So e^z is entire.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "e^x sin y".into(),
            "-e^x cos y".into(),
            "e^x cos y".into(),
            "e^y cos x".into(),
        ]),
    }
}

fn d2_contour(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "contour_integrals".into(),
        difficulty:  2,
        question:    "Compute ∮_C dz/z where C is the unit circle |z| = 1 traversed once counterclockwise.".into(),
        answer:      Answer::Choice(2),
        hint:        "Parametrize: z = e^(iθ), dz = ie^(iθ)dθ, θ ∈ [0, 2π].".into(),
        explanation: "∮_C dz/z = ∫₀^{2π} (ie^{iθ}/e^{iθ}) dθ = ∫₀^{2π} i dθ = 2πi. This is the winding number formula.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "0".into(),
            "π".into(),
            "2πi".into(),
            "2π".into(),
        ]),
    }
}

fn d3_cauchy_integral(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "cauchy_integral_formula".into(),
        difficulty:  3,
        question:    "Using the Cauchy Integral Formula, compute ∮_C e^z/(z-1) dz where C is |z|=2, counterclockwise.".into(),
        answer:      Answer::Choice(1),
        hint:        "CIF: ∮_C f(z)/(z-a) dz = 2πi f(a) when a is inside C.".into(),
        explanation: "z=1 is inside |z|=2. CIF gives 2πi · e^1 = 2πie.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "2πi".into(),
            "2πie".into(),
            "0".into(),
            "πie".into(),
        ]),
    }
}

fn d3_residue(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "residues".into(),
        difficulty:  3,
        question:    "Find the residue of f(z) = 1/(z(z-1)) at z = 0.".into(),
        answer:      Answer::Number(-1.0),
        hint:        "For a simple pole at z=a: Res(f, a) = lim_{z→a} (z-a)f(z).".into(),
        explanation: "Res(f, 0) = lim_{z→0} z · 1/(z(z-1)) = lim_{z→0} 1/(z-1) = 1/(0-1) = -1.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d4_laurent(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "laurent_series".into(),
        difficulty:  4,
        question:    "Find the Laurent series of f(z) = 1/(z(1-z)) for 0 < |z| < 1.".into(),
        answer:      Answer::Choice(0),
        hint:        "Partial fractions: 1/(z(1-z)) = 1/z + 1/(1-z). Expand 1/(1-z) as a power series for |z|<1.".into(),
        explanation: "1/(z(1-z)) = 1/z + 1/(1-z) = 1/z + Σ_{n=0}^∞ zⁿ = 1/z + 1 + z + z² + ···. This is a Laurent series with a simple pole at z=0.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "1/z + 1 + z + z² + ···".into(),
            "1/z - 1 - z - z² - ···".into(),
            "Σ_{n=-1}^∞ (-1)ⁿzⁿ".into(),
            "1/z² + 1/z + 1 + z + ···".into(),
        ]),
    }
}

fn d4_singularities(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "singularities".into(),
        difficulty:  4,
        question:    "What type of singularity does f(z) = sin(z)/z have at z = 0?".into(),
        answer:      Answer::Choice(1),
        hint:        "Expand sin(z) as a power series and divide by z.".into(),
        explanation: "sin(z)/z = (z - z³/6 + ···)/z = 1 - z²/6 + ··· has a Taylor series at z=0. The singularity is removable — define f(0)=1 to make f analytic.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Simple pole".into(),
            "Removable singularity".into(),
            "Essential singularity".into(),
            "Branch point".into(),
        ]),
    }
}

fn d5_residue_integral(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "residue_theorem".into(),
        difficulty:  5,
        question:    "Evaluate the real integral ∫_{-∞}^{∞} dx/(x²+1) using the residue theorem.".into(),
        answer:      Answer::Number(std::f64::consts::PI),
        hint:        "Close the contour in the upper half-plane. f(z) = 1/(z²+1) has poles at z = ±i; only z=i is in the UHP.".into(),
        explanation: "Res(f, i) = lim_{z→i}(z-i)/(z²+1) = 1/(2i). By the residue theorem: ∫ f dz = 2πi · (1/2i) = π. The semicircular arc vanishes by Jordan's lemma.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d5_conformal(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::ComplexAn,
        subtopic:    "conformal_maps".into(),
        difficulty:  5,
        question:    "The Möbius transformation f(z) = (z-i)/(z+i) maps the upper half-plane {Im(z)>0} to:".into(),
        answer:      Answer::Choice(2),
        hint:        "Check where the boundary (real axis) and one interior point (e.g., z=i) go.".into(),
        explanation: "The real axis maps to the unit circle |w|=1 (check: z real → |f(z)|=|z-i|/|z+i|=1). z=i maps to f(i)=0, inside the disk. So the UHP maps conformally to the unit disk.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "The right half-plane".into(),
            "The upper half-plane".into(),
            "The open unit disk".into(),
            "The exterior of the unit disk".into(),
        ]),
    }
}
