# Keyword Spotting with Reservoirs

## Task Definition and Constraints

Keyword spotting (KWS) is the task of detecting the presence of a specific word or phrase (the keyword) in a continuous audio stream, in real time. Unlike offline speech recognition, KWS must operate always-on, at low latency ($< 100$ ms detection delay), and with minimal energy consumption — constraints dictated by the deployment scenario: always-on microcontroller in a consumer device.

These constraints favor fixed-weight architectures over trained RNNs. Reservoir computing is naturally suited: the reservoir weights are fixed (no backpropagation needed during deployment), the computation is a simple matrix-vector product per frame, and the memory required is $O(N)$ for the reservoir state — compatible with microcontroller RAM.

## The Google Speech Commands Dataset

Warden [2018] introduced the Google Speech Commands dataset: 105,829 recordings of 35 different words (including 10 command words and 25 auxiliary words), spoken by 2,618 speakers at various background noise levels. The standard benchmark uses a 10-class or 35-class version, evaluated by accuracy on a held-out test set. The 10-command benchmark (yes, no, up, down, left, right, on, off, stop, go + silence + unknown) is the most common.

State-of-art accuracy on this benchmark with deep learning (MobileNet, transformer): $> 97\%$. Reservoir computing baseline: approximately 85–92% depending on reservoir size and feature extraction [Warden 2018].

## Reservoir Pipeline

A standard reservoir KWS pipeline consists of:

**Feature extraction:** 40-dimensional log-mel filterbank features at 25 ms frames, 10 ms hop. Each 1-second keyword window gives $T = 100$ frames.

**Reservoir processing:** Run the reservoir over all 100 frames:

$$\mathbf{x}_t = (1-\alpha)\mathbf{x}_{t-1} + \alpha \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{W}^{\text{in}} \mathbf{u}_t), \quad t = 1, \ldots, 100.$$

**State aggregation:** Compute the mean reservoir state $\bar{\mathbf{x}} = \frac{1}{100}\sum_t \mathbf{x}_t \in \mathbb{R}^N$.

**Classification readout:** Apply a softmax classifier: $p(k \mid \bar{\mathbf{x}}) = \text{softmax}(\mathbf{W}_k^\top \bar{\mathbf{x}})$. Trained by ridge regression (Chapter 10) or cross-entropy gradient descent on $\mathbf{W}$.

Alternatively, the final state $\mathbf{x}_{100}$ can be used instead of the mean, at the cost of more sensitivity to variable-length keywords.

## Energy Comparison

The energy advantage of reservoir computing for KWS is quantified by comparing the multiply-accumulate (MAC) operations per inference:

| Model | MACs/inference | Accuracy (10-class) |
|-------|---------------|---------------------|
| MobileNetV2 | $\sim 30\text{M}$ | $97.6\%$ |
| LSTM ($N=256$) | $\sim 1.5\text{M}$ | $94.8\%$ |
| ESN ($N=256$) | $\sim 0.1\text{M}$ | $88.5\%$ |
| ESN ($N=64$) | $\sim 0.006\text{M}$ | $83.1\%$ |

ESN with $N = 64$ requires approximately 1000× fewer MACs than MobileNetV2, at a cost of $\sim 14\%$ accuracy. For battery-constrained devices where the acceptable accuracy threshold is lower, this tradeoff may be favorable [Tanaka et al. 2019].

A further advantage: ESN weights are fixed, so no weight update computation is needed on-device. For LSTM, even inference requires the gated recurrence (4 weight matrices), while for ESN, only the fixed $\mathbf{W}^{\text{rec}}$ and $\mathbf{W}^{\text{in}}$ multiplications are needed.

## The Accuracy Gap and Its Sources

The 9–14% accuracy gap between ESN and deep learning on Speech Commands arises from several sources:

**Temporal context:** MobileNet and LSTM process the full spectrogram as a 2D image or fine-grained sequence, capturing fine temporal structure. ESN mean-pooling discards temporal ordering within the keyword window, losing sequence information.

**Feature learning:** Deep models learn their own feature representations from raw waveforms or spectrograms. ESN uses fixed MFCC features, which may not optimally represent the relevant acoustic properties.

**Nonlinearity depth:** MobileNet stacks many nonlinear layers, allowing complex decision boundaries in feature space. ESN has one layer of fixed nonlinearity plus a linear readout.

Partially closing the gap: using a deep ESN (multiple reservoir layers with different $\alpha_\ell$) and SVM readout can recover 2–4% accuracy, approaching the LSTM baseline at still-manageable energy cost [Tanaka et al. 2019].

---

## References

- Warden, P. (2018). Speech commands: A dataset for limited-vocabulary speech recognition. *arXiv preprint*, arXiv:1804.03209.
- Tanaka, G., Yamane, T., Héroux, J. B., Nakane, R., Kanazawa, N., Takeda, S., ... & Hirose, A. (2019). Recent advances in physical reservoir computing: A review. *Neural Networks*, 115, 100–123.
