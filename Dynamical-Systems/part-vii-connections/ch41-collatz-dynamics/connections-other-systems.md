# 41.5 Connections to Other Dynamical Systems

The Collatz map doesn't stand alone. It connects to $p$-adic dynamics, automatic sequences, symbolic dynamics, and (more speculatively) $L$-functions and the Riemann hypothesis.

## 41.5.1 Connections to Number Theory

**Theorem 41.5.1 (Connection to $p$-adic Dynamics).** The Collatz map is a special case of a $(p,q)$-Collatz-type map: $C(n) = n/q$ if $q|n$, else $C(n) = pn+r$. The $3x+1$ map is $(p,q) = (3,2)$. These are related to Mahler's $p$-adic measure theory.

The general $(p,q)$-Collatz maps are interesting objects in their own right. For $(p,q) = (2,3)$ (the inverse problem: triple to halve), the dynamics is very different — orbits go to infinity. The special relationship $p/q = 3/2 > 1$ but $\log p / \log q = \log 3 / \log 2 < 2$ is what makes the average drift negative.

**Connection to Automatic Sequences:** A sequence $a_n$ is *$k$-automatic* if it is computed by a $k$-state automaton reading the base-$k$ expansion of $n$. The parity sequence of Collatz orbits — $b_n = C^n(m) \pmod 2$ — is not automatic (it's "too complex"). This is evidence that Collatz is not a simple recurrence.

## 41.5.2 Connections to Symbolic Dynamics

**Definition 41.5.2.** The *Collatz graph* $\mathcal{G}$ has vertices ${\mathbb N}$ and edges $n \to C(n)$. The conjecture says $\mathcal{G}$ has a unique absorbing strongly connected component $\{1, 2, 4\}$ (the $1 \to 2 \to 1$ loop and $4 \to 2 \to 1 \to 4$ cycle, with $4 \to 2 \to 1$ the absorbing path).

**Theorem 41.5.3 (Symbolic Representation).** Encode the Collatz orbit of $n$ by the sequence $\omega_n = (\omega_0, \omega_1, \ldots)$ where $\omega_k = C^k(n) \pmod 2$. This is a binary sequence (the parity sequence). The Collatz conjecture is equivalent to: every parity sequence is eventually periodic with the unique period $(0, 0)$ (the cycle $1 \to 2 \to 1$) or $(1, 1, 0)$ (the cycle $1 \to 4 \to 2 \to 1$).

The symbolic representation converts the Collatz conjecture into a statement about parity sequences: every Collatz orbit eventually enters one of the two known cycles. But analyzing parity sequences turns out to be just as hard as the original conjecture — the sequences are deterministic and complicated.

What makes this connection to symbolic dynamics useful is that it allows techniques from shift spaces and substitution systems to be brought to bear. The Collatz shift — the set of all parity sequences of Collatz orbits — is a closed shift-invariant subset of $\{0,1\}^\mathbb{N}$, and understanding its topological properties might give insight into the conjecture.
