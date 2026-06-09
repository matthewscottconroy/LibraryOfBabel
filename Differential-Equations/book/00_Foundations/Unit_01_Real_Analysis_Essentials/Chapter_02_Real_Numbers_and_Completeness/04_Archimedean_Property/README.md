# The Archimedean Property

The Archimedean Property is a theorem, not an additional axiom: it follows from the field axioms, the order axioms, and the Completeness Axiom together. Its content is that the real number system has no "infinitely large" or "infinitely small" elements — natural numbers can be made as large as desired, and unit fractions $1/n$ can be made as small as desired. This seemingly modest statement is the engine behind every epsilon-argument in analysis.

## Statement and Proof

**Theorem (Archimedean Property).** For any $x \in \mathbb{R}$, there exists $n \in \mathbb{N}$ with $n > x$.

Equivalently: the set $\mathbb{N}$ is not bounded above in $\mathbb{R}$.

*Proof.* Suppose, for contradiction, that $\mathbb{N}$ is bounded above. Then by the Completeness Axiom, $\alpha = \sup \mathbb{N}$ exists. Since $\alpha$ is the least upper bound, $\alpha - 1$ is not an upper bound, so there exists $n \in \mathbb{N}$ with $n > \alpha - 1$, i.e., $n + 1 > \alpha$. But $n + 1 \in \mathbb{N}$, so $n + 1 \leq \alpha$ (since $\alpha$ is an upper bound). This contradicts $n + 1 > \alpha$. Therefore $\mathbb{N}$ is not bounded above. $\square$

Note the use of all three axiom sets: the Completeness Axiom is invoked to produce $\sup \mathbb{N}$, and the characterization of the supremum is used to derive the contradiction.

## Consequences

**Corollary 1.** For any $\varepsilon > 0$, there exists $n \in \mathbb{N}$ with $\frac{1}{n} < \varepsilon$.

*Proof.* By the Archimedean Property, there exists $n \in \mathbb{N}$ with $n > 1/\varepsilon$. Then $1/n < \varepsilon$. $\square$

This is the version used in every epsilon argument: given any positive tolerance $\varepsilon$, no matter how small, we can find a natural number $n$ large enough that $1/n$ is below that tolerance. It is what gives "for all $\varepsilon > 0$, there exists $N$..." its force.

**Corollary 2.** For any $a, b \in \mathbb{R}$ with $a > 0$, there exists $n \in \mathbb{N}$ with $na > b$.

*Proof.* Apply the Archimedean Property with $x = b/a$. $\square$

**Corollary 3 (Density of $\mathbb{Q}$ in $\mathbb{R}$).** For any $a, b \in \mathbb{R}$ with $a < b$, there exists $q \in \mathbb{Q}$ with $a < q < b$.

*Proof.* Since $b - a > 0$, by Corollary 1 there exists $n \in \mathbb{N}$ with $1/n < b - a$. Let $m$ be the smallest integer with $m > na$ (which exists by the Archimedean Property). Then $m - 1 \leq na$, so $m \leq na + 1$. Thus $q = m/n$ satisfies $q = m/n > a$ and
$$q = m/n \leq (na + 1)/n = a + 1/n < a + (b - a) = b.$$
So $a < q < b$ and $q \in \mathbb{Q}$. $\square$

This means the rationals are dense in $\mathbb{R}$: every open interval contains a rational number. Equally, every open interval contains an irrational number (exercise: find one between any two reals). The real line is not just the rationals with gaps filled — the irrationals are, in a precise sense, far more numerous (uncountably many) than the rationals.

## The Floor and Ceiling Functions

The proof of density used the "smallest integer greater than $na$." This is the **ceiling** function $\lceil x \rceil = \min\{m \in \mathbb{Z} : m \geq x\}$. The Archimedean Property guarantees this minimum exists. Similarly, the **floor** $\lfloor x \rfloor = \max\{m \in \mathbb{Z} : m \leq x\}$ exists. These satisfy $\lfloor x \rfloor \leq x < \lfloor x \rfloor + 1$ and $\lceil x \rceil - 1 < x \leq \lceil x \rceil$.

## Non-Archimedean Fields

The Archimedean Property fails in some ordered fields. In the field of rational functions $\mathbb{R}(x)$ ordered by the behavior as $x \to +\infty$, the element $x$ (as a function) is greater than every constant, and $1/x$ is positive but less than every positive rational. Such systems are "non-Archimedean" and contain genuine infinitesimals — positive elements smaller than every positive rational. Abraham Robinson's non-standard analysis makes rigorous use of such systems. The classical analysis developed in this course takes place firmly in $\mathbb{R}$, which is Archimedean.

## Worked Example

**Claim.** If $a \geq 0$ and $a < 1/n$ for all $n \in \mathbb{N}$, then $a = 0$.

*Proof.* Suppose $a > 0$. By Corollary 1, there exists $n \in \mathbb{N}$ with $1/n < a$. This contradicts the hypothesis $a < 1/n$. So $a \leq 0$, and combined with $a \geq 0$, we get $a = 0$. $\square$

This "squeeze" argument — showing $a = 0$ by forcing $a < \varepsilon$ for all $\varepsilon > 0$ — is one of the most used techniques in analysis. The Archimedean Property is what justifies the step "this must hold for $\varepsilon = 1/n$, and $1/n$ can be made arbitrarily small."

## Connection to Differential Equations

Numerical methods for ODEs work by choosing a step size $h > 0$ and advancing the approximate solution by that increment. The Archimedean Property guarantees that for any desired accuracy $\varepsilon > 0$, there exists a step size $h = 1/n$ small enough to achieve it. More abstractly, any proof of convergence of a numerical scheme is ultimately an epsilon argument, and the Archimedean Property is what allows that argument to go through: it guarantees that the sequence of errors, which converges to zero through steps of size $1/n$, genuinely reaches any tolerance. Without the Archimedean Property, the phrase "take $n$ large enough" would be meaningless.
