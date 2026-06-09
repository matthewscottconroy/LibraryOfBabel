use rand::RngCore;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::Pdes, difficulty: 1, generate: d1_classification },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 1, generate: d1_heat_steady },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 2, generate: d2_separation },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 2, generate: d2_wave_dAlembert },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 3, generate: d3_heat_fourier },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 3, generate: d3_laplace_eq },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 4, generate: d4_boundary },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 4, generate: d4_characteristics },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 5, generate: d5_greens_fn },
        GeneratorEntry { topic: Topic::Pdes, difficulty: 5, generate: d5_sobolev },
    ]
}

fn d1_classification(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "classification".into(),
        difficulty:  1,
        question:    "Classify the PDE u_tt = c²u_xx.".into(),
        answer:      Answer::Choice(0),
        hint:        "Compare with the general second-order PDE Au_xx + Bu_xy + Cu_yy = ... and compute B²-4AC.".into(),
        explanation: "For u_tt - c²u_xx = 0: A=-c², B=0, C=1. B²-4AC = 0-4(-c²)(1) = 4c² > 0. Hyperbolic.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Hyperbolic".into(),
            "Parabolic".into(),
            "Elliptic".into(),
            "Mixed type".into(),
        ]),
    }
}

fn d1_heat_steady(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "steady_state".into(),
        difficulty:  1,
        question:    "The steady-state (time-independent) solution of the heat equation u_t = u_xx on [0,L] with u(0)=0, u(L)=T satisfies which ODE?".into(),
        answer:      Answer::Choice(1),
        hint:        "Setting u_t = 0 in u_t = u_xx gives...".into(),
        explanation: "u_t = 0 → u_xx = 0 → u(x) = Ax + B. With u(0)=0 and u(L)=T: u(x) = Tx/L. The steady-state satisfies u'' = 0.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "u'' + u = 0".into(),
            "u'' = 0".into(),
            "u' = 0".into(),
            "u'' - u = 0".into(),
        ]),
    }
}

fn d2_separation(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "separation_of_variables".into(),
        difficulty:  2,
        question:    "Using separation of variables u(x,t) = X(x)T(t) in u_t = u_xx, we get X''/X = T'/T = constant. Why must this constant be negative?".into(),
        answer:      Answer::Choice(0),
        hint:        "Think about the boundary conditions X(0) = X(L) = 0 and what sign allows non-trivial solutions.".into(),
        explanation: "If λ > 0, X'' = λX has only exponentially growing solutions incompatible with X(0)=X(L)=0 (other than X=0). If λ=0, X is linear — same issue. Only λ = -μ² < 0 gives X = sin(μx), which can satisfy the BCs.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Boundary conditions force X(0)=X(L)=0; only negative λ gives sinusoidal X satisfying these.".into(),
            "The heat equation models diffusion, which is inherently dissipative.".into(),
            "T(t) must decay, so T'/T must be negative.".into(),
            "Separation requires the constant to be negative by convention.".into(),
        ]),
    }
}

fn d2_wave_dAlembert(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "wave_equation".into(),
        difficulty:  2,
        question:    "d'Alembert's solution of u_tt = c²u_xx is u(x,t) = f(x+ct) + g(x-ct). What do f and g represent physically?".into(),
        answer:      Answer::Choice(2),
        hint:        "Consider what happens to f(x-ct) as t increases: the argument x-ct = const means x = ct + const.".into(),
        explanation: "f(x+ct) is a wave traveling in the -x direction (leftward) at speed c; g(x-ct) is a wave traveling in the +x direction (rightward). Together they represent the general solution as two traveling waves.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Standing waves with different frequencies".into(),
            "The particular and homogeneous solutions".into(),
            "Left-traveling and right-traveling waves".into(),
            "Exponentially growing and decaying modes".into(),
        ]),
    }
}

fn d3_heat_fourier(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "heat_equation".into(),
        difficulty:  3,
        question:    "The solution of u_t = u_xx on [0,π] with u(0,t) = u(π,t) = 0 and u(x,0) = sin(3x) is:".into(),
        answer:      Answer::Choice(1),
        hint:        "Try u(x,t) = X(x)T(t). The initial condition matches a single eigenfunction.".into(),
        explanation: "Eigenfunction sin(3x) has eigenvalue λ = 9. Solution: u(x,t) = sin(3x)e^(-9t). Each mode decays as e^(-n²t).".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "sin(3x) · e^(9t)".into(),
            "sin(3x) · e^(-9t)".into(),
            "cos(3x) · e^(-9t)".into(),
            "sin(3x) · e^(-3t)".into(),
        ]),
    }
}

