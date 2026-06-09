# 26.2 Communication Complexity

Communication complexity asks: how much information must two parties exchange to compute a function of their combined inputs? This seems like a practical engineering question, but it turns out to be one of the most powerful tools in theoretical computer science — both for proving lower bounds on distributed computation and, through connections we'll see in Section 26.3, for proving lower bounds on circuit complexity.

## 26.2.1 The Model

Alice holds $x \in \mathcal{X}$ and Bob holds $y \in \mathcal{Y}$. They want to compute $f(x, y)$ for some function $f: \mathcal{X} \times \mathcal{Y} \to \mathcal{Z}$. They communicate by sending bits back and forth — they're allowed any protocol, but they pay for each bit.

**Definition 26.2.1.** The *communication complexity* of a function $f: \mathcal{X} \times \mathcal{Y} \to \mathcal{Z}$ is the minimum number of bits Alice (holding $x \in \mathcal{X}$) and Bob (holding $y \in \mathcal{Y}$) must exchange to compute $f(x,y)$, in the worst case over all $(x,y)$.

**Definition 26.2.2 (Communication Protocols).** A *deterministic protocol* is a binary tree where:
- Internal nodes are labeled by Alice or Bob (who speaks)
- Edges are labeled by bits
- Leaves are labeled by output values

The cost is the depth of the tree. The *deterministic communication complexity* $D(f)$ is the minimum cost over all protocols.

**Randomized complexity** $R(f)$: protocols that may use shared/private randomness, with error probability $\leq 1/3$.

**Quantum complexity** $Q(f)$: protocols where parties exchange qubits and share entanglement.

The model is clean but deceptively powerful. The key fact: deterministic communication complexity $D(f) \geq \log_2(\text{number of rows in the communication matrix})$ — Alice must send enough bits to distinguish all possible inputs she might have. This gives a lower bound from a linear-algebraic argument (rank lower bound).

But the most powerful lower bounds come from information theory.

## 26.2.2 Lower Bounds via Information Theory

The idea: any correct protocol must convey information about both inputs. The information complexity of a protocol measures exactly how much information is revealed about each party's input by the protocol transcript.

**Definition 26.2.3 (Information Complexity).** The *information complexity* of a protocol $\Pi$ with respect to distribution $\mu$ is:
$$IC_\mu(\Pi) = I(X; \Pi | Y) + I(Y; \Pi | X),$$
where $(X,Y) \sim \mu$ and $\Pi$ is the transcript. The *information complexity* of $f$ at $\mu$ is $IC_\mu(f) = \inf_{\Pi \text{ computes } f} IC_\mu(\Pi)$.

The two terms measure: how much does the transcript reveal about $X$ (to Bob, who already knows $Y$), and how much does it reveal about $Y$ (to Alice, who already knows $X$)? The sum is the total information exchanged about the inputs.

**Theorem 26.2.4 (Information Complexity Lower Bounds Communication).** For any distribution $\mu$:
$$D(f) \geq IC_\mu(f).$$

The proof is immediate: the transcript $\Pi$ is a sequence of bits, and the information content of $\Pi$ is at most the number of bits exchanged (since each bit contributes at most 1 bit to the mutual information). So the communication cost bounds the information complexity.

**Theorem 26.2.5 (Equality Lower Bound).** The communication complexity of the equality function $EQ_n(x,y) = [x = y]$ satisfies:
$$D(EQ_n) = n, \quad R(EQ_n) = O(\log n).$$

The randomized protocol: Alice sends a hash of $x$; this fails with probability $\leq 1/n$.

For deterministic equality testing: Alice and Bob must distinguish all $2^n$ possible pairs $(x, y)$ with $x = y$ from all pairs with $x \neq y$. The communication matrix of $EQ_n$ is the identity matrix, which has full rank $2^n$, requiring $n$ bits. But with randomization and hashing, they can succeed with only $O(\log n)$ bits: Alice sends $h(x)$ for a random hash function $h: \{0,1\}^n \to \{0,1\}^k$ with $k = O(\log n)$. Bob checks if $h(y) = h(x)$. The error probability is $\leq 2^{-k} = 1/n$ by universal hashing.

**Theorem 26.2.6 (Disjointness Lower Bound — Kalyanasundaram-Schnitger, Razborov).** The communication complexity of $DISJ_n(x,y) = [x \cap y = \emptyset]$ for $x,y \subseteq [n]$ satisfies:
$$R(DISJ_n) = \Omega(n).$$

*(proof sketch)* The information cost per coordinate is $\Omega(1)$ for any correct protocol. By a direct sum argument (each coordinate is independent under the hard distribution), the total cost is $\Omega(n)$.

The disjointness lower bound is the central result in communication complexity. Randomization doesn't help: even with shared randomness and error probability $1/3$, deciding whether two $n$-element sets are disjoint requires $\Omega(n)$ bits of communication. The proof via information complexity is clean: under the hard distribution (each coordinate is independently in $x \cap y$ with probability $1/2$), any correct protocol must convey $\Omega(1)$ bits of information per coordinate, and by the direct sum theorem, the total is $\Omega(n)$.

## 26.2.3 Direct Sum Theorems

**Theorem 26.2.7 (Direct Sum for Information Complexity — Bar-Yossef et al.).** Computing $f$ on $k$ independent instances requires $k \cdot IC(f)$ bits of information:
$$IC_{\mu^k}(f^k) = k \cdot IC_\mu(f).$$

**Remark 26.2.8.** Direct sum theorems are the key to proving communication complexity lower bounds for composed functions. The direct sum holds for information complexity but not always for communication complexity (an important distinction).

The direct sum theorem for information complexity is perhaps the deepest result in the area. It says: you can't "amortize" information — computing $k$ instances of $f$ requires $k$ times as much information as computing one instance. This is essentially the AEP for communication protocols: independent instances require independent information.

The failure of direct sum for communication complexity (as opposed to information complexity) is a subtlety that drove years of research. There are functions where computing $k$ instances is cheaper than $k$ times the cost of one instance, because the protocol for the $k$ instances can share some communication. Information complexity captures the "true" cost, while communication complexity can be smaller due to amortization effects.
