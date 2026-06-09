# Chapter 39 — One-Shot Information Theory

> *Shannon's theory is asymptotic — rates achieved in the limit of infinitely many channel uses. One-shot information theory asks: what can be done with a single channel use? The answer is smooth entropy — a family of entropic quantities that reduce to Shannon entropy in the i.i.d. limit but capture finite-blocklength behavior.*

**Prerequisites:** Chapter 16 (Shannon entropy, channel coding), Chapter 17 (Rényi entropy, min-entropy), Chapter 21 (quantum information, von Neumann entropy).

---

## What This Chapter Is About

Shannon's theorem tells you the capacity of a channel — the maximum rate of reliable communication in the limit of infinitely many channel uses. This is a beautiful asymptotic result, but real systems don't take infinitely many channel uses. A 5G network packet can't wait for $n \to \infty$.

One-shot information theory asks: what can you do in a single use? Or in $n$ uses for small, finite $n$? The answer involves smooth entropy — a family of entropic quantities indexed by an error parameter $\varepsilon$ that captures exactly the tradeoff between block length and performance.

The theory is both more general and more operationally precise than Shannon's asymptotic theory. In the i.i.d. limit, smooth entropy recovers Shannon entropy. For finite blocklength, it gives tight bounds with explicit $\sqrt{n}$ correction terms (the "dispersion"). And for single uses of quantum resources, it gives the correct bound for cryptographic tasks like privacy amplification.

This chapter develops the theory from first principles: smooth min- and max-entropy, one-shot source and channel coding, the Leftover Hash Lemma for privacy amplification, and the connection to quantum thermodynamics. The single-shot second law — you can extract at most $kT \cdot H_{\min}(\rho)$ work from a quantum system in one shot — is the thermodynamic payoff.

---

## Sections

- [39.1 The Need for One-Shot Theory](need-for-one-shot.md)
- [39.2 Smooth Min- and Max-Entropy](smooth-min-max-entropy.md)
- [39.3 One-Shot Source Coding](one-shot-source-coding.md)
- [39.4 One-Shot Channel Coding](one-shot-channel-coding.md)
- [39.5 One-Shot Cryptography](one-shot-cryptography.md)
- [39.6 The Second Law in Quantum Thermodynamics](quantum-second-law.md)
- [Exercises](exercises.md)
- [Chapter Notes](notes.md)
