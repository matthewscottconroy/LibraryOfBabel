# Chapter 11: Important Researchers

---

## Founders of PDE Theory

**Jean-Baptiste Joseph Fourier (1768–1830)**
French mathematician and physicist who introduced the heat equation and the method of solving PDEs by separation of variables in his *Théorie analytique de la chaleur* (1822). Fourier proposed that any periodic function could be expanded in a trigonometric series (Fourier series) — a claim that was controversial at the time and ultimately required 80 years of analysis to make rigorous. His work founded the theory of harmonic analysis and remains one of the most practically important mathematical ideas: Fourier analysis is used in MRI imaging, signal processing, audio compression, and quantum mechanics. He also coined the phrase "préparation physique" for what we now call dimensional analysis.

**Jean le Rond d'Alembert (1751–1783)**
French mathematician and philosopher who gave the first solution of the wave equation (the vibrating string problem) in 1747. D'Alembert's formula u = f(x−ct) + g(x+ct) decomposing the solution into traveling waves is the foundational result for hyperbolic PDEs. D'Alembert also contributed to celestial mechanics (the precession of the equinoxes), fluid mechanics (d'Alembert's paradox), and was co-editor (with Diderot) of the encyclopedist movement that characterized the French Enlightenment.

**Joseph Liouville (1809–1882)**
French mathematician who co-developed (with Sturm) the Sturm-Liouville theory of eigenvalue problems for second-order differential operators — the foundation for separation of variables and normal mode analysis in PDEs. A solution to a PDE with Sturm-Liouville structure expands in orthogonal eigenfunctions, with coefficients determined by the initial data. This is the PDE analogue of diagonalizing a matrix. Liouville also proved Liouville's theorem in complex analysis and in statistical mechanics.

**Carl Gustav Jacobi (1804–1851)**
See Chapter 7. In the PDE context: Jacobi invented the method of characteristics for first-order PDEs and developed Hamilton-Jacobi theory — a reformulation of classical mechanics as a PDE (the Hamilton-Jacobi equation ∂S/∂t + H(q, ∂S/∂q) = 0) that became the bridge to quantum mechanics via Schrödinger's wave equation.

---

## Well-Posedness and Distributions

**Jacques Hadamard (1865–1963)**
French mathematician who introduced the concept of well-posedness in a 1902 paper, arguing that a PDE problem is "correctly posed" only if it satisfies existence, uniqueness, and continuous dependence on data. The Hadamard example — Cauchy data for the Laplace equation gives an ill-posed problem — is the canonical counterexample demonstrating that not all PDEs admit every type of boundary/initial condition. Hadamard also proved the prime number theorem independently of de la Vallée-Poussin (1896), and the Hadamard matrix in numerical analysis bears his name.

**Laurent Schwartz (1915–2002)**
French mathematician who created the rigorous theory of distributions (generalized functions) in 1945–1950, receiving the Fields Medal in 1950 for this work. Schwartz's distributions provide the mathematical framework for Dirac's delta function, for Green's functions, and for the weak solutions of PDEs that arise when classical solutions don't exist. His two-volume *Théorie des distributions* is the foundational text. Schwartz was a committed political activist who was blacklisted from the US in the 1950s for his leftist views.

---

## GR and Mathematical PDE Theory

**Yvonne Choquet-Bruhat (1923–)**
French mathematician, the first woman elected to the French Académie des sciences. Her 1952 paper proved local existence and uniqueness for the Cauchy problem of the Einstein equations — establishing that GR is a well-posed PDE theory. She proved that the Einstein equations, in harmonic gauge, form a symmetric hyperbolic system, and the Cauchy-Kovalevskaya theorem gives local solutions. With Geroch (1969), she proved the existence of a maximal Cauchy development. Choquet-Bruhat's mathematical work established GR on firm analytic foundations.

**Richard Hamilton (1943–)**
American mathematician who introduced Ricci flow in 1982 — the "heat equation for Riemannian metrics" — and proved its short-time existence and uniqueness. Hamilton's program was to use Ricci flow to geometrize 3-manifolds (classify their topology via their curvature evolution), but the program stalled at singularities. Perelman completed Hamilton's program by analyzing these singularities. Hamilton's techniques — maximum principles for tensor equations, monotone quantities, Harnack inequalities — were the key mathematical tools.

**Grigori Perelman (1966–)**
Russian mathematician who, in three preprints (2002–2003), proved the Poincaré conjecture and the full geometrization conjecture for 3-manifolds using Ricci flow. Perelman introduced the entropy monotonicity (a Lyapunov function for Ricci flow) and the "reduced volume" and "reduced length" concepts that allowed control of singularities. He was awarded the Fields Medal (2006) and the Millennium Prize (\$1,000,000) for the Poincaré conjecture, but declined both. He has since withdrawn from mathematical life and lives reclusively in St. Petersburg.

---

## Green's Functions and Propagators

**George Green (1793–1841)**
See Chapter 8. Green's 1828 essay introduced the Green's function and the method of images in electrostatics. The Green's function for a PDE is the "inverse operator" — it converts the forcing function into the response — and its construction is the central tool in this chapter.

**Bernhard Riemann (1826–1866)**
See Chapter 4. In the PDE context: Riemann invented the Riemann function (a generalized Green's function for hyperbolic equations with variable coefficients). He also proved that the number of solutions to the Dirichlet problem for a domain in ℂ equals the dimension of H¹ — connecting PDEs to topology 40 years before de Rham.

---

## Gravitational Wave Theory

**Tullio Regge (1931–2014) and John Wheeler (1911–2008)**
See Chapter 10. Their 1957 paper reduced black hole perturbation theory to a wave equation — the Regge-Wheeler equation — whose solutions are the quasi-normal modes.

**Richard H. Price (1943–)**
American physicist who derived "Price's law" (1972): after a gravitational collapse, the field outside settles down to Schwarzschild with perturbations decaying as t^{−(2ℓ+2)} at late times. This "no-hair" theorem (black holes have no hair — only mass, charge, spin) is a consequence of the wave equation in curved spacetime and shows that all other information about the collapsing body is radiated away. Price's work is foundational for understanding what gravitational wave observations can and cannot tell us about the source.

**Yvette Choquet-Bruhat, James W. York, and Niall Ó Murchadha**
Contributors to the Lichnerowicz-York conformal decomposition of the constraint equations — the standard way to construct valid initial data for the Einstein equations (a system of elliptic PDEs). The conformal method separates the "physical" degrees of freedom (the gravitational waves) from the "gauge" degrees (coordinate choices), making the initial value problem tractable.
