# 35.2 The Foreman-Rudolph-Weiss Theorem

The isomorphism problem for ergodic systems was solved — then it wasn't. Ornstein's theorem classified Bernoulli shifts by entropy, and for forty years mathematicians worked to extend the classification. Foreman, Rudolph, and Weiss showed in 2011 that for general ergodic systems, no such classification can exist.

**Theorem 35.2.1 (Foreman-Rudolph-Weiss, 2011).** The isomorphism relation on ergodic MPTs of $[0,1]$ with Lebesgue measure is not classifiable by countable structures. More precisely, the isomorphism equivalence relation $\cong$ on $\text{Aut}(X, \mu)$ is not Borel reducible to the isomorphism relation of countable structures.

What this is saying: there is no way to assign to each ergodic MPT a countable structure — no countable graph, no countable group, no countable relational structure of any kind — that serves as a complete invariant. The isomorphism problem cannot be reduced to a problem about countable combinatorics.

**Corollary 35.2.2.** There is no Borel complete invariant for ergodic MPTs. In particular:
- No sequence of reals, no countable group, no countable graph, no countable field can serve as a complete invariant.
- The isomorphism problem for ergodic MPTs is "more complex" than classifying any class of countable algebraic structures.

**Theorem 35.2.3 (Complexity of the Isomorphism Relation).** The isomorphism relation on ergodic MPTs is:
- $\Sigma^1_1$-complete (analytic, not Borel)
- Not Borel (the class of Bernoulli shifts is Borel, but isomorphism on Bernoulli shifts is "non-Borel equivalent")
- Strictly above all orbit equivalence relations of Polish group actions

The phrase "strictly above orbit equivalence relations of Polish group actions" is key. It means the isomorphism problem is more complex than any classification problem you could naturally formulate — it sits at the very top of a hierarchy of complexity.

To be clear about what this does and doesn't say: this is not a Gödelian undecidability result. It's not saying there's no proof that two specific systems are isomorphic. It's saying there's no systematic, classifiable invariant that handles all cases. You can prove isomorphism for specific pairs by constructing an explicit conjugacy. But you cannot reduce the problem to a simple invariant.
