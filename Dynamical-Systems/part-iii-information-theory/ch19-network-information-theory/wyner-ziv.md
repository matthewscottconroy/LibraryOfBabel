# 19.4 Wyner-Ziv — Lossy Coding with Side Information

Slepian-Wolf covers lossless coding. But what if the decoder already has some side information — not the exact version, but a correlated signal — and the encoder does not? And what if we only want approximate reconstruction?

This is the Wyner-Ziv problem, solved in 1976. It is the lossy analogue of Slepian-Wolf, and it is equally surprising.

**Problem:** An encoder compresses $X$ to rate $R$ bits per sample. The decoder has access to correlated side information $Y$ (not available at the encoder). What rate $R$ is needed to achieve average distortion $D$?

The naive bound is the ordinary rate-distortion function $R(D)$ — the rate needed without any side information at the decoder. Surely having side information at the decoder should help? It does. But how much?

**Theorem 19.4.1 (Wyner-Ziv, 1976).** The rate-distortion function with decoder side information is:
$$R_{\text{WZ}}(D) = \min_{p(u|x),\, g: \mathcal{U} \times \mathcal{Y} \to \hat{\mathcal{X}}:\, E[d(X,g(U,Y))] \leq D} I(X; U \mid Y),$$
where the minimization is over auxiliary variables $U$ and reconstruction functions $g: U \times Y \to \hat{X}$ achieving distortion at most $D$.

The key result:

**Key result:** $R_{\text{WZ}}(D) = R(D \mid Y)$, the rate-distortion function with side information at *both* encoder and decoder. Side information at the decoder alone is "as good as" side information at both ends.

This is the Wyner-Ziv miracle: the decoder's side information $Y$ provides the same coding efficiency as if the encoder also knew $Y$ and could exploit it directly. The encoder, without knowing $Y$, can still code at the optimal rate — as long as the decoder has $Y$ at reconstruction time.

The mechanism is again random binning: the encoder assigns codewords for $U$ to bins, sends only the bin index, and the decoder uses $Y$ to identify which codeword in the bin is jointly typical with the received $Y$, then reconstructs $\hat{X}$ using $g(U, Y)$.

**Applications:** Wyner-Ziv coding appears naturally in:
- *Video compression*: frame $X$ is compressed with side information $Y$ = adjacent frame at the decoder. This is the basis of "distributed video coding" research.
- *Sensor networks*: sensor $X$ compresses observations, with the base station using other sensors' data $Y$ to help decode.
- *Genomics*: a reference genome $Y$ serves as side information when compressing an individual's genome $X$ — the differences are what matter.

In each case, the side information is not available at the encoder but is available at the decoder, and Wyner-Ziv theory gives the fundamental efficiency limit.
