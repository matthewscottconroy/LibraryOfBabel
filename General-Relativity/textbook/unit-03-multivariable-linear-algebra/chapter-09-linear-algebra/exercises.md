# Chapter 9: Exercises

---

## Section 9.1: Vector Spaces and Linear Maps

**Exercise 9.1.1.** Determine which of the following are vector spaces over ℝ, with the natural operations. For each, either verify all axioms or identify which axiom fails.
(a) The set of polynomials of degree exactly 2.
(b) The set of polynomials of degree at most 2.
(c) The set of continuous functions f: [0,1] → ℝ with f(0) = f(1).
(d) The set of 2×2 matrices with positive entries.
(e) The set of solutions to the ODE y'' + y = 0.
(f) The set of solutions to the ODE y'' + y = 1.

**Exercise 9.1.2.** Prove that in a vector space V, the zero vector 0 is unique, and that the additive inverse of any v ∈ V is unique.

**Exercise 9.1.3.** Let T: V → W be a linear map.
(a) Prove that T(0_V) = 0_W.
(b) Prove that ker(T) is a subspace of V.
(c) Prove that im(T) is a subspace of W.
(d) State and prove the rank-nullity theorem.

**Exercise 9.1.4** (Original). The tangent space to a smooth manifold at a point is a vector space, defined abstractly as the space of derivations. Let M = ℝ² and p = (0,0).
(a) Verify that derivations (linear maps D: C∞(ℝ²) → ℝ satisfying the Leibniz rule) form a vector space.
(b) Show that ∂/∂x|_p and ∂/∂y|_p are linearly independent derivations. [Hint: apply each to the functions f = x and g = y.]
(c) Show that every derivation at p is a linear combination of ∂/∂x|_p and ∂/∂y|_p. [Hint: use the first-order Taylor expansion f(x,y) = f(0,0) + x(∂f/∂x)|_0 + y(∂f/∂y)|_0 + O(r²), and argue that derivations kill constant functions and second-order terms.]

**Exercise 9.1.5.** Let V = C∞(ℝ) (smooth functions on ℝ). The derivative operator D: V → V, D(f) = f', is a linear map.
(a) What is ker(D)?
(b) Is D surjective? (Is every smooth function the derivative of a smooth function?)
(c) Is D injective? 
(d) How does this relate to the rank-nullity theorem in infinite dimensions?

---

## Section 9.2: Matrices

**Exercise 9.2.1.** For the matrix $A = \begin{pmatrix} 2 & 1 \\ 1 & 3 \end{pmatrix}$:
(a) Find a basis in which A is diagonal.
(b) Write A = PDP⁻¹ where D is diagonal.
(c) Verify that A and D represent the same linear map in different bases.

**Exercise 9.2.2.** Prove that det(AB) = det(A)det(B) using the three axioms of the determinant: (1) multilinearity in rows; (2) antisymmetry; (3) det(I) = 1.

**Exercise 9.2.3.** In GR, a coordinate change x → x'(x) transforms tensors by the Jacobian matrix Jⁱⱼ = ∂x'ⁱ/∂xʲ. A contravariant vector V^i transforms as V'^i = Jⁱⱼ Vʲ, and a covariant vector (1-form) Wᵢ transforms as Wᵢ' = (J⁻¹)ʲᵢ Wⱼ = (∂xʲ/∂x'ⁱ) Wⱼ.
(a) Show that the contraction V^i Wᵢ is invariant under coordinate changes.
(b) Show that the metric tensor gᵢⱼ transforms as gᵢⱼ' = (J⁻¹)ᵏᵢ (J⁻¹)ˡⱼ gₖₗ.
(c) The Minkowski metric in Cartesian coordinates is ηᵢⱼ = diag(−1,+1,+1,+1). Under the transformation to spherical coordinates (t, r, θ, φ), compute the metric components g_{rr}, g_{θθ}, g_{φφ}, g_{tt}.

**Exercise 9.2.4** (Original). The linearized Einstein equations around flat spacetime (hᵤᵥ = gᵤᵥ − ηᵤᵥ small) are:
$$\Box \bar{h}^{\mu\nu} = -16\pi G T^{\mu\nu}$$
where $\bar{h}^{\mu\nu} = h^{\mu\nu} - \frac{1}{2}\eta^{\mu\nu}h$ (trace-reversed perturbation) and $\Box = -\partial_t^2 + \nabla^2$.
(a) Viewing T^{μν} as a source vector and $\bar{h}^{\mu\nu}$ as an unknown vector in the "function space" C∞(ℝ^{1,3}), what kind of equation is this? (Linear? Matrix equation? Operator equation?)
(b) The Green's function G(x,x') for □ satisfies □G(x,x') = δ⁴(x−x'). Write the solution as a linear operation on the source.
(c) In what sense is this the infinite-dimensional analogue of solving Ax = b?

**Exercise 9.2.5.** Prove that similar matrices have the same determinant, trace, and eigenvalues. (Two matrices A and B are similar if B = PAP⁻¹ for invertible P.)

---

## Section 9.3: Eigenvalues and the Spectral Theorem

