# Appendix: Notation, Conventions, and Reference Identities

*This appendix fixes notation and conventions used throughout the book. GR notation is a minefield: different authors use different metric signatures, different conventions for the Riemann tensor, different unit systems, and different index placement rules. Here we state our conventions explicitly, explain the implications of each choice, and collect the key identities that are used repeatedly.*

---

## A.1 Index Conventions

### A.1.1 Placement of Indices

In this book we follow the standard GR convention:

- **Upper (contravariant) indices** label the components of vectors and contravariant tensors: $V^\mu$, $T^{\mu\nu}$, $g^{\mu\nu}$.
- **Lower (covariant) indices** label the components of covectors (1-forms) and covariant tensors: $\omega_\mu$, $T_{\mu\nu}$, $g_{\mu\nu}$.
- **Mixed tensors** carry both: $T^\mu_{\ \nu}$, $R^\mu_{\ \nu\rho\sigma}$.

The vertical positioning of mixed indices is significant: $T^\mu_{\ \nu} \neq T_{\ \nu}^\mu$ in general, because they are obtained from different combinations of index-raising/lowering operations.

### A.1.2 Einstein Summation Convention

A repeated index, once up and once down, is summed over all values (0, 1, 2, 3 in 4D spacetime):

$$V^\mu W_\mu \equiv \sum_{\mu=0}^{3} V^\mu W_\mu$$

This is a **contraction** and produces a scalar (invariant). We never write a free index in two different positions on the same side of an equation: $V^\mu V^\mu$ (two upper $\mu$s on the same term) is an error, not a sum.

**Dummy indices** (summed) can be freely renamed: $V^\mu W_\mu = V^\nu W_\nu = V^\alpha W_\alpha$.

**Free indices** must match on both sides of an equation: $T^\mu_{\ \nu} = A^\mu_{\ \rho}B^\rho_{\ \nu}$ has free indices $\mu$ (up) and $\nu$ (down) on both sides. ✓

### A.1.3 Abstract Index Notation vs. Component Notation

We use the **abstract index notation** (Penrose notation): indices are labels on the type of a tensor, not specific numerical values. $V^\mu$ denotes a vector (the $\mu$ labels it as a contravariant vector), not a specific component.

When we write a **specific component**, we use explicit values: $V^0 =$ time component, $V^1 = x$-component, etc.

In some places we use **boldface** for abstract 3-vectors in the Euclidean sense: $\mathbf{v} = (v^x, v^y, v^z)$ (in Newtonian physics or in an inertial frame locally).

### A.1.4 Spacetime vs. Space Indices

We follow the convention:
- **Greek letters** $\mu, \nu, \rho, \sigma, \ldots$ range over spacetime: $\mu = 0, 1, 2, 3$.
- **Latin letters** $i, j, k, \ldots$ range over space: $i = 1, 2, 3$.
- Index $0$ always denotes the time component.

Some texts use $a, b, c, \ldots$ for abstract spacetime indices and reserve $\mu, \nu$ for coordinate components. We do not follow this distinction here (all Latin/Greek spacetime indices are coordinate component indices).

**Tetrad (local frame) indices** when used: $a, b, c, \ldots$ with a hat or specific notation. $e^\mu_{\ a}$ is the tetrad: $g_{\mu\nu}e^\mu_{\ a}e^\nu_{\ b} = \eta_{ab}$.

---

## A.2 Metric Signature

We use the **mostly-plus convention** (Misner-Thorne-Wheeler (MTW) sign):

$$\eta_{\mu\nu} = \text{diag}(-1, +1, +1, +1)$$

**Alternative:** The mostly-minus convention $\text{diag}(+1,-1,-1,-1)$ is used by Landau-Lifshitz, Hawking-Ellis, and many particle physicists. The two conventions differ in the sign of many formulas.

