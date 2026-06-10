# Chapter 12: Important Concepts

---

**Holomorphic Function**: A function f: U ⊂ ℂ → ℂ that is complex-differentiable at every point of U. Equivalently, f = u + iv satisfies the Cauchy-Riemann equations ∂u/∂x = ∂v/∂y and ∂u/∂y = −∂v/∂x. Holomorphic functions are "rigid": they are determined by their values on any open set (identity theorem), infinitely differentiable, and analytic (have convergent Taylor series).

**Cauchy-Riemann Equations**: The conditions ∂u/∂x = ∂v/∂y and ∂u/∂y = −∂v/∂x that ensure f = u + iv is holomorphic. They ensure the limit defining f'(z) is the same from all directions in ℂ. In complex coordinates: ∂f/∂z̄ = 0 (zero ∂̄-derivative).

**Harmonic Function**: A real-valued function u: U ⊂ ℝ² → ℝ satisfying ∇²u = ∂²u/∂x² + ∂²u/∂y² = 0. Both the real and imaginary parts of a holomorphic function are harmonic. Harmonic functions satisfy the mean value property and the maximum principle (maximum on a bounded domain is on the boundary).

**Conformal Map**: A holomorphic bijection f: U → V with f'(z) ≠ 0 everywhere. It preserves angles (and their orientation) but stretches distances by the position-dependent factor |f'(z)|. The Riemann mapping theorem: every simply connected domain ≠ ℂ is conformally equivalent to the open unit disk.

**Analytic Continuation**: The process of extending a holomorphic function from a smaller domain to a larger one, using the fact that a holomorphic function is determined by its values on any convergent sequence. The extended function is unique (identity theorem). Analytic continuation allows extending the Riemann zeta function ζ(s) from Re(s) > 1 to all of ℂ except s = 1.

**Laurent Series**: The expansion $f(z) = \sum_{n=-\infty}^\infty c_n(z-z_0)^n$ valid in an annulus around z₀. The part with n < 0 is the **principal part**. The coefficient c_{-1} is the **residue**.

**Residue**: The coefficient c_{-1} in the Laurent series of f at an isolated singularity z₀. Computed by: Res = lim_{z→z₀}(z−z₀)f(z) for a simple pole; more complex formulas for higher poles. The residue theorem converts contour integrals to sums of residues.

**Residue Theorem**: $\oint_C f(z)dz = 2\pi i \sum_k \text{Res}_{z=z_k} f(z)$ where the sum is over poles inside the contour C. The most powerful computational tool in complex analysis. Used to evaluate real integrals, Laplace transforms, and partition functions.

**Cauchy Integral Formula**: $f(z_0) = \frac{1}{2\pi i}\oint_C \frac{f(z)}{z-z_0}dz$ for z₀ inside C. Expresses the value of a holomorphic function at any interior point as a weighted average of its boundary values. Implies f is infinitely differentiable: $f^{(n)}(z_0) = \frac{n!}{2\pi i}\oint_C \frac{f(z)}{(z-z_0)^{n+1}}dz$.

**Cauchy's Theorem**: $\oint_\gamma f(z)dz = 0$ for any holomorphic function on a simply connected domain and any closed curve γ. The integral depends only on the homotopy class of γ relative to the singularities of f. In the language of differential forms: f(z)dz is closed iff f is holomorphic.

**Liouville's Theorem**: A bounded entire function is constant. The fundamental tool for proving the fundamental theorem of algebra, and for showing that physical quantities described by holomorphic functions cannot remain bounded unless they are trivial.

**Picard's Great Theorem**: Near an essential singularity, a holomorphic function takes every complex value (with at most one exception) infinitely often in any punctured neighborhood. This extreme behavior near essential singularities contrasts with the mild behavior near poles.

**d'Alembertian (Wave Operator)**: □ = −∂_t²/c² + ∇² (in flat spacetime). The operator whose Green's function is the retarded propagator of the wave equation. In curved spacetime: □_g = g^{μν}∇_μ∇_ν. The linearized Einstein equations take the form □_g h̄_{μν} = −16πG T_{μν}/c⁴ in harmonic gauge.

**Hawking Temperature**: $T_H = \frac{\hbar c^3}{8\pi G M k_B}$ — the temperature at which a Schwarzschild black hole of mass M radiates. Derived by analytically continuing the Schwarzschild metric to Euclidean signature (t → iτ), identifying the periodicity of imaginary time (τ → τ + 4πr_s/c) with inverse temperature (τ → τ + β = ℏ/(k_B T)). For a solar-mass black hole, T_H ≈ 6 × 10⁻⁸ K — undetectable in practice.

**Conformal Compactification**: A conformal map that brings the infinite extent of spacetime into a finite diagram while preserving causal structure (null geodesics). The Penrose diagram is the result: Minkowski spacetime fits in a finite square, with boundaries representing spatial infinity (i⁰), timelike infinities (i±), and null infinities (ℐ±). Black hole spacetimes have additional boundaries (the singularity and the horizon).

**Analytic Continuation of Schwarzschild**: The Schwarzschild metric in coordinates (t, r, θ, φ) is singular at r = r_s in the apparent sense that g_{tt} and g_{rr} diverge. But the metric is actually smooth there in Kruskal-Szekeres coordinates — the singularity is a coordinate artifact. The Kruskal extension is the analytic continuation of the Schwarzschild metric through the coordinate singularity; it reveals the full maximal spacetime (two exterior regions, future and past singularities).

**Spectral Zeta Function**: For an operator L with eigenvalues λ_n, the zeta function ζ_L(s) = Σ λ_n^{−s} extends by analytic continuation to a meromorphic function. The regularized determinant of L is defined as det(L) = e^{−ζ'_L(0)}. This provides a finite definition for formally divergent quantities (like the vacuum energy Σ ω_n = ζ_L(−1)), used in Casimir effect calculations and quantum gravity one-loop corrections.

**Dispersion Relations (Kramers-Kronig)**: Relations between the real and imaginary parts of a causal response function, derived from the analyticity of the response function in the upper half-plane (which follows from causality). The real part (dispersion) is the Hilbert transform of the imaginary part (absorption), and vice versa. Physically: causality forces real and imaginary parts of the susceptibility to be related.

**Quasi-Normal Mode Frequencies**: The complex frequencies ω_n = ω_{Rn} − iω_{In} of the normal mode oscillations of a black hole. They are the poles of the black hole's Green's function in the frequency domain. The real part gives the oscillation frequency; the imaginary part gives the damping rate. They are labeled by (ℓ, m, n) like the spherical harmonics, and depend only on the black hole's mass, charge, and spin.
