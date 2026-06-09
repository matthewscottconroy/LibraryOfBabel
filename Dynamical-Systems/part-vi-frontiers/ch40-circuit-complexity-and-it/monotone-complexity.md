# 40.4 Monotone Complexity

Monotone circuits use only AND and OR gates — no NOT gates. Many natural combinatorial problems (clique, matching, connectivity) can be expressed as monotone functions, and monotone circuit complexity is better understood than general circuit complexity. Razborov's 1985 theorem proved exponential lower bounds for clique via the approximation method.

**Definition 40.4.1.** A circuit is *monotone* if it uses no NOT gates. The monotone circuit complexity $C^+(f)$ is the minimum size over all monotone circuits computing $f$.

**Theorem 40.4.2 (Razborov, 1985 — Clique Requires Exponential Monotone Circuits).** The clique function $\text{CLIQUE}_{k,n}$ (does the $n$-vertex graph have a $k$-clique?) requires monotone circuit size $\exp(\Omega(k))$ for $k = n^{1/4}$.

*(proof outline)* Razborov's *approximation method*: any monotone circuit for CLIQUE must approximate two distributions — random cliques and random sets without large cliques. Entropy bounds show no small monotone circuit can separate them.

The approximation method works by replacing each gate in the circuit with a "sunflower approximation" — a simpler Boolean function that is close to the gate on most inputs but is easy to analyze. After approximating all gates, you can count the number of approximators and bound the circuit size.

The entropy argument: to distinguish "has a clique" from "doesn't have a clique," the circuit must encode enough information to identify the clique. But monotone circuits of size $s$ can only encode $O(s)$ bits about the input. A clique of size $k$ requires $\Omega(k \log n)$ bits to specify. For $k = n^{1/4}$, this gives exponential lower bounds.

**Theorem 40.4.3 (Alon-Boppana, 1987).** For the bipartite matching problem on $n \times n$ bipartite graphs, monotone circuits require size $\exp(\Omega(n^{1/5}))$.

The Alon-Boppana theorem uses a different technique — the "sunflower lemma" of Erdős-Ko-Rado — combined with the approximation method. Together, these results show that many natural graph problems require exponential monotone circuits.

What about non-monotone circuits? This is where the difficulty lies. The clique function might have polynomial-size non-monotone circuits — we don't know. Monotone circuit complexity and general circuit complexity are separated: non-monotone circuits can be exponentially more efficient than monotone ones for some functions (bipartite perfect matching being the key example). So monotone lower bounds don't directly imply general lower bounds.
