# Field Axioms

Algebra is often learned as a collection of rules: you can add and multiply numbers, addition commutes, the distributive law holds, and so on. But these rules are not arbitrary conventions — they are axioms that define what kind of mathematical structure we are working with. A field is a set $F$ equipped with two binary operations, addition and multiplication, satisfying a specific list of axioms. The real numbers form a field; so do the rationals, the complex numbers, and many other systems. Understanding what these axioms are, and what follows from them, allows us to separate the universal from the particular: some properties of $\mathbb{R}$ hold for all fields, while others require additional structure specific to the reals.

## The Field Axioms

Let $F$ be a set and let $+$ and $\cdot$ be binary operations on $F$ (meaning $+: F \times F \to F$ and $\cdot: F \times F \to F$). We say $(F, +, \cdot)$ is a **field** if the following axioms hold:

**Axioms for Addition:**

(A1) Commutativity: $a + b = b + a$ for all $a, b \in F$.

(A2) Associativity: $(a + b) + c = a + (b + c)$ for all $a, b, c \in F$.

(A3) Additive identity: there exists $0 \in F$ such that $a + 0 = a$ for all $a \in F$.

(A4) Additive inverse: for each $a \in F$, there exists $-a \in F$ such that $a + (-a) = 0$.

**Axioms for Multiplication:**

(M1) Commutativity: $a \cdot b = b \cdot a$ for all $a, b \in F$.

(M2) Associativity: $(a \cdot b) \cdot c = a \cdot (b \cdot c)$ for all $a, b, c \in F$.

(M3) Multiplicative identity: there exists $1 \in F$ with $1 \neq 0$, such that $a \cdot 1 = a$ for all $a \in F$.

(M4) Multiplicative inverse: for each $a \in F$ with $a \neq 0$, there exists $a^{-1} \in F$ such that $a \cdot a^{-1} = 1$.

**Distributive Law:**

(D) $a \cdot (b + c) = a \cdot b + a \cdot c$ for all $a, b, c \in F$.

These nine axioms are the complete definition of a field. Everything that can be proved about fields — and there is a lot — follows from these nine statements alone, without any additional information about what the elements of $F$ actually are.

## Basic Consequences

Several familiar algebraic facts that students take for granted are actually theorems deduced from the field axioms. Proving them explicitly is an important exercise in using axioms carefully.

**Theorem.** The additive identity is unique: if $a + e = a$ for all $a \in F$, then $e = 0$.

*Proof.* In particular, $0 + e = 0$ (taking $a = 0$). But $0 + e = e + 0 = e$ by commutativity and the definition of $0$. Thus $e = 0$. $\square$

**Theorem.** The additive inverse of $a$ is unique.

*Proof.* Suppose $a + b = 0$ and $a + c = 0$. Then $b = b + 0 = b + (a + c) = (b + a) + c = 0 + c = c$. $\square$

**Theorem.** $a \cdot 0 = 0$ for all $a \in F$.

*Proof.* We have $a \cdot 0 = a \cdot (0 + 0) = a \cdot 0 + a \cdot 0$ by the distributive law. Adding $-(a \cdot 0)$ to both sides yields $0 = a \cdot 0$. $\square$

**Theorem.** $(-1) \cdot a = -a$ for all $a \in F$.

*Proof.* Consider $a + (-1) \cdot a = 1 \cdot a + (-1) \cdot a = (1 + (-1)) \cdot a = 0 \cdot a = 0$. The uniqueness of additive inverses gives $(-1) \cdot a = -a$. $\square$

**Theorem.** If $a \cdot b = 0$ and $a \neq 0$, then $b = 0$.

*Proof.* Since $a \neq 0$, $a^{-1}$ exists. Multiply both sides of $a \cdot b = 0$ on the left by $a^{-1}$: $a^{-1}(ab) = a^{-1} \cdot 0$. The left side is $(a^{-1}a)b = 1 \cdot b = b$; the right side is $0$. So $b = 0$. $\square$

This last result — the absence of zero divisors — is a fundamental property of fields that makes them particularly well-behaved for algebra. It is equivalent to the cancellation law: if $a \neq 0$ and $ab = ac$, then $b = c$.

## Fields and Subfields

$\mathbb{Q}$ and $\mathbb{R}$ are both fields, and $\mathbb{Q} \subset \mathbb{R}$. When a subset of a field is itself a field under the same operations, it is called a **subfield**. $\mathbb{Q}$ is a subfield of $\mathbb{R}$, and $\mathbb{R}$ is a subfield of $\mathbb{C}$.

Not every subset forms a subfield. The integers $\mathbb{Z}$ satisfy the addition axioms and the distributive law, but fail (M4): $2 \in \mathbb{Z}$ has no multiplicative inverse in $\mathbb{Z}$. So $\mathbb{Z}$ is a ring but not a field.

## Examples of Fields

Beyond $\mathbb{Q}$, $\mathbb{R}$, and $\mathbb{C}$, there are finite fields. For any prime $p$, the set $\mathbb{Z}/p\mathbb{Z} = \{0, 1, 2, \ldots, p-1\}$ with arithmetic modulo $p$ is a field. For instance, in $\mathbb{Z}/5\mathbb{Z}$, the multiplicative inverse of $3$ is $2$, since $3 \cdot 2 = 6 \equiv 1 \pmod{5}$. These finite fields appear in coding theory and cryptography but are not the focus of analysis.

## What Field Axioms Do Not Specify

The field axioms say nothing about order (which elements are "larger"), topology (which elements are "close"), or completeness (whether bounded sets have least upper bounds). All of these are additional structures layered on top of the field structure. This is why theorems that use only field axioms — algebraic manipulations of equations — apply in $\mathbb{Q}$, $\mathbb{R}$, and $\mathbb{C}$ equally, while theorems about convergence and limits require the additional structures of order and completeness, which distinguish $\mathbb{R}$ from $\mathbb{Q}$ and $\mathbb{C}$ from $\mathbb{R}$.

## Connection to Differential Equations

The field axioms guarantee that algebraic manipulations of solutions are valid. When two solutions $y_1$ and $y_2$ of a linear ODE are combined as $c_1 y_1 + c_2 y_2$, the calculation that this combination satisfies the equation uses linearity — which is an algebraic property. The fact that real coefficients $c_1, c_2$ can be freely chosen uses the field structure of $\mathbb{R}$. More deeply, the function space of continuous functions on $[a,b]$ forms a vector space over $\mathbb{R}$, and the field of scalars in that vector space is exactly the field $\mathbb{R}$ defined by the axioms in this section.
