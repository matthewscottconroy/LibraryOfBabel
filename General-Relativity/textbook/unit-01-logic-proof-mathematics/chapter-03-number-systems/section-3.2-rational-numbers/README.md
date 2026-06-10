# Section 3.2: The Rational Numbers

---

## Section Introduction

The integers are an adequate arena for most of arithmetic, but they fail at division. Given integers $a$ and $b$ with $b\neq 0$, the integer $a/b$ may not exist — $7/3$ is not an integer. The **rational numbers** $\mathbb{Q}$ are constructed precisely to fill this gap: they are the smallest field containing $\mathbb{Z}$.

The construction parallels the construction of $\mathbb{Z}$ from $\mathbb{N}$. Take pairs $(a, b)$ with $a \in \mathbb{Z}$ and $b \in \mathbb{Z}\setminus\{0\}$, representing "$a/b$." Define equivalence $(a,b)\sim (a',b')$ iff $ab' = a'b$. The rational numbers are the equivalence classes, with addition and multiplication defined by $[a/b] + [c/d] = [(ad+bc)/bd]$ and $[a/b]\cdot [c/d] = [ac/bd]$. These operations are well-defined on equivalence classes and give $\mathbb{Q}$ the structure of a **field**: a commutative ring in which every nonzero element has a multiplicative inverse.

The rationals seem rich enough to do all of mathematics. Every integer is rational; every rational number can be written as a ratio of integers in lowest terms; between any two rationals there are infinitely many others (the rationals are **dense** in themselves). And yet $\mathbb{Q}$ has a profound deficiency: it has "gaps." The sequence $1, 1.4, 1.41, 1.414, \ldots$ of rational decimal approximations to $\sqrt{2}$ gets closer and closer to $\sqrt{2}$, but $\sqrt{2}$ is not rational. The sequence converges — it has the feel of converging — but it does not converge *within* $\mathbb{Q}$. The irrational numbers live in the gaps.

This gap is not merely a technical nuisance. It means that $\mathbb{Q}$ is not **complete** — Cauchy sequences need not converge. The real numbers (Section 3.3) are constructed precisely to fill these gaps. The construction of $\mathbb{R}$ from $\mathbb{Q}$ is the completion that makes analysis possible. Understanding why $\mathbb{Q}$ is insufficient is the essential motivation for that construction.

---

## Subsections

- [3.2.1: Construction of ℚ as a Field of Fractions](3.2.1-construction.md)
- [3.2.2: Order and Density of ℚ](3.2.2-order-density.md)
- [3.2.3: Decimal Expansions and Periodic Decimals](3.2.3-decimal-expansions.md)
- [3.2.4: Irrationality of √2 and Other Numbers](3.2.4-irrationality.md)
- [3.2.5: The Incompleteness of ℚ](3.2.5-incompleteness.md)
