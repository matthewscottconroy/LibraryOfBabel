# Chapter Notes — Chapter 26

## On the entropy method in combinatorics

The entropy method — using entropy inequalities to prove combinatorial bounds — was systematized by Radhakrishnan, Kleitman, and others, though individual applications (like the Loomis-Whitney proof) go back further. The standard reference is:

- **Alon and Spencer**, *The Probabilistic Method*, Chapter 15 — covers Shearer's lemma and the main applications

The Loomis-Whitney inequality (1949) has many proofs; the entropy proof is due to Ruzsa and Szemerédi (though Loomis and Whitney's original proof is also elegant). Shearer's lemma itself is from:

- **Chung, Graham, Frankl, Shearer**, "Some intersection theorems for ordered sets and graphs," *J. Combinatorial Theory A* 43 (1986)

For Turán's theorem via entropy, and a broader survey of entropy methods in extremal combinatorics, see:

- **Radhakrishnan**, "Entropy and counting," in *Computational Mathematics, Combinatorics and Logic* (2001)

## On communication complexity

The foundational reference is:

- **Kushilevitz and Nisan**, *Communication Complexity* (Cambridge, 1997) — the standard textbook, covers the rank method, fooling sets, and the main results

The information complexity approach is developed in:

- **Bar-Yossef, Jayram, Kumar, Sivakumar**, "An information statistics approach to data stream and communication complexity," *FOCS* 2002
- **Braverman and Rao**, "Information equals amortized communication," *FOCS* 2011

The disjointness lower bound via information complexity is in Razborov's 1992 paper and the Kalyanasundaram-Schnitger 1992 paper. Bar-Yossef et al.'s direct sum theorem is the key tool.

## On circuit lower bounds

The Karchmer-Wigderson theorem is in:

- **Karchmer and Wigderson**, "Monotone circuits for connectivity require super-logarithmic depth," *STOC* 1988

For a survey of approaches to circuit lower bounds and the role of communication complexity:

- **Jukna**, *Boolean Function Complexity* (Springer, 2012) — comprehensive

The connection between information complexity and circuit complexity remains an active research frontier. The best current lower bounds for explicit functions are far from the $\Omega(2^n/n)$ bound Shannon's counting argument suggests for most functions.

## On expanders and extractors

The comprehensive survey is:

- **Hoory, Linial, and Wigderson**, "Expander graphs and their applications," *Bulletin of the AMS* 43 (2006) — 100 pages, covers everything

Ramanujan graphs (optimal expanders) are constructed using Hecke operators and are in:

- **Lubotzky, Phillips, and Sarnak**, "Ramanujan graphs," *Combinatorica* 8 (1988)

For extractors:

- **Shaltiel**, "Recent developments in explicit constructions of extractors," *Bulletin of the EATCS* 77 (2002) — survey of state of the art

## On polar codes

Arıkan's original paper is essential reading:

- **Arıkan**, "Channel polarization: A method for constructing capacity-achieving codes for symmetric binary-input memoryless channels," *IEEE Trans. Inf. Theory* 55 (2009)

The paper is unusually readable. The dynamical systems perspective on polarization (viewing it as an iterated map on the space of channels) is developed in:

- **Hassani, Alishahi, and Urbanke**, "On the scaling of polar codes: I. The behavior of polarized channels," *ISIT* 2010

For the proof that polarization achieves capacity, Arıkan's original proof is the cleanest. The finite-length performance and scaling laws are studied in the subsequent Hassani-Alishahi-Urbanke series.
