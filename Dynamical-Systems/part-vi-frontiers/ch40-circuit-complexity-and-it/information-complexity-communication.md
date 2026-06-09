# 40.6 Information Complexity and Communication

Information theory connects to circuit complexity through communication complexity. The Karchmer-Wigderson theorem says: the circuit depth of $f$ equals the communication complexity of a related communication game. This allows information-theoretic lower bounds on circuit depth via communication complexity.

**Theorem 40.6.1 (Information Complexity vs. Circuit Complexity).** For a Boolean function $f$ computed by a communication protocol with information complexity $IC(f)$:
$$C^{\text{cc}}(f) \geq IC(f) \geq \log C^{\text{circuit}}(f),$$
where $C^{\text{cc}}$ is communication complexity and $C^{\text{circuit}}$ is circuit complexity (via Karchmer-Wigderson).

Information complexity is the minimum amount of information the players must reveal to each other in any correct protocol for $f$. It's always at most the communication complexity (you can't communicate less information than you communicate), and it lower bounds the logarithm of the circuit complexity.

The information complexity approach to communication complexity has been remarkably successful. Using entropy bounds, you can prove tight communication complexity lower bounds for problems like Disjointness (the NDISJOINTNESS function requires $\Omega(n)$ bits of communication) that were previously proved by much more technical methods.

**Theorem 40.6.2 (Data Structure Lower Bounds via Entropy).** For a data structure that stores $n$ elements and answers queries:
- Any static data structure for predecessor queries requires $\Omega(\log\log n)$ probe time (van Emde Boas lower bound via entropy)
- Any dynamic data structure for union-find requires $\Omega(\alpha(n))$ amortized time (inverse Ackermann, by an entropy argument on the operation sequence)

The van Emde Boas bound is purely entropy-theoretic: a data structure that answers predecessor queries in $t$ probes can store at most $2^{O(n/2^{t})}$ elements efficiently, and the entropy bound forces $t \geq \log\log n$. The union-find bound is more subtle — the entropy argument uses the information-theoretic incompressibility of certain sequences of operations.

These data structure lower bounds show that information theory is not just for channels and sources — it's a tool for proving lower bounds for algorithms and data structures across all of computer science.
