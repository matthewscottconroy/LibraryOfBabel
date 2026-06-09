# 18.1 Computability — Background

Algorithmic information theory is built on computability theory — the mathematical study of what computers can and cannot compute. We need just enough of this foundation to define Kolmogorov complexity rigorously and to understand why the halting problem matters.

**Definition 18.1.1 (Turing Machine).** A *Turing machine* is an abstract device with a finite control (states), an infinite read-write tape, and a transition function. A function $f: \{0,1\}^* \to \{0,1\}^*$ is *computable* if some Turing machine computes it. A set $A \subseteq \mathbb{N}$ is *recursively enumerable (r.e.)* if some Turing machine enumerates it; *decidable* (recursive) if both $A$ and $A^c$ are r.e.

Turing machines are not just an abstraction — the Church-Turing thesis asserts that every "effectively computable" function is Turing-computable. This is a philosophical thesis, not a theorem, but it is supported by the fact that every other model of computation (lambda calculus, RAM machines, cellular automata, quantum computers for classical computations) computes exactly the same class of functions.

**Theorem 18.1.2 (Halting Problem).** The set $K = \{(M, x) : M\text{ halts on input }x\}$ is r.e. but not decidable. There is no Turing machine that decides, on input $(M, x)$, whether $M$ halts on $x$.

The halting problem is the archetype of an undecidable problem. The proof is a diagonal argument: if a halting decider existed, you could use it to build a machine that contradicts itself. The proof is easy, the consequence profound: not every mathematical question has a computable answer.

**Theorem 18.1.3 (Rice's Theorem).** Any nontrivial property of the *function* computed by a Turing machine is undecidable.

Rice's theorem says that you cannot decide, given a program, whether the function it computes has any non-trivial property: whether it always outputs 0, whether it halts on all inputs, whether it computes a prime number function. These questions are all undecidable, as consequences of the halting problem.

For algorithmic information theory, the key implication is this: Kolmogorov complexity is not computable. Given a string $x$, you cannot write a program that outputs $C(x)$. You can compute upper bounds (by trying all programs shorter than $|x|$), but the true complexity is uncomputable. This is not a limitation of current algorithms — it is a fundamental mathematical fact.

Despite being uncomputable, Kolmogorov complexity is well-defined and has powerful applications. The strategy throughout this chapter is to use it as an analytical tool rather than an algorithmic one: we reason about what the complexity is, even though we cannot compute it.
