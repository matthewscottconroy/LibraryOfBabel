# Speaker Identification with Reservoir Computing

## Task Definition

Speaker identification (SID) is the task of determining which individual spoke a given speech segment, given a closed set of $K$ known speakers. It is distinct from speaker verification (which answers a binary "is this speaker X?") and from speech recognition (which identifies what was said, not who said it).

SID is fundamentally a temporal pattern recognition problem: each speaker's voice has a characteristic spectral envelope, prosody, and temporal dynamics that must be extracted from a variable-length acoustic segment and mapped to a speaker identity. Reservoir computing is well-suited because it naturally handles variable-length inputs through its recurrent state [Tanaka et al. 2019].

## Feature Extraction: MFCC

The standard input representation for speaker identification is the mel-frequency cepstral coefficient (MFCC) vector. For each 25 ms speech frame (shifted by 10 ms), the following processing chain is applied:

1. Pre-emphasis filter: $s'(t) = s(t) - 0.97 s(t-1)$
2. Hamming window
3. DFT power spectrum
4. Mel filterbank: 26 triangular filters with mel-spaced center frequencies
5. Log compression
6. DCT: take first 13 coefficients → $\mathbf{c}_t \in \mathbb{R}^{13}$

Appending delta features ($\Delta \mathbf{c}_t = \mathbf{c}_t - \mathbf{c}_{t-1}$) and delta-delta features ($\Delta^2 \mathbf{c}_t = \Delta\mathbf{c}_t - \Delta\mathbf{c}_{t-1}$) gives a 39-dimensional input vector:

$$\mathbf{u}_t = [\mathbf{c}_t; \Delta\mathbf{c}_t; \Delta^2\mathbf{c}_t] \in \mathbb{R}^{39}.$$

The delta features encode temporal dynamics of the spectral envelope — information that the MFCC alone cannot provide. The reservoir then further integrates these frame-level features over the utterance duration [Reynolds et al. 2000].

## Reservoir Approach

The reservoir processes the MFCC sequence $\{\mathbf{u}_t\}_{t=1}^{T_{\text{utt}}}$ for each utterance of duration $T_{\text{utt}}$ frames. Two strategies for producing a fixed-length representation from the variable-length state sequence:

**Final state readout:** Use the reservoir state at the last frame $\mathbf{x}_{T_{\text{utt}}}$ as the utterance representation. This works when the utterance has a fixed temporal structure (same phoneme sequence for all speakers).

**Mean state readout:** Compute $\bar{\mathbf{x}} = \frac{1}{T_{\text{utt}}} \sum_t \mathbf{x}_t$. This is more robust to variable-length utterances and averaging over temporal fluctuations. It is equivalent to a sum-pooling operation over the reservoir state sequence.

The speaker classification readout is then applied to the fixed-length reservoir representation (either $\mathbf{x}_{T_{\text{utt}}}$ or $\bar{\mathbf{x}}$), using one of the classifiers from Chapter 10: ridge regression, LDA, or SVM [Tanaka et al. 2019].

## TIMIT Benchmark

The TIMIT corpus contains recordings from 630 speakers (438 male, 192 female), each speaking 10 sentences (2 identical across speakers, 8 unique). The standard closed-set speaker identification task uses 8 training utterances per speaker and 2 test utterances.

For a reservoir of $N = 500$ neurons with spectral radius $\rho = 0.9$ and ridge regression readout, speaker identification accuracy on TIMIT is approximately 85–92%, depending on utterance length. SVM readout with RBF kernel achieves 90–95%. LSTM baseline: 95–98%. The accuracy gap between reservoir and LSTM narrows for shorter utterances, where the LSTM's backpropagation through time provides diminishing benefit [Tanaka et al. 2019].

## Comparison with i-Vector Baseline

The i-vector framework [Dehak et al. 2011] is the classical state-of-art approach for speaker verification. It models the speaker-dependent mean of a universal background GMM as:

$$\boldsymbol{\mu}_s = \boldsymbol{\mu}_{\text{UBM}} + \mathbf{T} \mathbf{w}_s,$$

where $\mathbf{T} \in \mathbb{R}^{CF \times d_{\text{iv}}}$ is the total variability matrix, $C$ is the number of GMM components, $F = 39$ is the MFCC dimension, and $\mathbf{w}_s \in \mathbb{R}^{d_{\text{iv}}}$ ($d_{\text{iv}} = 400$) is the i-vector for speaker $s$ [Reynolds et al. 2000].

The i-vector is extracted by MAP estimation and classified by PLDA (probabilistic linear discriminant analysis). On TIMIT with all 630 speakers, i-vectors achieve identification accuracy $>99\%$ — substantially better than current reservoir approaches.

The reservoir is competitive in the regime of small $K$ (fewer than 200 speakers) and short utterances ($< 5$ s), where the i-vector framework's statistical modeling advantage is reduced. For large speaker sets or long-duration speaker verification, i-vectors with PLDA remain the established standard.

---

## References

- Tanaka, G., Yamane, T., Héroux, J. B., Nakane, R., Kanazawa, N., Takeda, S., ... & Hirose, A. (2019). Recent advances in physical reservoir computing: A review. *Neural Networks*, 115, 100–123.
- Reynolds, D. A., Quatieri, T. F., & Dunn, R. B. (2000). Speaker verification using adapted Gaussian mixture models. *Digital Signal Processing*, 10(1–3), 19–41.
