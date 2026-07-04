# 22.1.1 — The BB84 Protocol

## The Protocol

BB84 — Bennett and Brassard, 1984 [1] — uses four polarization states drawn from two *conjugate bases*:

$$\text{Z basis: } |H\rangle \equiv |0\rangle,\; |V\rangle \equiv |1\rangle \qquad \text{X basis: } |+\rangle = \frac{|H\rangle + |V\rangle}{\sqrt{2}},\; |-\rangle = \frac{|H\rangle - |V\rangle}{\sqrt{2}}$$

Each X state has probability $1/2$ of passing either Z-basis analyzer and vice versa: $|\langle H|+\rangle|^2 = 1/2$. Measuring in the wrong basis yields a coin flip and destroys the encoded bit. The protocol:

1. **Transmission.** For each of $N$ time slots, Alice picks a random bit and a random basis (Z or X), prepares the corresponding photon, and sends it. Bob independently picks a random measurement basis for each arriving photon and records the outcome.
2. **Sifting.** Over the public (authenticated, but not secret) classical channel, Bob announces *which* slots he detected and *which basis* he used — never the outcomes. Alice replies with her basis choices. They keep only the slots where bases matched: on average half the detected bits. This is the *sifted key*.
3. **Parameter estimation.** They sacrifice a random sample of sifted bits, comparing them publicly to estimate the *quantum bit error rate* (QBER) $Q$. In an ideal untapped system, matched bases mean perfectly correlated bits; every discrepancy is evidence of noise — all of which must be attributed, conservatively, to an eavesdropper.
4. **Error correction (information reconciliation).** Using public discussion (leaking a known number of bits, later subtracted), Alice and Bob correct the remaining errors — modern systems use LDPC codes or the interactive Cascade protocol — ending with identical strings.
5. **Privacy amplification.** They compress the reconciled key with a randomly chosen universal hash function, shrinking it by enough to erase Eve's estimated information. The result is the *secret key*: shorter, but secret except with exponentially small probability.

## Why It Is Secure

The intuition rests on the two pillars from the section introduction. Because Eve does not know the basis of any given photon, any information-extracting interaction necessarily disturbs the states. The canonical illustration is the **intercept-resend attack**: Eve measures every photon in a randomly guessed basis and resends her result. When she guesses wrong (probability $1/2$), the photon Bob receives is in the wrong basis; even when Bob's basis matches Alice's, his outcome is then random. The induced QBER is $\frac{1}{2}\times\frac{1}{2} = 25\%$ — a blazing alarm, since well-engineered systems run at $Q \sim 1\text{–}3\%$. Partial attacks (measuring a fraction of photons, or gentler entangling probes) trade less disturbance for less information, and the whole game of security proofs is to bound Eve's optimal tradeoff.

The modern proofs (Shor-Preskill, 2000 [3], following Lo-Chau and Mayers) establish *information-theoretic security against arbitrary attacks*, including coherent attacks on all photons jointly. The Shor-Preskill argument reduces BB84 to entanglement distillation with CSS codes and yields the celebrated asymptotic key fraction

$$r \;\geq\; 1 - h(Q_Z) - h(Q_X) \;=\; 1 - 2h(Q) \quad \text{(when } Q_Z = Q_X = Q\text{)}$$

where $h(x) = -x\log_2 x - (1-x)\log_2(1-x)$ is the binary entropy: $h(Q_Z)$ pays for error correction, $h(Q_X)$ for privacy amplification. The rate hits zero at $Q = 11\%$ — the famous BB84 threshold. Above it, no amount of post-processing rescues a secret key (for one-way post-processing); below it, every sifted bit yields $r$ secret bits asymptotically.

**Worked example.** At $Q = 3\%$: $h(0.03) = 0.194$, so $r = 1 - 2(0.194) = 0.61$ — each sifted bit yields 0.61 secret bits. At $Q = 8\%$: $h(0.08) = 0.402$, $r = 0.196$. At $Q = 10\%$: $r = 0.062$ — the cliff edge is steep, which is why QBER engineering (polarization drift compensation, detector timing, stray-light rejection) dominates QKD system design. A more general principle governs what is possible at all: the Csiszár-Körner bound states that a secret key is distillable with one-way communication whenever Bob's information advantage $I(A{:}B) - I(A{:}E) > 0$.

Two more ingredients are mandatory in practice. The classical channel must be *authenticated* (with a short pre-shared key, making QKD strictly a key-*growing* protocol — this is not a defect; the initial authentication key is tiny and used once). And finite-size effects matter: security proofs for finite $N$ subtract statistical-fluctuation penalties, which is why practical systems process blocks of $10^6$–$10^9$ bits.

## The Entanglement-Based Family: E91 and BBM92

