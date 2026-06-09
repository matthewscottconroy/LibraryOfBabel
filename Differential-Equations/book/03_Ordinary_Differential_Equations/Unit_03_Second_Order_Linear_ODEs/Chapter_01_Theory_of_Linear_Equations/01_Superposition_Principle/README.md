# The Superposition Principle

The superposition principle is the cornerstone of the theory of linear differential equations. It states that for a linear homogeneous equation, any linear combination of solutions is again a solution. This seemingly simple fact has profound consequences: it equips the solution set with the structure of a vector space, enabling the tools of linear algebra to be applied to the analysis of differential equations.

## The Principle for Homogeneous Equations

Consider the second-order linear homogeneous equation

$$L[y] = y'' + p(x)\,y' + q(x)\,y = 0,$$

where $p$ and $q$ are continuous on an open interval $I$.

**Theorem (Superposition).** If $y_1$ and $y_2$ are any two solutions of $L[y] = 0$ on $I$, then for any constants $c_1, c_2 \in \mathbb{R}$, the function $y = c_1 y_1 + c_2 y_2$ is also a solution on $I$.

**Proof.** Compute directly:

$$L[c_1 y_1 + c_2 y_2] = (c_1 y_1 + c_2 y_2)'' + p(c_1 y_1 + c_2 y_2)' + q(c_1 y_1 + c_2 y_2)$$
$$= c_1(y_1'' + py_1' + qy_1) + c_2(y_2'' + py_2' + qy_2) = c_1 L[y_1] + c_2 L[y_2] = c_1 \cdot 0 + c_2 \cdot 0 = 0.$$

The computation works because $L$ is a **linear operator**: it distributes over addition and commutes with scalar multiplication.

## The Solution Space as a Vector Space

The set of all solutions of $L[y] = 0$ on $I$ forms a vector space, denoted $\ker(L)$ (the kernel of the operator $L$). The superposition principle is precisely the statement that $\ker(L)$ is closed under addition and scalar multiplication.

**Theorem.** The solution space $\ker(L)$ of a second-order linear homogeneous ODE is a vector space of dimension exactly 2.

The dimension-2 claim requires two results. First, one shows that two linearly independent solutions exist: by the existence theorem, for any $x_0 \in I$, there exist unique solutions $y_1$ and $y_2$ satisfying the initial conditions

$$y_1(x_0) = 1,\; y_1'(x_0) = 0 \qquad \text{and} \qquad y_2(x_0) = 0,\; y_2'(x_0) = 1.$$

These two solutions are linearly independent (since their Wronskian $W(x_0) = 1 \cdot 1 - 0 \cdot 0 = 1 \neq 0$). Second, one shows that every solution is a linear combination of $y_1$ and $y_2$: if $y$ is any solution with $y(x_0) = a$ and $y'(x_0) = b$, then $y = ay_1 + by_2$ by uniqueness.

A pair $\{y_1, y_2\}$ of linearly independent solutions is called a **fundamental set of solutions**, and $c_1 y_1 + c_2 y_2$ (with $c_1, c_2$ arbitrary constants) is the **general solution**.

## Superposition for Nonhomogeneous Equations

For the nonhomogeneous equation $L[y] = g(x)$, superposition takes a different form.

**Theorem.** If $y_p$ is any particular solution of $L[y] = g$ and $y_h$ is the general solution of $L[y] = 0$, then $y = y_h + y_p$ is the general solution of $L[y] = g$.

**Proof.** Let $y$ be any solution of $L[y] = g$. Then $L[y - y_p] = L[y] - L[y_p] = g - g = 0$, so $y - y_p$ is in $\ker(L)$. Therefore $y - y_p = c_1 y_1 + c_2 y_2$ for some constants, giving $y = c_1 y_1 + c_2 y_2 + y_p$.

**Theorem (Superposition of Forcings).** If $y_{p1}$ satisfies $L[y] = g_1$ and $y_{p2}$ satisfies $L[y] = g_2$, then $\alpha_1 y_{p1} + \alpha_2 y_{p2}$ satisfies $L[y] = \alpha_1 g_1 + \alpha_2 g_2$.

This allows complex forcing functions to be decomposed into simpler pieces, each handled separately. If $g(x) = g_1(x) + g_2(x)$, one finds $y_{p1}$ and $y_{p2}$ separately and adds them. This principle is used constantly in Chapter 3.

## Why Nonlinear Equations Lack Superposition

For a nonlinear equation like $y'' + y^2 = 0$, the sum of two solutions is generally not a solution. If $y_1$ and $y_2$ both satisfy the equation:

$$(y_1 + y_2)'' + (y_1 + y_2)^2 = y_1'' + y_2'' + y_1^2 + 2y_1 y_2 + y_2^2 = -y_1^2 - y_2^2 + y_1^2 + 2y_1 y_2 + y_2^2 = 2y_1 y_2,$$

which is zero only when $y_1 y_2 = 0$. The cross term $2y_1 y_2$ is the signature of nonlinearity and is the obstruction to superposition.

This contrast makes precise the sense in which linear equations are fundamentally simpler than nonlinear ones: linearity gives the solution set a global algebraic structure (vector space) that nonlinearity destroys.
