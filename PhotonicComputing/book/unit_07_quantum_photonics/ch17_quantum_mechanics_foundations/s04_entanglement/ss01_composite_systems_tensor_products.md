# 17.4.1 Composite Systems and Tensor Products

## The Composition Postulate

**Postulate (composition).** *The Hilbert space of a composite system is the tensor product of the component spaces:*

$$\mathcal{H}_{AB} = \mathcal{H}_A \otimes \mathcal{H}_B$$

If $\{|i\rangle_A\}$ is a basis of $\mathcal{H}_A$ (dimension $d_A$) and $\{|j\rangle_B\}$ of $\mathcal{H}_B$ (dimension $d_B$), then $\{|i\rangle_A \otimes |j\rangle_B\}$ is a basis of $\mathcal{H}_{AB}$, which has dimension $d_A d_B$ — *product*, not sum. We abbreviate $|i\rangle_A \otimes |j\rangle_B \equiv |ij\rangle$. Two qubits live in a 4-dimensional space spanned by $\{|00\rangle, |01\rangle, |10\rangle, |11\rangle\}$; $n$ qubits live in $2^n$ dimensions. This exponential growth — 300 qubits outstrip the number of atoms in the observable universe — is the raw capacity quantum computing draws on, and the reason classical simulation of quantum systems chokes (Feynman's 1982 observation that opened this unit).

Operators compose likewise: $\hat{A} \otimes \hat{B}$ acts as $(\hat{A}\otimes\hat{B})|ij\rangle = (\hat{A}|i\rangle)\otimes(\hat{B}|j\rangle)$, and "act on A alone" means $\hat{A}\otimes\mathbb{1}_B$. Inner products factor: $(\langle i'|\otimes\langle j'|)(|i\rangle\otimes|j\rangle) = \langle i'|i\rangle\langle j'|j\rangle$.

## Product States versus Entangled States

A **product (separable) state** has each subsystem in a definite pure state of its own:

$$|\psi\rangle_{AB} = |\phi\rangle_A \otimes |\chi\rangle_B$$

But $\mathcal{H}_{AB}$ contains far more than products: *superpositions* of product states are states too, and most cannot be refactored. The standard example:

$$|\Phi^+\rangle = \frac{|00\rangle + |11\rangle}{\sqrt{2}}$$

Suppose $|\Phi^+\rangle = (\alpha|0\rangle + \beta|1\rangle)\otimes(\gamma|0\rangle + \delta|1\rangle)$. Matching coefficients: $\alpha\gamma = \beta\delta = 1/\sqrt{2}$ but $\alpha\delta = \beta\gamma = 0$. The last pair forces one factor of each cross product to vanish, contradicting the first pair. No factorization exists: $|\Phi^+\rangle$ is **entangled**. A pure state is entangled precisely when it is not a product.

The physical reading: in $|\Phi^+\rangle$, neither qubit *has* a state. Ask "what is the polarization of photon A?" and quantum mechanics declines to answer — not out of ignorance, but because the question presupposes a local property that does not exist. What is definite is *joint*: the two qubits are certainly identical in the $\{|0\rangle,|1\rangle\}$ basis (and, as 17.4.2 shows, correlated in every other basis too).

## The Partial Trace and Reduced States

If we hold only subsystem A — photon B has flown to another lab — what do our local measurements see? The answer is obtained by the **partial trace** over B:

$$\rho_A = \mathrm{Tr}_B\left(\rho_{AB}\right) = \sum_j \langle j|_B\, \rho_{AB}\, |j\rangle_B$$

$\rho_A$ is the unique operator reproducing all local statistics: $\langle \hat{A}\otimes\mathbb{1}\rangle = \mathrm{Tr}(\rho_A \hat{A})$ for every local observable. Here density matrices earn their keep: even when the *global* state is pure, the *local* state generally is not.

**Worked example.** For $|\Phi^+\rangle$:

$$\rho_{AB} = \frac{1}{2}\left(|00\rangle\langle 00| + |00\rangle\langle 11| + |11\rangle\langle 00| + |11\rangle\langle 11|\right)$$

Tracing over B kills the cross terms ($\langle 0|1\rangle = 0$):

$$\rho_A = \frac{1}{2}\left(|0\rangle\langle 0| + |1\rangle\langle 1|\right) = \frac{\mathbb{1}}{2}$$

The reduced state is **maximally mixed** — purity $\mathrm{Tr}(\rho_A^2) = 1/2$, Bloch vector zero. Locally, each photon of a polarization-entangled Bell pair is completely unpolarized: every polarizer at every angle transmits it with probability exactly $1/2$. All the state's definiteness resides in correlations that no local measurement can see. This is the converse of 17.1.1's third source of mixedness, now made quantitative: *local mixedness of a globally pure state is the signature — and, in 17.4.4, the measure — of entanglement.*

Two practical corollaries. First, entanglement cannot be detected locally, and it cannot be used to signal: Bob's local statistics ($\rho_B = \mathbb{1}/2$) are unchanged by anything Alice does to her photon (measure, rotate, discard), which is why entanglement coexists peacefully with relativity even while violating Bell inequalities. Second, **decoherence is entanglement with the environment**: a qubit that entangles with stray degrees of freedom (a which-path marker, a scattered photon, timing jitter) has, from the local view, been partially traced — its coherences fade. Distinguishability *is* entanglement with an unwanted system; this identity will do real work in the Hong-Ou-Mandel analysis of Section 18.2.

## The Schmidt Decomposition

Every bipartite *pure* state can be written in a canonical form. There exist orthonormal bases $\{|u_k\rangle_A\}$, $\{|v_k\rangle_B\}$ and non-negative reals $\lambda_k$ (Schmidt coefficients, $\sum_k \lambda_k^2 = 1$) such that

$$|\psi\rangle_{AB} = \sum_k \lambda_k\, |u_k\rangle_A \otimes |v_k\rangle_B$$

with the number of nonzero terms — the **Schmidt rank** — at most $\min(d_A, d_B)$. (The proof is the singular value decomposition of the coefficient matrix $c_{ij}$, one more instance of this book's recurring theme that SVD organizes everything.) The Schmidt form makes structural facts obvious: the state is a product iff its Schmidt rank is 1; the reduced states are $\rho_A = \sum_k \lambda_k^2 |u_k\rangle\langle u_k|$ and $\rho_B = \sum_k \lambda_k^2|v_k\rangle\langle v_k|$ — *identical spectra*, so both halves of a pure state are exactly equally mixed; and the entanglement entropy of 17.4.4 will be just the Shannon entropy of $\{\lambda_k^2\}$. In photonics, the Schmidt decomposition of a photon pair's joint spectral amplitude quantifies spectral entanglement, and Chapter 19 will demand Schmidt rank $\approx 1$ (spectral *purity*) from heralded single-photon sources — an unusual case where the engineering goal is to *avoid* entanglement in one degree of freedom (frequency) while keeping it in another (polarization).
