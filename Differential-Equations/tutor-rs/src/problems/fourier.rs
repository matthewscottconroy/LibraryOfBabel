use rand::RngCore;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::Fourier, difficulty: 1, generate: d1_periodic },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 1, generate: d1_coefficients },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 2, generate: d2_fourier_series },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 2, generate: d2_even_odd },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 3, generate: d3_parseval },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 3, generate: d3_convergence },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 4, generate: d4_fourier_transform },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 4, generate: d4_convolution },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 5, generate: d5_gibbs },
        GeneratorEntry { topic: Topic::Fourier, difficulty: 5, generate: d5_uncertainty },
    ]
}

fn d1_periodic(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "periodicity".into(),
        difficulty:  1,
        question:    "What is the fundamental period of f(x) = cos(3x)?".into(),
        answer:      Answer::Choice(1),
        hint:        "cos(ωx) has period T = 2π/ω.".into(),
        explanation: "ω = 3, so T = 2π/3.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "2π".into(),
            "2π/3".into(),
            "π/3".into(),
            "3π".into(),
        ]),
    }
}

fn d1_coefficients(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "fourier_coefficients".into(),
        difficulty:  1,
        question:    "For a Fourier series on [-π, π], the formula for the constant term a₀ is:".into(),
        answer:      Answer::Choice(0),
        hint:        "a₀ is twice the average value of f over the period.".into(),
        explanation: "a₀ = (1/π) ∫_{-π}^{π} f(x) dx. This is twice the mean value of f.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "(1/π) ∫_{-π}^{π} f(x) dx".into(),
            "(1/2π) ∫_{-π}^{π} f(x) dx".into(),
            "∫_{-π}^{π} f(x) dx".into(),
            "(2/π) ∫_{0}^{π} f(x) dx".into(),
        ]),
    }
}

fn d2_fourier_series(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "fourier_series".into(),
        difficulty:  2,
        question:    "The Fourier series of the square wave f(x) = sign(sin x) on [-π,π] contains only odd harmonics. True or False?".into(),
        answer:      Answer::Bool(true),
        hint:        "The square wave is an odd function. What does symmetry imply about its Fourier coefficients?".into(),
        explanation: "f is odd → aₙ = 0 for all n. f also has half-wave symmetry → bₙ = 0 for even n. Only odd sin harmonics remain: (4/π)(sin x + sin(3x)/3 + sin(5x)/5 + ···).".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d2_even_odd(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "even_odd_functions".into(),
        difficulty:  2,
        question:    "If f is an even function, which Fourier coefficients vanish?".into(),
        answer:      Answer::Choice(1),
        hint:        "Even functions have the symmetry f(-x) = f(x). What does this imply for bₙ = (1/π)∫f(x)sin(nx)dx?".into(),
        explanation: "sin(nx) is odd. The product f(x)sin(nx) with f even is odd, so ∫_{-π}^{π} f(x)sin(nx)dx = 0. All bₙ vanish.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "All aₙ (cosine coefficients)".into(),
            "All bₙ (sine coefficients)".into(),
            "All coefficients with even n".into(),
            "No coefficients vanish".into(),
        ]),
    }
}

fn d3_parseval(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "parseval".into(),
        difficulty:  3,
        question:    "Parseval's theorem states: (1/π) ∫_{-π}^{π} |f(x)|² dx = a₀²/2 + Σ(aₙ² + bₙ²). This expresses which principle?".into(),
        answer:      Answer::Choice(2),
        hint:        "Think of Fourier coefficients as coordinates in an infinite-dimensional inner product space.".into(),
        explanation: "Parseval's theorem is conservation of energy (L² norm): the total power of f equals the sum of powers in each harmonic. It reflects the fact that Fourier basis functions are orthonormal.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Uniqueness of Fourier representation".into(),
            "Uniform convergence of Fourier series".into(),
            "Conservation of energy / Plancherel identity".into(),
            "The Gibbs phenomenon".into(),
        ]),
    }
}

