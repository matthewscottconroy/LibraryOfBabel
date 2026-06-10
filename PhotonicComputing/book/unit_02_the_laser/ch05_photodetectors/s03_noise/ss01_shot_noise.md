# 5.3.1 Shot Noise

## The Quantum Origin of Shot Noise

Shot noise arises from the discrete nature of charge: electric current is not a continuous fluid but a stream of individual electrons. Even in a perfect circuit with no thermal fluctuations, the stochastic arrival of electrons at the detector terminal produces current fluctuations.

Photons arrive at the detector as a Poisson process (for coherent laser light): the number of photons detected in a time interval $\Delta t$ is Poisson-distributed with mean $\bar{n} = R P_{in} \Delta t / e$. For a Poisson process, the variance equals the mean:

$$\langle(\Delta n)^2\rangle = \bar{n}$$

Converting from photon number to current fluctuation: $\Delta i = e \Delta n / \Delta t$, so:

$$\langle(\Delta i)^2\rangle = \frac{e^2}{\Delta t^2}\langle(\Delta n)^2\rangle = \frac{e^2}{\Delta t^2}\bar{n} = \frac{e^2}{\Delta t^2}\frac{I_{ph}\Delta t}{e} = \frac{e I_{ph}}{\Delta t}$$

In a bandwidth $B = 1/(2\Delta t)$ (Nyquist sampling):

$$\langle i^2_{shot}\rangle = 2e I_{ph} B$$

This is the **shot noise** formula. It depends on the mean photocurrent $I_{ph} = \mathcal{R}P_{in}$ and the measurement bandwidth $B$. It is a *white noise* (flat spectrum) up to frequencies of order $1/\tau_{transit}$.

## Physical Interpretation

The $2e$ factor (rather than $e$) reflects the two-sided bandwidth convention. The noise power spectral density of shot noise is:

$$S_i(f) = 2e I_{ph} \quad \text{[A²/Hz, one-sided]}$$

This is the fundamental quantum noise floor for optical detection. It cannot be reduced by any classical means. The only way to reduce $\langle i^2_{shot}\rangle$ for a given $B$ is to increase $I_{ph}$ — i.e., increase the optical signal power — which reduces the *relative* noise (SNR improves as $\sqrt{P_{in}}$).

## Shot Noise from Dark Current

Even without optical input, thermal generation of electron-hole pairs in the depletion region produces a dark current $I_d$. This dark current is also a Poisson process and contributes shot noise:

$$\langle i^2_{dark}\rangle = 2e I_d B$$

Total shot noise including dark current:

$$\langle i^2_{shot,total}\rangle = 2e(I_{ph} + I_d)B$$

For a Ge-on-Si detector with $I_d = 100$ nA and bandwidth $B = 10$ GHz:
$$\langle i^2_{dark}\rangle = 2 \times 1.6\times10^{-19} \times 100\times10^{-9} \times 10^{10} = 3.2\times10^{-19} \text{ A}^2$$
$$i_{dark,rms} = 17.9 \text{ nA}$$

For a signal current of 1 mA (1 mW input at $\mathcal{R} = 1$ A/W):
$$i_{signal,shot,rms} = \sqrt{2\times1.6\times10^{-19}\times10^{-3}\times10^{10}} = 1.79 \text{ μA}$$

The dark current noise is 100× smaller than signal shot noise at 1 mW — negligible. At 1 μW, they become comparable.

## Shot-Noise-Limited Operation

A receiver is said to be **shot-noise limited** when shot noise dominates over all other noise sources. In this regime:

$$\text{SNR}_{shot} = \frac{I_{ph}^2}{2eI_{ph}B} = \frac{I_{ph}}{2eB} = \frac{\mathcal{R}P_{in}}{2eB}$$

This is the fundamental quantum limit of direct detection. For $\mathcal{R} = 1$ A/W, $P_{in} = 1$ mW, $B = 10$ GHz:

$$\text{SNR}_{shot} = \frac{10^{-3}}{2\times1.6\times10^{-19}\times10^{10}} = 3.125\times10^5 = 55 \text{ dB}$$

This corresponds to ENOB $= (55 - 1.76)/6.02 \approx 8.8$ bits — an excellent precision for analog photonic computing. But achieving shot-noise-limited operation requires that thermal noise and laser RIN be smaller than shot noise, which is a non-trivial condition.
