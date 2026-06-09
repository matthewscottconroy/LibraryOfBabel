# Chapter 02: Real Numbers and Completeness

The rational numbers seem, at first, like a perfectly adequate number system: every finite decimal is rational, every measurement we can make is rational, and $\mathbb{Q}$ is dense in the sense that between any two rationals there is another. Yet $\mathbb{Q}$ has a fundamental flaw — it has holes. The equation $x^2 = 2$ has no rational solution; the sequence $3, 3.1, 3.14, 3.141, 3.1415, \ldots$ is a sequence of rationals that approaches no rational limit. These gaps are not philosophical curiosities: they mean that the limit of a sequence of rational approximations to a solution of a differential equation might not exist within $\mathbb{Q}$, and the entire machinery of analysis collapses. The real number system closes these holes, and this chapter explains precisely how.

## The Axiomatic Approach

Rather than constructing $\mathbb{R}$ from the ground up (via Dedekind cuts or equivalence classes of Cauchy sequences of rationals), this chapter takes the axiomatic approach: we list the properties that the real numbers are required to satisfy and prove consequences from those properties. This has the advantage of being direct and connecting immediately to the algebra and order that students know from calculus.

The axioms fall into three groups:

**Field Axioms** specify how addition and multiplication work — associativity, commutativity, distributivity, and the existence of identities and inverses. These are the rules that make $\mathbb{R}$ a field, shared by $\mathbb{Q}$, $\mathbb{C}$, and many other algebraic structures.

**Order Axioms** specify a total ordering on $\mathbb{R}$ that is compatible with the field operations: if $a < b$ and $c > 0$, then $ac < bc$; if $a < b$ and $c < d$, then $a + c < b + d$. These axioms describe $\mathbb{R}$ as an ordered field.

**The Completeness Axiom** is the distinguishing property: every nonempty subset of $\mathbb{R}$ that is bounded above has a least upper bound in $\mathbb{R}$. This single axiom, also called the Least Upper Bound Property, is what separates $\mathbb{R}$ from $\mathbb{Q}$. The set $\{q \in \mathbb{Q} : q^2 < 2\}$ is a nonempty, bounded-above subset of $\mathbb{Q}$ whose least upper bound ($\sqrt{2}$) does not belong to $\mathbb{Q}$; in $\mathbb{R}$, it does.

## How the Sections Build on Each Other

Section 1 (Field Axioms) establishes the algebraic ground rules and derives basic consequences: the uniqueness of additive and multiplicative identities, the uniqueness of inverses, and the standard rules of algebra such as $(-1) \cdot a = -a$ and $a \cdot 0 = 0$.

Section 2 (Order Axioms) introduces the ordering and derives properties of absolute value and inequalities. The triangle inequality, $|a + b| \leq |a| + |b|$, is established here; it is used in virtually every subsequent analytic argument.

Section 3 (Completeness Axiom) introduces the least upper bound (supremum) and greatest lower bound (infimum), proves their existence and uniqueness, and begins drawing consequences. The Nested Interval Property — that a nested sequence of closed intervals $[a_n, b_n]$ with $b_n - a_n \to 0$ has a unique common point — follows from completeness.

Section 4 (Archimedean Property) proves that for any real $x > 0$ and $y > 0$, there is a natural number $n$ with $nx > y$. Equivalently, the natural numbers are not bounded above in $\mathbb{R}$, and for any $\varepsilon > 0$, there is an $n \in \mathbb{N}$ with $1/n < \varepsilon$. This property is used in virtually every epsilon argument in analysis — it is the guarantee that $\varepsilon$'s can always be made as small as needed.

## Why This Chapter Matters for Differential Equations

The Completeness Axiom is the reason that iterative methods for solving differential equations converge. Picard's theorem constructs a sequence of approximate solutions $\phi_0, \phi_1, \phi_2, \ldots$ and uses completeness (in the form of a completeness theorem for function spaces built on completeness of $\mathbb{R}$) to guarantee that the sequence converges to an actual solution. Numerical methods — Euler, Runge-Kutta — generate sequences of numbers, and whether those numbers approach the true answer depends on completeness arguments.

More concretely, the Archimedean Property guarantees that error bounds can always be driven to zero by taking step sizes small enough, and the existence of suprema allows one to bound the error of an approximation uniformly over an interval. These connections will be made explicit when existence and uniqueness are discussed in later parts of the course.
