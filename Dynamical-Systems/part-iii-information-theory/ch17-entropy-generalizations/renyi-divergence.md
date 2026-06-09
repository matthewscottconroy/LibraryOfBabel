# 17.3 Rényi Divergence

Just as Rényi generalized Shannon entropy to a one-parameter family, there is a corresponding generalization of KL divergence. Rényi divergence unifies several quantities that appear in hypothesis testing, large deviations, and information spectrum methods.

**Definition 17.3.1 (Rényi Divergence).** The *Rényi divergence of order $\alpha$* between distributions $P$ and $Q$ is:
$$D_\alpha(P \| Q) = \frac{1}{\alpha - 1} \log \sum_x p(x)^\alpha q(x)^{1-\alpha}.$$

The limits recover familiar quantities:
- $D_1(P\|Q) = D_{\text{KL}}(P\|Q)$ (KL divergence, recovered as $\alpha \to 1$).
- $D_\infty(P\|Q) = \log \max_x p(x)/q(x)$ (log of the maximum likelihood ratio).
- $D_{1/2}(P\|Q) = -2\log \sum_x \sqrt{p(x)q(x)} = -2\log F(P,Q)$ where $F$ is the Bhattacharyya coefficient (related to the quantum fidelity).

Rényi divergence appears naturally in large deviations: the probability that an empirical distribution falls far from the true distribution decays exponentially at a rate controlled by $D_\alpha$ for appropriate $\alpha$. It also appears in the error exponents for hypothesis testing — different testing strategies optimize different values of $\alpha$.

The most important property is the data processing inequality, which holds for all $\alpha$:

**Theorem 17.3.2 (Data Processing Inequality for Rényi Divergence).** For all $\alpha \geq 0$:
$$D_\alpha(P_{f(X)} \| Q_{f(X)}) \leq D_\alpha(P_X \| Q_X)$$
for any measurable function $f$.

This is the Rényi version of the same principle we saw for KL divergence: processing cannot distinguish distributions better than before. The inequality holds for the entire Rényi family simultaneously — applying any function to your data can only reduce the distinguishability of $P$ from $Q$.

For $\alpha = 1$, this reduces to the familiar KL data processing inequality. For $\alpha \to \infty$, it says that the maximum likelihood ratio cannot increase under processing. In between, it gives a family of constraints on how well any observer can distinguish the two distributions after processing the data.

The Rényi divergence framework is particularly useful in quantum information theory, where it has several natural quantum generalizations (sandwiched Rényi divergence, Petz Rényi divergence) that play key roles in proving achievability and converse bounds for quantum channels. We will encounter these in Chapter 21.
