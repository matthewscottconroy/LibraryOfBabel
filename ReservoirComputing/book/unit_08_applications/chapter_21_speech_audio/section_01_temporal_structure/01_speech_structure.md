# Section 21.1: Temporal Structure of Speech

## 21.1.1 The Acoustic Signal

A speech signal is a time-varying pressure wave $p(t)$ typically sampled at 8 kHz (telephone quality) or 16 kHz (wideband). The raw waveform carries all acoustic information but is extremely high-dimensional for direct reservoir input: a 100 ms window at 16 kHz contains 1600 samples. More importantly, the raw waveform exhibits strong phase sensitivity that is perceptually irrelevant — our auditory system processes phase only at very low frequencies ($< 1$ kHz).

The standard preprocessing pipeline converts raw waveforms to a sequence of feature vectors representing the time-frequency content of the signal:

### Short-Time Fourier Transform (STFT)

The STFT computes the local spectrum at each time frame:

$$\text{STFT}(n, k) = \sum_{m=0}^{M-1} p(nH + m) \cdot w(m) \cdot e^{-j2\pi km/M}$$

where:
- $n$ is the frame index
- $H$ is the hop size (typically 10 ms)
- $M$ is the window length (typically 20–25 ms)
- $w(m)$ is the analysis window (Hann or Hamming)
- $k$ is the discrete frequency bin index

The resulting spectrogram $|STFT(n,k)|^2$ is a 2D representation of energy as a function of time and frequency.

### Mel-Frequency Cepstral Coefficients (MFCCs)

MFCCs are the standard acoustic features for speech recognition, designed to mimic the frequency selectivity of the human auditory system [DavisAndMermelstein1980]:

**Step 1**: Compute the power spectrum $P(n,k) = |\text{STFT}(n,k)|^2$.

**Step 2**: Apply a bank of $B$ triangular filters on the mel scale. The mel-frequency scale is a perceptual frequency scale in which equal distances correspond to equal perceived pitch differences:

$$\text{mel}(f) = 2595 \cdot \log_{10}\left(1 + \frac{f}{700}\right)$$

The center frequencies of the mel filterbank are equally spaced on the mel scale, corresponding to:

$$f_b = 700\left(10^{b \cdot \Delta_{\text{mel}} / 2595} - 1\right), \qquad b = 1, \ldots, B$$

Each filter integrates the power spectrum over its triangular support: $E_b(n) = \sum_k H_b(k) P(n,k)$.

**Step 3**: Take the log: $\log E_b(n)$.

**Step 4**: Apply the Discrete Cosine Transform to decorrelate the log-filterbank energies:

$$c_i(n) = \sqrt{\frac{2}{B}} \sum_{b=1}^B \log E_b(n) \cdot \cos\left(\frac{\pi(2b-1)i}{2B}\right)$$

The first $D$ coefficients $\{c_1(n), \ldots, c_D(n)\}$ form the MFCC feature vector. Typically $D = 13$. The zeroth cepstral coefficient $c_0$ is often excluded or replaced by the log-energy.

**Delta and delta-delta features**: To capture temporal dynamics, the first derivative (delta) and second derivative (delta-delta) of the MFCCs are appended:

$$\Delta c_i(n) = \frac{\sum_{\tau=1}^K \tau \left[c_i(n+\tau) - c_i(n-\tau)\right]}{2\sum_{\tau=1}^K \tau^2}$$

This yields a 39-dimensional feature vector per frame (13 MFCCs + 13 deltas + 13 delta-deltas) at 100 frames per second.

## 21.1.2 Phonemes and Formants

**Phonemes** are the smallest units of sound that distinguish meaning in a language. English has approximately 44 phonemes. They divide naturally into:

- **Vowels** (e.g., /æ/ in "cat", /iː/ in "meet"): voiced, quasi-periodic sounds characterized by resonant frequencies called formants
- **Fricatives** (e.g., /s/, /f/): turbulent, noise-like sounds with broadband high-frequency energy
- **Stops** (e.g., /p/, /b/, /t/): characterized by a closure period (silence) followed by a brief burst release
- **Nasals** (e.g., /m/, /n/): characterized by anti-resonances (spectral zeros) from the nasal cavity

**Formants** are the resonant frequencies of the vocal tract. For a simple tube model of length $L$ and speed of sound $c$, the resonant frequencies are:

$$F_n = \frac{(2n-1)c}{4L}, \qquad n = 1, 2, 3, \ldots$$

