# Chapter 18: Important Concepts

---

**Gauge Invariance**
The freedom to transform $A_\mu \to A_\mu + \partial_\mu\chi$ without changing the physical fields $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$. The gauge transformation corresponds to the redundancy in describing a U(1) connection by its local representative. Gauge invariance is not merely a mathematical nicety — it is the organizing principle of all fundamental interactions: U(1) for electromagnetism, SU(2) for the weak force, SU(3) for the strong force, and diffeomorphism invariance for GR.

**The 4-Potential**
$A^\mu = (\phi/c, \mathbf{A})$: a Lorentz 4-vector whose antisymmetric derivative gives the Faraday tensor $F_{\mu\nu} = \partial_\mu A_\nu - \partial_\nu A_\mu$. The gauge freedom is $A_\mu \to A_\mu + \partial_\mu\chi$. The 4-potential carries 4 components; 1 is eliminated by gauge freedom, leaving 3; 1 more is eliminated by the Lorenz gauge condition $\partial_\mu A^\mu = 0$, leaving 2 — the two physical polarizations of the photon.

**Gauge Fixing**
Choosing a specific $\chi$ to simplify the equations. Common choices: Coulomb gauge ($\nabla\cdot\mathbf{A} = 0$, useful for non-relativistic problems), Lorenz gauge ($\partial_\mu A^\mu = 0$, covariant, gives wave equations), temporal gauge ($A^0 = 0$). No single gauge is "best" — the choice depends on the problem. GR analog: choosing coordinates (diffeomorphism gauge fixing). In GR, harmonic coordinates ($\partial_\mu(\sqrt{-g}g^{\mu\nu}) = 0$) play the role of Lorenz gauge.

**Lorenz Gauge**
$\partial_\mu A^\mu = 0$: manifestly Lorentz covariant (not to be confused with "Lorentz gauge"). In this gauge, Maxwell's equations reduce to $\Box A^\mu = \mu_0 J^\mu$ — four decoupled wave equations. The residual gauge freedom (with $\Box\chi = 0$) can fix the initial data.

**The Aharonov-Bohm Effect**
A quantum-mechanical interference experiment showing that the vector potential $A_\mu$ (not just the field $F_{\mu\nu}$) affects observable quantities, even in regions where $\mathbf{B} = 0$. The phase shift $\Delta\phi = (e/\hbar)\oint A_\mu dx^\mu = e\Phi_B/\hbar$ (where $\Phi_B$ is the magnetic flux through the solenoid) shifts the interference fringes. This is a topological effect: the holonomy of the U(1) connection is non-trivial even when the curvature (field) vanishes. The GR analog: holonomy of the Levi-Civita connection around non-contractible loops can be non-trivial even in flat space.

**Electromagnetic Action**
$S[A] = \int(-\frac{1}{4\mu_0}F_{\mu\nu}F^{\mu\nu} + A_\mu J^\mu)d^4x$: the action for the electromagnetic field coupled to sources. Gauge invariant (up to boundary terms) when $\partial_\mu J^\mu = 0$. The Euler-Lagrange equations give $\partial_\mu F^{\mu\nu} = \mu_0 J^\nu$. This action is the prototype for all gauge theory actions.

**U(1) Gauge Theory**
Electromagnetism as a gauge theory of the Abelian group U(1) = {$e^{i\alpha}$ : $\alpha \in \mathbb{R}$}. Under local gauge transformation $\psi \to e^{iq\chi}\psi$ (matter field), $A_\mu \to A_\mu + \partial_\mu\chi$. The covariant derivative $D_\mu = \partial_\mu - iqA_\mu$ transforms covariantly: $D_\mu\psi \to e^{iq\chi}D_\mu\psi$. The field strength $F_{\mu\nu} = (i/q)[D_\mu, D_\nu] = \partial_\mu A_\nu - \partial_\nu A_\mu$ is gauge-invariant. This structure is the template for Yang-Mills theory (with non-Abelian gauge groups SU(2), SU(3)).

**Covariant Derivative**
$D_\mu = \partial_\mu - iqA_\mu$ (for a field of charge $q$): the gauge-covariant version of the partial derivative. Transforms as $D_\mu\psi \to e^{iq\chi}D_\mu\psi$ under gauge transformations. The squared norm $|D_\mu\psi|^2$ is gauge-invariant and appears in the matter Lagrangian. In GR, the covariant derivative $\nabla_\mu = \partial_\mu + \Gamma_\mu$ (with Christoffel symbols) plays the same role — it is the covariant derivative with respect to the Levi-Civita connection.

**Dirac Quantization Condition**
$qg = n\hbar c/2$ ($n \in \mathbb{Z}$): if a magnetic monopole of charge $g$ exists, all electric charges must be multiples of $\hbar c/(2g)$. This would explain charge quantization. Derived by Dirac (1931) from the single-valuedness of quantum wave functions. The mathematical structure is: the monopole represents a non-trivial U(1) bundle over $S^2$ with first Chern class $c_1 = n$.

**Fiber Bundle Structure**
The modern mathematical framework for gauge theory: a principal G-bundle over spacetime, where G is the gauge group (U(1) for electromagnetism, SU(2)×U(1) for electroweak, etc.). The gauge field $A_\mu$ is the connection on this bundle; $F_{\mu\nu}$ is the curvature (field strength). The gauge transformation is the transition function between local trivializations. The Aharonov-Bohm effect is holonomy. GR is the gauge theory of local Lorentz invariance — the bundle is the frame bundle, the connection is the spin connection $\omega^\mu_{\ \nu}$, the curvature is the Riemann tensor.

**GR as Gauge Theory**
In the tetrad (vierbein) formulation, GR is a gauge theory of local Lorentz invariance SO(3,1) (or its double cover $SL(2,\mathbb{C})$). The tetrad $e^a_{\ \mu}$ plays the role of the gauge field; the spin connection $\omega^a_{\ b\mu}$ is the connection; the Riemann tensor is the curvature. The Einstein-Hilbert action $\int R\sqrt{-g}d^4x$ is the Yang-Mills-type action for this gauge theory (though linear in curvature, not quadratic).