**Implications of the $(-,+,+,+)$ convention:**
- Timelike vectors have $V^\mu V_\mu < 0$: e.g., the 4-velocity $u^\mu u_\mu = -c^2 < 0$.
- Spacelike vectors have $V^\mu V_\mu > 0$.
- Null (lightlike) vectors have $V^\mu V_\mu = 0$.
- The spacetime interval: $ds^2 = g_{\mu\nu}dx^\mu dx^\nu = -c^2dt^2 + d\mathbf{x}^2$.
- The 4-momentum norm: $p^\mu p_\mu = -m^2c^2$.
- The Ricci scalar $R > 0$ for a sphere (positive curvature).

**Translation to $(+,-,-,-)$:** If you need to use a reference with the other signature, make the substitution $g_{\mu\nu}\to -g_{\mu\nu}$ (equivalently, flip all signs on the metric). Tensors with an even number of lowered indices using $g$ are unaffected; those with an odd number flip sign. The Riemann tensor convention also varies between references (see Section A.4).

---

## A.3 Units

### A.3.1 SI Units

In most of the book we use SI units with explicit factors of $c$, $G$, $\hbar$:

$$[c] = \text{m/s}, \quad [G] = \text{m}^3/(\text{kg·s}^2), \quad [\hbar] = \text{J·s}$$

The Schwarzschild radius: $r_s = 2GM/c^2$.
The Planck length: $\ell_P = \sqrt{\hbar G/c^3} = 1.616\times10^{-35}$ m.
The Planck mass: $m_P = \sqrt{\hbar c/G} = 2.176\times10^{-8}$ kg.
The Planck time: $t_P = \sqrt{\hbar G/c^5} = 5.391\times10^{-44}$ s.

### A.3.2 Natural Units

In discussions of quantum gravity, Hawking radiation, and high-energy physics, we set $c = \hbar = k_B = 1$ (natural units) and sometimes $G = 1$ (Planck units). In Planck units:

$$c = \hbar = G = k_B = 1$$

The Schwarzschild radius becomes $r_s = 2M$ and the Hawking temperature $T_H = 1/(8\pi M)$.

**Recovering SI:** Any formula in natural units can be restored to SI by dimensional analysis. The key substitutions: $E\to E/c^2$ (mass), $L\to Lc$ (time$\to$length), etc.

### A.3.3 Geometric Units

In many GR textbooks (Carroll, MTW), **geometric units** set $G = c = 1$ but keep $\hbar$. Then:
- Mass has dimensions of length: $r_s = 2M$ (in meters: $r_s = 2GM/c^2$).
- Time also has dimensions of length: $T = ct$ (in meters).

---

## A.4 Curvature Conventions

Curvature conventions vary between references in two ways: the **sign** of the Riemann tensor and the **index ordering**.

### A.4.1 Riemann Tensor

**Our convention** (Carroll, MTW, Wald):

$$R^\rho_{\ \sigma\mu\nu} = \partial_\mu\Gamma^\rho_{\nu\sigma} - \partial_\nu\Gamma^\rho_{\mu\sigma} + \Gamma^\rho_{\mu\lambda}\Gamma^\lambda_{\nu\sigma} - \Gamma^\rho_{\nu\lambda}\Gamma^\lambda_{\mu\sigma}$$

