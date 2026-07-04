# 22.2.1 — The No-Cloning Theorem

## The Theorem and Its Four-Line Proof

**No-cloning theorem** (Wootters & Zurek, 1982 [1]; Dieks, 1982 [2]): *There is no physical process that copies an arbitrary unknown quantum state* — no unitary $\hat{U}$ and blank state $|b\rangle$ such that

$$\hat{U}\,|\psi\rangle|b\rangle = |\psi\rangle|\psi\rangle \quad \text{for all } |\psi\rangle$$

*Proof.* Suppose such a $\hat{U}$ clones two states: $\hat{U}|\psi\rangle|b\rangle = |\psi\rangle|\psi\rangle$ and $\hat{U}|\phi\rangle|b\rangle = |\phi\rangle|\phi\rangle$. Unitaries preserve inner products, so equating the inner products of the left and right sides:

$$\langle\phi|\psi\rangle\,\langle b|b\rangle = \langle\phi|\psi\rangle^2 \quad\Rightarrow\quad \langle\phi|\psi\rangle = \langle\phi|\psi\rangle^2$$

A number equal to its own square is 0 or 1: the two states must be either identical or orthogonal. No device can clone a *general* state — in particular, none can clone both $|H\rangle$ and $|+\rangle = (|H\rangle + |V\rangle)/\sqrt{2}$, whose overlap is $1/\sqrt{2}$. $\blacksquare$

The proof's brevity hides its reach. Cloning is forbidden not by engineering difficulty but by the *linearity* of quantum mechanics itself: the cloning map would have to act nonlinearly on superpositions ($|\psi\rangle = \alpha|0\rangle + \beta|1\rangle$ must go to $|\psi\rangle|\psi\rangle$, which contains cross terms $\alpha\beta$ that linear evolution of $\alpha|0\rangle|b\rangle + \beta|1\rangle|b\rangle$ can never produce). Note the crucial qualifier *unknown/arbitrary*: **known** states can be manufactured in any quantity (that is what a laser does), and **orthogonal** sets can be copied (that is what classical copying is — classical information is exactly the information carried by orthogonal, hence distinguishable, states).

## Why Amplifiers Cannot Save Quantum Communication

An ideal amplifier is a cloner. "Take my fading qubit and output two (or $G$) faithful copies" is precisely the forbidden map, so a noiseless deterministic quantum amplifier cannot exist. The loophole-free version of this statement is quantitative: any phase-insensitive linear amplifier of gain $G$ must add noise of at least $(G-1)/2$ vacuum units per quadrature (Caves, 1982 [3]) — exactly enough added noise to prevent the output from violating no-cloning. An erbium amplifier in a QKD link would not extend the key; it would *be* the eavesdropper, its spontaneous-emission noise indistinguishable in principle from an attack. Related no-go results round out the picture: unknown states cannot be *deleted* against a copy (no-deleting), and imperfect cloning is possible but bounded — the optimal universal cloner copies a qubit with fidelity exactly $5/6$ (Bužek & Hillery, 1996 [4]), and measuring a single unknown qubit reveals at most one bit's worth of imperfect information about its continuous parameters.

Three consequences structure this whole chapter:

1. **QKD is secure.** Eve cannot copy the flying qubit and defer measurement; anything she extracts disturbs the original measurably. No-cloning is the reason information gain implies disturbance — the same theorem that dooms the amplifier underwrites the key. (The optimal-cloning fidelity of $5/6$ reappears as the QBER an ideal cloning attack inflicts: $1/6 \approx 16.7\%$, comfortably above BB84's 11% abort threshold.)
2. **Range cannot be regenerated.** Every photodetection-and-resend station is a measurement, collapsing the state; every amplifier adds disqualifying noise. Loss compounds untreated: direct quantum transmission is exponentially doomed (Section 22.1.2).
3. **The repeater must be architecturally different.** Since the *payload* cannot be copied or refreshed, the repeater refreshes something copyable-by-retrying instead: *entanglement*. Bell pairs are standard, known states — if a distribution attempt fails, you simply try again, because you are not trying to preserve an unknown input. Success is *heralded* (a click announces which attempts worked), memories hold the successes, and entanglement swapping (22.2.3) chains segments. The unknown qubit itself, when there finally is one, travels by teleportation — consuming the pre-shared entanglement, never traversing the lossy channel at all.

## The Repeater Principle in One Paragraph

Divide $L = 1{,}000$ km into $n$ segments of $L_0 = L/n$. Over one segment, photon survival is $\eta_0 = 10^{-\alpha L_0/10}$ — for $L_0 = 50$ km, about $10\%$: painful but retryable at MHz rates. Each segment independently attempts entanglement generation until it succeeds (heralded), parking success in memory. Adjacent successes are fused by Bell measurement at the shared node. The end-to-end entanglement rate now degrades only *polynomially* with $L$ (the precise scaling, and its stringent demand on memory lifetime, is derived in 22.2.3), compared with $10^{-20}$ for direct transmission. The entire construction stands on two legs — a component that stores flying qubits (quantum memory, next subsection) and an operation that splices entanglement (entanglement swapping, 22.2.3) — plus purification to keep fidelity from eroding as segments multiply.

## Summary

- No-cloning: unitarity/linearity forbids copying unknown states; only orthogonal (classical) alphabets are copyable; known states are manufacturable at will.
- Ideal amplification is cloning; real linear amplifiers must add $\geq (G-1)/2$ vacuum units of noise (Caves), which is why EDFAs cannot extend quantum links.
- Optimal universal cloning reaches fidelity $5/6$ — imperfect enough to keep QKD secure (cloning attacks induce $\sim 16.7\%$ QBER).
- The same theorem that secures QKD forbids brute-force range extension; repeaters respond by distributing *retryable, heralded* entanglement segment-by-segment and moving payloads via teleportation.

---

*References*

[1] Wootters, W.K. & Zurek, W.H. (1982). A single quantum cannot be cloned. *Nature*, 299, 802–803. [DOI: 10.1038/299802a0]

[2] Dieks, D. (1982). Communication by EPR devices. *Physics Letters A*, 92(6), 271–272. [DOI: 10.1016/0375-9601(82)90084-6]

[3] Caves, C.M. (1982). Quantum limits on noise in linear amplifiers. *Physical Review D*, 26(8), 1817–1839. [DOI: 10.1103/PhysRevD.26.1817]

[4] Bužek, V. & Hillery, M. (1996). Quantum copying: Beyond the no-cloning theorem. *Physical Review A*, 54(3), 1844–1852. [DOI: 10.1103/PhysRevA.54.1844] [The optimal universal cloner.]

[5] Briegel, H.-J., Dür, W., Cirac, J.I., & Zoller, P. (1998). Quantum repeaters: The role of imperfect local operations in quantum communication. *Physical Review Letters*, 81(26), 5932–5935. [DOI: 10.1103/PhysRevLett.81.5932] [The founding repeater proposal.]
