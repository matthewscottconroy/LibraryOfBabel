# 26.3 Information Complexity and Circuit Lower Bounds

One of the great open problems in theoretical computer science is proving that $P \neq NP$ — that there are problems in NP that cannot be solved in polynomial time. A key step would be proving strong circuit lower bounds: showing that certain functions require exponentially large circuits to compute.

We know that most functions require exponential circuits (Shannon's 1949 counting argument), but we can't prove it for any *specific* function we care about (like SAT). The connection between communication complexity and circuit depth — the Karchmer-Wigderson theorem — gives the most structured approach to circuit lower bounds currently known.

## 26.3.1 Entropy and Circuit Complexity

**Theorem 26.3.1 (Shannon, 1949).** A random Boolean function $f: \{0,1\}^n \to \{0,1\}$ requires circuit size $\Omega(2^n / n)$. In particular, most functions require exponential circuit size.

*(proof)* Count: there are $2^{2^n}$ Boolean functions on $n$ bits. But circuits of size $s$ have at most $s^{2s}$ distinct functions. For $s = c \cdot 2^n/n$, this count is much smaller than $2^{2^n}$, so most functions cannot be computed by size-$s$ circuits.

**Information-Theoretic Interpretation:** The circuit size needed to compute $f$ is related to the Kolmogorov complexity $K(f)$ — a random function has $K(f) \approx 2^n$ bits, requiring exponential circuits.

The counting argument is entropy in disguise. There are $2^{2^n}$ Boolean functions — the entropy of a uniform random Boolean function is $2^n$ bits. A circuit of size $s$ can be described in $O(s \log s)$ bits (for each gate, specify its type and which gates feed into it). For $s = c \cdot 2^n / n$, the description is much shorter than $2^n$ bits, so most functions can't be computed by such circuits.

This is an entropy lower bound: to compute a function with $2^n$ bits of description complexity, you need a circuit with description complexity at least $2^n$ — which means size at least $2^n/n$.

## 26.3.2 Communication Complexity and Circuit Lower Bounds

**Theorem 26.3.2 (Karchmer-Wigderson).** The depth of a circuit computing $f: \{0,1\}^n \to \{0,1\}$ equals the communication complexity of the relation $KW_f \subseteq f^{-1}(1) \times f^{-1}(0) \times [n]$:
$$\text{depth}(f) = D^{rel}(KW_f),$$
where Alice holds $x \in f^{-1}(1)$, Bob holds $y \in f^{-1}(0)$, and they must find a coordinate $i$ with $x_i \neq y_i$.

The Karchmer-Wigderson theorem is one of the most elegant results in complexity theory. It says: depth-$d$ circuits are exactly protocols that solve the KW relation in $d$ rounds of communication. To prove that a function requires deep circuits, prove that the KW relation has high communication complexity.

The intuition: a circuit that computes $f$ "distinguishes" accepting inputs from rejecting inputs. To distinguish them, it must find some coordinate on which they differ — this is the communication task of the KW relation. Each gate of the circuit corresponds to one bit of the communication protocol.

**Theorem 26.3.3 (Monotone Circuit Lower Bounds via Communication).** The Karchmer-Wigderson relation for monotone circuits uses only monotone messages. Lower bounds on $D(KW_f)$ under monotone protocols give depth lower bounds for monotone circuits.

Using this connection and information-theoretic lower bounds on communication complexity, Razborov proved exponential lower bounds on monotone circuit complexity for matching. The information-theoretic approach to communication lower bounds (information complexity, direct sum theorems) is currently the most promising avenue for extending these lower bounds to general (non-monotone) circuits.

The reason this matters for our book: circuit complexity, communication complexity, and information theory are all measuring the same underlying resource — *information*. A circuit computes by manipulating bits; a communication protocol transmits bits; information theory measures bits of uncertainty. The connections between these three are not coincidental — they reflect the fact that "computation" is, fundamentally, the transformation and revelation of information.

This is the deepest connection in Chapter 26, and it points to open problems at the frontier of theoretical computer science. Whether information-theoretic methods can resolve $P \neq NP$ is unknown, but the Karchmer-Wigderson connection is our best current lead.
