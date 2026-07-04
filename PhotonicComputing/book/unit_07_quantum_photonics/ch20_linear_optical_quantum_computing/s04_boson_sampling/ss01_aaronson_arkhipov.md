# 20.4.1 The Aaronson-Arkhipov Result

## The Problem, Stated Plainly

Fix a linear-optical network — an $m \times m$ unitary $U$ — and prepare $n$ single photons in the first $n$ input modes, $|1_1,\dots,1_n,0,\dots,0\rangle$. Let the photons pass through the interferometer and measure the output occupation $S = (s_1,\dots,s_m)$ with $\sum_j s_j = n$. The **boson-sampling problem** is simply to produce samples $S$ from the resulting distribution. There is no input to choose and no answer to read; the interferometer is drawn Haar-randomly and left fixed. Aaronson and Arkhipov (2011) proved that this featureless task is, under believable assumptions, beyond any efficient classical algorithm. Two design choices make the argument work: the interferometer is random (so its submatrices look like random Gaussian matrices), and the modes vastly outnumber the photons, $m \gg n^2$, so that photons almost never collide and each output is a distinct subset of modes.

## The Permanent, and Why Bosons Are Hard but Fermions Are Not

The amplitude to find output pattern $S$ given input pattern $T$ is governed by the **permanent** of the submatrix $U_{S,T}$ obtained by taking the rows and columns picked out by $S$ and $T$ (with multiplicity for repeated modes):

$$\langle S | \hat U | T \rangle = \frac{\operatorname{Perm}(U_{S,T})}{\sqrt{\prod_j s_j!\,\prod_k t_k!}}, \qquad \operatorname{Perm}(A) = \sum_{\sigma \in S_n}\prod_{i=1}^{n} A_{i\,\sigma(i)}.$$

The permanent is the determinant stripped of its signs. That one difference is decisive. The determinant, $\det(A) = \sum_\sigma (-1)^{\operatorname{sgn}\sigma}\prod_i A_{i\sigma(i)}$, is computable in $O(n^3)$ by Gaussian elimination — the signs enable cancellation and pivoting. Fermions scatter with amplitudes given by determinants (the antisymmetry of their wavefunction supplies the minus signs), so **fermion sampling is classically easy**. Bosons scatter with permanents: every permutation contributes with the same sign, no cancellation structure exists, and Valiant (1979) proved computing the permanent is **#P-hard** — as hard as counting the solutions to any NP problem, a class believed to dwarf NP itself. The best exact algorithm, Ryser's formula, runs in $O(2^n n)$, exponential in the photon number.

## Why Efficient Classical Sampling Would Collapse the Polynomial Hierarchy

Hardness of computing a single amplitude does not immediately imply hardness of *sampling* — a sampler need never name a probability. Aaronson and Arkhipov close the gap with a counting argument. Suppose a classical randomized algorithm could sample the boson-sampling distribution efficiently. Then Stockmeyer's theorem would let one approximate individual output probabilities $p(S) = |\operatorname{Perm}(U_{S,T})|^2 / \prod_j s_j!$ to within a multiplicative factor using an $\mathrm{NP}$ oracle, i.e. within the third level of the polynomial hierarchy (PH). But those probabilities are essentially squared permanents of random complex matrices, whose approximation is #P-hard. By Toda's theorem, $\mathrm{P}^{\#\mathrm{P}} \supseteq \mathrm{PH}$; a #P-hard quantity falling inside the third level of PH would force **PH to collapse to that level**. Because PH is universally believed to be infinite, no such classical sampler exists.

That chain proves hardness of *exact* sampling. Real machines sample only *approximately*, within some total-variation distance $\varepsilon$. Extending the argument there needs two still-unproven conjectures: the **permanent-of-Gaussians conjecture** (approximating $|\operatorname{Perm}(X)|^2$ for a random Gaussian $X$ is #P-hard) and an **anti-concentration conjecture** (the permanents are not too bunched near zero, so typical probabilities are large enough to matter). Both are widely believed and remain open — the load-bearing assumptions beneath every experimental advantage claim in Section 20.4.3. Note what boson sampling is *not*: it is not universal, carries no error correction, and solves no problem anyone posed. Its entire value is as a clean, physically minimal witness that quantum dynamics can outrun classical simulation.

## Worked Example: Permanents and Counting Outcomes

**Small permanents.** For a $2\times 2$ matrix,

$$\operatorname{Perm}\begin{pmatrix} a & b \\ c & d\end{pmatrix} = ad + bc,$$

differing from $\det = ad - bc$ only in the sign. For $3\times 3$ the permanent is the sum over all $3! = 6$ permutations, every term positive:

$$\operatorname{Perm}(A) = a_{11}a_{22}a_{33} + a_{11}a_{23}a_{32} + a_{12}a_{21}a_{33} + a_{12}a_{23}a_{31} + a_{13}a_{21}a_{32} + a_{13}a_{22}a_{31}.$$

By brute force the $n\times n$ permanent needs $n! \, (n{-}1)$ multiplications; Ryser's inclusion-exclusion formula reduces this to $O(2^n n)$ — still exponential, and the wall that classical simulation runs into.

**Counting outputs.** With $n$ photons in $m$ modes, the number of output occupation patterns (multisets, allowing collisions) is $\binom{m+n-1}{n}$, while the number of *collision-free* patterns (at most one photon per mode) is $\binom{m}{n}$. Take $n = 3$, $m = 9$: there are $\binom{11}{3} = 165$ patterns in all but $\binom{9}{3} = 84$ collision-free ones. The probability that two of $n$ photons land in the same mode scales as $\binom{n}{2}/m$; at $m \sim n^2$ this is $O(1)$, which is why the theorem demands $m \gg n^2$ — a bosonic "birthday paradox" that pushes the machine into the collision-free regime, where each amplitude is a permanent of a *distinct* $n\times n$ submatrix and the hardness argument applies cleanly.

**A permanent you already know.** The Hong-Ou-Mandel dip (Chapter 18) is boson sampling's $n=2$, $m=2$ special case. For a 50/50 splitter $U = \tfrac{1}{\sqrt2}\begin{pmatrix} 1 & i \\ i & 1\end{pmatrix}$ with one photon in each input, the amplitude to find one photon in each output is

$$\frac{\operatorname{Perm}(U)}{\sqrt{1}} = \operatorname{Perm}\!\left[\tfrac{1}{\sqrt2}\begin{pmatrix} 1 & i \\ i & 1\end{pmatrix}\right] = \tfrac{1}{2}(1\cdot 1 + i\cdot i) = 0.$$

The permanent vanishes — the coincidence amplitude cancels — which *is* HOM bunching, the founding two-photon interference of quantum optics. Boson sampling is this same interference scaled to $n$ photons and $m$ modes, where the permanent no longer collapses to a slogan but to a #P-hard sum with no efficient closed form.