fn d3_laplace_eq(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "laplace_equation".into(),
        difficulty:  3,
        question:    "A solution u of Laplace's equation Δu = 0 in a domain D achieves its maximum on:".into(),
        answer:      Answer::Choice(1),
        hint:        "This is the maximum principle for harmonic functions.".into(),
        explanation: "Harmonic functions satisfy the maximum principle: if Δu = 0 in D, then max u is attained on the boundary ∂D (unless u is constant). This follows from the mean value property.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Some interior point of D".into(),
            "The boundary ∂D".into(),
            "The centroid of D".into(),
            "Possibly interior, depending on the boundary conditions".into(),
        ]),
    }
}

fn d4_boundary(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "boundary_conditions".into(),
        difficulty:  4,
        question:    "Which boundary condition type specifies the normal derivative ∂u/∂n on the boundary?".into(),
        answer:      Answer::Choice(1),
        hint:        "There are three classical types: Dirichlet (value), Neumann (flux), Robin (mixed).".into(),
        explanation: "Neumann boundary conditions specify ∂u/∂n on ∂D. Physically this corresponds to prescribed flux (e.g., insulated boundary when ∂u/∂n = 0 in heat flow).".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Dirichlet".into(),
            "Neumann".into(),
            "Robin".into(),
            "Cauchy".into(),
        ]),
    }
}

fn d4_characteristics(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "method_of_characteristics".into(),
        difficulty:  4,
        question:    "For the first-order PDE u_t + c·u_x = 0, characteristics are lines x - ct = const. Along these lines, u is:".into(),
        answer:      Answer::Choice(0),
        hint:        "Compute d/dt[u(x(t), t)] along x(t) = x₀ + ct.".into(),
        explanation: "Along x = ct + x₀: du/dt = u_t + c·u_x = 0. So u is constant along characteristics. This is why the solution is u(x,t) = u₀(x-ct).".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "Constant".into(),
            "Linearly increasing".into(),
            "Exponentially decaying".into(),
            "Equal to the initial data u(x₀, 0)".into(),
        ]),
    }
}

fn d5_greens_fn(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "greens_functions".into(),
        difficulty:  5,
        question:    "The Green's function G(x, y; ξ, η) for the Laplacian on a domain D satisfies ΔG = δ(x-ξ, y-η) with G = 0 on ∂D. In free space (ℝ²), G is:".into(),
        answer:      Answer::Choice(1),
        hint:        "The fundamental solution of Δu = δ in ℝ² involves a logarithm.".into(),
        explanation: "G(x,y;ξ,η) = (1/2π) ln(1/r) where r = √((x-ξ)²+(y-η)²). In ℝ³ it would be -1/(4πr). The log arises because the fundamental solution of the Laplacian in 2D is proportional to ln r.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "-1/(4πr)  where r = |x-ξ|".into(),
            "(1/2π) ln(1/r)  where r = √((x-ξ)²+(y-η)²)".into(),
            "e^(-r²)".into(),
            "sin(πr)/r".into(),
        ]),
    }
}

fn d5_sobolev(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Pdes,
        subtopic:    "sobolev_spaces".into(),
        difficulty:  5,
        question:    "The Sobolev space H¹(Ω) consists of functions with which properties?".into(),
        answer:      Answer::Choice(0),
        hint:        "H^k means L² with k weak derivatives also in L².".into(),
        explanation: "H¹(Ω) = W^{1,2}(Ω) consists of functions u ∈ L²(Ω) whose weak first derivatives also lie in L²(Ω). The norm is ||u||_{H¹}² = ||u||_{L²}² + ||∇u||_{L²}². It is the natural solution space for elliptic PDEs.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "u ∈ L²(Ω) and the weak first derivatives ∇u ∈ L²(Ω)".into(),
            "u is once continuously differentiable on Ω̄".into(),
            "u ∈ L¹(Ω) with distributional derivatives in L¹".into(),
            "u is analytic on Ω".into(),
        ]),
    }
}
