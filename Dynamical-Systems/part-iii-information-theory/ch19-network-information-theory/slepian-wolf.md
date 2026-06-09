# 19.3 Distributed Source Coding — Slepian-Wolf

Here is a scenario that looks impossible at first glance. Two sensors, positioned at different locations, each observing a correlated but different signal. They cannot talk to each other — they communicate only with a central decoder, over separate channels. The question: what rates do they each need to send?

The obvious answer is that each sensor should send enough bits to describe its own signal independently — since they can't coordinate, they can't exploit their correlation. But this answer is wrong. The Slepian-Wolf theorem, proved in 1973, shows that the sensors *can* achieve rates as if they had coordinated, even though they haven't. The coordination happens at the decoder, not the encoder.

**Problem statement:** Two correlated sources $X$ and $Y$ encode their data *separately* (no communication between encoders) but send to a *common decoder*. What rates are needed to reconstruct $(X, Y)$ reliably?

**Theorem 19.3.1 (Slepian-Wolf, 1973).** The achievable rate region for distributed lossless coding of correlated sources $(X, Y)$ is:
$$R_X \geq H(X \mid Y), \quad R_Y \geq H(Y \mid X), \quad R_X + R_Y \geq H(X, Y).$$

The remarkable feature: encoder $X$ needs only $H(X \mid Y)$ bits per symbol — the entropy of $X$ *given* $Y$ — even though the encoder does not know $Y$! The encoder for $X$ essentially sends a random bin index, and the decoder uses both bin indices plus the known joint distribution to find the unique jointly typical pair.

**Proof idea (achievability):** Encoder $X$ partitions sequences $x^n$ into $2^{n H(X|Y)}$ bins (using a random code). Encoder $Y$ partitions $y^n$ into $2^{n H(Y|X)}$ bins. The decoder receives both bin indices and finds the unique jointly typical pair $(x^n, y^n)$ in the intersection of the given bins. By the joint typicality lemma, this pair exists and is unique with high probability.

**Example 19.3.2.** If $X = Y$ (perfectly correlated): $H(X \mid Y) = 0$, so encoder $X$ needs 0 bits. The second encoder's description is enough to reconstruct both — and this makes sense, since the decoder can recover $X$ from $Y$. If $X \perp Y$ (independent): $H(X \mid Y) = H(X)$, so each encoder needs the full entropy — no savings, which is also the right answer since there is no correlation to exploit.

Slepian-Wolf is one of the most surprising results in information theory, because it shows that the gains from correlated sources do not require coordination at the encoder. The correlation is "recovered" at the decoder through joint decoding. This has practical implications: in sensor networks, for instance, spatially separated sensors can compress to near-optimal rates without communicating with each other — they just need a sufficiently powerful decoder at the base station.

The theory extends to the lossy setting (Wyner-Ziv, Section 19.4) and to more than two sources, though the multi-source case introduces subtleties that are still being worked out.
