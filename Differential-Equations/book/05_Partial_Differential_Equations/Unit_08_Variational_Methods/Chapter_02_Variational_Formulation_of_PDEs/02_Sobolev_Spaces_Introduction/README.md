# Introduction to Sobolev Spaces

Sobolev spaces are the function spaces in which weak solutions of elliptic PDEs naturally live. The basic Sobolev space $H^1(\Omega) = W^{1,2}(\Omega)$ consists of all $L^2(\Omega)$ functions whose first-order weak partial derivatives are also in $L^2(\Omega)$. These spaces are Hilbert spaces with norms that control both the function and its derivatives, and they encode exactly the regularity needed for the bilinear form of a second-order elliptic PDE to be well-defined. This section develops the definition, basic properties, key inequalities (Poincaré, Sobolev embedding), and trace theory for Sobolev spaces.

## Definition

Let $\Omega\subset\mathbb{R}^n$ be an open set and $k\geq 0$, $p\geq 1$.

**Definition.** The **Sobolev space** $W^{k,p}(\Omega)$ consists of all locally integrable functions $u:\Omega\to\mathbb{R}$ such that for every multi-index $\alpha$ with $|\alpha|\leq k$, the weak partial derivative $D^\alpha u$ exists and belongs to $L^p(\Omega)$:

$$W^{k,p}(\Omega) = \{u\in L^p(\Omega): D^\alpha u\in L^p(\Omega) \text{ for all }|\alpha|\leq k\}.$$

**Norm:** $\|u\|_{W^{k,p}}^p = \sum_{|\alpha|\leq k}\|D^\alpha u\|_{L^p(\Omega)}^p$ (for $1\leq p < \infty$).

**Hilbert spaces.** For $p=2$: $H^k(\Omega) = W^{k,2}(\Omega)$ with inner product $\langle u,v\rangle_{H^k} = \sum_{|\alpha|\leq k}\int_\Omega D^\alpha u\, D^\alpha v\,dx$. This makes $H^k(\Omega)$ a Hilbert space.

**Completeness.** $W^{k,p}(\Omega)$ is complete (Banach space) for all $k\geq 0$ and $1\leq p\leq\infty$. For $p=2$, it is a Hilbert space. Proof: every Cauchy sequence in $W^{k,p}$ is Cauchy in $L^p$ (for each derivative), hence convergent by $L^p$ completeness; the limit has the required weak derivatives.

## Examples

