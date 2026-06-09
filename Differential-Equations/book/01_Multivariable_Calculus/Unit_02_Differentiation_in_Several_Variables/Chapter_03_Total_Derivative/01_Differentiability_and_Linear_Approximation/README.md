# Differentiability and Linear Approximation

In single-variable calculus, $f'(a)$ is defined as the limit of the difference quotient, and its geometric meaning is the slope of the tangent line. The tangent line is also the best linear approximation to $f$ near $a$: $f(a+h) \approx f(a) + f'(a)h$, with error that is $o(h)$ (smaller than any multiple of $h$ as $h\to 0$). This second interpretation — differentiability as the existence of a best linear approximation — is the one that generalizes cleanly to several variables. In $\mathbb{R}^n$, "slope" has no meaning (it's direction-dependent), but "best linear approximation" does.

## The Definition of Differentiability

A function $f: D \subseteq \mathbb{R}^n \to \mathbb{R}^m$ is **differentiable at $\mathbf{a} \in \text{int}(D)$** if there exists a linear map $L: \mathbb{R}^n \to \mathbb{R}^m$ such that

$$\lim_{\mathbf{h}\to\mathbf{0}} \frac{\|f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a}) - L(\mathbf{h})\|}{\|\mathbf{h}\|} = 0.$$

The linear map $L$, when it exists, is unique and is called the **total derivative** or **Fréchet derivative** of $f$ at $\mathbf{a}$, denoted $Df(\mathbf{a})$ or $f'(\mathbf{a})$.

The condition can be rewritten: $f(\mathbf{a}+\mathbf{h}) = f(\mathbf{a}) + L(\mathbf{h}) + \mathbf{r}(\mathbf{h})$ where the remainder satisfies $\|\mathbf{r}(\mathbf{h})\| = o(\|\mathbf{h}\|)$ (i.e., $\|\mathbf{r}(\mathbf{h})\|/\|\mathbf{h}\|\to 0$ as $\mathbf{h}\to\mathbf{0}$). The linear function $\mathbf{a}+\mathbf{h} \mapsto f(\mathbf{a}) + L(\mathbf{h})$ is the best affine approximation to $f$ near $\mathbf{a}$.

## Uniqueness of the Total Derivative

If $L_1$ and $L_2$ both satisfy the definition, then for any fixed nonzero $\mathbf{v}\in\mathbb{R}^n$:

$\frac{\|(L_1-L_2)(t\mathbf{v})\|}{t\|\mathbf{v}\|} = \frac{\|(L_1-L_2)(\mathbf{v})\|}{\|\mathbf{v}\|} \to 0$ as $t\to 0$, which forces $(L_1-L_2)(\mathbf{v}) = \mathbf{0}$. Since $\mathbf{v}$ was arbitrary, $L_1 = L_2$.

## Differentiability Implies Continuity

**Theorem.** If $f$ is differentiable at $\mathbf{a}$, then $f$ is continuous at $\mathbf{a}$.

**Proof.** $\|f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a})\| \leq \|L(\mathbf{h})\| + \|\mathbf{r}(\mathbf{h})\| \leq \|L\|\|\mathbf{h}\| + \|\mathbf{r}(\mathbf{h})\| \to 0$ as $\mathbf{h}\to\mathbf{0}$, using the fact that linear maps are bounded and $\mathbf{r}(\mathbf{h}) = o(\|\mathbf{h}\|)$.

## Partial Derivatives vs. Differentiability

Having all partial derivatives at a point does not imply differentiability. A famous counterexample:

$$f(x,y) = \begin{cases} \frac{xy}{x^2+y^2} & (x,y)\neq(0,0) \\ 0 & (x,y)=(0,0) \end{cases}.$$

$f_x(0,0) = \lim_{h\to 0} f(h,0)/h = 0$ and $f_y(0,0) = 0$. But as shown earlier, $\lim_{(x,y)\to(0,0)} f(x,y)$ does not exist, so $f$ is not continuous at $(0,0)$, hence not differentiable there. Partial derivatives exist, yet differentiability fails.

**The gap:** Partial derivatives test only the behavior along the coordinate axes. Differentiability tests behavior in all directions simultaneously.

