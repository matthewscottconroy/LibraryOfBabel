# 2.6 Probability Theory

Probability theory is measure theory with the normalization condition $\mu(\Omega) = 1$. Everything we've developed applies, and the additional structure (total mass equals one) allows us to interpret integrals as expected values and measures as probability distributions. We also gain the concept of *independence*, which is central to the probabilistic intuition behind ergodic theory.

## 2.6.1 Probability Spaces and Random Variables

**Definition 2.6.1.** A *probability space* is a measure space $(\Omega, \mathcal{F}, P)$ with $P(\Omega) = 1$. A *random variable* is a measurable function $X: \Omega \to \mathbb{R}$ (or into any measurable space). The *distribution* of $X$ is the probability measure $\mu_X = X_*P$ on $\mathbb{R}$ defined by $\mu_X(B) = P(X^{-1}(B)) = P(X \in B)$.

The expected value of $X$ is $E[X] = \int X\,dP$ — just the Lebesgue integral of $X$ against the probability measure. All the convergence theorems from Section 2.3 apply, just with $P$ in place of $\mu$.

## 2.6.2 Independence

Independence is the key probabilistic concept that has no analog in pure measure theory (without the normalization $P(\Omega) = 1$):

**Definition 2.6.2.** Events $A_1, \ldots, A_n \in \mathcal{F}$ are *independent* if $P(A_{i_1} \cap \cdots \cap A_{i_k}) = P(A_{i_1}) \cdots P(A_{i_k})$ for all subcollections. Random variables $X_1, \ldots, X_n$ are independent if $P(X_1 \in B_1, \ldots, X_n \in B_n) = \prod_i P(X_i \in B_i)$ for all Borel sets $B_i$.

Equivalently, $X_1, \ldots, X_n$ are independent if the $\sigma$-algebras $\sigma(X_1), \ldots, \sigma(X_n)$ they generate are independent — meaning: knowing the value of any subcollection gives no information about the others.

Independent random variables are the probabilistic analog of an uncorrelated system — but independence is strictly stronger than zero correlation. (Zero correlation is a statement about second moments; independence is a statement about all joint distributions.)

## 2.6.3 Convergence Concepts

There are several different ways a sequence of random variables can converge, and understanding the relationships between them is important:

**Definition 2.6.3.** A sequence $(X_n)$ of random variables converges to $X$:
- *almost surely (a.s.)* if $P(\lim_n X_n = X) = 1$
- *in probability* if $P(|X_n - X| > \varepsilon) \to 0$ for all $\varepsilon > 0$
- *in $L^p$* if $E[|X_n - X|^p] \to 0$
- *in distribution* if $E[f(X_n)] \to E[f(X)]$ for all bounded continuous $f$

The implications: a.s. $\Rightarrow$ in probability $\Rightarrow$ in distribution; $L^p \Rightarrow$ in probability. None of the converses hold in general.

Almost sure convergence is the strongest: the sequence converges pointwise, except possibly on a set of probability zero. Convergence in distribution is the weakest: only the distributions converge, not necessarily the random variables themselves. For ergodic theory, almost sure convergence is the gold standard — the Birkhoff Ergodic Theorem gives convergence almost everywhere.

## 2.6.4 Conditional Expectation

Conditional expectation is the most important concept in probability theory for ergodic applications. It generalizes the elementary formula $E[X|A] = P(A)^{-1} \int_A X\,dP$ to conditioning on a $\sigma$-algebra rather than a single event.

**Definition 2.6.4.** Let $(\Omega, \mathcal{F}, P)$ be a probability space, $X \in L^1(P)$, and $\mathcal{G} \subseteq \mathcal{F}$ a sub-$\sigma$-algebra. The *conditional expectation* $E[X | \mathcal{G}]$ is the unique (a.s.) $\mathcal{G}$-measurable random variable satisfying
$$\int_G E[X | \mathcal{G}]\,dP = \int_G X\,dP \quad \text{for all } G \in \mathcal{G}.$$

*Existence:* The measure $\nu(G) = \int_G X\,dP$ is absolutely continuous with respect to $P|_\mathcal{G}$, so Radon-Nikodym gives $d\nu/d(P|_\mathcal{G}) = E[X|\mathcal{G}]$.

This is the Radon-Nikodym theorem at work: the conditional expectation exists because we can take a Radon-Nikodym derivative. The definition says: $E[X|\mathcal{G}]$ is the best $\mathcal{G}$-measurable approximation to $X$, in the sense that it matches the integrals of $X$ over all $\mathcal{G}$-measurable sets.

The key properties:

**Properties of Conditional Expectation:**
1. $E[E[X|\mathcal{G}]] = E[X]$ (tower property — average of the conditional average is the average)
2. If $X$ is $\mathcal{G}$-measurable, then $E[X|\mathcal{G}] = X$ (already known, no new information)
3. If $X$ is independent of $\mathcal{G}$, then $E[X|\mathcal{G}] = E[X]$ (independence means no information)
4. $E[\cdot|\mathcal{G}]$ is the orthogonal projection from $L^2(\mathcal{F})$ onto $L^2(\mathcal{G})$
5. Tower property: $E[E[X|\mathcal{F}_1]|\mathcal{F}_2] = E[X|\mathcal{F}_2]$ if $\mathcal{F}_2 \subseteq \mathcal{F}_1$

Property (4) is the connection to Hilbert space geometry: conditioning on $\mathcal{G}$ is projecting onto the subspace of $\mathcal{G}$-measurable functions in $L^2$. The Projection Theorem from Section 1.6.2 guarantees this projection exists and is unique.

**Application in Dynamics.** Conditional expectation is the key to defining the *information function* $I_\mu(\xi | \eta)$ of a partition $\xi$ given a partition $\eta$. This is the foundation of entropy theory: the entropy of $\xi$ measures how much information $\xi$ gives on average, and the conditional entropy measures how much information $\xi$ gives beyond what $\eta$ already told you. We build this in Chapter 22.
