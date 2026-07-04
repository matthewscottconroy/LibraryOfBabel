# 20.4.2 Gaussian Boson Sampling

## Squeeze Instead of Herald

The Aaronson-Arkhipov machine needs $n$ identical single photons arriving at the interferometer at once. That is the hardest thing in Chapter 19: heralded SPDC sources fire probabilistically, so producing $n$ simultaneous photons succeeds with probability scaling as (per-source rate)$^n$, and even quantum-dot sources demand demultiplexing. Gaussian boson sampling (GBS), introduced by Hamilton et al. (2017), sidesteps the bottleneck by changing the input. Instead of Fock states, inject **single-mode squeezed vacuum** into every input port. Squeezing is produced *deterministically* by a parametric process — turn up the pump and the squeezed light is simply there, every pulse, no heralding required. The squeezed states pass through the same kind of Haar-random interferometer and are counted at the output. Removing the requirement for on-demand single photons is what made GBS the platform of choice for the largest experiments (Section 20.4.3).

## The Hafnian Replaces the Permanent

Changing the input changes the matrix function that governs the output statistics. For Gaussian inputs the probability of a photon-number pattern $S$ is proportional to the **Hafnian** of a matrix built from the output pattern and the network's covariance matrix:

$$p(S) \;\propto\; \frac{\big|\operatorname{Haf}(A_S)\big|^2}{\prod_j s_j!}, \qquad \operatorname{Haf}(A) = \sum_{M \in \mathrm{PMP}(2n)} \prod_{(i,j)\in M} A_{ij},$$

where the sum runs over all **perfect matchings** of $2n$ objects. The Hafnian counts weighted perfect matchings of a graph exactly as the permanent counts perfect matchings of a *bipartite* graph — indeed the permanent is a special case, $\operatorname{Perm}(B) = \operatorname{Haf}\!\left(\begin{smallmatrix} 0 & B \\ B^{\mathsf T} & 0\end{smallmatrix}\right)$. The Hafnian is #P-hard for the same sign-free reason as the permanent, so GBS inherits an Aaronson-Arkhipov-style hardness argument (resting on analogous, still-open, conjectures). When the detectors only click ("photon or no photon") rather than count, the relevant function becomes the **Torontonian** (Quesada et al., 2018), a sum over the Hafnian's sub-patterns that captures threshold detection and is likewise classically hard.

## The Physical Matrix Behind the Hafnian

The matrix $A_S$ is not arbitrary — it is carved from the device. In the pure-state limit, the Gaussian state leaving an interferometer $U$ fed with squeezed vacua of parameters $r_j$ is described by $A = U\,\operatorname{diag}(\tanh r_1,\dots,\tanh r_m)\,U^{\mathsf T}$; a detection pattern $S$ selects the rows and columns of $A$ picked out by the occupied modes (with repetition) to build $A_S$, and the probability is $|\operatorname{Haf}(A_S)|^2$ normalized by the covariance-matrix determinant. Because $A$ is symmetric, it is the weighted adjacency matrix of a graph, and its Hafnian is that graph's perfect-matching sum — the exact bridge to the graph-theoretic applications below.

## A Feature, Not Only a Stunt

Because the Hafnian is the graph-theoretic perfect-matching function, a GBS device programmed with an interferometer encoding a symmetric matrix $A$ *samples subgraphs weighted by their matching structure*. This connects GBS to real computational problems: identifying dense subgraphs and maximum cliques, computing graph similarity, and — through a mapping between vibrational modes and optical modes (Huh et al., 2015) — predicting molecular vibronic spectra. These applications, and the Xanadu programme that pursues them commercially, belong to the continuous-variable story of Chapter 21; here the point is only that GBS is not purely a hardness demonstration but a sampler whose output has exploitable structure.

## Worked Example: A Four-Photon Hafnian and Squeezed-Light Statistics

**Hafnian of a $4\times 4$ matrix.** For a symmetric $A$ (diagonal irrelevant), the perfect matchings of the four indices $\{1,2,3,4\}$ are $\{(12)(34)\}$, $\{(13)(24)\}$, $\{(14)(23)\}$ — the $(4{-}1)!! = 3$ pairings — so

$$\operatorname{Haf}(A) = A_{12}A_{34} + A_{13}A_{24} + A_{14}A_{23}.$$

A $6\times 6$ Hafnian already has $5!! = 15$ terms and a $2n\times 2n$ one has $(2n-1)!!$; the best known algorithms cost $O(n^3\,2^{n/2})$, so the classical effort grows as $2^{N/2}$ in the number of *detected photons* $N$. This is why $N \sim 50$–$100$ photons is the advantage frontier: $2^{50} \approx 10^{15}$ Hafnian evaluations already exhaust a supercomputer.

**Why squeezed light supplies even photon numbers.** A single-mode squeezed vacuum with squeezing parameter $r$ is

$$|r\rangle = \frac{1}{\sqrt{\cosh r}}\sum_{n=0}^{\infty} \frac{\sqrt{(2n)!}}{2^n\, n!}\,(\tanh r)^{n}\,|2n\rangle,$$

populated **only in even photon numbers** — pairs, the optical fingerprint of the parametric process that creates photons two at a time. Its mean photon number is $\langle \hat n\rangle = \sinh^2 r$; at a modest $r = 1.0$ (about $8.7$ dB) this is $\sinh^2(1) \approx 1.38$ photons per mode. Feed, say, $50$ such modes into the interferometer and the *total* detected photon number fluctuates around $50\sinh^2 r \approx 69$, comfortably in the regime where each output pattern's probability is a Hafnian of a $\sim 70\times 70$ matrix — classically intractable, yet generated by a device whose only quantum inputs are deterministically squeezed beams.

**Two-mode squeezed vacuum on a 50/50 splitter.** The state $|\text{TMSV}\rangle = \sqrt{1-\lambda^2}\sum_n \lambda^n |n,n\rangle$ (with $\lambda = \tanh r$) shows the same physics from the other side: a 50/50 beam splitter maps a TMSV into *two independent single-mode squeezed states*, and, run backwards, two single-mode squeezers plus a beam splitter synthesize a TMSV — the standard recipe for entangled Gaussian light. The reduced state of either TMSV mode alone is thermal, $p_n = (1-\lambda^2)\lambda^{2n}$, with mean $\langle\hat n\rangle = \lambda^2/(1-\lambda^2) = \sinh^2 r$, matching the single-mode figure above. Each individual output of a squeezer network thus looks thermal and featureless; it is the *interference* of these locally thermal beams across all $m$ modes that the Hafnian encodes, and that no efficient classical algorithm is believed to reproduce.

The experimental burden has shifted from "make $n$ identical single photons on demand" to "squeeze hard and keep the interferometer low-loss," a trade that Section 20.4.3 shows was worth making.
