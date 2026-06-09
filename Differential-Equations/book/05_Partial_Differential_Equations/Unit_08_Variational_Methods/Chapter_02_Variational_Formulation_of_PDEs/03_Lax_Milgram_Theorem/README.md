# The Lax-Milgram Theorem

The Lax-Milgram theorem is the fundamental existence and uniqueness result for weak solutions of elliptic PDEs. It says: any continuous coercive bilinear form on a Hilbert space is invertible. This abstract result, when applied to the bilinear form $a(u,v) = \int_\Omega\nabla u\cdot\nabla v\,dx$ (or more general elliptic forms) on the Hilbert space $H = H^1_0(\Omega)$, immediately gives existence and uniqueness of the weak solution to the Dirichlet problem for a large class of linear elliptic PDEs. The proof is constructive and provides an explicit bound on the solution in terms of the data.

## The Abstract Theorem

**Theorem (Lax-Milgram, 1954).** Let $H$ be a real Hilbert space with inner product $\langle\cdot,\cdot\rangle$ and norm $\|\cdot\|$. Suppose:

1. $a:H\times H\to\mathbb{R}$ is a **bilinear form** (linear in each argument).
2. $a$ is **continuous (bounded):** There exists $M > 0$ such that $|a(u,v)| \leq M\|u\|\,\|v\|$ for all $u,v\in H$.
3. $a$ is **coercive:** There exists $\alpha > 0$ such that $a(u,u) \geq \alpha\|u\|^2$ for all $u\in H$.
4. $F:H\to\mathbb{R}$ is a **bounded linear functional:** $|F(v)| \leq \Lambda\|v\|$ for all $v\in H$.

Then there exists a **unique** $u\in H$ such that:

$$a(u,v) = F(v) \quad \text{for all }v\in H. \tag{Lax-Milgram}$$

Moreover, $\|u\| \leq \Lambda/\alpha = \|F\|_{H^*}/\alpha$.

**Note on symmetry.** The Lax-Milgram theorem does not require $a$ to be symmetric. If $a$ is symmetric, the problem reduces to minimizing $\mathcal{E}[u] = \frac{1}{2}a(u,u) - F(u)$ (the unique minimizer satisfies the Euler-Lagrange equation $a(u,v) = F(v)$). For non-symmetric $a$, no such minimization interpretation exists, but the theorem still applies.

## Proof

**Step 1: Reduce to a fixed point problem.** For each fixed $u\in H$, the map $v\mapsto a(u,v)$ is a bounded linear functional: $|a(u,v)|\leq M\|u\|\|v\|$. By the Riesz representation theorem, there exists a unique $Au\in H$ with $a(u,v) = \langle Au,v\rangle$ for all $v\in H$, and $\|Au\| \leq M\|u\|$. The map $A:H\to H$ is a bounded linear operator.

Similarly, $F(v) = \langle f,v\rangle$ for a unique $f\in H$ (Riesz representation of $F$), with $\|f\| \leq \Lambda$.

The equation $a(u,v) = F(v)$ for all $v$ becomes $\langle Au,v\rangle = \langle f,v\rangle$ for all $v$, i.e., $Au = f$.

**Step 2: Banach contraction.** We solve $Au = f$ by fixed-point iteration. Consider the map $T_\rho:H\to H$ defined by $T_\rho u = u - \rho(Au - f)$ for some $\rho > 0$. Then $u$ solves $Au = f$ iff $u = T_\rho u$.

**Estimate:** $\|T_\rho u - T_\rho w\|^2 = \|(u-w) - \rho A(u-w)\|^2 = \|u-w\|^2 - 2\rho\langle A(u-w),(u-w)\rangle + \rho^2\|A(u-w)\|^2$.

Using $\langle A(u-w),(u-w)\rangle = a(u-w,u-w) \geq \alpha\|u-w\|^2$ and $\|A(u-w)\| \leq M\|u-w\|$:

$$\|T_\rho u - T_\rho w\|^2 \leq (1-2\rho\alpha + \rho^2 M^2)\|u-w\|^2.$$

Choose $\rho = \alpha/M^2$: then $1 - 2\rho\alpha + \rho^2M^2 = 1 - 2\alpha^2/M^2 + \alpha^2/M^2 = 1 - \alpha^2/M^2 =: \beta^2 < 1$.

So $\|T_\rho u - T_\rho w\| \leq \beta\|u-w\|$ — $T_\rho$ is a contraction. By the **Banach fixed-point theorem**, $T_\rho$ has a unique fixed point $u^* = T_\rho u^* = u^* - \rho(Au^*-f)$, i.e., $Au^* = f$.

**Bound:** $\|u^*\| = \|A^{-1}f\| \leq \|f\|/\alpha = \Lambda/\alpha$. (From $a(u^*,u^*) = F(u^*) \leq \Lambda\|u^*\|$ and $a(u^*,u^*) \geq \alpha\|u^*\|^2$.) $\square$

## Application to Poisson's Equation

**Problem.** Find $u\in H^1_0(\Omega)$ with $-\Delta u = f$ weakly, i.e., $\int_\Omega\nabla u\cdot\nabla v\,dx = \int_\Omega fv\,dx$ for all $v\in H^1_0(\Omega)$.

**Verification of hypotheses.** Take $H = H^1_0(\Omega)$ with norm $\|\nabla\cdot\|_{L^2}$ (Dirichlet norm, equivalent to $H^1$ norm by Poincaré):

