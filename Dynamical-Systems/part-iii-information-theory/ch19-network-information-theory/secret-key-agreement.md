# 19.7 Secret Key Agreement

The wiretap channel assumes a specific channel structure where the eavesdropper is noisier than the legitimate receiver. But what if Alice and Bob want to establish a secret key using a public channel, with an eavesdropper who can see everything they transmit?

This is the *secret key agreement* problem, and it turns out that correlated information — observations that are statistically related between Alice and Bob but differ from what the eavesdropper sees — can be leveraged to generate shared secret bits even over a fully public channel.

**Problem:** Two parties observe $(X^n, Y^n)$ (a correlated source) while an eavesdropper observes $Z^n$, correlated with both. They communicate over a public channel (visible to the eavesdropper, including the eavesdropper who sees everything). How many secret key bits can they generate?

**Theorem 19.7.1 (Maurer 1993; Ahlswede-Csiszár 1993).** The *secret key capacity* with one-way communication (Alice to Bob only) is:
$$C_K = I(X; Y) - I(X; Z).$$

With two-way communication (both directions allowed), the capacity can be higher:
$$C_K^{(2)} = \sup [\text{agreement information}] - [\text{eavesdropper information}],$$
though the full characterization with two-way communication remains open in general.

The one-way formula $C_K = I(X;Y) - I(X;Z)$ says: the rate of secret key bits equals the advantage of Alice-Bob's correlation over the eavesdropper's correlation. If the eavesdropper's observations are independent of Alice's, then $I(X;Z) = 0$ and Alice-Bob can extract $I(X;Y)$ secret bits per sample. If the eavesdropper can perfectly reconstruct $X$ from $Z$, then $I(X;Z) = I(X;Y)$ and no secret key can be generated.

The mechanism is ingenious: Alice and Bob use their correlated observations to agree on a shared value, then apply privacy amplification (using the smooth min-entropy framework from Section 17.2) to distill secret bits from this shared value. The public communication helps them correct any disagreements in their shared value without revealing too much to the eavesdropper.

Secret key agreement from correlated sources is the information-theoretic foundation of quantum key distribution (QKD). In QKD, Alice and Bob share a quantum channel, and after measurement and classical post-processing, they obtain correlated bit strings with quantifiable eavesdropper information. The secret key capacity formula then tells them how many secure bits they can distill from their correlated strings — this is exactly the final step of every QKD protocol.
