# Chapter 34 — Sofic Groups and Sofic Entropy

> *Sofic groups are groups that can be approximated by finite symmetric groups. For actions of sofic groups, Lewis Bowen defined a new notion of entropy in 2010 that extends Kolmogorov-Sinai entropy beyond the amenable setting. This is one of the most significant developments in ergodic theory in decades.*

**Prerequisites:** Chapter 7 (KS entropy, Ornstein's theorem), Chapter 33 (orbit equivalence, amenable groups), Chapter 12 (symbolic dynamics, sofic shifts).

---

## What This Chapter Is About

Kolmogorov-Sinai entropy was defined in 1958. It is one of the great achievements of ergodic theory — a number that classifies Bernoulli shifts and measures the "complexity" of any measure-preserving system. For fifty years, KS entropy worked beautifully, but only for actions of amenable groups. For non-amenable groups, nobody knew what entropy should even mean.

Then in 2010, Lewis Bowen defined sofic entropy, and everything changed.

The key insight was to use "microstates" — finite approximations to the infinite action. If you can approximate the group by finite symmetric groups (this is the sofic condition), then you can count how many ways to approximate the action on finite sets. The logarithm of that count, normalized, gives you entropy. For amenable groups, this recovers KS entropy exactly. For non-amenable groups, it gives something genuinely new.

Bowen's first application was immediate and spectacular: he proved that two Bernoulli shifts of a free group are isomorphic if and only if they have the same base entropy. This extended Ornstein's theorem — which classified Bernoulli shifts for amenable groups by entropy — to the non-amenable setting.

There are two open problems that define the frontier here. First: does every group have a sofic approximation? No non-sofic group is known to exist. This is one of the biggest open problems in group theory. Second: does sofic entropy depend on the choice of sofic approximation? Kerr and Li proved independence for all actions, but this was a major theorem, not obvious at all. And for actions of groups where the sofic approximation matters, the story is still being written.

---

## Sections

- [34.1 Sofic Groups](sofic-groups.md)
- [34.2 Bowen's Sofic Entropy](bowens-sofic-entropy.md)
- [34.3 The Non-Bernoulli Problem and Sofic Entropy](non-bernoulli-problem.md)
- [34.4 Entropy Beyond Sofic Groups](entropy-beyond-sofic-groups.md)
- [34.5 Topological Sofic Entropy](topological-sofic-entropy.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