**$H^1(\mathbb{R})$:**
- $u(x) = e^{-|x|}\in H^1(\mathbb{R})$ (with $u'(x) = -\text{sgn}(x)e^{-|x|}\in L^2$).
- $u(x) = |x|^{1/2}\in L^2_{\text{loc}}(\mathbb{R})$ but $u'(x) = \frac{1}{2}|x|^{-1/2}\notin L^2((-1,1))$ (not in $H^1((-1,1))$).

**$H^1(\Omega)$ in $\mathbb{R}^n$:**
- In dimension $n\leq 3$: $H^1(\Omega)\subset L^6(\Omega)$ (Sobolev embedding for $p=2$, $k=1$, $n=3$: $p^* = 2n/(n-2) = 6$).
- In dimension $n=2$: $H^1(\Omega)\subset L^q(\Omega)$ for all $q < \infty$ (but not $L^\infty$).
- Corner singularity: the function $u(r,\theta) = r^{\pi/\omega}\sin(\pi\theta/\omega)$ on a wedge of angle $\omega$ is in $H^1$ but not in $H^2$ when $\omega > \pi$.

## The Space $H^1_0(\Omega)$

**Definition.** $H^1_0(\Omega)$ is the closure of $C_c^\infty(\Omega)$ in $H^1(\Omega)$:

$$H^1_0(\Omega) = \overline{C_c^\infty(\Omega)}^{H^1(\Omega)}.$$

Equivalently (for Lipschitz $\Omega$): $H^1_0(\Omega) = \{u\in H^1(\Omega): u|_{\partial\Omega} = 0\}$ (in the trace sense).

**Physical interpretation.** $H^1_0(\Omega)$ is the correct function space for problems with homogeneous Dirichlet boundary conditions: functions that are $L^2$, have $L^2$ gradients, and vanish on the boundary.

## The Poincaré Inequality

**Theorem (Poincaré inequality).** Let $\Omega\subset\mathbb{R}^n$ be bounded in at least one direction. Then there exists a constant $C_P = C_P(\Omega) > 0$ such that for all $u\in H^1_0(\Omega)$:

$$\|u\|_{L^2(\Omega)} \leq C_P\|\nabla u\|_{L^2(\Omega)}.$$

**Proof (for $\Omega\subset[0,L]\times\mathbb{R}^{n-1}$).** For $u\in C_c^\infty(\Omega)$:

$$u(x) = u(x) - u(0,x') = \int_0^{x_1}\frac{\partial u}{\partial x_1}(t,x')\,dt$$

(since $u(0,x') = 0$ for smooth $u$ vanishing at the boundary). By Cauchy-Schwarz:

$$|u(x)|^2 \leq x_1\int_0^{x_1}\left|\frac{\partial u}{\partial x_1}\right|^2\,dt \leq L\int_0^L\left|\frac{\partial u}{\partial x_1}\right|^2\,dt.$$

Integrating over $\Omega$: $\|u\|_{L^2}^2 \leq L^2\|\partial_{x_1}u\|_{L^2}^2 \leq L^2\|\nabla u\|_{L^2}^2$. So $C_P = L$. By density, the inequality extends to all $u\in H^1_0(\Omega)$. $\square$

**Consequence.** The Poincaré inequality shows that $\|\nabla u\|_{L^2}$ is equivalent to the full $H^1$ norm on $H^1_0(\Omega)$: $\|\nabla u\|_{L^2}^2 \leq \|u\|_{H^1}^2 = \|u\|_{L^2}^2 + \|\nabla u\|_{L^2}^2 \leq (1+C_P^2)\|\nabla u\|_{L^2}^2$. This is why the Dirichlet energy provides a coercive bilinear form on $H^1_0(\Omega)$.

## Sobolev Embedding Theorem

**Theorem (Sobolev embedding).** Let $\Omega\subset\mathbb{R}^n$ be a bounded Lipschitz domain.

1. If $k < n/p$: $W^{k,p}(\Omega)\hookrightarrow L^{p^*}(\Omega)$ continuously, where $p^* = np/(n-kp)$ (Sobolev exponent). Moreover, this embedding is compact into $L^q$ for all $q < p^*$.

2. If $k = n/p$: $W^{k,p}(\Omega)\hookrightarrow L^q(\Omega)$ for all $q < \infty$.

3. If $k > n/p$: $W^{k,p}(\Omega)\hookrightarrow C^{k-\lfloor n/p\rfloor-1,\alpha}(\bar\Omega)$ (Hölder continuous functions), where $\alpha = \lfloor n/p\rfloor + 1 - n/p$.

**Key cases for $p=2$:**
- $n=1$: $H^1((a,b))\hookrightarrow C^{0,1/2}([a,b])$ (Lipschitz continuous) since $k=1 > n/p = 1/2$.
- $n=2$: $H^1(\Omega)\hookrightarrow L^q(\Omega)$ for all $q < \infty$, but not $L^\infty$.
- $n=3$: $H^1(\Omega)\hookrightarrow L^6(\Omega)$ ($p^* = 6$); $H^2(\Omega)\hookrightarrow C^{0,1/2}(\bar\Omega)$.

**Physical significance.** The Sobolev embedding controls the size of nonlinear terms. For the semilinear equation $-\Delta u = u^p$ in $\Omega\subset\mathbb{R}^3$: if $u\in H^1_0(\Omega)$, then $u\in L^6(\Omega)$, so $u^p\in L^{6/p}(\Omega)$. For $p \leq 5$ ($= 6-1$), $u^p\in L^{6/5}(\Omega)\subset (H^1_0)^*$, making the nonlinear term a bounded functional on $H^1_0$ — the critical Sobolev exponent $p_c = 5$ ($= (n+2)/(n-2)$ for $n=3$) emerges naturally.

## Trace Theorem

**Theorem (Trace).** Let $\Omega\subset\mathbb{R}^n$ be a bounded Lipschitz domain. There exists a bounded linear operator $T:H^1(\Omega)\to L^2(\partial\Omega)$ (the trace operator) such that $Tu = u|_{\partial\Omega}$ for all $u\in C(\bar\Omega)\cap H^1(\Omega)$. Moreover:

$$\|Tu\|_{L^2(\partial\Omega)} \leq C\|u\|_{H^1(\Omega)}.$$

More precisely, $T$ maps $H^1(\Omega)$ onto $H^{1/2}(\partial\Omega)$ (a fractional Sobolev space on the boundary).

**Significance.** The trace theorem allows boundary conditions to be imposed on $H^1$ functions, which are defined only almost everywhere. The condition $u = g$ on $\partial\Omega$ for a weak solution is interpreted as $Tu = g$ in $L^2(\partial\Omega)$ (or $H^{1/2}(\partial\Omega)$).

The null space of $T$ is exactly $H^1_0(\Omega)$: $T^{-1}(0) = H^1_0(\Omega)$. This confirms that $H^1_0(\Omega)$ is the right space for homogeneous Dirichlet conditions.

## Dual Space and Negative Sobolev Spaces

The dual of $H^1_0(\Omega)$ is denoted $H^{-1}(\Omega)$:

$$H^{-1}(\Omega) = (H^1_0(\Omega))^* = \{F: H^1_0(\Omega)\to\mathbb{R}: F \text{ bounded linear}\}.$$

By the Riesz representation theorem, every $F\in H^{-1}(\Omega)$ has the form $F(v) = \langle f_0, v\rangle_{H^{-1},H^1_0}$ for some $f_0\in H^{-1}(\Omega)$.

**Examples of $H^{-1}$ functionals:**
- $F(v) = \int fv\,dx$ for $f\in L^2(\Omega)$: bounded since $|F(v)| \leq \|f\|_{L^2}\|v\|_{L^2} \leq C_P\|f\|_{L^2}\|\nabla v\|_{L^2}$.
- $F(v) = \int_\Omega g\cdot\nabla v\,dx$ for $g\in L^2(\Omega;\mathbb{R}^n)$: bounded by Cauchy-Schwarz.
- $F(v) = v(x_0)$ for $n=1$: bounded (since $H^1\hookrightarrow C^{0,1/2}$ in 1D).
- $F(v) = v(x_0)$ for $n\geq 2$: NOT bounded (point evaluation is not continuous in $H^1$ for $n\geq 2$).

## Rellich-Kondrachov Compactness

**Theorem.** For $q < p^*$ (strictly less than the Sobolev exponent), the embedding $W^{k,p}(\Omega)\hookrightarrow L^q(\Omega)$ is **compact**: every bounded sequence in $W^{k,p}$ has a convergent subsequence in $L^q$.

This compactness is essential for the direct method of the calculus of variations: to extract a convergent subsequence from the minimizing sequence. It is also used to prove that the spectrum of $-\Delta$ on a bounded domain is discrete (compact resolvent implies discrete spectrum).
