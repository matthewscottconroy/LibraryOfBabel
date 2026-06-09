# 19.1 Multiple Access Channels

The first natural extension of the point-to-point model is the *multiple access channel*: two or more senders transmitting simultaneously to a single receiver. Think of two users sharing the same radio frequency to talk to a base station. They interfere with each other. What are the limits?

**Definition 19.1.1 (Multiple Access Channel).** A *multiple access channel (MAC)* has two senders (with inputs $X_1 \in \mathcal{X}_1$, $X_2 \in \mathcal{X}_2$) and one receiver (output $Y \in \mathcal{Y}$), with channel $p(y|x_1, x_2)$.

The question is no longer "what is the maximum rate for one sender?" but "what pairs of rates $(R_1, R_2)$ are simultaneously achievable?" The answer is a *capacity region* — a two-dimensional set.

**Definition 19.1.2 (Capacity Region of MAC).** The *capacity region* $\mathcal{C}_{\text{MAC}}$ is the closure of the set of achievable rate pairs $(R_1, R_2)$: rates at which senders 1 and 2 can simultaneously communicate reliably to the receiver.

The capacity region was characterized independently by Ahlswede and Liao in 1971:

**Theorem 19.1.3 (MAC Capacity Region — Ahlswede, Liao 1971).** The capacity region of the MAC with channel $p(y|x_1,x_2)$ is the convex hull of the union over all product input distributions $p(x_1)p(x_2)$ of the region:
$$\{(R_1, R_2) : R_1 \leq I(X_1;Y|X_2),\ R_2 \leq I(X_2;Y|X_1),\ R_1+R_2 \leq I(X_1,X_2;Y)\}.$$

Three constraints, each with a clean information-theoretic interpretation:

**Intuition:**
- $R_1 \leq I(X_1; Y \mid X_2)$: sender 1 can communicate at the rate given by the channel capacity *given that the receiver knows sender 2's message*. This is the cooperative bound — the best sender 1 can do even with perfect help from sender 2's side.
- $R_2 \leq I(X_2; Y \mid X_1)$: symmetrically for sender 2.
- $R_1 + R_2 \leq I(X_1, X_2; Y)$: the total information from both senders is bounded by the channel's total mutual information with both inputs.

The region is a pentagon in the $(R_1, R_2)$ plane — a rectangle with one corner cut off by the sum-rate constraint. Operating at different points on the boundary corresponds to different ways of dividing the channel resources between the two users.

**Example 19.1.4 (Gaussian MAC).** $Y = X_1 + X_2 + Z$, $Z \sim N(0,1)$, power constraints $E[X_i^2] \leq P_i$. The capacity region is:
$$R_1 \leq \frac{1}{2}\log(1+P_1),\quad R_2 \leq \frac{1}{2}\log(1+P_2),\quad R_1+R_2 \leq \frac{1}{2}\log(1+P_1+P_2).$$

Notice the sum-rate bound: $\frac{1}{2}\log(1 + P_1 + P_2) = \frac{1}{2}\log(1 + P_1) + \frac{1}{2}\log\left(1 + \frac{P_2}{1+P_1}\right)$. The second user can achieve nearly their full single-user capacity even while the first user is transmitting — you just need the decoder to do successive cancellation (decode user 1 first, subtract, then decode user 2).

This is the key insight of MAC capacity: clever receivers can untangle the simultaneous transmissions, achieving rates much higher than simple frequency division would suggest. The sum-rate capacity $\frac{1}{2}\log(1+P_1+P_2)$ is the same as what a single user with total power $P_1 + P_2$ could achieve — multi-user "interference" is not wasteful if decoded correctly.
