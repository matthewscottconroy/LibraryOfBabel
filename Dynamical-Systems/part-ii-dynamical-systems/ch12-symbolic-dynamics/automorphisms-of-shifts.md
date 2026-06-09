# 12.8 Automorphisms of Shifts

A self-conjugacy of a shift — a homeomorphism of the shift space that commutes with the shift — is called an automorphism. The collection of all automorphisms forms the *automorphism group*, and it is a surprisingly rich algebraic object. Understanding this group is both intrinsically interesting and practically important for classification.

**Definition 12.8.1.** The *automorphism group* $\text{Aut}(\sigma)$ of the full shift $(\mathcal{A}^{\mathbb Z}, \sigma)$ consists of all homeomorphisms $\phi: \mathcal{A}^{\mathbb Z} \to \mathcal{A}^{\mathbb Z}$ that commute with $\sigma$: $\phi \circ \sigma = \sigma \circ \phi$.

What do such homeomorphisms look like? The key result says they must all be sliding block codes:

**Theorem 12.8.2 (Curtis-Hedlund-Lyndon).** Every automorphism of a shift is a *sliding block code*: there exists a window size $N$ and a function $\Phi: \mathcal{A}^{2N+1} \to \mathcal{A}$ such that $\phi(x)_n = \Phi(x_{n-N}, \ldots, x_{n+N})$.

This is a remarkable rigidity result. The homeomorphism must be "local" — the output at position $n$ depends only on a finite window of input symbols around position $n$. There are no long-range correlations possible. Any homeomorphism that commutes with the shift must see the world through a finite window.

**Theorem 12.8.3.** The automorphism group $\text{Aut}(\sigma)$ of the full 2-shift is a countable group containing:
- All finite-order homeomorphisms arising from finite permutations of $\mathcal{A}$ and of symbol windows
- All powers of $\sigma$ (so $\mathbb{Z} \hookrightarrow \text{Aut}(\sigma)$), which form the center: $\text{Center}(\text{Aut}(\sigma)) = \langle \sigma \rangle$ (Ryan's theorem)
- The *marker automorphisms* — automorphisms constructed by "marking" specific combinatorial patterns in sequences and locally rearranging them
- Free groups and other exotic algebraic structures

What this is saying is: the automorphism group of the full shift is enormously complicated. It contains every finite group, free groups of all ranks, and many other groups that appear exotic. Ryan's theorem says that the only automorphisms that commute with *everything else* are powers of the shift — a strong rigidity statement at the center of the group.

The automorphism group of an SFT (rather than the full shift) is often much smaller and more tractable. For example, the automorphism group of the golden mean shift has been analyzed carefully and is significantly more constrained than the full-shift case.

The Curtis-Hedlund-Lyndon theorem has a broader significance: it characterizes all cellular automata. Every cellular automaton is a sliding block code on the full shift, and vice versa. Automorphisms of shifts are invertible cellular automata. The automorphism group of the full shift is thus the group of invertible cellular automata — a group of fundamental importance in theoretical computer science.
