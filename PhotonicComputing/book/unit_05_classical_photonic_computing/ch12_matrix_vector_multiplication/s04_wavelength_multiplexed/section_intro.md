# Section 12.4: Wavelength-Multiplexed Incoherent Computing

## What This Section Is About

The MZI mesh computes with the *field*: complex amplitudes interfere, and the phase relationships between waveguides must be maintained to a small fraction of a wavelength. There is a second, entirely different way to multiply with light — compute with *power*. Encode each input on its own wavelength, weight each wavelength with a tunable filter, and let a photodetector do the summation: a photodiode is blind to optical phase and simply adds up the photocurrents contributed by every wavelength landing on it. No interference, no phase stabilization, and a natural marriage with the wavelength-division multiplexing (WDM) technology that optical communications spent thirty years perfecting (Chapter 9).

The price: optical power is non-negative, so negative weights require a differential trick; and the number of independent wavelength channels — hence the vector dimension $N$ — is bounded by the free spectral range and linewidth of the filters.

**12.4.1: Microring Weight Banks** — The add-drop microring as a continuously tunable weight; the balanced-photodetector trick that yields signed weights from non-negative transmissions; crosstalk, channel count, tuning power, and demonstrated weight precision.

**12.4.2: Broadcast-and-Weight** — The Princeton network protocol that turns weight banks into full recurrent neural networks; the modulator neuron; scaling analysis and the fiber-nonlinearity-compensation demonstration.

**12.4.3: Incoherent vs. Coherent** — A structured comparison of the two great architectures of this chapter, plus the hybrid systems — phase-change-material crossbars and time-wavelength interleaved accelerators fed by microcombs — that pushed incoherent photonic computing past 10 TOPS.
