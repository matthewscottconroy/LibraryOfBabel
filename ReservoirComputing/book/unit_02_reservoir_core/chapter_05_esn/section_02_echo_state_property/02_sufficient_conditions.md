# Section 2.2: Sufficient Conditions for the Echo State Property

## 2.8 Overview

In Section 2.1 we proved that $\|W^{rec}\|_2 < 1$ is sufficient for the ESP. But in practice, ESNs routinely work with $\|W^{rec}\|_2 > 1$ — the typical design rule targets the **spectral radius** $\rho(W^{rec}) \approx 0.9$, which can be much smaller than the spectral norm. In this section we work through a hierarchy of sufficient conditions, from the weakest that is practically useful to the strongest that provides the clearest intuition. Along the way we derive the relationship between the spectral norm and the spectral radius (via the Gelfand formula) and explain why spectral radius $< 1$ is *sufficient but not necessary*.

---

## 2.9 The Spectral Norm Condition

We established in Section 2.1 (equation 2.10) that the ESP holds if:

$$\|W^{rec}\|_2 \equiv \sigma_{max}(W^{rec}) < 1 \tag{2.11}$$

where $\sigma_{max}$ is the largest singular value of $W^{rec}$.

For the leaky integrator ESN, the condition from (2.9) is $\gamma = (1-\alpha) + \alpha \|W^{rec}\|_2 < 1$, which gives:

$$\|W^{rec}\|_2 < 1 \tag{2.11}$$

regardless of $\alpha$. The leaking rate drops out because it only scales the contraction factor but does not change the fundamental requirement on $W^{rec}$.

**Why the spectral norm, not the spectral radius?** The spectral norm $\|W^{rec}\|_2$ controls how much the matrix amplifies vectors in $\ell^2$ norm. For an arbitrary vector $v$, we have $\|W^{rec} v\| \leq \sigma_{max} \|v\|$. The spectral radius $\rho(W^{rec})$ controls amplification only along eigenvectors. For a normal matrix (one where eigenvectors are orthogonal), $\|W^{rec}\|_2 = \rho(W^{rec})$, but for general matrices $\sigma_{max} \geq \rho$, so the spectral norm condition is more conservative.

---

## 2.10 Refining to the Activation Function: The $\tanh'$ Bound

The contraction argument used $|\tanh'(z)| \leq 1$ globally. But this bound is not tight: the derivative of $\tanh$ achieves its maximum value of $1$ only at $z = 0$, and $|\tanh'(z)| \to 0$ as $|z| \to \infty$. We can sharpen the argument by bounding $|\tanh'|$ locally.

**Jacobian approach.** The map $F(x; u) = (1-\alpha)x + \alpha \tanh(W^{rec} x + c)$ has Jacobian:

