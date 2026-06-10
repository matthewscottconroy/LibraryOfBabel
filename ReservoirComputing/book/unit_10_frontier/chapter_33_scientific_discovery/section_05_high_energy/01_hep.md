# Reservoir Computing in High-Energy Physics

## 33.5.1 The High-Energy Physics Context

High-energy physics (HEP) experiments at particle colliders such as the Large Hadron Collider (LHC) at CERN produce data at staggering rates. At the LHC, proton-proton collisions occur at a rate of approximately $10^9$ per second. Each collision produces hundreds of charged particles tracked by detectors with millions of channels, generating raw data at $\sim 1$ TB/s. Storing and analyzing this entire data stream is physically impossible. The solution is a **trigger system**: a real-time signal processing pipeline that classifies each collision event within $\sim 100$ ns and discards $\sim 99.99\%$ of events as uninteresting.

The trigger challenge is therefore a real-time pattern recognition problem with the most stringent latency requirements in experimental science: $< 100$ ns from collision to accept/reject decision. This latency budget is comparable to the transit time of light across the detector ($\sim 30$ m / $c \approx 100$ ns), leaving no room for sequential computation on conventional CPUs. The current LHC trigger relies on custom FPGA (field-programmable gate array) hardware running highly optimized algorithms.

Machine learning has been proposed as a means to improve trigger quality — particularly for exotic signatures that simple threshold-based algorithms miss. The challenge is implementing machine learning at nanosecond speeds. This is where physical reservoir computing becomes relevant.

## 33.5.2 Jet Classification

A **jet** is a collimated spray of hadrons produced by the hadronization of a quark or gluon. Different types of jets have different internal structure:

- **QCD jets** (background): produced by light quarks/gluons, relatively diffuse structure
- **Top quark jets**: three-pronged structure from $t \to Wb \to q\bar{q}'b$
- **$W/Z$ boson jets**: two-pronged structure from hadronic decays
- **Higgs boson jets**: two-pronged with $b$-quark tagging

**Jet tagging** — classifying a jet by its origin — is one of the key analysis tasks at the LHC. State-of-the-art jet taggers use deep neural networks with $\sim 10^6$ parameters [Duarte et al. 2018 (hls4ml)]. These networks achieve excellent classification performance but require microsecond-scale inference times on GPUs — too slow for the hardware trigger, though suitable for the software trigger.

## 33.5.3 The Reservoir Computing Approach

Reservoir computing offers a potential path to nanosecond-speed jet classification via **physical (optoelectronic) reservoirs**. The key insight is that a physical RC system can be implemented in hardware with inherent sub-nanosecond dynamics, without any digital computation.

**Signal encoding.** The jet is represented as a time series: the $k$ highest-$p_T$ (transverse momentum) constituents are sorted by $p_T$ and their four-momenta $(p_T, \eta, \phi, m)$ form an input sequence of length $k$. The reservoir processes this sequence in real time as the particles arrive at the detector.

**Photonic reservoir.** [Coadou et al. 2022] proposed an optoelectronic reservoir for LHC jet classification. The system consists of:
- A single-mode laser with delayed optical feedback (Mackey-Glass-like reservoir)
- The $k$ jet constituents multiplexed in time, each corresponding to one virtual node
- A photodetector and ADC for readout
- A linear readout trained offline via ridge regression

The proposed photonic reservoir operates at clock speeds of $\sim 10$ GHz (100 ps per time step), giving a total latency of $\sim k \times 100$ ps $= 10$ ns for $k = 100$ constituents — well within the LHC trigger budget.

## 33.5.4 Mathematical Analysis of the Trigger Problem

The jet classification task can be framed as follows. Let $\mathcal{J} = \{(p_T^{(i)}, \eta^{(i)}, \phi^{(i)}, m^{(i)})\}_{i=1}^k$ be the jet constituents. The classifier must compute a score $s(\mathcal{J}) \in [0,1]$ indicating the probability that the jet originated from a signal process (e.g., top quark decay).

The reservoir processes the sequence:

