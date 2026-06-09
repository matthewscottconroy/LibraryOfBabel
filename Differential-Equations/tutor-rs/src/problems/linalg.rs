use rand::RngCore;
use rand::Rng;
use super::types::*;

pub fn generators() -> Vec<GeneratorEntry> {
    vec![
        GeneratorEntry { topic: Topic::Linalg, difficulty: 1, generate: d1_matrix_mult },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 1, generate: d1_dot_product },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 2, generate: d2_determinant },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 2, generate: d2_inverse },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 3, generate: d3_eigenvalue },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 3, generate: d3_rank },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 4, generate: d4_diagonalization },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 4, generate: d4_gram_schmidt },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 5, generate: d5_svd_concept },
        GeneratorEntry { topic: Topic::Linalg, difficulty: 5, generate: d5_jordan },
    ]
}

fn d1_matrix_mult(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "matrix_multiplication".into(),
        difficulty:  1,
        question:    "Compute the (1,1) entry of the product AB where A = [[1,2],[3,4]] and B = [[5,6],[7,8]].".into(),
        answer:      Answer::Number(19.0),
        hint:        "The (1,1) entry is the dot product of row 1 of A with column 1 of B.".into(),
        explanation: "(AB)_{11} = 1·5 + 2·7 = 5 + 14 = 19.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d1_dot_product(rng: &mut dyn RngCore) -> Problem {
    let a: [i32; 3] = [rng.gen_range(1..5), rng.gen_range(1..5), rng.gen_range(1..5)];
    let b: [i32; 3] = [rng.gen_range(1..5), rng.gen_range(1..5), rng.gen_range(1..5)];
    let dot = a[0]*b[0] + a[1]*b[1] + a[2]*b[2];
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "dot_product".into(),
        difficulty:  1,
        question:    format!("Compute the dot product of u = ({},{},{}) and v = ({},{},{}).", a[0],a[1],a[2],b[0],b[1],b[2]),
        answer:      Answer::Number(dot as f64),
        hint:        "Dot product: u·v = u₁v₁ + u₂v₂ + u₃v₃.".into(),
        explanation: format!("u·v = {}·{} + {}·{} + {}·{} = {} + {} + {} = {}.",
            a[0],b[0],a[1],b[1],a[2],b[2],a[0]*b[0],a[1]*b[1],a[2]*b[2],dot),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d2_determinant(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "determinant".into(),
        difficulty:  2,
        question:    "Compute det(A) for A = [[3,1],[5,2]].".into(),
        answer:      Answer::Number(1.0),
        hint:        "For a 2×2 matrix [[a,b],[c,d]], det = ad - bc.".into(),
        explanation: "det = 3·2 - 1·5 = 6 - 5 = 1.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d2_inverse(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "matrix_inverse".into(),
        difficulty:  2,
        question:    "Does the matrix A = [[1,2],[2,4]] have an inverse?".into(),
        answer:      Answer::Bool(false),
        hint:        "Check the determinant first.".into(),
        explanation: "det(A) = 1·4 - 2·2 = 4 - 4 = 0. A singular matrix has no inverse.".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d3_eigenvalue(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "eigenvalues".into(),
        difficulty:  3,
        question:    "Find the eigenvalues of A = [[2,1],[0,3]].".into(),
        answer:      Answer::Choice(2),
        hint:        "Solve det(A - λI) = 0. This is a triangular matrix — eigenvalues are on the diagonal.".into(),
        explanation: "det(A - λI) = (2-λ)(3-λ) = 0 → λ = 2 or λ = 3.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "λ = 1 and λ = 4".into(),
            "λ = 2 only".into(),
            "λ = 2 and λ = 3".into(),
            "λ = 0 and λ = 5".into(),
        ]),
    }
}

fn d3_rank(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "rank".into(),
        difficulty:  3,
        question:    "What is the rank of the matrix A = [[1,2,3],[4,5,6],[7,8,9]]?".into(),
        answer:      Answer::Number(2.0),
        hint:        "Row reduce to echelon form. Note that row 3 = row 1 + 2·(row 2 - row 1).".into(),
        explanation: "Rows are linearly dependent (r3 = 2r2 - r1), so rank = 2. Two linearly independent rows.".into(),
        problem_type: ProblemType::Numeric,
        choices:     None,
    }
}

fn d4_diagonalization(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "diagonalization".into(),
        difficulty:  4,
        question:    "A 3×3 matrix with eigenvalues 1, 2, 3 (all distinct). Is it diagonalizable over ℝ?".into(),
        answer:      Answer::Bool(true),
        hint:        "A matrix with n distinct eigenvalues always has n linearly independent eigenvectors.".into(),
        explanation: "Distinct eigenvalues guarantee linearly independent eigenvectors, so A = PDP⁻¹ where D = diag(1,2,3).".into(),
        problem_type: ProblemType::TrueFalse,
        choices:     None,
    }
}

fn d4_gram_schmidt(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "gram_schmidt".into(),
        difficulty:  4,
        question:    "Apply Gram-Schmidt to {v₁=(1,1,0), v₂=(1,0,1)}. What is the first orthonormal basis vector e₁?".into(),
        answer:      Answer::Choice(1),
        hint:        "e₁ = v₁ / ||v₁||. Compute ||v₁|| = sqrt(1²+1²+0²).".into(),
        explanation: "||v₁|| = √2, so e₁ = (1/√2, 1/√2, 0).".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "(1, 1, 0)".into(),
            "(1/√2, 1/√2, 0)".into(),
            "(1/√3, 1/√3, 1/√3)".into(),
            "(1/√2, 0, 1/√2)".into(),
        ]),
    }
}

fn d5_svd_concept(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "svd".into(),
        difficulty:  5,
        question:    "For an m×n matrix A with rank r, how many nonzero singular values does it have?".into(),
        answer:      Answer::Choice(2),
        hint:        "Singular values are the square roots of the nonzero eigenvalues of AᵀA (or AAᵀ).".into(),
        explanation: "The number of nonzero singular values equals the rank r. AᵀA has rank r and thus r positive eigenvalues.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "min(m, n)".into(),
            "max(m, n)".into(),
            "r".into(),
            "m + n - r".into(),
        ]),
    }
}

fn d5_jordan(_rng: &mut dyn RngCore) -> Problem {
    Problem {
        topic:       Topic::Linalg,
        subtopic:    "jordan_form".into(),
        difficulty:  5,
        question:    "A 3×3 matrix has a single eigenvalue λ=2 with algebraic multiplicity 3 and geometric multiplicity 1. What is its Jordan normal form?".into(),
        answer:      Answer::Choice(0),
        hint:        "Geometric multiplicity 1 means one Jordan block of size 3.".into(),
        explanation: "One Jordan block of size 3 for λ=2: J = [[2,1,0],[0,2,1],[0,0,2]]. The 1s above the diagonal correspond to the defect.".into(),
        problem_type: ProblemType::MultipleChoice,
        choices:     Some(vec![
            "[[2,1,0],[0,2,1],[0,0,2]]".into(),
            "[[2,0,0],[0,2,0],[0,0,2]]".into(),
            "[[2,1,0],[0,2,0],[0,0,2]]".into(),
            "[[2,0,0],[1,2,0],[0,1,2]]".into(),
        ]),
    }
}
