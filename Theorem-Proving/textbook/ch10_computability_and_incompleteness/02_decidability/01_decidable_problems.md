# Decidable Problems

## Decidability and Semi-Decidability

A **language** $L \subseteq \Sigma^*$ (a set of strings) is:

**Decidable** (recursive): there is a Turing machine $M$ that halts on every input $w$, accepting if $w \in L$ and rejecting if $w \notin L$.

**Semi-decidable** (recursively enumerable, r.e.): there is a TM $M$ that accepts every $w \in L$ (and halts to accept), but may run forever on $w \notin L$.

**Co-semi-decidable**: $\bar{L}$ (the complement) is semi-decidable — there is a TM that accepts every $w \notin L$.

**Key theorem**: $L$ is decidable iff both $L$ and $\bar{L}$ are semi-decidable.

## Examples of Decidable Problems

| Problem | Why Decidable |
|---------|--------------|
| Is $n$ prime? | Trial division by all $m \leq \sqrt{n}$ |
| Is $\varphi$ a propositional tautology? | Truth table enumeration (exponential but finite) |
| Is graph $G$ connected? | BFS/DFS |
| Does DFA $M$ accept $w$? | Simulate $M$ on $w$ (finite steps) |
| Does regex $r$ match $w$? | Convert to DFA, simulate |
| Is Presburger arithmetic sentence $\varphi$ true? | Decision procedure exists (doubly exponential) |
| Does real closed field sentence $\varphi$ hold in $\mathbb{R}$? | Tarski-Seidenberg decision procedure |

## Undecidable Problems (for comparison)

| Problem | Why Undecidable |
|---------|----------------|
| Does TM $M$ halt on $w$? | The halting problem (Turing 1936) |
| Is TM $M$'s language empty? | Reduction from halting problem |
| Does context-free grammar $G$ generate all strings? | Reduction from Post correspondence problem |
| Is first-order arithmetic sentence $\varphi$ true? | Gödel 1931 |

## Complexity Within Decidable Problems

Even among decidable problems, efficiency matters enormously:
- **P** (polynomial time): problems solvable in $O(n^k)$ steps — tractable
- **NP** (non-deterministic polynomial): solutions verifiable in polynomial time (SAT, graph coloring)
- **PSPACE**, **EXPTIME**: larger classes for harder decidable problems

The P vs. NP question — is every NP problem in P? — is the most famous open problem in theoretical computer science.

## Exercises
See [problems/ch10_computability/](../../../problems/ch10_computability/)