The commutator of covariant derivatives:
$$[\nabla_\mu,\nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$$

**Alternative** (Landau-Lifshitz, some other texts): opposite sign $R^\rho_{\ \sigma\mu\nu}\to -R^\rho_{\ \sigma\mu\nu}$.

Check: for the unit sphere $S^2$, $R^\theta_{\ \phi\theta\phi} = \sin^2\!\theta$ (in our convention). This is positive — consistent with positive curvature.

### A.4.2 Ricci Tensor

**Our convention:**

$$R_{\mu\nu} = R^\rho_{\ \mu\rho\nu} = g^{\rho\sigma}R_{\rho\mu\sigma\nu}$$

(Contraction on the **first and third** indices of $R_{\rho\mu\sigma\nu}$.)

**Alternative:** Some texts use $R_{\mu\nu} = R^\rho_{\ \nu\rho\mu}$ (different contraction), giving the opposite sign.

For the sphere: $R_{\theta\theta} = 1$, $R_{\phi\phi} = \sin^2\!\theta$, $R = 2$ (positive).

### A.4.3 Einstein Tensor

$$G_{\mu\nu} = R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$$

The Einstein field equations (without cosmological constant):

$$G_{\mu\nu} = \frac{8\pi G}{c^4}T_{\mu\nu}$$

Some texts write this as $G_{\mu\nu} = \kappa T_{\mu\nu}$ where $\kappa = 8\pi G/c^4$.

### A.4.4 Christoffel Symbols

$$\Gamma^\rho_{\mu\nu} = \frac{1}{2}g^{\rho\sigma}(\partial_\mu g_{\nu\sigma} + \partial_\nu g_{\mu\sigma} - \partial_\sigma g_{\mu\nu})$$

Symmetry: $\Gamma^\rho_{\mu\nu} = \Gamma^\rho_{\nu\mu}$ (torsion-free connection).

---

## A.5 Symbol Table

The following symbols are used throughout the book with the definitions given here.

### Spacetime Geometry

| Symbol | Name | Definition |
|---|---|---|
| $g_{\mu\nu}$ | Metric tensor | Symmetric (0,2) tensor |
| $g^{\mu\nu}$ | Inverse metric | $g^{\mu\alpha}g_{\alpha\nu} = \delta^\mu_{\ \nu}$ |
| $g$ | Metric determinant | $g = \det(g_{\mu\nu})$ |
| $\eta_{\mu\nu}$ | Minkowski metric | $\text{diag}(-1,+1,+1,+1)$ |
| $\Gamma^\rho_{\mu\nu}$ | Christoffel symbols | Levi-Civita connection components |
| $\nabla_\mu$ | Covariant derivative | With Levi-Civita connection |
| $\nabla^2$ | Laplace-Beltrami | $g^{\mu\nu}\nabla_\mu\nabla_\nu$ |
| $\Box$ | d'Alembertian | $g^{\mu\nu}\nabla_\mu\nabla_\nu$ (= $\nabla^2$ on scalars) |
| $R^\rho_{\ \sigma\mu\nu}$ | Riemann tensor | See Section A.4.1 |
| $R_{\mu\nu}$ | Ricci tensor | $R^\rho_{\ \mu\rho\nu}$ |
| $R$ | Ricci scalar | $g^{\mu\nu}R_{\mu\nu}$ |
| $G_{\mu\nu}$ | Einstein tensor | $R_{\mu\nu} - \frac{1}{2}g_{\mu\nu}R$ |
| $\Lambda$ | Cosmological constant | Energy of vacuum |
| $T_{\mu\nu}$ | Stress-energy tensor | Source of curvature |

### Differential Geometry

| Symbol | Name | Definition/Note |
|---|---|---|
| $M$ | Spacetime manifold | 4D smooth manifold |
| $T_p M$ | Tangent space at $p$ | Vector space of tangent vectors |
| $T^*_p M$ | Cotangent space | Dual to $T_pM$ |
| $\mathcal{L}_X$ | Lie derivative | Along vector field $X$ |
| $\xi^\mu$ | Killing vector | $\nabla_{(\mu}\xi_{\nu)} = 0$ |
| $d$ | Exterior derivative | Antisymmetrized partial derivative |
| $\star$ | Hodge dual | $(\star\omega)_{\mu_1\cdots\mu_{n-p}} = \frac{1}{p!}\varepsilon^{\nu_1\cdots\nu_p}_{\ \ \ \mu_1\cdots\mu_{n-p}}\omega_{\nu_1\cdots\nu_p}$ |
| $\varepsilon_{\mu\nu\rho\sigma}$ | Levi-Civita tensor | $\varepsilon_{0123} = \sqrt{-g}$, $\hat\varepsilon_{0123} = 1$ |

### Physical Quantities

| Symbol | Name | SI Units |
|---|---|---|
| $c$ | Speed of light | $2.998\times10^8$ m/s |
| $G$ | Gravitational constant | $6.674\times10^{-11}$ m³/(kg·s²) |
| $\hbar$ | Reduced Planck constant | $1.055\times10^{-34}$ J·s |
| $k_B$ | Boltzmann constant | $1.381\times10^{-23}$ J/K |
| $r_s = 2GM/c^2$ | Schwarzschild radius | m |
| $M_\odot$ | Solar mass | $1.989\times10^{30}$ kg |
| $H_0$ | Hubble constant | 67.4 km/s/Mpc |
| $\ell_P$ | Planck length | $1.616\times10^{-35}$ m |
| $m_P$ | Planck mass | $2.176\times10^{-8}$ kg |
| $T_P$ | Planck temperature | $1.417\times10^{32}$ K |

### Cosmology

| Symbol | Name | Definition |
|---|---|---|
| $a(t)$ | Scale factor | $a(t_0) = 1$ normalized |
| $H = \dot{a}/a$ | Hubble parameter | Rate of expansion |
| $z$ | Redshift | $1+z = a_0/a_e$ |
| $\Omega_X$ | Density parameter | $\rho_X/\rho_c$ |
| $\rho_c = 3H^2/(8\pi G)$ | Critical density | Flat universe density |
| $q = -\ddot{a}a/\dot{a}^2$ | Deceleration parameter | |

---

## A.6 Key Identities

### A.6.1 Ricci Identity

For a vector $V^\mu$:

$$[\nabla_\mu, \nabla_\nu]V^\rho = R^\rho_{\ \sigma\mu\nu}V^\sigma$$

For a covector $\omega_\mu$:

$$[\nabla_\mu, \nabla_\nu]\omega_\rho = -R^\sigma_{\ \rho\mu\nu}\omega_\sigma$$

For a $(2,0)$ tensor $T^{\mu\nu}$:

$$[\nabla_\rho, \nabla_\sigma]T^{\mu\nu} = R^\mu_{\ \lambda\rho\sigma}T^{\lambda\nu} + R^\nu_{\ \lambda\rho\sigma}T^{\mu\lambda}$$

General rule: each upper index gets a $+R\cdot T$ term with the Riemann tensor contracting on that index; each lower index gets a $-R\cdot T$ term.

### A.6.2 Algebraic (First) Bianchi Identity

$$R_{\mu[\nu\rho\sigma]} = 0 \quad\Leftrightarrow\quad R_{\mu\nu\rho\sigma} + R_{\mu\rho\sigma\nu} + R_{\mu\sigma\nu\rho} = 0$$

This is a consequence of the torsion-free property of the Levi-Civita connection.

### A.6.3 Differential (Second) Bianchi Identity

$$\nabla_{[\lambda}R_{\mu\nu]\rho\sigma} = 0 \quad\Leftrightarrow\quad \nabla_\lambda R_{\mu\nu\rho\sigma} + \nabla_\mu R_{\nu\lambda\rho\sigma} + \nabla_\nu R_{\lambda\mu\rho\sigma} = 0$$

### A.6.4 Contracted Bianchi Identity

Contracting the differential Bianchi identity with $g^{\mu\rho}$:

$$\nabla^\mu R_{\nu\mu} = \frac{1}{2}\nabla_\nu R \quad\Leftrightarrow\quad \nabla^\mu G_{\mu\nu} = 0$$

This is the foundation of energy-momentum conservation in GR.

### A.6.5 Covariant Divergence Formula

For a vector $V^\mu$:

$$\nabla_\mu V^\mu = \frac{1}{\sqrt{-g}}\partial_\mu(\sqrt{-g}\,V^\mu)$$

For an antisymmetric tensor $A^{\mu\nu} = -A^{\nu\mu}$:

$$\nabla_\nu A^{\mu\nu} = \frac{1}{\sqrt{-g}}\partial_\nu(\sqrt{-g}\,A^{\mu\nu})$$

(No Christoffel symbol contribution from the antisymmetric pair, since $\Gamma^\nu_{\nu\rho} = \partial_\rho\ln\sqrt{-g}$ is the only surviving term and it cancels.)

### A.6.6 Volume Element

The invariant (coordinate-independent) volume element:

$$d^4V = \sqrt{-g}\,d^4x = \sqrt{-g}\,dx^0\,dx^1\,dx^2\,dx^3$$

For a space-like hypersurface $\Sigma$ with unit normal $n_\mu$:

$$d^3V_\mu = n_\mu\sqrt{h}\,d^3x$$

where $h = \det h_{ij}$ is the determinant of the induced metric.

### A.6.7 Symmetries of the Riemann Tensor

$$R_{\mu\nu\rho\sigma} = -R_{\nu\mu\rho\sigma} = -R_{\mu\nu\sigma\rho} = R_{\rho\sigma\mu\nu}$$

Independent components: in $n = 4$ dimensions, $\frac{n^2(n^2-1)}{12} = 20$ independent components.

### A.6.8 Geodesic Equation

$$\frac{d^2x^\mu}{d\lambda^2} + \Gamma^\mu_{\nu\rho}\frac{dx^\nu}{d\lambda}\frac{dx^\rho}{d\lambda} = 0$$

For massive particles: $\lambda = \tau$ (proper time), with normalization $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = -c^2$.
For massless particles (photons): $\lambda$ is an affine parameter, $g_{\mu\nu}\dot{x}^\mu\dot{x}^\nu = 0$.

---

## A.7 Reference — Comparison of Conventions

The table below allows translation between common GR references.

| Convention | Carroll | MTW | Wald | Landau-Lifshitz | Hawking-Ellis |
|---|---|---|---|---|---|
| Metric signature | $(-,+,+,+)$ | $(-,+,+,+)$ | $(-,+,+,+)$ | $(+,-,-,-)$ | $(-,+,+,+)$ |
| Riemann tensor | $+$ | $+$ | $+$ | $-$ | $+$ |
| Ricci tensor | $R_{\mu\nu} = R^\rho_{\ \mu\rho\nu}$ | same | same | opposite | same |
| Einstein eq. | $G_{\mu\nu} = 8\pi G T_{\mu\nu}$ | same | same | same | same |

**Rule of thumb:** Carroll (2004) is the most widely used modern graduate text; its conventions agree with MTW (1973) and Wald (1984). When consulting older texts or particle physics literature using $(+,-,-,-)$, the Riemann tensor and Ricci tensor flip sign.

---

## A.8 Notation for Specific Spacetimes

### Schwarzschild Metric (Schwarzschild Coordinates)

$$ds^2 = -\left(1-\frac{r_s}{r}\right)c^2dt^2 + \left(1-\frac{r_s}{r}\right)^{-1}dr^2 + r^2d\Omega^2$$

where $d\Omega^2 = d\theta^2 + \sin^2\!\theta\,d\phi^2$ and $r_s = 2GM/c^2$.

### Kerr Metric (Boyer-Lindquist Coordinates)

$$ds^2 = -\left(1-\frac{r_s r}{\Sigma}\right)c^2dt^2 - \frac{2r_s r a\sin^2\!\theta}{\Sigma}c\,dt\,d\phi + \frac{\Sigma}{\Delta}dr^2 + \Sigma\,d\theta^2 + \left(r^2+a^2+\frac{r_s r a^2\sin^2\!\theta}{\Sigma}\right)\sin^2\!\theta\,d\phi^2$$

where $\Sigma = r^2 + a^2\cos^2\!\theta$, $\Delta = r^2 - r_s r + a^2$, and $a = J/(Mc)$ is the specific angular momentum.

Horizons: $\Delta = 0 \Rightarrow r_\pm = \frac{r_s}{2}\pm\sqrt{\left(\frac{r_s}{2}\right)^2-a^2}$.

### FLRW Metric (Flat Case, $k=0$)

$$ds^2 = -c^2dt^2 + a^2(t)(dx^2+dy^2+dz^2) = -c^2dt^2 + a^2(t)d\mathbf{x}^2$$

Conformal time: $\eta$ defined by $cd\eta = dt/a(t)$, giving $ds^2 = a^2(\eta)(-c^2d\eta^2 + d\mathbf{x}^2)$.
