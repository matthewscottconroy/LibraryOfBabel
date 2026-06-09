# Chapter 2: Sobolev Spaces

The classical approach to differential equations requires functions to be differentiable in the ordinary sense. For elliptic PDEs, however, one seeks solutions in spaces where derivatives are square-integrable (not necessarily continuous), and boundary conditions are imposed in an integrated sense rather than pointwise. Sobolev spaces, denoted $W^{k,p}(\Omega)$, provide exactly the right setting: they consist of $L^p$ functions whose weak derivatives up to order $k$ are also in $L^p$. These spaces are the natural domain for the variational (weak formulation) approach to PDEs.

## Why Sobolev Spaces

Classical solutions of a PDE require the unknown function to have all derivatives appearing in the equation and for the equation to hold pointwise. But this classical regularity may not be achievable for all boundary data, and proving existence of classical solutions is difficult.

The variational approach instead multiplies the PDE by a test function $v$, integrates by parts, and looks for a **weak solution**: a function $u$ such that the integrated identity holds for all test functions. The integration by parts shifts derivatives from $u$ to $v$, requiring only one order less differentiability from $u$ than the classical formulation. The natural space for weak solutions of second-order PDEs is $H^1(\Omega)$ (first derivatives in $L^2$), which is the Sobolev space $W^{1,2}(\Omega)$.

## Chapter Structure

**Section 1: Weak Derivatives.** The key concept is the weak (distributional) derivative: $g$ is the weak $\alpha$-derivative of $f$ if $\int f D^\alpha\phi = (-1)^{|\alpha|}\int g\phi$ for all test functions $\phi$. This extends the classical derivative to non-smooth functions and is the building block of Sobolev spaces.

**Section 2: Sobolev Spaces $W^{k,p}$.** The Sobolev space $W^{k,p}(\Omega)$ consists of $L^p$ functions all of whose weak derivatives up to order $k$ exist and are in $L^p$. With the natural Sobolev norm, $W^{k,p}$ is a Banach space (Hilbert for $p = 2$, denoted $H^k$). The space $W^{k,p}_0$ is the closure of $C_c^\infty$ in $W^{k,p}$, corresponding to zero boundary conditions.

**Section 3: Sobolev Embedding Theorems.** These theorems relate Sobolev regularity to pointwise or higher-norm regularity. For example, in $\mathbb{R}^n$ with $k > n/p$, the embedding $W^{k,p}(\mathbb{R}^n) \hookrightarrow C^0$ shows that sufficient Sobolev regularity implies continuity. The Rellich-Kondrachov theorem gives compactness of the embedding $H^1(\Omega) \hookrightarrow L^2(\Omega)$ on bounded domains.

**Section 4: Trace Theorems.** Functions in $W^{k,p}(\Omega)$ need not be continuous up to the boundary $\partial\Omega$, so their boundary values are not defined pointwise. The trace theorem constructs a bounded linear map $\gamma: W^{1,p}(\Omega) \to L^p(\partial\Omega)$ (the "trace" or boundary restriction), which agrees with the ordinary boundary restriction for smooth functions. This enables rigorous statement of Dirichlet boundary conditions in the Sobolev setting.

## The Role in PDE Theory

The Sobolev framework converts the question "does the PDE have a classical solution?" into the more tractable "does the weak formulation have a Sobolev solution?" The Lax-Milgram theorem (Chapter 1, Section 4) guarantees existence and uniqueness of weak solutions in $H^1_0$ for coercive elliptic operators. Elliptic regularity theory then shows that these weak solutions are in fact smooth (when the data is smooth), recovering classical solutions. The Sobolev framework is thus not an alternative to classical PDE theory but its rigorous foundation.
