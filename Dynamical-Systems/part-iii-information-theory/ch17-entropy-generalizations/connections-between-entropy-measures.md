# 17.8 Connections Between Entropy Measures

Having introduced the main members of the entropy family, let's step back and map the relationships between them. These connections are not just organizational — they are often useful in proofs.

**Pinsker's Inequality** is the bridge between KL divergence and total variation distance:
$$\|P - Q\|_1 \leq \sqrt{2 D_{\text{KL}}(P\|Q)}.$$

KL divergence controls the $L^1$ distance between distributions. If two distributions are close in KL divergence, they are close in total variation. The converse is false — you can have $\|P - Q\|_1$ small while $D_{\text{KL}}(P\|Q) = \infty$ (if $P$ has support where $Q$ gives probability zero).

A sharper version is the **Bretagnolle-Huber-Carol inequality**:
$$\|P - Q\|_1^2 \leq 2(1 - e^{-D_{\text{KL}}(P\|Q)}).$$

This is tighter than Pinsker's at large divergences and has better constants in some regimes.

**Rényi ordering:** The Rényi entropies are totally ordered:
$$H_\infty(X) \leq H(X) \leq H_\alpha(X) \leq H_0(X) \quad \text{for } \alpha \leq 1.$$

Min-entropy is smallest; Hartley entropy is largest; Shannon entropy sits in between. This ordering quantifies the sense in which Shannon entropy is a "balanced" measure — neither as harsh as min-entropy (which only cares about the worst case) nor as forgiving as Hartley entropy (which ignores probabilities entirely).

**Chain rules:** One of the most important structural differences between entropy measures is how they behave under conditioning.

- Shannon entropy has an *exact* chain rule: $H(X|Y) = H(X,Y) - H(Y)$.
- Min-entropy has only an *inequality*: $H_\infty(X|Y) \leq H_\infty(X,Y) - H_\infty(Y)$.
- Rényi entropy has *no clean chain rule* in general.

This is why min-entropy and Rényi entropies are harder to work with in sequential arguments: you cannot decompose a conditional entropy into a difference of marginal entropies. One-shot information theory has developed workarounds — chain rules that hold up to additive corrections in smooth entropies — but they require more care than the classical case.

**Quantum analogues:** All the classical measures have quantum counterparts, and the relationships broadly survive quantization:
- Shannon entropy $\to$ von Neumann entropy $S(\rho)$
- KL divergence $\to$ quantum relative entropy $D(\rho\|\sigma) = \text{Tr}[\rho(\log\rho - \log\sigma)]$
- Rényi divergence $\to$ sandwiched Rényi divergence $\tilde{D}_\alpha(\rho\|\sigma)$ (preferred for operational tasks)
- Min-entropy $\to$ quantum min-entropy $H_\infty(A|B)_\rho$ (conditional on a quantum system)

The connections between these quantum measures are more subtle: there are multiple inequivalent ways to generalize KL divergence to non-commuting matrices, and different quantum analogues are appropriate for different tasks.

Together, these entropy measures form a rich family, each adapted to a particular operational setting. The skill is knowing which one to reach for. In the chapters that follow — network information theory, information geometry, quantum information — each will find its natural home.