$$
\mathbf{x}(i) = \tanh\!\left(\alpha\mathbf{x}(i-1) + \beta\mathbf{x}(i-1-\tau) + W^{\mathrm{in}}\mathbf{u}(i)\right),
$$

where $\alpha$ is the self-coupling coefficient, $\beta$ is the delayed feedback coefficient, $\tau$ is the delay time (measured in numbers of time steps), and $\mathbf{u}(i) = (p_T^{(i)}, \eta^{(i)}, \phi^{(i)}, m^{(i)})$ is the $i$-th constituent. The readout is

$$
s(\mathcal{J}) = \sigma\!\left(\mathbf{w}^T\mathbf{x}(k)\right),
$$

where $\sigma$ is the sigmoid function. Training proceeds via binary cross-entropy minimization with ridge regularization.

**Performance benchmark.** [Coadou et al. 2022] reported AUC (area under the ROC curve) of 0.87–0.92 for top quark jet tagging, compared to 0.93–0.95 for deep neural network taggers. The photonic reservoir achieves $\sim 90\%$ of the neural network performance at $\sim 100\times$ lower latency — a favorable tradeoff for the hardware trigger.

## 33.5.5 Anomaly Detection at Colliders

Beyond jet classification, reservoir computing has been proposed for **anomaly detection**: identifying collision events that are inconsistent with Standard Model predictions without specifying the new physics model in advance. This is an unsupervised learning task.

The reservoir approach is:
1. Train the reservoir to reconstruct Standard Model events (autoencoder-like setup).
2. At test time, flag events with large reconstruction error as potential new physics signals.

The fading memory of the reservoir is advantageous: it can capture the temporal structure of the detector response (calorimeter pulses, track curvature evolution) without requiring a handcrafted feature extraction pipeline.

[Govorkova et al. 2022] demonstrated anomaly detection using autoencoder neural networks on LHC data; the extension to reservoir-based autoencoders is natural but remains to be demonstrated experimentally.

## 33.5.6 The hls4ml Framework

The practical pathway to deploying machine learning in LHC triggers is the **hls4ml** (high-level synthesis for machine learning) framework [Duarte et al. 2018]. hls4ml translates trained neural networks into FPGA firmware automatically, enabling the inference of machine learning models at nanosecond speeds.

For reservoir computing, hls4ml would need to be extended to support recurrent architectures — specifically, the delay-line structure of single-node delay-feedback reservoirs. The linear readout is trivially implemented; the challenge is the recurrent reservoir state update, which introduces data dependencies that complicate FPGA pipelining.

## 33.5.7 Epistemic Status

**Current status:** Photonic reservoir computing for LHC triggers is a proposal with simulation results [Coadou et al. 2022]. It has not been demonstrated on real hardware connected to LHC detectors. The performance gap relative to deep networks is $\sim 1$–$3\%$ in AUC, which may be acceptable for the hardware trigger (where speed is paramount) but not for the offline analysis.

**What would constitute success:** A photonic reservoir operating at $\sim 10$ GHz, integrated with LHC detector electronics, demonstrating both $< 100$ ns latency and competitive classification performance on real collision data.

**Timeline:** Given the LHC upgrade schedule (High-Luminosity LHC operations beginning $\sim 2029$) and the typical development time for detector electronics, realistic deployment would require prototype demonstration by $\sim 2026$–$2027$. This is ambitious but not implausible.

## References

- Coadou, Y., Fontaine, G., Lugard, A., Miagkikh, V., Nass, K., and Womersley, R. (2022). Reservoir computing for fast jet classification at the LHC. *Journal of Instrumentation*, 17, P08022.
- Duarte, J., et al. (2018). Fast inference of deep neural networks in FPGAs for particle physics. *Journal of Instrumentation*, 13, P07027. [hls4ml]
- Govorkova, E., et al. (2022). Autoencoders on field-programmable gate arrays for real-time, unsupervised new physics detection at 40 MHz at the Large Hadron Collider. *Nature Machine Intelligence*, 4, 154–161.
- Salam, G. P. (2010). Towards jetography. *The European Physical Journal C*, 67(3), 637–686.
- Shelton, J. (2013). Jet substructure. In *Proceedings of TASI 2012*, World Scientific.
