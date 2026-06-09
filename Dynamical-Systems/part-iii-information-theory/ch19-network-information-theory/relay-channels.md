# 19.5 Relay Channels

Communication over long distances often requires intermediate helpers. A ship at sea communicates with the mainland through a satellite relay. A mobile user at the edge of a cell communicates through another user closer to the base station. The relay channel model captures this situation: source, relay, and destination.

**Setup:** Source $X$ sends to destination $Y$ via an intermediate relay $Y_r$ (which can also transmit $X_r$ to $Y$). The relay can hear the source and retransmit to help the destination. What rate is achievable?

Cover and El Gamal analyzed this problem in 1979 and derived two important inner bounds:

**Theorem 19.5.1 (Cover-El Gamal, 1979).**

The *decode-and-forward* inner bound: if the relay fully decodes the source message and re-encodes before forwarding:
$$R \leq \max_{p(x,x_r)} \min\{I(X, X_r; Y),\ I(X; Y, Y_r \mid X_r)\}.$$

The first argument of the min is the capacity from the source-plus-relay to the destination. The second is the capacity from the source to the destination-plus-relay (the "inner" link). The minimum captures the bottleneck.

The *compress-and-forward* inner bound: the relay compresses its observation and sends the compressed version to the destination:
$$R \leq \max_{p(x)p(x_r)p(\hat{y}_r|y_r,x_r)} I(X; Y, \hat{Y}_r \mid X_r) \quad \text{subject to } I(X_r; Y) \geq I(Y_r; \hat{Y}_r \mid X_r, Y).$$

The constraint says that the relay's transmission rate is enough to carry the compressed relay observation $\hat{Y}_r$ to the destination.

The capacity of the relay channel is not known in general — one of the major open problems in network information theory. The best known bounds (decode-and-forward, compress-and-forward, and the cut-set outer bound) do not in general coincide.

The relay channel illustrates a general challenge in network information theory: the optimal coding strategy may require the relay to do something more subtle than either fully decoding or simply compressing — a "partial decoding" strategy that has not been fully characterized.

For specific relay channels — degraded relay channels, reversely degraded relay channels, and Gaussian relay channels — the capacity is known. For the Gaussian relay channel with infinite relay power, the capacity approaches the direct link capacity, showing that a powerful relay can completely eliminate the channel's noise. But without such idealizations, the general case remains open.