For an adult male with $L \approx 17$ cm and $c = 35000$ cm/s:
- $F_1 \approx 500$ Hz (first formant)
- $F_2 \approx 1500$ Hz (second formant)
- $F_3 \approx 2500$ Hz (third formant)

Vowels are primarily distinguished by their first two formant frequencies $(F_1, F_2)$. For example:
- /iː/ ("feet"): $F_1 \approx 300$ Hz, $F_2 \approx 2300$ Hz
- /ɑː/ ("father"): $F_1 \approx 700$ Hz, $F_2 \approx 1100$ Hz
- /uː/ ("boot"): $F_1 \approx 300$ Hz, $F_2 \approx 870$ Hz

The vowel quadrilateral in $(F_1, F_2)$ space provides an acoustic map of the vowel inventory that is a fundamental tool in phonetics.

### Why Formants Matter for Reservoirs

Formants are slowly varying features (changing on the timescale of vocal tract movement, $\sim 50$ ms) riding on top of a rapidly varying carrier (the glottal pulse rate, typically 80–250 Hz for adult voices). A reservoir that operates at the frame rate (100 Hz, i.e., 10 ms steps) will naturally distinguish these timescales:

- Fast-time-constant reservoir units ($\tau_{\text{res}} \sim 10$ ms) will track the rapid acoustic variations
- Slow-time-constant units ($\tau_{\text{res}} \sim 100$ ms) will integrate over formant patterns

This natural multi-scale representation is one of the key reasons reservoirs perform well on speech tasks.

## 21.1.3 Prosody and Suprasegmental Features

Prosody refers to the rhythmic, stress, and intonational patterns of speech — the "music" of language as opposed to its "words." Prosodic features include:

**Fundamental frequency (F0)**: The pitch contour, determined by the rate of vocal fold vibration. Ranges from $\sim 80$ Hz (low male voice) to $\sim 400$ Hz (high female or child voice).

**Duration**: The length of phonemes and words. Stressed syllables are longer; content words are longer than function words.

**Amplitude**: Loudness variation, correlated with stress.

Prosody operates at timescales of 100 ms to several seconds — much longer than phoneme-level features. Reservoir networks with long-time-constant dynamics (large spectral radius, Chapter 5) can in principle capture prosodic patterns, but in practice the very long dependencies involved (several seconds) pose challenges.

### Temporal Hierarchy Summary

| Level | Timescale | Feature | RC Relevant? |
|---|---|---|---|
| Glottal pulse | 4–12 ms | F0, periodicity | Sometimes (frame rate > 1 kHz) |
| Phoneme | 30–100 ms | Formants, spectral shape | Yes (primary) |
| Syllable | 150–300 ms | Rhythm, onset/offset | Yes (medium $\tau$) |
| Word | 200–800 ms | Phoneme sequence | Yes (long memory) |
| Phrase/sentence | 1–5 s | Prosody, syntax | Challenging |

## 21.1.4 Feature Extraction for Reservoir Input

The preprocessing pipeline for reservoir-based speech processing:

1. **Framing**: Segment the waveform into overlapping frames ($M = 400$ samples, $H = 160$ samples at 16 kHz, giving 25 ms frames at 10 ms steps).

2. **MFCC extraction**: Compute 13 MFCCs per frame using a 40-filterbank mel spectrum.

3. **CMN (Cepstral Mean Normalization)**: Subtract the mean MFCC over the utterance to reduce channel effects: $\tilde{c}_i(n) = c_i(n) - \bar{c}_i$.

4. **Delta features** (optional): Append 13 delta-MFCCs, giving 26-dimensional feature vectors.

5. **Input to reservoir**: Feed the feature sequence $\mathbf{u}(n)$ as the reservoir input at each frame $n$.

This pipeline converts a variable-length audio waveform into a sequence of fixed-dimensional feature vectors, which is the natural input format for a recurrent reservoir. The reservoir then processes this sequence online, frame by frame, maintaining a hidden state that encodes the relevant history.

### Alternative: Raw Waveform and Learned Features

For reservoir computing with hardware implementations (Chapter 19), it may be preferable to skip the MFCC preprocessing and drive the reservoir directly from the raw waveform or a simple bandpass-filtered version. Physical reservoirs operating at hardware sampling rates can process raw audio at GHz rates, making MFCC computation a potential bottleneck. For software ESNs operating at 100 frames/second, MFCC features are standard practice.

Learned feature representations (e.g., from a pre-trained convolutional neural network) can also be used as reservoir input [MarrerEtAl2021], combining the representational power of deep learning with the temporal processing capabilities of reservoir computing. This hybrid approach is increasingly competitive on medium-vocabulary tasks.
