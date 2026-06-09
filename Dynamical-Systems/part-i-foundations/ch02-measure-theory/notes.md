# Chapter 2 — Notes

---

The standard references for measure theory are Folland's *Real Analysis* (Chapters 1-3) and Royden-Fitzpatrick's *Real Analysis*. Folland is the one I'd recommend: it's clean, comprehensive, and the exercises are excellent. Royden is slightly more verbose, which some people find helpful and others find noisy.

For the probability theory side — Sections 2.6 and 2.7 — Durrett's *Probability: Theory and Examples* is the standard graduate text. It covers the law of large numbers, the CLT, martingales, and conditional expectation at a level that's directly relevant for ergodic theory. If you want the foundations done extremely carefully, Billingsley's *Probability and Measure* is beautiful, if dense.

The two theorems that recur most frequently in the rest of this book are the Radon-Nikodym theorem (Section 2.5) and conditional expectation (Section 2.6). If you feel shaky on either one, work through the material again before proceeding — the payoff is immediate. Conditional entropy $H(\xi | \eta)$ is built from $E[I(\xi)|\sigma(\eta)]$ where $I(\xi)$ is the information function; you can't make sense of entropy without conditional expectation.

Prokhorov's theorem (Section 2.8) is the measure-theoretic analog of Arzelà-Ascoli, and it plays exactly the same role in the theory of invariant measures that Arzelà-Ascoli plays in the theory of invariant functions. Both are compactness theorems that extract subsequential limits from sequences of objects satisfying a uniform condition (equicontinuity, or tightness). Keep this analogy in mind; it will help you remember which theorem to reach for.

One thing not covered here: the theory of *martingales* — sequences of random variables $E[X|\mathcal{F}_n]$ with nested $\sigma$-algebras $\mathcal{F}_1 \subseteq \mathcal{F}_2 \subseteq \cdots$. Martingale theory is a powerful tool for proving convergence results and has connections to the theory of filtrations and stopping times. The Birkhoff ergodic theorem can be proved using martingale methods, though we'll give a more direct argument. If martingales are new to you, Durrett has a thorough treatment.