In 1991, Artur Ekert independently reinvented QKD from a different starting point [2]: let a central source distribute *entangled pairs* $|\Phi^+\rangle = (|HH\rangle + |VV\rangle)/\sqrt{2}$, with Alice and Bob each measuring in randomly chosen bases. Matched bases yield perfectly correlated bits (the key); a subset of deliberately mismatched bases is used to test a **Bell/CHSH inequality** (Chapter 17). The security logic is beautiful: if the measured correlations violate the CHSH bound ($S > 2$), the outcomes could not have pre-existed — so no eavesdropper could have copies. Eve's intervention degrades the entanglement and lowers $S$ toward the classical regime. Monogamy of entanglement makes the argument quantitative: the closer $S$ is to $2\sqrt{2}$, the less correlated *anything else in the universe* can be with the outcomes.

Bennett, Brassard, and Mermin showed in 1992 (BBM92) that with the source treated as untrusted but the Bell test replaced by simple QBER estimation, entanglement-based QKD becomes formally equivalent to BB84 — the source can even be operated by Eve, since a source emitting anything other than good entangled pairs reveals itself as errors. Entanglement-based protocols cost more hardware (a pair source instead of a laser) but confer two advantages: no random-number generator is needed at the source (measurement randomness does the work), and the Ekert logic extends to **device-independent QKD**, where security is certified by the Bell violation alone, without trusting the inner workings of *any* device. Device-independent QKD was demonstrated in proof-of-principle experiments in 2022 (with trapped ions, atoms, and photons), though at rates and distances far from practical deployment. Entanglement-based BBM92 is also exactly the protocol Micius ran between two ground stations 1,120 km apart (Section 22.1.3).

## From Protocol to Photons

Mapping BB84 onto hardware from earlier chapters: Alice is a pulsed laser, an intensity/polarization modulator (Chapter 9), and a calibrated attenuator; Bob is a passive or switched basis selector (a polarizing beam splitter preceded by a random half-wave-plate setting or its fiber-optic equivalent) and two-to-four single-photon detectors (Chapter 19). Polarization survives free space well but wanders in fiber, so fiber systems usually encode in *phase* or *time-bin* (Chapter 20's dual-rail cousin): an asymmetric Mach-Zehnder pair converts BB84 into interference between early and late pulse components. Detector quality translates directly into reach: every 3 dB of loss halves the raw rate, and the key survives only while the signal rate exceeds the dark-count floor — the quantitative treatment, with decoy states and record distances, is the subject of the next subsection.

## Summary

- BB84 encodes random bits in randomly chosen conjugate bases; sifting keeps matched-basis events; QBER estimation bounds the eavesdropper; error correction plus privacy amplification distill the secret key.
- Intercept-resend forces 25% QBER; the general one-way secret fraction is $r \geq 1 - 2h(Q)$, positive only below $Q = 11\%$; at $Q = 3\%$, $r \approx 0.61$.
- Security is information-theoretic (Shor-Preskill), resting on the indistinguishability of non-orthogonal states and on information-disturbance tradeoffs; the classical channel needs only authentication.
- E91 derives security from CHSH violation; BBM92 shows entanglement-based QKD with an untrusted source reduces to BB84; the same logic culminates in device-independent QKD.
- Fiber implementations favor phase/time-bin encoding over polarization; detectors and loss set the practical limits developed in 22.1.2.

---

*References*

[1] Bennett, C.H. & Brassard, G. (1984). Quantum cryptography: Public key distribution and coin tossing. *Proceedings of the IEEE International Conference on Computers, Systems and Signal Processing*, Bangalore, 175–179. [Reprinted: *Theoretical Computer Science*, 560, 7–11 (2014). DOI: 10.1016/j.tcs.2014.05.025]

[2] Ekert, A.K. (1991). Quantum cryptography based on Bell's theorem. *Physical Review Letters*, 67(6), 661–663. [DOI: 10.1103/PhysRevLett.67.661]

[3] Shor, P.W. & Preskill, J. (2000). Simple proof of security of the BB84 quantum key distribution protocol. *Physical Review Letters*, 85(2), 441–444. [DOI: 10.1103/PhysRevLett.85.441]

[4] Bennett, C.H., Brassard, G., & Mermin, N.D. (1992). Quantum cryptography without Bell's theorem. *Physical Review Letters*, 68(5), 557–559. [DOI: 10.1103/PhysRevLett.68.557] [BBM92.]

[5] Gisin, N., Ribordy, G., Tittel, W., & Zbinden, H. (2002). Quantum cryptography. *Reviews of Modern Physics*, 74(1), 145–195. [DOI: 10.1103/RevModPhys.74.145] [The classic review covering protocol and early implementations.]

[6] Bennett, C.H., Bessette, F., Brassard, G., Salvail, L., & Smolin, J. (1992). Experimental quantum cryptography. *Journal of Cryptology*, 5(1), 3–28. [DOI: 10.1007/BF00191318] [The first QKD experiment — 32 cm of free space.]