## Sufficient Condition: Continuous Partial Derivatives

**Theorem.** If $f: D\subseteq\mathbb{R}^n\to\mathbb{R}^m$ has partial derivatives $\partial f_i/\partial x_j$ on an open set $U\subseteq D$, and all these partial derivatives are continuous at $\mathbf{a}\in U$, then $f$ is differentiable at $\mathbf{a}$.

**Proof sketch (for $f:\mathbb{R}^2\to\mathbb{R}$).** Write $f(a+h, b+k) - f(a,b) = [f(a+h,b+k)-f(a,b+k)] + [f(a,b+k)-f(a,b)]$. Apply the single-variable mean value theorem to each bracket (holding the other variable fixed):

$= h\,f_x(\xi, b+k) + k\,f_y(a, \eta)$

for some $\xi$ between $a$ and $a+h$, some $\eta$ between $b$ and $b+k$.

$= h\,f_x(a,b) + k\,f_y(a,b) + h[f_x(\xi,b+k) - f_x(a,b)] + k[f_y(a,\eta)-f_y(a,b)]$.

The candidate linear map is $L(h,k) = f_x(a,b)h + f_y(a,b)k$. The remainder is $r(h,k) = h[f_x(\xi,b+k)-f_x(a,b)] + k[f_y(a,\eta)-f_y(a,b)]$. Since $|\xi - a| \leq |h| \leq \|(h,k)\|$ and $f_x$ is continuous at $(a,b)$, the quantity $f_x(\xi,b+k)-f_x(a,b)\to 0$ as $(h,k)\to(0,0)$, and similarly for the second term. Thus $|r(h,k)| \leq \|(h,k)\|\cdot[\text{something}\to 0]$, so $r = o(\|(h,k)\|)$.

## The Tangent Plane

For $f:\mathbb{R}^2\to\mathbb{R}$, differentiability at $(a,b)$ means the graph $z = f(x,y)$ has a tangent plane at the point $(a, b, f(a,b))$. The equation of this plane is

$$z = f(a,b) + f_x(a,b)(x-a) + f_y(a,b)(y-b).$$

This is the linear approximation $L$ evaluated at $(x-a, y-b)$, shifted up by $f(a,b)$. The fact that the error $\|f(\mathbf{a}+\mathbf{h})-f(\mathbf{a})-L(\mathbf{h})\| = o(\|\mathbf{h}\|)$ means the surface and the tangent plane become indistinguishable near the point of tangency, to first order.

## Worked Example

$f(x,y) = x^2 + 3xy$. Find the linear approximation at $(1,2)$.

$f(1,2) = 1 + 6 = 7$.
$f_x = 2x + 3y$, $f_x(1,2) = 2 + 6 = 8$.
$f_y = 3x$, $f_y(1,2) = 3$.

Linear approximation: $L(x,y) = 7 + 8(x-1) + 3(y-2) = 7 + 8x - 8 + 3y - 6 = 8x + 3y - 7$.

Check: $f(1.1, 2.1) = (1.1)^2 + 3(1.1)(2.1) = 1.21 + 6.93 = 8.14$.
$L(1.1, 2.1) = 8(1.1) + 3(2.1) - 7 = 8.8 + 6.3 - 7 = 8.1$.
Error: $|8.14 - 8.1| = 0.04$, while $\|(0.1,0.1)\| = \sqrt{0.02} \approx 0.14$. Ratio $\approx 0.28$, which goes to $0$ as the increment shrinks.

## Common Pitfalls

The condition $f(\mathbf{a}+\mathbf{h}) - f(\mathbf{a}) - L(\mathbf{h}) = o(\|\mathbf{h}\|)$ requires the error to go to $0$ faster than $\|\mathbf{h}\|$ in every direction, not just along the axes. Verifying this from the definition is harder than just checking partial derivatives exist.

A function can satisfy $f(\mathbf{a}+t\mathbf{v}) - f(\mathbf{a}) = tL(\mathbf{v}) + o(t)$ for every fixed $\mathbf{v}$ (i.e., all directional derivatives exist and fit a linear formula) but still fail to be differentiable, if the error term $o(t)$ is not uniform in $\mathbf{v}$.
