# 40.1 Boolean Circuits and Complexity

A Boolean circuit is the computational model that underlies most complexity theory. It's simpler than a Turing machine — a fixed, acyclic circuit with no loops — and it's the right model for studying non-uniform computation and proving lower bounds.

**Definition 40.1.1.** A *Boolean circuit* $C$ on $n$ inputs is a directed acyclic graph where:
- Input nodes: labeled $x_1, \ldots, x_n$
- Internal gates: labeled AND ($\wedge$), OR ($\vee$), NOT ($\neg$) with fan-in 2 (or unlimited for *unbounded fan-in*)
- One output node

The *size* $|C|$ is the number of gates; the *depth* is the length of the longest path from input to output.

Size measures the total computational work; depth measures the parallel computation time. Both are important, and trading one for the other is a key theme.

**Definition 40.1.2 (Circuit Complexity Classes).** 
- $\mathbf{P/poly}$: functions computable by polynomial-size circuits
- $\mathbf{NC}^1$: functions computable by $O(\log n)$-depth, polynomial-size circuits  
- $\mathbf{AC}^0$: functions computable by constant-depth, polynomial-size circuits with unbounded fan-in AND/OR gates
- $\mathbf{ACC}^0$: $\mathbf{AC}^0$ augmented with $\text{MOD}_m$ gates (modular counting)

Hierarchy: $\mathbf{NC}^1 \subseteq \mathbf{AC}^0 \subseteq \mathbf{ACC}^0 \subseteq \mathbf{NC} \subseteq \mathbf{P/poly}$.

The key open question: is $\mathbf{P} \subseteq \mathbf{P/poly}$? (Yes, trivially — polynomial-time algorithms give polynomial-size circuits.) But is $\mathbf{NP} \subseteq \mathbf{P/poly}$? If not, then $\mathbf{P} \neq \mathbf{NP}$. We believe NP $\not\subseteq$ P/poly but cannot prove it.

The hierarchy within circuit classes is better understood. We know AC$^0 \subsetneq$ NC$^1$, NC$^1 \subsetneq$ NC, and many other separations within the hierarchy. What we cannot do is compare NP to P/poly unconditionally.
