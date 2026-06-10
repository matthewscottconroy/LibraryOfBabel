# Chapter 21 Exercises

## Conceptual Exercises

**21.1** (Timescale Matching). An ESN with leaking rate $\alpha = 0.2$ processes MFCC features at 100 frames per second (10 ms per frame).

(a) Compute the effective time constant $\tau_{\text{eff}} = \Delta t / \alpha$ and express it in milliseconds. Which linguistic feature level (phoneme, syllable, word) does this timescale correspond to?

(b) A vowel formant transition (e.g., /æ/ $\to$ /ɑ/) takes approximately 50–80 ms. Does the ESN's time constant match this rate? If not, what value of $\alpha$ would you choose?

(c) A sentence-level prosodic contour spans approximately 3 s. What leaking rate $\alpha$ would be needed to capture this, and why might such a small $\alpha$ be problematic in practice?

**21.2** (Formant Analysis). The first two formant frequencies of a vowel are $F_1 = 650$ Hz and $F_2 = 1200$ Hz.

(a) Locate this vowel in the vowel quadrilateral (F1 vs. F2 space). Which vowel is it approximately?

(b) Using the tube model $F_n = (2n-1)c/(4L)$ with $c = 35000$ cm/s, what vocal tract length $L$ produces $F_1 = 650$ Hz?

(c) The mel-frequency scale maps $F_1 = 650$ Hz and $F_2 = 1200$ Hz to mel values of approximately 670 mel and 1070 mel respectively. Why does the mel scale improve ASR performance compared to linear frequency analysis?

**21.3** (MFCC Computation). Implement MFCC computation from scratch (without librosa) for a 1 s sinusoidal signal $x(t) = \sin(2\pi \cdot 440 \cdot t)$ sampled at 16 kHz.

(a) Apply a Hann window of length 400 samples with 160-sample hop. How many frames result?

(b) Compute the power spectrum of each frame using the DFT. What is the frequency resolution (Hz per bin) for a 400-sample window at 16 kHz?

(c) Construct a 40-filterbank mel filterbank covering 80–8000 Hz. Show the filter shapes at the three center frequencies closest to 440 Hz.

(d) Apply the filterbank and DCT to get 13 MFCCs. Verify that all frames have nearly identical MFCC vectors (since the signal is periodic). What causes small frame-to-frame variations?

**21.4** (Co-articulation Effect). The phoneme /d/ is realized differently before a high front vowel /iː/ (as in "deed") versus a back vowel /ɑː/ (as in "dah"). The difference is due to anticipatory co-articulation: the tongue already starts moving toward the next vowel during the stop closure.

(a) Explain how a reservoir with $\alpha = 0.3$ would incorporate context information about the following vowel into its representation of the stop /d/. Does fading memory help here?

(b) Would bidirectional reservoir processing (running the ESN both forward and backward in time, as in bidirectional LSTMs) help address co-articulation? What would be the practical trade-offs?

## Computational Exercises

**21.5** (FSDD Baseline). Download the FSDD dataset and run the provided ESN code with the default parameters.

(a) Report the leave-one-speaker-out accuracy for each speaker separately. Which speaker has the lowest accuracy? Can you explain why by listening to their recordings?

(b) Plot the confusion matrix. Which pairs of digits are most frequently confused? Is there a phonological explanation?

(c) Increase the reservoir size from $N = 500$ to $N = 2000$. Does accuracy improve? How much does computation time increase?

**21.6** (Hyperparameter Study). Using the FSDD dataset (or the synthetic demo):

(a) Vary the spectral radius $\rho \in \{0.5, 0.7, 0.9, 0.95, 0.99, 1.05\}$ and plot accuracy vs. $\rho$. Where is the optimum?

(b) Vary the leaking rate $\alpha \in \{0.1, 0.2, 0.3, 0.5, 0.7, 1.0\}$ and plot accuracy vs. $\alpha$.

(c) Explain the performance drop when $\rho > 1$. What happens to the reservoir dynamics?

(d) Find the best $(\rho, \alpha)$ combination and compute its improvement over the default settings.

**21.7** (Feature Comparison). Compare the performance of different input feature representations on FSDD:

(a) 13 MFCCs (no deltas)
(b) 13 MFCCs + 13 delta-MFCCs (26-dim)
(c) 13 MFCCs + 13 delta + 13 delta-delta (39-dim)
(d) 40 log-mel filterbank energies (no DCT)
(e) 1 feature: log-energy only

Report accuracy for each. Which feature set performs best? Explain the trend in terms of the information content and the reservoir's ability to exploit temporal derivatives.

**21.8** (Aggregation Methods). Using a fixed ESN ($N = 500$, $\rho = 0.95$, $\alpha = 0.3$):

(a) Compare the four aggregation methods: mean, final, max, concat_stats.

(b) For each method, visualize the distribution of reservoir representations for each digit class using a 2D PCA projection. Which aggregation produces the most separable class clusters?

(c) Propose and implement a "learned aggregation" in which the frame-level states are pooled using attention weights: $\mathbf{r} = \sum_t a_t \mathbf{x}(t)$ where $a_t = \text{softmax}(\mathbf{v}^\top \mathbf{x}(t))$ and $\mathbf{v}$ is a learned attention vector. Note that training $\mathbf{v}$ jointly with $W_{\text{out}}$ requires a simple iterative optimization (or fixed-point iteration). Implement this and report accuracy.

**21.9** (Speaker-Independent vs. Speaker-Dependent). Train separate ESNs for each speaker (speaker-dependent models) and compare their accuracy to the speaker-independent (leave-one-out) model.

(a) How much does accuracy improve for speaker-dependent models?

(b) Speaker-dependent models require only $\sim 50$ training examples per class (5 speakers × 10 reps). Is the reservoir representation rich enough to train a good model from so few examples?

(c) Implement speaker adaptation: train a speaker-independent ESN, then fine-tune only the readout weights $W_{\text{out}}$ using 5–10 adaptation utterances from a new speaker. Report accuracy before and after adaptation.

## Advanced Exercises

**21.10** (Physical Reservoir for Speech). A photonic delay-based reservoir (Chapter 17) operates at 500 MHz with 50 virtual nodes per input symbol.

(a) At a frame rate of 100 Hz (10 ms per frame), how many MFCC feature vectors can the photonic reservoir process per second?

(b) Each feature vector has $d_{\text{in}} = 13$ components. The photonic reservoir processes a scalar input; propose an encoding scheme that converts the 13-dimensional MFCC vector to a scalar input signal for the photonic reservoir.

(c) Estimate the energy per inference (per utterance) for the photonic reservoir versus a GPU-based ESN, assuming the GPU operates at 300 W and the photonic reservoir at 10 mW.

**21.11** (Beyond Digits: Phoneme Recognition on TIMIT). The TIMIT corpus contains 630 speakers reading 10 sentences each, with frame-level phoneme labels. This is a more demanding benchmark than FSDD.

(a) Modify the provided ESN code to perform frame-level phoneme classification (outputting a phoneme label per frame rather than a digit label per utterance). What changes are needed?

(b) The standard TIMIT evaluation uses 48 phoneme classes, reduced to 39 for scoring. Implement this reduction (a standard mapping table is available at https://catalog.ldc.upenn.edu/LDC93S1).

(c) Investigate how performance changes with context: instead of using only the current reservoir state $\mathbf{x}(t)$, also include $\mathbf{x}(t-2)$, $\mathbf{x}(t-1)$, $\mathbf{x}(t+1)$, $\mathbf{x}(t+2)$ (a 5-frame window of reservoir states) as the readout input. This provides explicit temporal context. Does it help?
