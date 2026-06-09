# AGM Belief Revision Theory

AGM belief revision theory, developed by Carlos Alchourrón, Peter Gärdenfors, and David Makinson in their foundational paper "On the Logic of Theory Change" (1985), provides a formal framework for rational belief revision. The theory specifies the rational conditions under which a belief state should change when an agent receives new information, particularly information that contradicts current beliefs.

## The Problem

When an agent receives new information E, three situations may arise:

**Expansion**: E is consistent with the current belief set. The agent simply adds E to their beliefs.

**Revision**: E is inconsistent with the current belief set. The agent wants to accept E but cannot do so without removing some prior beliefs. Which beliefs to remove?

**Contraction**: The agent decides to give up a belief E (perhaps because they receive evidence against E, or simply decide to retract it). What other beliefs must go?

AGM theory provides rational postulates (constraints) for each of these operations. The revision case is the most interesting: rational revision requires choosing which prior beliefs to give up in order to accommodate the new information.

## The AGM Postulates for Revision

Gärdenfors formulated postulates for rational revision. A revised belief state K*E (the result of revising K by E) should satisfy:

1. K*E is a belief set (closed under logical consequence).
2. E ∈ K*E (you do accept the new information).
3. K*E ⊆ K + E (you don't add anything beyond what's needed to accommodate E).
4. If ¬E ∉ K, then K + E = K*E (if E doesn't contradict K, revision is just expansion).
5. K*E is inconsistent only if E is a contradiction.
6. If E and F are logically equivalent, K*E = K*F (logically equivalent inputs give the same result).
7. K*(E ∧ F) ⊆ (K*E) + F (iterating revision).
8. If ¬F ∉ K*E, then (K*E) + F ⊆ K*(E ∧ F) (consistency with iterated revision).

These postulates constrain revision without fully determining it. Multiple operations can satisfy them; they provide necessary conditions for rationality, not a complete decision procedure.

## The Principle of Minimal Change

The informal principle behind AGM is *minimal change* (sometimes called "informational economy"): when revising beliefs, change as little as possible while accommodating the new information. Don't throw out more than you need to.

This principle is compelling but difficult to make precise. "Minimal change" must be cashed out in terms of some ordering over belief sets or possible worlds. Grove's "systems of spheres" representation, and the equivalent possible-worlds semantics (Lewis's similarity orderings), provide a way to make this precise: revision selects the "closest" possible worlds in which the new information is true.

## Limitations of AGM

**Iterated revision**: The original AGM framework doesn't handle iterated revision well. If you revise by E and then by F, what is the result? The postulates constrain the result of each single revision, but Postulates 7 and 8 don't fully determine the result of iterated revision. This is the *iterated revision problem*, addressed by Darwiche and Pearl's framework.

**Non-prioritized revision**: AGM assumes the new information always takes priority over prior beliefs. But sometimes we receive new information that doesn't necessarily override prior beliefs — we want to evaluate the new information against our background knowledge. Semi-revision and screened revision models address this.

**Dynamic belief revision vs. static snapshot**: AGM models a single revision event, not a process over time. For modeling agents who revise repeatedly, dynamic models are needed.

## Ranking Theory

Wolfgang Spohn's *Ranking Theory* (2012) provides an alternative to both AGM belief revision and Bayesian epistemology that aims to capture the dynamics of belief revision. Ranking functions assign to each possible world a "rank" — an integer measuring how surprising that world would be. Belief revision can then be modeled as adjustment of the ranking function.

Ranking theory has the advantage of handling both full beliefs (propositions with rank 0) and the dynamics of revision in a unified framework, without the binary character of AGM and without the probabilistic structure of Bayesianism.

## Applications

AGM belief revision has applications in:
- **Artificial intelligence**: AI systems that need to update knowledge bases when new information arrives
- **Law and database theory**: How legal databases or knowledge bases are updated when laws change or new facts emerge
- **Linguistics**: The semantics of conditionals — "if E had been true, what would you believe?" — connects to counterfactual reasoning and belief revision
