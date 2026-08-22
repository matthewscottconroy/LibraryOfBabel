# Logic in Cryptography

> "The enemy knows the system. Design your cipher so that even if the enemy knows the algorithm, they cannot break it without the key."
> — Auguste Kerckhoffs (Kerckhoffs's principle, 1883)

## Cryptography is Applied Mathematics

Modern cryptography is not primarily about secret codes — it is about **provable security**. A modern cryptographic protocol comes with a mathematical proof: "Breaking this system is at least as hard as solving Problem X" — where Problem X (integer factorization, discrete logarithm, finding short lattice vectors) is believed to be computationally intractable.

Logic and formal methods play an increasingly central role in cryptography:
- **Correctness proofs** of cryptographic protocols (they do what they claim)
- **Security proofs** under formal threat models (they are hard to break)
- **Verification of implementations** (the code matches the specification)
- **Zero-knowledge proofs** (a logical construction for proving knowledge without revealing it)

## The RSA Cryptosystem: Logic and Number Theory

RSA (Rivest, Shamir, Adleman, 1977) is one of the most widely used public-key cryptosystems. Its security rests on the **computational hardness of integer factorization**.

**Key generation**:
1. Choose two large primes $p$ and $q$
2. Compute $n = p \cdot q$ (the **modulus**)
3. Compute $\phi(n) = (p-1)(q-1)$ (Euler's totient — the number of integers coprime to $n$ in $[1, n]$)
4. Choose $e$ with $\gcd(e, \phi(n)) = 1$ (the **public exponent**; commonly $e = 65537$)
5. Compute $d$ with $e \cdot d \equiv 1 \pmod{\phi(n)}$ (the **private exponent**; via extended Euclidean)

Public key: $(n, e)$. Private key: $(n, d)$.

**Encryption**: $c = m^e \bmod n$
**Decryption**: $m = c^d \bmod n$

**Why it works**: Euler's theorem states $m^{\phi(n)} \equiv 1 \pmod{n}$ for $\gcd(m, n) = 1$. Since $ed \equiv 1 \pmod{\phi(n)}$, we have $ed = 1 + k\phi(n)$ for some integer $k$, and:
$$c^d = (m^e)^d = m^{ed} = m^{1 + k\phi(n)} = m \cdot (m^{\phi(n)})^k \equiv m \cdot 1^k = m \pmod{n}$$

**Why it is secure (informally)**: Computing $d$ from $e$ and $n$ (without knowing $\phi(n)$) requires factoring $n$. Factoring large $n$ is believed computationally intractable — the best known algorithms take sub-exponential but super-polynomial time.

**Formal proof in Lean**:
```lean
import Mathlib.Data.ZMod.Basic
import Mathlib.NumberTheory.EulersTotient

-- Euler's theorem: m^φ(n) ≡ 1 (mod n) when gcd(m,n) = 1
#check ZMod.pow_totient  -- m ^ φ n = 1 in ZMod n (for units m)

-- The RSA correctness proof follows from:
-- (m^e)^d = m^(ed) = m^(1 + k*φ(n)) = m * (m^φ(n))^k ≡ m (mod n)
```

## Zero-Knowledge Proofs

One of the most striking applications of logic to cryptography is the **zero-knowledge proof** (Goldwasser, Micali, Rackoff, 1985).

**The problem**: Peggy wants to prove to Victor that she knows a secret (e.g., the solution to a problem) without revealing *anything* about the secret itself.

**Example — graph 3-coloring**: Given a graph $G$, Peggy claims to know a valid 3-coloring (assign colors to vertices so no two adjacent vertices share a color). She wants to prove this to Victor without revealing the coloring.

**The protocol** (repeated $k$ times):
1. Peggy randomly permutes the three colors and commits to the coloring (sends locked boxes for each vertex)
2. Victor picks a random edge $(u, v)$
3. Peggy opens the boxes for $u$ and $v$, revealing their (permuted) colors
4. Victor verifies: the colors are different ✓

If Peggy is cheating (does not have a valid coloring), she has at most a $\frac{E-1}{E}$ chance of surviving each round (where $E$ is the number of edges). After $k = \mathcal{O}(E)$ rounds, the probability of cheating and surviving is negligibly small.

**Zero-knowledge property**: Victor learns that the two revealed colors are different at the challenge edge — but since the colors are *randomly permuted each round*, he learns nothing about the actual coloring. Each round gives him only: "these two vertices have different colors at a randomly chosen edge" — which he already knew was true of any valid coloring.

**Formal properties**:
- **Completeness**: If Peggy has a valid coloring, she convinces Victor with probability 1
- **Soundness**: If Peggy is cheating, Victor catches her with high probability
- **Zero-knowledge**: Victor learns nothing about the coloring beyond the fact that it exists

## Formal Protocol Verification

Modern cryptographic protocols (TLS, SSH, Signal, blockchain consensus) are complex and subtle. Formal verification tools verify their security properties:

**ProVerif** (Bruno Blanchet): A protocol verifier that models protocols as Horn clauses and checks security properties automatically. Used to verify:
- TLS 1.3 (the HTTPS protocol)
- Signal protocol (WhatsApp, Signal messenger)
- Various authentication protocols

**CryptoVerif**: Similar to ProVerif but produces computational security proofs rather than symbolic ones — connects to real-world hardness assumptions.

**Tamarin Prover**: Verifies security protocols with complex key management, including forward secrecy and post-compromise security. Used for 5G authentication protocols.

**Example (simplified ProVerif-style modeling)**:
```
(* Model a simple authentication protocol *)
(* A -> B: {A, n_A}_{K_B}  -- A sends nonce encrypted for B *)
(* B -> A: {n_A, n_B}_{K_A} -- B responds with both nonces *)
(* A -> B: {n_B}_{K_B}      -- A confirms nonce *)

(* Security property: after the protocol, B knows A is authentic *)
(* ProVerif proves: if Dolev-Yao attacker cannot learn private keys, *)
(*   then no attacker can make B believe A authenticated when A didn't *)
```

## Lattice-Based Cryptography and Post-Quantum Security

Current cryptographic systems (RSA, elliptic curves) will be broken by a sufficiently powerful quantum computer using **Shor's algorithm**. The mathematical basis: Shor's algorithm efficiently solves integer factorization and discrete logarithm on quantum hardware — the exact hard problems that RSA and ECC rely on.

**Post-quantum cryptography** replaces these with problems believed hard even for quantum computers. The leading candidate: **lattice problems**.

A **lattice** is a discrete additive subgroup of $\mathbb{R}^n$ — the set of all integer linear combinations of basis vectors. Key hard problems:
- **SVP** (Shortest Vector Problem): Find the shortest non-zero vector in a lattice
- **CVP** (Closest Vector Problem): Find the lattice point closest to a given point
- **LWE** (Learning With Errors): Solve a system of approximate linear equations over a finite field

**Formal security proof**: "Breaking CRYSTALS-Kyber (an LWE-based key exchange) is at least as hard as solving the Module-LWE problem." This is a *reduction* — a logical proof that the protocol's security follows from the assumed hardness of the mathematical problem.

The **NIST Post-Quantum Standardization** (2016–2024) selected several algorithms:
- **CRYSTALS-Kyber** (key encapsulation, based on Module-LWE)
- **CRYSTALS-Dilithium** (digital signatures, based on Module-LWE)
- **SPHINCS+** (signatures, based on hash functions — minimal algebraic assumptions)

All come with security proofs — formal reductions showing that breaking the cryptosystem implies solving a hard mathematical problem.

## Logic and Blockchain

Blockchain smart contracts present a verification challenge: **the code is law**. When a smart contract says "transfer $X to whoever calls this function," it will do exactly that — regardless of whether that was intended.

The DAO attack (2016, Ethereum): \$60 million stolen through a reentrancy vulnerability. The code was "correct" — it did what it said. But what it said was not what was intended.

**Formal verification of smart contracts**:
- **Solidity verifiers** (K framework, Certora Prover): Verify that a smart contract satisfies a formal specification written in predicate logic
- **Coq-certified contracts**: Write the smart contract in Coq, prove it correct, then extract to executable code (or compile the certified specification)
- **SMT-based tools** (Manticore, Mythril): Use Z3/CVC5 to find property violations automatically

**Example Z3 verification of a token contract property**:
```python
from z3 import *

# Model token balances and transfer
balances = Array('balances', IntSort(), IntSort())
sender, recipient, amount = Ints('sender recipient amount')
sender_balance = Select(balances, sender)

# Precondition: sender has enough tokens
pre = sender_balance >= amount

# New balances after transfer
new_balances = Store(Store(balances, sender, sender_balance - amount),
                     recipient, Select(balances, recipient) + amount)

# Postcondition: conservation of tokens
# Total supply is preserved: all balances sum to the same amount
# (simplified to a two-account check)
post = (Select(new_balances, sender) + Select(new_balances, recipient) ==
        Select(balances, sender) + Select(balances, recipient))

# Prove postcondition holds whenever precondition holds
s = Solver()
s.add(pre)
s.add(Not(post))
result = s.check()
print("Conservation violated?" , result == sat)  # unsat means safe
```

## Exercises
See [problems/ch13_applications/03_cryptography_problems.md](../../../problems/ch13_applications/03_cryptography_problems.md)
