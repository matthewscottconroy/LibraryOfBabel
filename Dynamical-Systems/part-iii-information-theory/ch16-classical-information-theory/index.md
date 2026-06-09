# Chapter 16 — Classical Information Theory

> *Shannon's 1948 paper is the most important engineering paper of the 20th century. But it is also a paper in pure mathematics — about the fundamental limits of communication, compression, and computation with probabilistic sources.*

**Prerequisites:** Chapter 2 (probability theory, measure theory).

---

In the summer of 1948, Claude Shannon published a paper in the Bell System Technical Journal that would reshape the twentieth century. It was called "A Mathematical Theory of Communication," and in one move it founded the field of information theory, gave engineers a rigorous foundation for thinking about data, noise, and compression, and posed questions that mathematicians are still answering today.

Shannon's central insight was almost philosophical: forget what messages *mean*. Forget language, semantics, intent. Strip a message down to its bare mathematical skeleton — a sequence of symbols drawn from some source — and ask a much simpler question: *how much information is in it?* How much does it surprise you? How efficiently can it be represented?

The answers he found were exact and universal. The entropy of a source is not just a useful number — it is the precise theoretical limit on how compactly you can represent its output. The capacity of a noisy channel is not just a rough guide — it is the exact threshold between reliable and unreliable communication. Below capacity, you can communicate with arbitrarily low error. Above it, you cannot. This is not an engineering approximation. It is a theorem.

This chapter develops the core of Shannon's theory. We begin with the information measures themselves: entropy, joint entropy, conditional entropy, mutual information, and KL divergence. These are the basic vocabulary of the field. We then turn to the *asymptotic equipartition property* (AEP), which is the law of large numbers in disguise and the conceptual engine behind everything that follows. From there we cover the two great theorems of classical information theory: the source coding theorem (data compression to the entropy limit) and the noisy channel coding theorem (reliable communication up to channel capacity). We close with rate-distortion theory, which handles the case where perfect reconstruction is impossible or unnecessary.

Throughout, keep in mind that these are not separate results — they are facets of a single unified framework. The same entropy appears in compression, in channel coding, in statistical inference, and (as we will see in later chapters) in dynamical systems and quantum mechanics. That universality is what makes Shannon's theory so powerful.

**What this chapter builds:**
- Shannon entropy and its operational meaning
- The asymptotic equipartition property (AEP) and typical sequences
- Source coding: data compression to the entropy limit
- The noisy channel coding theorem: reliable communication up to channel capacity
- Rate-distortion theory: optimal lossy compression
- Fano's inequality and the information inequalities

**Sections:**
- [16.1 Information Measures](information-measures.md)
- [16.2 The Asymptotic Equipartition Property](asymptotic-equipartition-property.md)
- [16.3 Source Coding](source-coding.md)
- [16.4 The Noisy Channel](noisy-channel.md)
- [16.5 Rate-Distortion Theory](rate-distortion-theory.md)
- [16.6 Information Inequalities](information-inequalities.md)
- [Exercises](exercises.md)
- [Notes](notes.md)