**Exercise 9.3.1.** Find the eigenvalues and eigenvectors of:
(a) $A = \begin{pmatrix} 3 & 1 \\ 0 & 3 \end{pmatrix}$ — note repeated eigenvalue.
(b) $A = \begin{pmatrix} 0 & -1 \\ 1 & 0 \end{pmatrix}$ — rotation by π/2.
(c) $A = \begin{pmatrix} 1 & 0 & 0 \\ 0 & 2 & 0 \\ 0 & 0 & 3 \end{pmatrix}$.

**Exercise 9.3.2.** Prove the spectral theorem for 2×2 real symmetric matrices: if A = Aᵀ is 2×2 with real entries, then A has two real eigenvalues and two orthogonal eigenvectors.

**Exercise 9.3.3.** The tidal tensor in Newtonian gravity is $E_{ij} = -\partial_i \partial_j \Phi$ (second derivatives of the gravitational potential). For a point mass at the origin (Φ = −GM/r):
(a) Compute E_{ij} at a point r = (r, 0, 0) on the x-axis.
(b) Find the eigenvalues and eigenvectors of E_{ij}. Interpret physically: which direction is stretched? Which is compressed?
(c) Show that tr(E_{ij}) = −∇²Φ = 0 outside the mass. What does this say about the sum of tidal forces?

**Exercise 9.3.4** (Original — Petrov classification). The Weyl tensor C_{abcd} in GR encodes the "free" gravitational field (tidal forces and gravitational waves). The Petrov classification is the classification of the Weyl tensor by the algebraic types of its principal null directions.

In 2×2 matrix analogy: classify the following matrices by Jordan form type:
(a) $\begin{pmatrix} 1 & 0 \\ 0 & 2 \end{pmatrix}$ — two distinct eigenvalues (Petrov type I analogue)
(b) $\begin{pmatrix} 1 & 1 \\ 0 & 1 \end{pmatrix}$ — repeated eigenvalue, non-diagonal Jordan block (Petrov type D or N analogue)
(c) $\begin{pmatrix} 1 & 0 \\ 0 & 1 \end{pmatrix}$ — scalar matrix (Petrov type O analogue)

Explain what these analogies suggest about the Weyl tensor classification in GR. Which type describes gravitational waves? Which describes the Schwarzschild black hole?

**Exercise 9.3.5.** Use the SVD to prove that for any matrix A, rank(A) = rank(Aᵀ) = rank(AᵀA).

---

## Section 9.4: Inner Products, Dual Spaces, and Tensors

**Exercise 9.4.1.** Let V = ℝⁿ with the standard basis {eᵢ}. Identify the dual basis {eⁱ} explicitly as linear functionals, and verify that eⁱ(eⱼ) = δⁱⱼ.

**Exercise 9.4.2.** Prove that on a finite-dimensional inner product space V, the map v ↦ ⟨v, ·⟩ is an isomorphism V → V*. Show this isomorphism is not canonical (basis-dependent) — it depends on the metric.

**Exercise 9.4.3.** In Minkowski spacetime with metric η_{μν} = diag(−1, +1, +1, +1) and the 4-vector V^μ = (V^0, V^1, V^2, V^3):
(a) Lower the index: compute V_μ = η_{μν} V^ν.
(b) Compute the Minkowski inner product η_{μν} V^μ V^ν.
(c) For the 4-velocity u^μ = γ(c, v, 0, 0) (motion in x-direction), compute u_μ and u_μ u^μ. What value do you get?
(d) For two 4-velocities u^μ and w^μ (two observers), compute η_{μν} u^μ w^ν. Interpret for (i) the same observer, (ii) two observers at relative velocity v.

**Exercise 9.4.4** (Index gymnastics). Let T^{μν} be a (2,0) tensor and g_{μν} a metric. Compute:
(a) T^{μ}_{\ ν} = g_{νρ} T^{μρ} (lower one index)
(b) T_{μν} = g_{μρ} g_{νσ} T^{ρσ} (lower both indices)
(c) T = g_{μν} T^{μν} (full trace)
(d) The Einstein tensor G_{μν} = R_{μν} − (1/2) g_{μν} R. Raise both indices: G^{μν} = g^{μρ} g^{νσ} G_{ρσ}.
(e) Show that G^{μ}_{\ μ} = −R (the trace of the Einstein tensor).

**Exercise 9.4.5** (Original). The stress-energy tensor T^{μν} in GR encodes energy density, momentum flux, and stress. For a perfect fluid with rest-frame energy density ρ, pressure p, and 4-velocity u^μ:
$$T^{\mu\nu} = (\rho + p) u^\mu u^\nu + p g^{\mu\nu}$$
(a) In the fluid's rest frame (u^μ = (1, 0, 0, 0)/c in coordinates where g_{μν} = diag(−c², +1, +1, +1)), compute all components of T^{μν}.
(b) Identify T^{00}, T^{0i}, T^{ij} physically.
(c) The conservation law ∇_μ T^{μν} = 0 gives two equations: the energy equation and the momentum equation. For a pressureless fluid (p = 0) in flat spacetime, show these reduce to the continuity equation ∂ρ/∂t + ∇·(ρv) = 0 and Newton's second law (no force term).

