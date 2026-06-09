# The Space of Test Functions

The space of test functions is the arena in which distributions act. It must be large enough to "probe" all the features of a distribution (so test functions must be dense in natural function spaces), but small enough that every distribution is continuous on it (so the topology must be strong). The Schwartz space $\mathcal{D}(\mathbb{R}^n) = C_c^\infty(\mathbb{R}^n)$ of compactly supported smooth functions is the canonical choice, achieving both goals. This section develops its properties, topology, and role.

## Compactly Supported Smooth Functions

**Definition.** The **space of test functions** is

$$\mathcal{D}(\mathbb{R}^n) = C_c^\infty(\mathbb{R}^n) = \{\phi \in C^\infty(\mathbb{R}^n) : \text{supp}(\phi) \text{ is compact}\},$$

where $\text{supp}(\phi) = \overline{\{x \in \mathbb{R}^n : \phi(x) \neq 0\}}$ is the support of $\phi$.

Elements of $\mathcal{D}$ are smooth (infinitely differentiable) and vanish outside some bounded set. This is a very strong requirement: the functions are simultaneously very smooth (no constraint on differentiability) and very small at infinity (compactly supported).

## Existence of Test Functions

The existence of non-trivial test functions is not obvious: most "elementary" smooth functions (polynomials, $e^x$, $\sin x$) are not compactly supported. The key example is:

$$\rho(x) = \begin{cases} \exp\left(-\frac{1}{1 - |x|^2}\right) & |x| < 1 \\ 0 & |x| \geq 1. \end{cases}$$

**Claim.** $\rho \in C^\infty(\mathbb{R}^n)$.

**Proof.** The function $t \mapsto e^{-1/t}$ for $t > 0$ is smooth with all derivatives tending to $0$ as $t \to 0^+$. Therefore all derivatives of $\rho$ at boundary points of the unit ball (where $1 - |x|^2 = 0$) are zero. By induction on the order of differentiation, $\rho$ is $C^\infty$ everywhere. $\square$

Normalizing: $\phi(x) = \rho(x) / \int \rho$. Any compactly supported function can be approximated by multiples of shifted and scaled copies of $\phi$. Moreover, any function $f \in C^k_c(\mathbb{R}^n)$ can be approximated in the $C^k$ topology by test functions (by mollification: $f * \phi_\varepsilon$ where $\phi_\varepsilon(x) = \varepsilon^{-n}\phi(x/\varepsilon)$).

## Mollifiers and Approximation

**Definition.** A **mollifier** is a function $\phi_\varepsilon(x) = \varepsilon^{-n}\phi(x/\varepsilon)$ where $\phi$ is a non-negative test function with $\int \phi = 1$. The **mollification** of a locally integrable function $f$ is the convolution $f_\varepsilon = f * \phi_\varepsilon$.

**Theorem.** If $f \in L^p_{\text{loc}}(\mathbb{R}^n)$ for $1 \leq p < \infty$, then $f_\varepsilon \in C^\infty$ and $f_\varepsilon \to f$ in $L^p_{\text{loc}}$ as $\varepsilon \to 0$. If $f \in C^k$, then $f_\varepsilon \to f$ in $C^k$.

Mollification shows that $C^\infty$ functions are dense in $L^p$, and that test functions are dense in a wide variety of function spaces. This density is what allows distributions to "see" the full function-theoretic structure.

## The Topology on $\mathcal{D}$

A sequence $(\phi_j) \subset \mathcal{D}$ **converges to $\phi$ in $\mathcal{D}$** if:
1. There exists a compact set $K \subset \mathbb{R}^n$ such that $\text{supp}(\phi_j) \subset K$ for all $j$.
2. For every multi-index $\alpha = (\alpha_1, \ldots, \alpha_n)$ with $|\alpha| = \alpha_1 + \cdots + \alpha_n$:

$$\sup_{x \in K} |D^\alpha\phi_j(x) - D^\alpha\phi(x)| \to 0 \quad \text{as } j \to \infty,$$

where $D^\alpha = \partial^{|\alpha|}/(\partial x_1^{\alpha_1} \cdots \partial x_n^{\alpha_n})$.

This is convergence of all derivatives, uniformly on the common support. The resulting topology makes $\mathcal{D}$ an **LF-space** (strict inductive limit of Fréchet spaces): for each compact $K$, $\mathcal{D}_K = \{\phi \in \mathcal{D} : \text{supp}(\phi) \subset K\}$ is a Fréchet space with seminorms $p_m(\phi) = \sum_{|\alpha| \leq m} \sup_K |D^\alpha\phi|$, and $\mathcal{D} = \bigcup_K \mathcal{D}_K$ with the inductive limit topology.

**Key property.** A linear map $T: \mathcal{D} \to V$ (to any topological vector space) is continuous if and only if its restriction to each $\mathcal{D}_K$ is continuous. For $V = \mathbb{R}$, this means: $T$ is continuous if and only if for every compact $K$, there exist $C > 0$ and $m \in \mathbb{N}$ such that $|T(\phi)| \leq C \sum_{|\alpha| \leq m} \sup_K |D^\alpha\phi|$ for all $\phi \in \mathcal{D}_K$.

## The Schwartz Space

For Fourier transform theory, the space $\mathcal{D}$ is insufficient (the Fourier transform of a compactly supported function need not be compactly supported). A larger test function space with better Fourier behavior is the **Schwartz space**:

$$\mathcal{S}(\mathbb{R}^n) = \left\{\phi \in C^\infty(\mathbb{R}^n) : \sup_x |x^\beta D^\alpha \phi(x)| < \infty \text{ for all multi-indices } \alpha, \beta\right\}.$$

Schwartz functions decay, along with all derivatives, faster than any polynomial. The topology is given by the seminorms $\|\phi\|_{\alpha,\beta} = \sup_x |x^\beta D^\alpha\phi(x)|$.

The spaces are related by inclusions:
$$\mathcal{D}(\mathbb{R}^n) \subset \mathcal{S}(\mathbb{R}^n) \subset L^2(\mathbb{R}^n),$$
and each inclusion is dense. The Fourier transform is a topological isomorphism of $\mathcal{S}$ onto itself, and the dual $\mathcal{S}' = \mathcal{S}'(\mathbb{R}^n)$ is the space of **tempered distributions**, on which the Fourier transform also acts isomorphically.

## Convergence in $\mathcal{D}'$

A sequence $(T_j) \subset \mathcal{D}'$ **converges weakly** (in the weak-* topology) to $T \in \mathcal{D}'$ if:

$$\langle T_j, \phi \rangle \to \langle T, \phi \rangle \text{ for every } \phi \in \mathcal{D}.$$

This is also called **sequential convergence in $\mathcal{D}'$**. For example, the approximations $\delta_\varepsilon$ of the delta function converge to $\delta$ in $\mathcal{D}'$: for any test function $\phi$, $\langle \delta_\varepsilon, \phi \rangle = \int \delta_\varepsilon(x)\phi(x) \, dx \to \phi(0) = \langle \delta, \phi \rangle$ as $\varepsilon \to 0$.

A crucial property: weak limits of sequences of distributions always exist (under mild conditions), unlike limits in $L^p$ spaces. This makes $\mathcal{D}'$ extremely convenient for taking limits and studying differential equations.
