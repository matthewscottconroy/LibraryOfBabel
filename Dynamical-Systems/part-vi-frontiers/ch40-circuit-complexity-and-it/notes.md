# Chapter Notes — Chapter 40

**Foundational texts.** Arora-Barak's *Computational Complexity: A Modern Approach* (Cambridge, 2009) is the standard comprehensive reference — it covers all the material in this chapter and much more. Papadimitriou's *Computational Complexity* (Addison-Wesley, 1994) is older but has great exposition.

**Shannon's counting argument.** The original: *The synthesis of two-terminal switching circuits* (Bell System Tech. J., 1949). Shannon's insight that most functions are hard is often overlooked — it was the first theorem in complexity theory.

**AC⁰ lower bounds.** Furst-Saxe-Sipser first proved parity is not in AC$^0$ in 1984, but with weaker bounds. Håstad's switching lemma: *Computational Limitations of Small-Depth Circuits* (MIT Press PhD thesis, 1987; conference version in STOC 1986). The Fourier analysis of Boolean functions approach was developed by Linial-Mansour-Nisan (STOC, 1989). O'Donnell's *Analysis of Boolean Functions* (Cambridge, 2014) is the comprehensive reference for Fourier analysis — free draft available online.

**Monotone lower bounds.** Razborov's original: *Lower bounds on monotone complexity of the logical permanent* (Math. Notes USSR, 1985). Alon-Boppana: *The monotone circuit complexity of Boolean functions* (Combinatorica, 1987).

**Natural proof barrier.** Razborov-Rudich: *Natural proofs* (J. Computer and System Sciences, 1997). Aaronson's blog posts on natural proofs are accessible and insightful. The three barriers together are surveyed in Aaronson-Wigderson's *Algebrization: A New Barrier in Complexity Theory* (J. ACM, 2009).

**Information complexity.** Braverman-Rao's *Tight bounds for set disjointness in the message-passing model* is the key paper. The information complexity framework is developed in Braverman's *Interactive Information Complexity* (SICOMP, 2015). Weinstein-Yehudayoff have a more recent survey.

**Where things stand.** Ryan Williams proved ACC$^0 \subsetneq$ NEXP in 2011 (STOC 2011) — the first new circuit lower bound against explicit functions in 25 years. The proof avoids all three barriers via a clever "meta-algorithmic" technique. This is currently the frontier of what's known.