$$J_F(x; u) = (1-\alpha)I + \alpha \cdot \text{diag}(\tanh'(W^{rec} x + c)_i) \cdot W^{rec} \tag{2.12}$$

where $\tanh'(z) = 1 - \tanh^2(z)$ and the diagonal matrix contains these values along the diagonal. The map is a local contraction at $x$ if $\|J_F(x; u)\|_2 < 1$.

By the submultiplicativity of matrix norms and the triangle inequality:

$$\|J_F(x; u)\|_2 \leq (1-\alpha) + \alpha \|\text{diag}(\tanh'(\cdot))\|_2 \cdot \|W^{rec}\|_2$$

Now $\|\text{diag}(\tanh'(\cdot))\|_2 = \max_i |\tanh'((W^{rec} x + c)_i)|$. Let:

$$\delta_{max}(x, u) = \max_i \left|1 - \tanh^2\!\left((W^{rec} x + c)_i\right)\right| \leq 1 \tag{2.13}$$

Then:

$$\|J_F(x; u)\|_2 \leq (1-\alpha) + \alpha \cdot \delta_{max}(x,u) \cdot \|W^{rec}\|_2 \tag{2.14}$$

If the reservoir is operating in a regime where the activations are large (neurons are saturated), then $\delta_{max} \ll 1$ and the effective contraction is much stronger than the naive bound suggests. This is one reason why the practical rule $\rho < 1$ works: in the saturating regime, the effective Lipschitz constant of $\tanh$ is much less than 1, making up for $\rho$ being close to 1.

**A uniform bound.** Suppose we know that for the input class $\mathcal{U}$ of interest and any reservoir trajectory, the activations are bounded: $|(W^{rec} x + c)_i| \geq a > 0$ for all $i, t$. Then $\delta_{max} \leq \tanh'(a) = 1 - \tanh^2(a) < 1$, and the condition for a contraction becomes:

$$\|W^{rec}\|_2 < \frac{1}{\tanh'(a)} = \frac{1}{1 - \tanh^2(a)} \tag{2.15}$$

This is a weaker (less conservative) condition than (2.11), allowing $\|W^{rec}\|_2 > 1$ while still guaranteeing ESP, as long as the reservoir operates in a sufficiently saturated regime.

---

## 2.11 The Spectral Radius and the Gelfand Formula

For most ESN design purposes, the **spectral radius** $\rho(W^{rec})$ is the key quantity. To understand why, we need the Gelfand formula relating spectral radius to matrix norms.

**Definition 2.4 (Spectral Radius).** For a square matrix $A \in \mathbb{R}^{N \times N}$, the spectral radius is:

$$\rho(A) = \max_{i} |\lambda_i(A)| \tag{2.16}$$

where $\lambda_1, \ldots, \lambda_N$ are the (possibly complex) eigenvalues of $A$.

**Theorem 2.2 (Gelfand Formula).** For any matrix norm $\|\cdot\|$ that is sub-multiplicative ($\|AB\| \leq \|A\|\|B\|$):

$$\rho(A) = \lim_{k \to \infty} \|A^k\|^{1/k} \tag{2.17}$$

This beautiful result says that the spectral radius is the "long-run growth rate" of matrix powers. More precisely:
- If $\rho(A) < 1$: $\|A^k\| \to 0$ exponentially fast, at rate $\rho(A)^k$.
- If $\rho(A) = 1$: $\|A^k\|$ may grow polynomially, stay bounded, or oscillate.
- If $\rho(A) > 1$: $\|A^k\| \to \infty$ exponentially fast.

**Relationship to spectral norm.** We always have $\rho(A) \leq \|A\|_2 = \sigma_{max}(A)$. The gap between them can be large for non-normal matrices. For example, consider:

$$A = \begin{pmatrix} 0 & 10 \\ 0 & 0 \end{pmatrix}$$

Then $\rho(A) = 0$ (both eigenvalues are zero) but $\|A\|_2 = 10$. The spectral norm condition $\|A\|_2 < 1$ would reject this matrix entirely, even though it is nilpotent ($A^2 = 0$) and hence trivially satisfies the ESP.

This example motivates looking for conditions based on $\rho$ rather than $\|W^{rec}\|_2$.

---

## 2.12 Spectral Radius $< 1$ is Sufficient for ESP

**Theorem 2.3.** For the ESN map $F(x; u) = (1-\alpha)x + \alpha \tanh(W^{rec} x + W^{in} u + b)$ with $u$ drawn from a compact input space $\mathcal{U}$, if $\rho(W^{rec}) < 1$, then the ESP holds.

**Proof.** We need to show that $\rho(W^{rec}) < 1$ implies exponential forgetting.

**Step 1: Linearize.** Consider the difference $d_t = x_t - x_t'$ between two trajectories driven by the same input. We have:

$$d_{t+1} = (1-\alpha) d_t + \alpha \left[\tanh(W^{rec} x_t + c_t) - \tanh(W^{rec} x_t' + c_t)\right]$$

By the mean value theorem applied componentwise, there exists a diagonal matrix $D_t$ with $0 \leq (D_t)_{ii} = \tanh'(\xi_{i,t}) \leq 1$ such that:

$$d_{t+1} = (1-\alpha) d_t + \alpha D_t W^{rec} d_t = \left[(1-\alpha) I + \alpha D_t W^{rec}\right] d_t \equiv M_t d_t \tag{2.18}$$

So $d_t = M_{t-1} M_{t-2} \cdots M_0 \, d_0$, and we need to show that the product of matrices $M_t$ goes to zero.

**Step 2: Use the Gelfand formula.** Since $\rho(W^{rec}) < 1$, by the Gelfand formula there exist constants $C > 0$ and $\rho < r < 1$ such that $\|W^{rec,k}\| \leq C r^k$ for all $k \geq 0$.

**Step 3: Bound $M_t$.** For the vanilla ESN ($\alpha = 1$), $M_t = D_t W^{rec}$, and since $\|D_t\|_2 \leq 1$:

$$\|M_t\|_2 \leq \|D_t\|_2 \|W^{rec}\|_2 \leq \|W^{rec}\|_2$$

This alone is not enough if $\|W^{rec}\|_2 > 1$. The key is to use the **long-run behavior** of the product $\prod M_t$.

By a result in [Jaeger2001] (relying on the joint spectral radius theory), when $D_t$ satisfies $0 \leq D_t \leq I$ componentwise and $\rho(W^{rec}) < 1$, the product $M_{t-1} \cdots M_0$ decays to zero exponentially. The argument proceeds by noting that for any $\epsilon > 0$, there exists $k$ such that for a random $D$, the product of $k$ matrices $D_j W^{rec}$ has spectral radius $< 1-\epsilon$, with high probability. Since the $D_t$ are contractive (all entries $\leq 1$), the product's long-run growth rate is bounded by $\rho(W^{rec}) < 1$.

**For the leaky integrator ($0 < \alpha < 1$):** The matrix $M_t = (1-\alpha)I + \alpha D_t W^{rec}$ is a convex combination of the identity and $D_t W^{rec}$. The spectral radius of $M_t$ satisfies:

$$\rho(M_t) \leq (1-\alpha) + \alpha \rho(D_t W^{rec}) \leq (1-\alpha) + \alpha \rho(W^{rec})$$

(using the fact that $D_t \leq I$ implies $\rho(D_t W^{rec}) \leq \rho(W^{rec})$ for non-negative $D_t$). If $\rho(W^{rec}) < 1$, then $\rho(M_t) \leq (1-\alpha) + \alpha \rho(W^{rec}) < 1$, and the product contracts. $\square$

**Important caveat.** The above proof sketch glosses over a subtle point: the spectral radius of a product is not the product of spectral radii in general. The rigorous proof requires the theory of **joint spectral radius** [Rota1960] or, alternatively, the observation that when the $D_t$ are scalar multiples of the identity (e.g., when $\tanh'$ is approximately constant), the argument reduces cleanly to the scalar case. Jaeger's original proof in [Jaeger2001] uses a different approach, establishing the result via the intermediate condition that the **global Lyapunov exponent** is negative.

---

## 2.13 Why $\rho < 1$ is Necessary in the Linear Case

To understand the "not necessary" qualifier, consider the **linear** reservoir (no nonlinearity, or equivalently $f = \text{id}$):

$$x_{t+1} = W^{rec} x_t + W^{in} u_{t+1}$$

In this case, $d_{t+1} = W^{rec} d_t$, so $d_t = (W^{rec})^t d_0$. For $d_t \to 0$ for all $d_0$, we need $(W^{rec})^t \to 0$, which holds if and only if $\rho(W^{rec}) < 1$. In the **linear** case, $\rho < 1$ is both necessary and sufficient.

The nonlinearity changes things. When $\tanh$ saturates, neurons with large activations have derivatives near zero, effectively "disconnecting" part of the recurrent graph. This means the effective connectivity can be much less than the nominal connectivity given by $W^{rec}$, allowing the ESP to hold even when $\rho(W^{rec}) > 1$.

---

## 2.14 A Worked Example: The Gap Between $\rho$ and Spectral Norm

Consider $N = 2$ and:

$$W^{rec} = \begin{pmatrix} 0 & 2 \\ 0 & 0 \end{pmatrix}$$

**Spectral radius:** $\rho(W^{rec}) = 0$ (both eigenvalues are 0).

**Spectral norm:** $\|W^{rec}\|_2 = 2$.

**Naive contraction condition (2.11):** $\|W^{rec}\|_2 = 2 > 1$. Condition fails.

**But:** $(W^{rec})^2 = 0$, so for the linear map $x_{t+1} = W^{rec} x_t$, we have $x_{t+2} = 0$ for any initial condition. The ESN trivially satisfies the ESP.

For the nonlinear map, we have:

$$d_{t+1} = \left[(1-\alpha)I + \alpha D_t W^{rec}\right] d_t$$

with $\|D_t\| \leq 1$. The matrix $M_t = (1-\alpha)I + \alpha D_t W^{rec}$ has the form:

$$M_t = \begin{pmatrix} 1-\alpha & 2\alpha (D_t)_{11} \\ 0 & 1-\alpha \end{pmatrix}$$

Its eigenvalues are both $1-\alpha < 1$. The matrix is upper triangular, and the product $M_{t-1} \cdots M_0$ converges to zero at rate $(1-\alpha)^t$.

This example shows concretely why $\rho < 1$ is sufficient but the spectral norm condition (2.11) is overly conservative.

---

## 2.15 Summary of Sufficient Conditions

We can arrange the conditions from most restrictive to least restrictive:

| Condition | Strength | Comment |
|-----------|----------|---------|
| $\|W^{rec}\|_2 < 1$ | Strong (overcautious) | Guarantees $\gamma$-contraction for all states |
| $\rho(W^{rec}) < 1$ | Moderate (standard) | Necessary and sufficient for the linear case; sufficient for the nonlinear case |
| $\rho(W^{rec}) < 1/\tanh'(a)$ where $|W^{rec} x + c|_i \geq a$ | Weaker | Accounts for saturation; $> 1$ possible |
| ESP empirically tested | Weakest | No guarantee; depends on specific input statistics |

In practice, the rule $\rho(W^{rec}) \lesssim 1$ (often $\approx 0.9$) is used as the design target, and it works well because:
1. It is sufficient in the linear regime.
2. It is approximately sufficient in the saturating regime.
3. It is easy to enforce: just scale $W^{rec}$ so that $\rho(W^{rec}/\rho_0 \cdot \rho_{target}) = \rho_{target}$.

The next section takes a detailed look at the spectral radius itself — how to compute it, what it means physically, and why $\rho \approx 1$ is optimal.