**Exercise 9.4.6.** The Levi-Civita symbol. Let ε_{ijk} be +1 if (i,j,k) is an even permutation of (1,2,3), −1 for odd, 0 otherwise.
(a) Show ε_{ijk} ε_{ilm} = δ_{jl}δ_{km} − δ_{jm}δ_{kl}. [This is the BAC−CAB rule in disguise.]
(b) Use this to prove (A×B)×C = (A·C)B − (B·C)A.
(c) The volume of a parallelepiped with edges a, b, c is |a·(b×c)| = |ε_{ijk} aⁱ bʲ cᵏ|. Show this is |det(M)| where M is the matrix with a, b, c as rows.
(d) On a curved spacetime, the antisymmetric symbol ε_{μνρσ} must be replaced by the Levi-Civita tensor $\tilde{\epsilon}_{\mu\nu\rho\sigma} = \sqrt{-g}\, \epsilon_{\mu\nu\rho\sigma}$. Why is the √(−g) factor necessary for it to transform as a tensor?

---

## Thought Experiments

**Thought Experiment 9.1: Coordinates Are Not Physical**

Einstein's principle of general covariance says that the laws of physics should have the same form in all coordinate systems. This is implemented mathematically by writing everything in terms of tensors.

(a) Consider the statement "the gravitational field at a point P is (0, 0, −9.8 m/s²)." Is this a tensorial statement? Does it have the same form in all coordinate systems?

(b) Now consider the statement "the proper acceleration experienced by an observer at P is 9.8 m/s²." Is this tensorial? What tensor is it?

(c) If you transform to a freely-falling coordinate system at P, the first statement changes completely. What happens to the second? What does this illustrate about the equivalence principle?

(d) The metric g_{μν} is a tensor. Its components in a given coordinate system are specific numbers. But can you make the components at a single point p take any desired values (satisfying the sign constraint for a Lorentzian metric) by choosing coordinates?

**Thought Experiment 9.2: The Eigenvalue Problem for Gravity**

The Riemann curvature tensor at a point can be viewed as a linear map from 2-forms to 2-forms (the Riemann map). Its "eigenvalues" are related to the sectional curvatures.

(a) In Newtonian gravity, the tidal tensor Eᵢⱼ = −∂ᵢ∂ⱼΦ is symmetric. By the spectral theorem, it has real eigenvalues λ₁, λ₂, λ₃. Outside a mass distribution, Laplace's equation gives λ₁ + λ₂ + λ₃ = 0. What does this mean for tidal deformation of a freely-falling sphere of test masses?

(b) For the Schwarzschild metric, the radial tidal force stretches in the r-direction and compresses in the θ, φ directions. The eigenvalues are λ_r = +2GM/r³ and λ_θ = λ_φ = −GM/r³. Verify that these sum to zero.

(c) As a spacetime singularity is approached (r → 0 in Schwarzschild), the tidal eigenvalues diverge. This divergence — tidal forces becoming infinite — is the physical definition of a spacetime singularity according to the singularity theorems. Explain the connection to eigenvalues of the curvature tensor.

---

## Laboratory Exercises

**Lab 9.1: Measuring the Metric**

The metric tensor is defined by its effect on distances: ds² = g_{μν} dx^μ dx^ν. On a curved 2D surface, we can measure the metric experimentally.

**Procedure**: Blow a large soap bubble (a sphere of radius R). Draw a small coordinate grid on the bubble with a marker. Measure the actual distances between grid points (using a flexible tape measure on the surface) and the coordinate differences (reading off the grid). Compute the ratio (distance)² / (coordinate difference)² for various directions.

**Questions**: (a) What metric do you find? It should be ds² = R² dθ² + R² sin²θ dφ². (b) What is the ratio of circumference to radius for a circle drawn on the sphere? How does it compare to 2π? (c) This discrepancy from 2π measures the curvature. Relate your measurement to the Gaussian curvature K = 1/R² of the sphere.

**Lab 9.2: The Inertia Tensor as a Real Symmetric Matrix**

The moment of inertia tensor Iᵢⱼ = ∫ ρ(r)(|r|²δᵢⱼ − rᵢrⱼ) dV is a real symmetric 3×3 matrix. By the spectral theorem, it has three real eigenvalues (principal moments of inertia) and three orthogonal eigenvectors (principal axes).

**Procedure**: Take an asymmetric solid object (a book, a wooden block). Experimentally find the three principal axes by: (a) Toss the object gently and observe which rotational axes are stable (those correspond to the largest and smallest principal moments). (b) Measure the oscillation frequency when the object rotates about each axis, and use I = τ/α to estimate each principal moment. (c) Verify that the axis with intermediate principal moment is unstable (this is the tennis racket theorem / intermediate axis theorem).

**Questions**: What does instability of the intermediate axis have to do with the eigenvalue structure of Iᵢⱼ? (The intermediate axis is unstable because nearby initial conditions rotate toward one of the stable axes — this is related to the saddle-point structure of the energy surface in angular momentum phase space.)
