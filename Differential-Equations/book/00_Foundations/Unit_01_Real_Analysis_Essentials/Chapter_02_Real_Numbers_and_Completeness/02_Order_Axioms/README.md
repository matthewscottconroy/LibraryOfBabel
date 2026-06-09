# Order Axioms

The field axioms describe the algebraic structure of $\mathbb{R}$: how its elements can be added and multiplied. But they say nothing about which elements are larger than which. The ordering of the real numbers is a separate structure, and it is the one that makes statements like "$f(x)$ is close to $L$" or "$a_n$ is eventually positive" meaningful. The order axioms specify this structure and its compatibility with the algebraic operations.

## The Order Axioms

An **ordered field** is a field $F$ together with a strict total order $<$ satisfying:

(O1) **Trichotomy**: for all $a, b \in F$, exactly one of the following holds: $a < b$, $a = b$, or $a > b$ (where $a > b$ means $b < a$).

(O2) **Transitivity**: if $a < b$ and $b < c$, then $a < c$.

(O3) **Addition compatibility**: if $a < b$, then $a + c < b + c$ for all $c \in F$.

(O4) **Multiplication compatibility**: if $a < b$ and $c > 0$, then $ac < bc$.

From these four axioms, together with the field axioms, all the familiar rules for manipulating inequalities follow.

## Derived Order Properties

**Theorem.** For any $a \in F$, $a^2 \geq 0$, with equality iff $a = 0$.

*Proof.* If $a > 0$, then by (O4), $a \cdot a > 0 \cdot a = 0$. If $a < 0$, then $-a > 0$ by (O3) (adding $-a$ to both sides of $a < 0$ gives $0 < -a$), so $(-a)^2 > 0$, i.e., $a^2 > 0$. If $a = 0$, then $a^2 = 0 \cdot 0 = 0$. $\square$

**Corollary.** $1 > 0$.

*Proof.* $1 = 1^2 \geq 0$, and $1 \neq 0$ by (M3). So $1 > 0$. $\square$

**Theorem (Multiplication by a Negative Reverses Inequality).** If $a < b$ and $c < 0$, then $ac > bc$.

*Proof.* Since $c < 0$, we have $-c > 0$. By (O4), $a(-c) < b(-c)$, i.e., $-ac < -bc$. Adding $ac + bc$ to both sides: $bc < ac$. $\square$

**Theorem.** If $a < b$ and $c < d$, then $a + c < b + d$.

*Proof.* By (O3), $a + c < b + c$ and $b + c < b + d$. Transitivity gives $a + c < b + d$. $\square$

## Absolute Value

The **absolute value** of $a \in F$ is:
$$|a| = \begin{cases} a & \text{if } a \geq 0 \\ -a & \text{if } a < 0 \end{cases}$$

Absolute value measures "size" without regard to sign. It satisfies:

(i) $|a| \geq 0$, with $|a| = 0$ iff $a = 0$.

(ii) $|-a| = |a|$.

(iii) $|ab| = |a| \cdot |b|$.

(iv) **Triangle Inequality**: $|a + b| \leq |a| + |b|$.

The triangle inequality is the most important of these. Its proof: by (i) and the definition, $-|a| \leq a \leq |a|$ and $-|b| \leq b \leq |b|$. Adding: $-(|a| + |b|) \leq a + b \leq |a| + |b|$. This is equivalent to $|a + b| \leq |a| + |b|$.

**Reverse Triangle Inequality:** $\bigl||a| - |b|\bigr| \leq |a - b|$.

*Proof.* $|a| = |(a - b) + b| \leq |a - b| + |b|$, giving $|a| - |b| \leq |a - b|$. Symmetrically $|b| - |a| \leq |b - a| = |a - b|$. Taking the maximum: $\bigl||a| - |b|\bigr| \leq |a - b|$. $\square$

## Intervals and Neighborhoods

The order on $\mathbb{R}$ gives rise to intervals. For $a < b$:
- $(a, b) = \{x \in \mathbb{R} : a < x < b\}$ (open interval)
- $[a, b] = \{x \in \mathbb{R} : a \leq x \leq b\}$ (closed interval)
- $[a, b)$ and $(a, b]$ are half-open intervals

An **$\varepsilon$-neighborhood** of a point $a$ is the open interval $(a - \varepsilon, a + \varepsilon) = \{x : |x - a| < \varepsilon\}$. The condition $|x - a| < \varepsilon$ is the standard way to say "$x$ is within distance $\varepsilon$ of $a$", and it is the basis of all limit definitions.

## Bounds and Bounded Sets

A set $S \subseteq \mathbb{R}$ is **bounded above** if there exists $M \in \mathbb{R}$ with $s \leq M$ for all $s \in S$; such $M$ is called an **upper bound** of $S$. Similarly, $S$ is **bounded below** if there exists $m$ with $s \geq m$ for all $s \in S$. A set is **bounded** if it is both bounded above and below, equivalently if it is contained in some interval $[-M, M]$.

Note that upper bounds need not belong to $S$. The set $(0, 1) = \{x : 0 < x < 1\}$ is bounded above by $1, 2, 100$, and infinitely many other values, but $1$ does not belong to $(0,1)$.

## Density of the Rationals

A crucial property that $\mathbb{Q}$ and $\mathbb{R}$ share (derived from the field and order axioms alone, without completeness) is:

**Theorem (Density of $\mathbb{Q}$ in itself).** For any $a, b \in \mathbb{Q}$ with $a < b$, there exists $c \in \mathbb{Q}$ with $a < c < b$.

*Proof.* Take $c = (a + b)/2$. Since $a < b$, we have $a = (a+a)/2 < (a+b)/2 < (b+b)/2 = b$. $\square$

This density means that no matter how close together two rationals are, there is another rational between them. But density alone does not imply completeness: $\mathbb{Q}$ is dense, yet it has holes. The ordering structure tells you the rationals are tightly packed, but it cannot tell you whether the gaps between them are filled.

## Why Order Alone Is Not Enough

The rational numbers $\mathbb{Q}$, with the usual ordering, satisfy all four order axioms and all field axioms. So the order axioms, by themselves, do not imply completeness. The two ordered field axioms together characterize $\mathbb{R}$ up to isomorphism only when the Completeness Axiom is added. This is why all three sets of axioms — field, order, and completeness — are needed to pin down $\mathbb{R}$ uniquely.

## Connection to Analysis

The ordered field structure is what gives meaning to statements like "eventually positive" (there exists $N$ such that $a_n > 0$ for all $n > N$) and "arbitrarily close" ($|a_n - L| < \varepsilon$ for all sufficiently large $n$). Every epsilon-delta argument in analysis is, at its core, an argument about the order on $\mathbb{R}$: it shows that a distance can be made smaller than any positive threshold. The triangle inequality is invoked in nearly every such argument, usually in the form $|f(x) - L| = |(f(x) - f(a)) + (f(a) - L)| \leq |f(x) - f(a)| + |f(a) - L|$ to split a single error into manageable pieces. Fluency with inequalities and absolute values, built on the order axioms, is the mechanical foundation on which all of analysis rests.
