# 16.6 Information Inequalities

Every subject has its fundamental inequalities — the results that do the heavy lifting in most proofs. In information theory, a handful of inequalities appear over and over again, and recognizing them is half the art.

**Theorem 16.6.1 (Data Processing Inequality).** If $X \to Y \to Z$ form a Markov chain, then:
$$I(X; Z) \leq I(X; Y).$$
Processing cannot increase mutual information.

*Proof:* The Markov chain $X \to Y \to Z$ means $Z \perp X \mid Y$ (knowing $Y$, $Z$ gives no extra information about $X$). So:
$$I(X; Y, Z) = I(X; Y) + I(X; Z \mid Y) = I(X; Y),$$
since $I(X; Z \mid Y) = 0$. But also:
$$I(X; Y, Z) = I(X; Z) + I(X; Y \mid Z) \geq I(X; Z). \quad \square$$

The data processing inequality is intuitively obvious — stochastic processing can only destroy information, never create it — but its formal statement is what enables rigorous proofs of channel capacity converses, lower bounds in statistics, and impossibility results in learning theory. No matter how cleverly you process $Y$ to estimate $X$, you cannot extract more than $I(X;Y)$ bits of information about $X$.

**Theorem 16.6.2 (Log-Sum Inequality).** For nonnegative numbers $a_1, \ldots, a_n$ and $b_1, \ldots, b_n$:
$$\sum_i a_i \log \frac{a_i}{b_i} \geq \left(\sum_i a_i\right)\log\frac{\sum_i a_i}{\sum_i b_i}.$$

The log-sum inequality is the workhorse behind most proofs of entropy properties. It implies the nonnegativity of KL divergence, the convexity of entropy in $p$, and dozens of other facts. Whenever you need to bound a sum of "logarithmic" terms, reach for the log-sum inequality.

**Theorem 16.6.3 (Subadditivity of Entropy).** $H(X_1, \ldots, X_n) \leq \sum_i H(X_i)$, with equality iff $X_1, \ldots, X_n$ are mutually independent.

Subadditivity says: the total uncertainty in a collection of random variables is no more than the sum of their individual uncertainties. Correlations can only reduce total uncertainty. This is again intuitively clear — knowing about one variable can help predict others — but the formal statement is what you use in proofs.

These three inequalities — data processing, log-sum, and subadditivity — together with Fano's inequality from Section 16.4.6, constitute the standard toolkit of classical information theory. They appear in channel coding converses, source coding lower bounds, statistical estimation lower bounds, and the information-theoretic proofs of computational complexity lower bounds.

As we move into entropy generalizations (Chapter 17), network information theory (Chapter 19), and quantum information (Chapter 21), we will encounter analogues of these inequalities in more complex settings. Some generalize cleanly (data processing inequality holds for all Rényi divergences, and for quantum channels). Others are more subtle — the chain rule, for instance, fails for Rényi entropy, and subadditivity breaks in unexpected ways for quantum entropy. Knowing the classical versions cold is the prerequisite for understanding when and why they fail to generalize.
