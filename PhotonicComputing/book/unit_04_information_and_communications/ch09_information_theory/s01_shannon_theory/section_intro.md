# Section 9.1: Shannon Theory and the Optical Channel

Information theory begins with a question that sounds philosophical but is actually mathematical: what is information? Shannon's answer — given in terms of entropy and mutual information — transformed the question into a precisely computable quantity, and in doing so created the mathematical foundation for all of modern digital communications.

This section develops Shannon's theory from its foundations and applies it to the optical channel. We begin with the concept of entropy as a measure of uncertainty (Subsection 9.1.1), derive the channel capacity theorem and the Shannon-Hartley formula (Subsection 9.1.2), and then apply these results to the specific noise model of the optical channel — including the quantum limit where shot noise (the photon counting noise) is the fundamental barrier (Subsection 9.1.3).

The key results of this section:
- **Entropy**: $H(X) = -\sum_i p_i \log_2 p_i$ bits, measuring the average information content per symbol.
- **Channel capacity**: $C = \max_{p(x)} I(X; Y)$ bits per channel use, where $I(X;Y)$ is the mutual information.
- **Shannon-Hartley theorem**: $C = B\log_2(1 + \text{SNR})$ for the AWGN channel.
- **Optical channel capacity**: Set by amplifier ASE noise in long-haul systems, or by shot noise (photon counting) in the quantum limit.
- **Quantum limit**: $C_q \approx \bar{n}\log_2(1 + 1/\bar{n})$ nats per mode (for coherent state encoding with photon counting detection), where $\bar{n}$ is the mean photon number.
