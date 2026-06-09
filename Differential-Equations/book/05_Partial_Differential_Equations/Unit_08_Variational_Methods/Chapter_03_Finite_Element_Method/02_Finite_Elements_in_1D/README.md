# Finite Elements in 1D

The one-dimensional finite element method provides the clearest illustration of the key ideas: mesh generation, basis function construction, element stiffness matrix assembly, and error analysis. In 1D, the finite element basis consists of piecewise polynomial "hat functions" (or higher-degree Lagrange polynomials), and the resulting stiffness matrix is tridiagonal — the same structure as the finite difference matrix for the Laplacian, but derived with rigorous approximation-theoretic error control from the variational framework.

## Setting and Mesh

**Problem.** Find $u\in H^1_0(0,1)$ satisfying $-u'' = f$ on $(0,1)$ with $u(0) = u(1) = 0$. Weak form: $\int_0^1 u'v'\,dx = \int_0^1 fv\,dx$ for all $v\in H^1_0(0,1)$.

**Uniform mesh.** Divide $[0,1]$ into $n$ equal subintervals (elements): $[x_{j-1}, x_j]$ for $j=1,\ldots,n$, with nodes $x_j = j/n = jh$ (mesh size $h = 1/n$). The interior nodes are $x_1, x_2, \ldots, x_{n-1}$ ($N = n-1$ unknowns; $x_0 = 0$ and $x_n = 1$ have $u = 0$).

**Piecewise linear finite element space:**

$$V_h = \{v_h\in C^0([0,1]): v_h\text{ is linear on each }[x_{j-1},x_j], v_h(0) = v_h(1) = 0\}.$$

This is an $(n-1)$-dimensional subspace of $H^1_0(0,1)$: continuous piecewise linear functions vanishing at the endpoints.

## Hat Functions (Basis)

The standard basis for $V_h$ is the set of **hat functions** $\{\phi_j\}_{j=1}^{n-1}$:

$$\phi_j(x) = \begin{cases}(x-x_{j-1})/h & x\in[x_{j-1},x_j] \\ (x_{j+1}-x)/h & x\in[x_j,x_{j+1}] \\ 0 & \text{otherwise}\end{cases}.$$

Properties:
- $\phi_j(x_i) = \delta_{ij}$ (Lagrange interpolation property).
- $\text{supp}(\phi_j) = [x_{j-1},x_{j+1}]$ (support spans two elements).
- $\phi_j' = 1/h$ on $(x_{j-1},x_j)$ and $\phi_j' = -1/h$ on $(x_j,x_{j+1})$, zero elsewhere.

## Stiffness Matrix

The $(i,j)$ entry of the stiffness matrix:

$$K_{ij} = a(\phi_j,\phi_i) = \int_0^1\phi_j'(x)\phi_i'(x)\,dx.$$

**Diagonal entry ($i=j$):**

