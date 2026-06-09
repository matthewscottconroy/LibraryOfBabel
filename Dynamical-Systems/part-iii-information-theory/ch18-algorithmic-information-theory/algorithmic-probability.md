# 18.4 Algorithmic Probability

The Kraft inequality $\sum_x 2^{-K(x)} \leq 1$ says that the numbers $\{2^{-K(x)}\}$ behave like probabilities. In fact, they define a probability measure — and this measure turns out to be the most natural prior over all computable sequences.

**Definition 18.4.1 (Solomonoff's Universal Distribution).** The *algorithmic probability* of a string $x$ is:
$$\mathbf{m}(x) = \sum_{p : U(p) = x} 2^{-|p|}.$$

Rather than taking only the shortest program (as in $K$), algorithmic probability sums over *all* programs that produce $x$, weighted by $2^{-|p|}$. This is the analogue of assigning prior probability $2^{-|p|}$ to each program — shorter programs are exponentially more probable — and marginalizing over all programs that produce the given output.

By the Kraft inequality, $\sum_x \mathbf{m}(x) \leq 1$, so $\mathbf{m}$ is indeed a probability distribution (or a semi-measure if the sum is strictly less than 1).

The remarkable property of $\mathbf{m}$ is its universality:

**Theorem 18.4.2 (Universality of $\mathbf{m}$).** For any computable probability measure $\mu$ on $\{0,1\}^*$, there exists $c > 0$ such that $\mathbf{m}(x) \geq c \cdot \mu(x)$ for all $x$.

In other words, $\mathbf{m}$ *dominates* all computable probability measures — up to a constant factor. No computable prior assigns more probability to any string than $\mathbf{m}$ does, modulo a distribution-dependent constant.

This makes $\mathbf{m}$ the universal Bayesian prior over all computable hypotheses. The connection to $K$:

**Connection to $K$:** $K(x) = -\log \mathbf{m}(x) + O(1)$. The Kolmogorov complexity is (essentially) the negative log of the universal prior.

This is striking: complexity and probability are two sides of the same coin. A string with low complexity (short description) has high algorithmic probability — many programs produce it. A high-complexity string is algorithmically improbable — very few short programs produce it.

**Solomonoff Induction:** Given a sequence $x_1 x_2 \cdots x_n$, the best prediction for $x_{n+1}$ — in the sense of minimizing expected log-loss over all possible continuations — is $\mathbf{m}(x_{n+1} \mid x_1 \cdots x_n)$. This is Solomonoff's theory of inductive inference: the optimal Bayesian prediction over all computable hypotheses.

Solomonoff induction is theoretically optimal but practically uncomputable (since $K$ is uncomputable). It serves as the gold standard against which all practical prediction algorithms are measured. The celebrated *minimum description length* (MDL) principle in statistics is a computable approximation to Solomonoff induction: choose the hypothesis that minimizes description length (model code + data code), which approximates maximizing $\mathbf{m}(data | model)$.