- $a(u,v) = \int_\Omega\nabla u\cdot\nabla v\,dx$: bilinear. ✓
- **Boundedness:** $|a(u,v)| \leq \|\nabla u\|_{L^2}\|\nabla v\|_{L^2}$ (Cauchy-Schwarz), so $M = 1$. ✓
- **Coercivity:** $a(u,u) = \|\nabla u\|_{L^2}^2 = \|u\|_H^2$, so $\alpha = 1$. ✓
- $F(v) = \int_\Omega fv\,dx$: linear. $|F(v)| \leq \|f\|_{L^2}\|v\|_{L^2} \leq C_P\|f\|_{L^2}\|\nabla v\|_{L^2} = C_P\|f\|_{L^2}\|v\|_H$. Bounded with $\Lambda = C_P\|f\|_{L^2}$. ✓

**Conclusion.** By Lax-Milgram, there exists a unique $u\in H^1_0(\Omega)$ solving the weak Poisson problem, with $\|\nabla u\|_{L^2} \leq C_P\|f\|_{L^2}$.

## General Elliptic Equations

The Lax-Milgram theorem applies to general second-order linear elliptic PDEs:

$$-\sum_{i,j=1}^n\frac{\partial}{\partial x_i}\!\left(a_{ij}(x)\frac{\partial u}{\partial x_j}\right) + c(x)u = f \quad \text{in }\Omega, \qquad u = 0 \text{ on }\partial\Omega.$$

**Bilinear form:** $a(u,v) = \int_\Omega\left[\sum_{i,j}a_{ij}(x)\partial_i u\,\partial_j v + c(x)uv\right]dx$.

**Hypotheses:**
- **Uniform ellipticity:** $\sum_{i,j}a_{ij}(x)\xi_i\xi_j \geq \lambda|\xi|^2$ for all $\xi\in\mathbb{R}^n$, some $\lambda > 0$.
- **Boundedness of coefficients:** $a_{ij}, c\in L^\infty(\Omega)$.
- **Non-negativity of $c$:** $c(x) \geq 0$ a.e.

**Boundedness of $a$:** $|a(u,v)| \leq (\|a_{ij}\|_\infty + \|c\|_\infty)(1 + C_P^2)\|\nabla u\|_{L^2}\|\nabla v\|_{L^2}$.

**Coercivity of $a$:** $a(u,u) \geq \lambda\|\nabla u\|_{L^2}^2 \geq \frac{\lambda}{1+C_P^2}\|u\|_{H^1}^2$.

By Lax-Milgram: unique weak solution in $H^1_0(\Omega)$.

## The Non-Coercive Case: Gårding's Inequality

For $c(x) \leq 0$ (not sign-definite), coercivity can fail. However, a **Gårding inequality** often holds: there exist $\alpha > 0$ and $\beta \geq 0$ such that $a(u,u) \geq \alpha\|u\|_{H^1}^2 - \beta\|u\|_{L^2}^2$. This gives coercivity on $H = H^1_0(\Omega)$ modulo the $L^2$ norm, and existence follows via the Fredholm alternative: either the homogeneous problem $a(u,v) = 0$ for all $v$ has only the trivial solution (and the non-homogeneous problem has a unique solution), or the homogeneous problem has a non-trivial solution (corresponding to an eigenvalue).

## Worked Example: Anisotropic Diffusion

**Problem.** In $\Omega = (0,1)^2$, find $u\in H^1_0(\Omega)$ satisfying:

$$-\frac{\partial}{\partial x}(a(x,y)u_x) - \frac{\partial}{\partial y}(b(x,y)u_y) = f \quad \text{in }\Omega,$$

where $a,b \in L^\infty(\Omega)$ with $0 < \lambda \leq a,b \leq \Lambda$.

**Bilinear form:** $\tilde{a}(u,v) = \int_\Omega[a\,u_x v_x + b\,u_y v_y]\,dx\,dy$.

**Boundedness:** $|\tilde{a}(u,v)| \leq \Lambda(\|u_x\|_{L^2}\|v_x\|_{L^2} + \|u_y\|_{L^2}\|v_y\|_{L^2}) \leq 2\Lambda\|\nabla u\|_{L^2}\|\nabla v\|_{L^2}$.

**Coercivity:** $\tilde{a}(u,u) = \int au_x^2 + bu_y^2 \geq \lambda\int(u_x^2 + u_y^2) = \lambda\|\nabla u\|_{L^2}^2 = \lambda\|u\|_H^2$.

**Conclusion:** By Lax-Milgram, there exists a unique $u\in H^1_0(\Omega)$ with $\|\nabla u\|_{L^2} \leq C_P\|f\|_{L^2}/\lambda$. The anisotropy ($a \neq b$) is handled without difficulty — the theorem cares only about the lower bound $\lambda$, not the specific structure of the coefficients.

## From Abstract to Concrete: the Stiffness Matrix

In the finite element method, the abstract equation $a(u_h, v_h) = F(v_h)$ for all $v_h\in V_h\subset H$ becomes a linear system when $\{e_1,\ldots,e_N\}$ is a basis for $V_h$:

$$\sum_{j=1}^N u_j a(e_j, e_i) = F(e_i), \quad i=1,\ldots,N,$$

or in matrix form: $\mathbf{K}\mathbf{u} = \mathbf{f}$, where $K_{ij} = a(e_j,e_i)$ (stiffness matrix) and $f_i = F(e_i)$ (load vector). The coercivity of $a$ implies $\mathbf{K}$ is positive definite (or positive semidefinite if $a$ is not strictly coercive), and the Lax-Milgram bound gives $\|\mathbf{u}\| \leq \|\mathbf{f}\|/\alpha$ — the linear system is well-conditioned with condition number $\leq M/\alpha$.