$$K_{jj} = \int_0^1(\phi_j')^2\,dx = \int_{x_{j-1}}^{x_j}\frac{1}{h^2}\,dx + \int_{x_j}^{x_{j+1}}\frac{1}{h^2}\,dx = \frac{1}{h^2}\cdot h + \frac{1}{h^2}\cdot h = \frac{2}{h}.$$

**Superdiagonal/subdiagonal ($|i-j|=1$, say $j=i+1$):**

$$K_{i,i+1} = \int_{x_i}^{x_{i+1}}\frac{-1}{h}\cdot\frac{1}{h}\,dx = \frac{-1}{h^2}\cdot h = \frac{-1}{h}.$$

(The supports of $\phi_i$ and $\phi_{i+1}$ overlap only on $[x_i, x_{i+1}]$, where $\phi_i' = -1/h$ and $\phi_{i+1}' = 1/h$.)

**Off-diagonal ($|i-j|\geq 2$):** $K_{ij} = 0$ (disjoint supports).

The stiffness matrix is:

$$\mathbf{K} = \frac{1}{h}\begin{pmatrix}2 & -1 & & \\ -1 & 2 & -1 & \\ & \ddots & \ddots & \ddots \\ & & -1 & 2\end{pmatrix}.$$

This is exactly the standard second-order finite difference discretization of $-d^2/dx^2$! The FEM and finite differences coincide for piecewise linear elements on a uniform mesh — a reassuring consistency check.

## Load Vector

The load vector entry:

$$F_i = F(\phi_i) = \int_0^1 f(x)\phi_i(x)\,dx.$$

For general $f$: compute $F_i = \int_{x_{i-1}}^{x_i}f(x)\frac{x-x_{i-1}}{h}\,dx + \int_{x_i}^{x_{i+1}}f(x)\frac{x_{i+1}-x}{h}\,dx$.

**Approximation (lumped mass).** For slowly varying $f$: $F_i \approx f(x_i)\int_0^1\phi_i\,dx = f(x_i)\cdot h$ (trapezoidal rule on each element). This gives the finite difference right-hand side $F_i = h\cdot f(x_i)$.

The linear system $\mathbf{KU} = \mathbf{F}$ with the tridiagonal $\mathbf{K}$ and $F_i = hf(x_i)$ is the classic $(-U_{j-1}+2U_j-U_{j+1})/h^2 = f(x_j)$ — the centered finite difference scheme for $-u'' = f$.

## Error Analysis

**Interpolation estimate.** For $u\in H^2(0,1)$, let $u_I = \sum_{j=1}^{n-1}u(x_j)\phi_j$ be the piecewise linear interpolant. Then:

$$\|u - u_I\|_{H^1(0,1)} \leq Ch\|u''\|_{L^2(0,1)},$$

where $C$ is an absolute constant ($C = 1/\sqrt{8}$ for the standard estimate). This follows from the interpolation estimate on each element: $\|u - u_I\|_{H^1([x_{j-1},x_j])} \leq C h\|u''\|_{L^2([x_{j-1},x_j])}$ (standard local interpolation theory).

**FEM error (by Céa).** For $M = \alpha = 1$ (Poisson):

$$\|u - u_h\|_{H^1} \leq \inf_{v_h\in V_h}\|u-v_h\|_{H^1} \leq \|u - u_I\|_{H^1} \leq Ch\|u''\|_{L^2}.$$

So $\|u-u_h\|_{H^1} = O(h)$ — first-order convergence in $H^1$.

**$L^2$ error by Aubin-Nitsche.** The Aubin-Nitsche (duality) trick gives one extra power of $h$. Let $w\in H^1_0$ solve the dual problem $a(w,v) = \int(u-u_h)v\,dx$ for all $v$ (so $-w'' = u-u_h$). Then:

$$\|u-u_h\|_{L^2}^2 = a(u-u_h,w) = a(u-u_h,w-w_h)$$

(Galerkin orthogonality removes $w_h\in V_h$). By the Cauchy-Schwarz inequality and the interpolation estimate on the dual solution:

$$\|u-u_h\|_{L^2}^2 \leq \|u-u_h\|_{H^1}\|w-w_h\|_{H^1} \leq Ch\|u''\|_{L^2}\cdot Ch\|w''\|_{L^2}.$$

Since $\|w''\|_{L^2} = \|u-u_h\|_{L^2}$ (dual problem regularity): $\|u-u_h\|_{L^2}^2 \leq C^2h^2\|u''\|_{L^2}\|u-u_h\|_{L^2}$, giving:

$$\|u-u_h\|_{L^2} \leq C^2h^2\|u''\|_{L^2}.$$

Second-order convergence in $L^2$ — one extra power of $h$ compared to $H^1$.

## Higher-Degree Elements

**Quadratic elements ($k=2$).** Add midpoint nodes $x_{j-1/2} = (x_{j-1}+x_j)/2$. The basis includes:
- Vertex hat functions $\phi_j$ (with $\phi_j(x_i) = \delta_{ij}$).
- Midpoint bubble functions $\psi_{j-1/2}$ (with $\psi_{j-1/2}(x_{j-1/2}) = 1$, zero at all nodes and midpoints).

These Lagrange quadratic basis functions span the space of functions that are quadratic on each element and globally continuous. The error: $\|u-u_h\|_{H^1} \leq Ch^2\|u'''\|_{L^2}$ and $\|u-u_h\|_{L^2} \leq Ch^3\|u'''\|_{L^2}$.

**General $k$.** Lagrange elements of degree $k$: $k+1$ nodes per element (nodes at $x_{j-1}, x_{j-1/k}, \ldots, x_j$). Error: $O(h^k)$ in $H^1$, $O(h^{k+1})$ in $L^2$.

## Worked Example: $n = 3$, $f = 1$

Mesh: $h = 1/3$, nodes $x_0=0, x_1=1/3, x_2=2/3, x_3=1$. Interior nodes: $x_1, x_2$ ($N=2$).

**Stiffness matrix:** $\mathbf{K} = \frac{1}{h}\begin{pmatrix}2 & -1 \\ -1 & 2\end{pmatrix} = 3\begin{pmatrix}2 & -1 \\ -1 & 2\end{pmatrix}$.

**Load vector:** $F_1 = F_2 = \int_0^1\phi_j\,dx = h = 1/3$ (for $f=1$, since $\int\phi_j = h$).

**Linear system:** $3\begin{pmatrix}2 & -1 \\ -1 & 2\end{pmatrix}\begin{pmatrix}U_1 \\ U_2\end{pmatrix} = \begin{pmatrix}1/3 \\ 1/3\end{pmatrix}$.

**Solution:** By symmetry, $U_1 = U_2 = U$. From either equation: $3(2U - U) = 1/3$, so $3U = 1/3$, $U = 1/9$.

**Exact solution:** $u(x) = x(1-x)/2$. Values: $u(1/3) = (1/3)(2/3)/2 = 1/9$, $u(2/3) = (2/3)(1/3)/2 = 1/9$.

**Perfect agreement at nodes:** $U_1 = u(1/3) = 1/9$ and $U_2 = u(2/3) = 1/9$. The FEM solution is exact at the nodes (superconvergence for this particular problem — the exact solution is quadratic, and piecewise linear interpolation is exact for quadratic functions in the nodal sense).

**Between nodes:** $u_h$ is piecewise linear with $u_h(1/6) = U_1/2 = 1/18$ vs. $u(1/6) = (1/6)(5/6)/2 = 5/72 \approx 0.0694$. The maximum error is $u(1/2) - u_h(1/2) = 1/8 - 1/9 = 1/72 \approx 0.014$, consistent with $O(h^2)$ convergence in $L^2$.
