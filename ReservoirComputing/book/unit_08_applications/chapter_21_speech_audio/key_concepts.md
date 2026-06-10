# Chapter 21: Key Concepts

## Mel-Frequency Cepstral Coefficients (MFCCs)

The standard acoustic features for speech recognition. Computed by: (1) framing the waveform and computing the power spectrum, (2) applying a triangular filterbank on the mel frequency scale, (3) taking logarithms, (4) applying the Discrete Cosine Transform. The mel scale $\text{mel}(f) = 2595\log_{10}(1 + f/700)$ approximates the logarithmic frequency resolution of the human auditory system. Delta and delta-delta MFCCs append first and second temporal derivatives, yielding a 39-dimensional feature vector per 10 ms frame.

## Formants

Resonant frequencies of the vocal tract during vowel production. The first formant $F_1$ (300–1000 Hz) and second formant $F_2$ (700–2500 Hz) together characterize vowel identity: each vowel occupies a distinct region of the $(F_1, F_2)$ vowel quadrilateral. Formants change on the 50–100 ms phoneme timescale, which should match the ESN's effective time constant $\tau_{\text{eff}} = \Delta t / \alpha$.

## Phoneme

The minimal sound unit that distinguishes meaning in a language. English has approximately 44 phonemes. Phonemes are categorized by manner of articulation (vowels, fricatives, stops, nasals, liquids) and place of articulation. Automatic phoneme recognition requires temporal context because phoneme acoustics depend strongly on neighboring phonemes (co-articulation).

## Co-articulation

The phenomenon in which the acoustic realization of a phoneme is influenced by neighboring phonemes due to continuous movement of the articulators. A consonant before a high front vowel will show anticipatory tongue raising. Co-articulation creates context dependence that is addressed in reservoir computing by the reservoir's fading memory — the reservoir state at any frame implicitly encodes a window of recent acoustic history.

## Cepstral Mean Normalization (CMN)

Subtracting the mean MFCC vector over an utterance: $\tilde{c}_i(n) = c_i(n) - \bar{c}_i$. Removes slowly varying channel effects (microphone frequency response, room acoustics) that shift MFCC values systematically across the utterance. Applied per utterance, CMN is a simple but effective normalization for speaker and channel variability.

## Free Spoken Digit Dataset (FSDD)

An open-source benchmark dataset for spoken digit recognition: 3000 recordings (6 speakers × 10 digits × 50 repetitions) at 8 kHz. The standard evaluation protocol is leave-one-speaker-out cross-validation, which tests generalization across speakers. ESNs with $N = 500$ and MFCC features typically achieve $\sim 97\%$ accuracy on FSDD.

## Leaking Rate (Speech Context)

The leaking rate $\alpha$ in the leaky-integrator ESN $\mathbf{x}(n) = (1-\alpha)\mathbf{x}(n-1) + \alpha\tanh(\cdots)$ determines the effective time constant $\tau_{\text{eff}} = \Delta t / \alpha$ of reservoir neurons. For speech processing with 10 ms frames, $\alpha = 0.1$ gives $\tau_{\text{eff}} = 100$ ms (phoneme timescale), while $\alpha = 0.3$ gives $\tau_{\text{eff}} \approx 33$ ms. Matching $\tau_{\text{eff}}$ to the timescale of the target feature is a key hyperparameter design principle.

## Utterance-Level Aggregation

The process of collapsing a variable-length sequence of reservoir states $\{\mathbf{x}(1), \ldots, \mathbf{x}(T)\}$ to a fixed-length feature vector for classification. Common methods: mean pooling (average state), max pooling (maximum activation), final state, concatenated mean and standard deviation. For digit recognition, concatenating mean and std typically outperforms single statistics.

## Short-Time Fourier Transform (STFT)

The local spectrum of a windowed speech segment: $\text{STFT}(n,k) = \sum_m p(nH+m)w(m)e^{-j2\pi km/M}$. Provides the time-frequency representation of speech. The magnitude STFT $|\text{STFT}(n,k)|$ is the starting point for MFCC computation and for spectrogram-based deep learning approaches. Frame length 25 ms and hop size 10 ms are standard choices.

## TI-46 Corpus

A larger spoken digit/letter dataset with 46 speakers, used as a more challenging benchmark than FSDD. The higher speaker diversity and recording variability make TI-46 more representative of real-world ASR conditions. RC performance on TI-46 digit recognition: $\sim 94$–$97\%$, depending on features and reservoir size.

## Prosody

Suprasegmental speech features including pitch contour (F0), duration patterns, and amplitude variation that convey stress, intonation, and phrasing. Prosody operates at timescales of 100 ms to several seconds, beyond the memory horizon of standard ESN configurations. Capturing prosody requires long-memory reservoirs (small $\alpha$, large $\rho$) or explicit multi-scale architectures.
