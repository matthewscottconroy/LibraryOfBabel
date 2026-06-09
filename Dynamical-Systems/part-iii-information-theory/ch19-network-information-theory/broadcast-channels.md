# 19.2 Broadcast Channels

Flip the MAC problem around: one sender, two receivers. Think of a television broadcaster or a satellite transmitting to users with different channel qualities. Different receivers will receive different quality signals — how should the sender allocate its communication?

**Definition 19.2.1 (Broadcast Channel).** A *broadcast channel (BC)* has one sender (input $X \in \mathcal{X}$) and two receivers (outputs $Y_1 \in \mathcal{Y}_1$, $Y_2 \in \mathcal{Y}_2$), with channel $p(y_1, y_2 \mid x)$.

Again, the answer is a capacity region of achievable rate pairs $(R_1, R_2)$ — simultaneously achievable rates for receivers 1 and 2. But the broadcast channel is significantly harder than the MAC, and its general capacity region is still unknown after fifty years.

The one case that is fully understood is the *degraded broadcast channel*, where one receiver is a "noisy" version of the other:

**Theorem 19.2.3 (Degraded Broadcast Channel — Cover, Bergmans 1972).** If $X \to Y_1 \to Y_2$ form a Markov chain (receiver 1 is less noisy than receiver 2), the capacity region is:
$$R_1 \leq I(X; Y_1 \mid U), \quad R_2 \leq I(U; Y_2),$$
for all auxiliary random variables $U$ such that $U \to X \to (Y_1, Y_2)$ form a Markov chain.

The key to understanding this is the auxiliary variable $U$. The sender uses a strategy called *superposition coding*: first encode a "base layer" message for receiver 2 into $U$, then encode a "refinement" message for receiver 1 on top of $U$. The degraded receiver 2 can only decode the base layer; the better receiver 1 decodes both layers and recovers more information.

The rate to receiver 2 is $I(U; Y_2)$ — what the base layer delivers through the noisy channel to receiver 2. The rate to receiver 1 is $I(X; Y_1 \mid U)$ — the additional information in the refinement layer, given that receiver 1 already has the base layer.

This is exactly how modern video streaming codecs work: a base-quality stream that all receivers can decode, plus enhancement layers that only high-quality connections can use. The information-theoretic foundation dates to 1972.

**General BCs:** The capacity region of general (non-degraded) broadcast channels is unknown. Marton's inner bound and the UV outer bound are the best known in general, but they do not in general coincide. The gap between best inner and outer bounds is one of the major open problems in network information theory.

The contrast with the MAC is striking: MACs are "solved" (full capacity region known for any MAC), while general BCs remain open. This asymmetry is not for lack of effort — it reflects a genuine structural difficulty. Receiving is "easier" than decoding, and the problem of allocating one transmission to multiple decoders with different views of the channel seems to require fundamentally new ideas.
