# Audio Generation with Generative Reservoirs

## The Audio Generation Problem

Audio generation requires producing a time series with specific spectro-temporal structure: correct pitch, timbre, rhythm, and temporal envelope. Unlike audio recognition (which tolerates some latency and operates offline), audio generation must produce samples at or faster than real-time, with stable long-term dynamics. This places it squarely in the domain of generative reservoir computing and FORCE learning.

At its most basic, audio generation by a reservoir means the reservoir, in autonomous mode, produces a waveform matching a target audio signal. At higher abstraction levels, it means generating audio conditioned on a control signal (pitch, style, intensity) without direct waveform supervision — using the reservoir's learned attractor to produce the correct temporal pattern on demand.

## Reservoir in Autonomous Mode

For waveform generation, the reservoir is trained in teacher-forced mode on the target audio signal $y^*(t)$, then operated autonomously:

$$x_t = \tanh(\mathbf{W}^{\text{rec}} x_{t-1} + \mathbf{W}^{\text{fb}} y_{t-1}), \quad y_t = \mathbf{w}^{\text{out} \top} \mathbf{x}_t.$$

During training, $y_{t-1} = y^*_{t-1}$. During generation, $y_{t-1}$ is the reservoir's own output from the previous step. As discussed in Chapter 10, teacher-forced training does not guarantee stable autonomous generation; FORCE learning (Chapter 11) is required for stability [Jaeger & Haas 2004].

For periodic audio (sinusoids, musical notes), the target is a limit cycle. For rhythmically complex audio (drum patterns, speech rhythm), the target is a more complex periodic orbit. In all cases, the FORCE-trained reservoir must have an attracting orbit at the target waveform.

## FORCE Learning for Audio

FORCE learning for audio generation proceeds identically to the abstract formulation of Chapter 11, applied to a 1D audio waveform. The RLS update at each sample maintains $e(t) = y^*(t) - y(t) \approx 0$ throughout training. For audio at a sample rate of $f_s = 22{,}050$ Hz, a 1-second training signal requires $T = 22{,}050$ RLS updates, each costing $O(N^2)$ — approximately $5 \times 10^9$ operations for $N = 500$, which is computationally expensive but tractable.

The stability of autonomous audio generation after FORCE training depends on the Lyapunov exponents of the closed-loop system. For periodic audio, all Lyapunov exponents must be negative (stable limit cycle). For quasi-periodic audio (two incommensurate frequencies), the Lyapunov spectrum should have two zero exponents (corresponding to phase drift along the quasi-periodic torus) and the rest negative [Sussillo & Abbott 2009].

## Frequency Control Problem

A key challenge for audio generation is frequency control: the reservoir must learn to oscillate at a specified target frequency $f_0$. In a simulated reservoir, the natural oscillation frequency is determined by the spectral radius and reservoir architecture. If $f_0$ does not match the reservoir's natural frequency, FORCE learning must either synchronize the reservoir to $f_0$ or create an entirely new frequency-specific attractor.

For a reservoir with $N$ neurons and spectral radius $\rho \approx 1$, the natural frequencies are approximately:

$$f_{\text{nat}} \approx \frac{\alpha(1-\rho)}{2\pi} \times f_s \quad \text{(continuous-time analog)}.$$

Providing the target frequency as an explicit input feature (e.g., a sinusoidal oscillator at $f_0$ as part of the input) greatly simplifies FORCE learning: the reservoir entrains to the oscillator rather than generating the frequency from scratch. This is the central pattern generator (CPG) approach to audio generation.

## Application: Central Pattern Generator for Rhythmic Sounds

Central pattern generators (CPGs) are neural circuits that produce rhythmic output without rhythmic input. Biological CPGs drive locomotion, breathing, and chewing. A reservoir-based CPG for audio produces rhythmic sound patterns (percussion, ambient textures) autonomously once trained.

The procedure: train a FORCE-learning reservoir on a target rhythm (e.g., 120 BPM drum pattern for $T = 5$ seconds = 600 beats). After training, the reservoir in autonomous mode produces the rhythm indefinitely, triggered by an initial state perturbation. Varying the readout weights (not the reservoir weights) can switch between different stored rhythms — the conceptor framework (Chapter 12) provides a principled method for storing and switching between multiple audio rhythms [Jaeger 2014].

## Conceptors for Audio: Switching Between Attractors

Conceptors provide a natural mechanism for audio style switching. Store conceptor $\mathbf{C}_k$ for audio pattern $p_k$ (e.g., different instrument timbres, different rhythms). During generation, apply:

$$\mathbf{x}_t = \mathbf{C}_k \tanh(\mathbf{W}^{\text{rec}} \mathbf{x}_{t-1} + \mathbf{b}),$$

switching $k$ on the fly. The conceptor routes the reservoir to the appropriate audio attractor without retraining. Smooth interpolation between patterns $j$ and $k$ is achieved using the OR conceptor $\mathbf{C}_j \vee \mathbf{C}_k$ with tuned aperture — producing a morphed audio pattern [Jaeger 2014].

---

## References

- Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
- Jaeger, H. (2014). Controlling recurrent neural networks by conceptors. *arXiv preprint*, arXiv:1403.3369.
- Sussillo, D., & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
