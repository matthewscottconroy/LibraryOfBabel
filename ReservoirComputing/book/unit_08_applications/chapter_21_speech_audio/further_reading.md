# Chapter 21: Further Reading

## Reservoir Computing for Speech

**Verstraeten, D., Schrauwen, B., Haesbroeck, M., & Stroobandt, D. (2006).** An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.
Comparative study of ESN, LSM, and BPDC approaches on speech benchmark tasks. One of the first systematic evaluations of RC for speech processing.

**Schrauwen, B., Wardermann, M., Verstraeten, D., Steil, J. J., & Stroobandt, D. (2008).** Improving reservoirs using intrinsic plasticity. *Neurocomputing*, 71(7–9), 1159–1171.
Demonstrates that intrinsic plasticity — adapting neuron gain to achieve exponential output distributions — significantly improves speech recognition performance.

**Tino, P., & Rodan, A. (2013).** Short-term memory in input-driven linear dynamical systems. *Neurocomputing*, 112, 58–71.
Theoretical analysis of how spectral radius and leaking rate interact to determine memory depth, with implications for speech timescale matching.

## Acoustic Features

**Davis, S., & Mermelstein, P. (1980).** Comparison of parametric representations for monosyllabic word recognition in continuously spoken sentences. *IEEE Transactions on Acoustics, Speech, and Signal Processing*, 28(4), 357–366.
The original MFCC paper. Establishes the theoretical justification for the mel-frequency cepstral representation and its advantages over linear prediction coefficients.

**Rabiner, L., & Juang, B. H. (1993).** *Fundamentals of Speech Recognition*. Prentice Hall.
The standard textbook reference for classical speech recognition, covering HMMs, acoustic modeling, feature extraction, and language modeling. Essential background for understanding the competitive landscape for RC speech systems.

**O'Shaughnessy, D. (1987).** *Speech Communication: Human and Machine*. Addison-Wesley.
A classic reference for phonetics and acoustic phonology, covering formants, co-articulation, and prosody at the level needed for this chapter.

## Datasets

**Jackson, Z. (2018).** Free Spoken Digit Dataset. GitHub repository: github.com/Jakobovski/free-spoken-digit-dataset.
The FSDD dataset used as the primary benchmark in this chapter.

**Garofolo, J. S., et al. (1993).** TIMIT acoustic-phonetic continuous speech corpus. *Linguistic Data Consortium*. doi:10.35111/17gk-bn40.
The standard reference corpus for phoneme recognition research. Contains 630 speakers, hand-labeled phoneme boundaries, and is required for competitive benchmarking.

## Competitive Methods

**Radford, A., Kim, J. W., Xu, T., Brockman, G., McLeavey, C., & Sutskever, I. (2023).** Robust speech recognition via large-scale weak supervision. In *ICML 2023*.
The Whisper model — state-of-the-art large-vocabulary speech recognition. Provides the upper-bound competitive baseline for isolated digit recognition.

**Graves, A., Mohamed, A. R., & Hinton, G. (2013).** Speech recognition with deep recurrent neural networks. In *ICASSP 2013*.
The paper establishing LSTMs with CTC decoding as competitive with HMMs for phoneme recognition. Defines the modern deep learning approach against which RC must be benchmarked.

## Online and Adaptive Methods

**Jaeger, H., Lukosevicius, M., Popovici, D., & Siewert, U. (2007).** Optimization and applications of echo state networks with leaky-integrator neurons. *Neural Networks*, 20(3), 335–352.
Introduces the leaky-integrator ESN and analyzes its properties for time-series tasks including speech. Derives the effective time constant formula used throughout this chapter.
