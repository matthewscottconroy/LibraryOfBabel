# Superposition and the Wronskian for Systems

The superposition principle for the homogeneous system $\mathbf{x}' = A(t)\mathbf{x}$ states: if $\mathbf{x}_1, \ldots, \mathbf{x}_k$ are solutions, then $c_1\mathbf{x}_1 + \cdots + c_k\mathbf{x}_k$ is also a solution for any constants $c_1, \ldots, c_k$. The proof is identical to the scalar case: $L$ is linear.

## The Wronskian

For $n$ solutions $\mathbf{x}_1, \ldots, \mathbf{x}_n$ of the $n \times n$ system, the **Wronskian** is

$$W(t) = \det\Phi(t), \qquad \Phi(t) = [\mathbf{x}_1(t) \mid \cdots \mid \mathbf{x}_n(t)].$$

**Liouville's Formula (Abel's Identity for Systems):**

$$W(t) = W(t_0)\exp\!\left(\int_{t_0}^t \mathrm{tr}\,A(s)\,ds\right).$$

**Proof.** Let $\mathbf{x}_k = (x_{k1}, \ldots, x_{kn})^T$. The derivative of the determinant is a sum of $n$ determinants, each obtained by differentiating one row of $\Phi$ while keeping the others fixed:

$$W'(t) = \sum_{j=1}^n \det[\mathbf{x}_1 \mid \cdots \mid \mathbf{x}_j' \mid \cdots \mid \mathbf{x}_n].$$

Since $\mathbf{x}_j' = A\mathbf{x}_j$, the column $\mathbf{x}_j'$ equals $\sum_k a_{kj}$ ... more precisely, computing explicitly for $2 \times 2$ gives $W' = \mathrm{tr}(A)W$, and this extends to $n \times n$ by the multilinearity of the determinant. Solving $W' = \mathrm{tr}(A)\cdot W$ gives Liouville's formula.

## Linear Independence

$n$ solutions are linearly independent if and only if $W(t_0) \neq 0$ for some (hence any) $t_0$. By Liouville's formula, $W$ is either identically zero or never zero.

## The Solution Space is $n$-Dimensional

For any $t_0$, the $n$ solutions satisfying the standard initial conditions $\mathbf{x}_k(t_0) = \mathbf{e}_k$ (standard basis vectors) are linearly independent (their $W(t_0) = \det I = 1$) and span the solution space. Every solution $\mathbf{x}$ with $\mathbf{x}(t_0) = \mathbf{x}_0$ equals $\sum x_{0k}\mathbf{x}_k$ by uniqueness. Hence the solution space has dimension exactly $n$.