fn d3_convergence(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "convergence".into(),
        difficulty:  3,
        question:    "At a jump discontinuity x₀ of a piecewise smooth function f, the Fourier series converges to:".into(),
        answer:      Answer::Choice(1),
        hint:        "Dirichlet's theorem handles jump discontinuities.".into(),
        explanation: "The Fourier series converges to the average of the left and right limits: [f(x₀⁻) + f(x₀⁺)]/2.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "f(x₀⁺)  (the right limit)".into(),
            "[f(x₀⁻) + f(x₀⁺)] / 2".into(),
            "0".into(),
            "The series diverges at jump discontinuities".into(),
        ]),
    }
}

fn d4_fourier_transform(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "fourier_transform".into(),
        difficulty:  4,
        question:    "What is the Fourier transform F̂(ξ) = ∫_{-∞}^{∞} e^{-x²} e^{-2πiξx} dx?".into(),
        answer:      Answer::Choice(0),
        hint:        "The Gaussian e^{-x²} is its own Fourier transform (up to a constant). Complete the square in the exponent.".into(),
        explanation: "∫_{-∞}^{∞} e^{-x²} e^{-2πiξx} dx = √π · e^{-π²ξ²}. The Gaussian is self-dual under the Fourier transform.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "√π · exp(-π²ξ²)".into(),
            "exp(-ξ²)".into(),
            "√π · exp(-ξ²/4)".into(),
            "1/(1 + 4π²ξ²)".into(),
        ]),
    }
}

fn d4_convolution(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "convolution_theorem".into(),
        difficulty:  4,
        question:    "The convolution theorem states: F{f * g} = F{f} · F{g}. What does * denote in the frequency domain for F{fg}?".into(),
        answer:      Answer::Choice(2),
        hint:        "Multiplication in time ↔ ? in frequency; convolution in time ↔ multiplication in frequency.".into(),
        explanation: "Multiplication of f and g in the time domain corresponds to convolution of their transforms in the frequency domain: F{fg} = F{f} * F{g}.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Addition: F{f} + F{g}".into(),
            "Division: F{f} / F{g}".into(),
            "Convolution: F{f} * F{g}".into(),
            "Pointwise product: F{f} · F{g}".into(),
        ]),
    }
}

fn d5_gibbs(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "gibbs_phenomenon".into(),
        difficulty:  5,
        question:    "The Gibbs phenomenon: near a jump discontinuity, the partial Fourier sum overshoots by approximately what percentage of the jump height, regardless of how many terms are included?".into(),
        answer:      Answer::Number(9.0),
        hint:        "This is a universal constant approximately equal to (2/π)∫₀^π sin(t)/t dt - 1 ≈ 0.0895.".into(),
        explanation: "The overshoot is approximately 8.9% (≈ 9%) of the jump. This arises from Si(π) = ∫₀^π sin(t)/t dt ≈ 1.8519, giving an overshoot factor of (2/π)·Si(π) - 1 ≈ 0.0895 ≈ 9%.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d5_uncertainty(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Fourier,
        subtopic:    "uncertainty_principle".into(),
        difficulty:  5,
        question:    "The Heisenberg uncertainty principle in signal processing states Δt · Δω ≥ ?, where Δt is time spread and Δω is frequency spread (using the variance definition and ω = 2πξ convention).".into(),
        answer:      Answer::Choice(2),
        hint:        "This is a consequence of the Cauchy-Schwarz inequality applied to f and its Fourier transform.".into(),
        explanation: "Δt · Δω ≥ 1/2. Equality holds for Gaussian functions. This is a mathematical theorem, not a physical law, stating that a signal cannot be simultaneously concentrated in time and frequency.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "1".into(),
            "2π".into(),
            "1/2".into(),
            "ℏ/2".into(),
        ]),
    }
}
