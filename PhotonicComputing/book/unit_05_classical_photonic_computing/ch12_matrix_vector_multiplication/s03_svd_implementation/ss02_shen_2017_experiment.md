# Subsection 12.3.2: The Shen et al. 2017 Experiment

## Orientation

Every research field has a paper that converts a plausible idea into an unavoidable one. For photonic neural networks that paper is Shen et al., "Deep learning with coherent nanophotonic circuits," published in *Nature Photonics* in June 2017. The experiment itself was modest — a two-layer network classifying vowel sounds with 76.7% accuracy, worse than a laptop — but it demonstrated every element of the architecture this chapter has developed (MZI mesh, SVD programming, calibration, readout) working together on a fabricated silicon chip, and it stated the energy-scaling argument in a form that attracted both physicists and venture capital. This subsection reconstructs the experiment in enough detail that you can evaluate both its achievement and its limitations.

---

## 12.3.2.1 The Hardware: A Programmable Nanophotonic Processor

The heart of the experiment was a **programmable nanophotonic processor (PNP)**: a silicon-on-insulator photonic chip containing **56 Mach-Zehnder interferometers**, each with thermo-optic phase shifters on the internal arm ($\theta$, setting the splitting ratio) and at one input ($\phi$, setting the differential phase). The chip had been designed originally for quantum optics experiments in Dirk Englund's group at MIT — programmable interferometer meshes are equally useful for bosonic quantum walks and for classical linear algebra, a point developed further in Unit VII.

Key device parameters:

| Parameter | Value |
|---|---|
| Platform | Silicon-on-insulator, SiO$_2$ cladding | 
| MZI count | 56 (mesh of cascaded $4\times4$ unitary blocks) |
| Phase control | Thermo-optic heaters, $\sim$10 mW per $\pi$ shift |
| Modes used per layer | 4 |
| MZIs per $4\times4$ unitary | 6 (Reck/Clements count: $N(N-1)/2 = 6$) |
| Wavelength | C-band, $\sim$1550 nm |
| Readout | Off-chip photodetection of output mode powers |

Each neural network layer was mapped onto the chip as an **optical interference unit (OIU)** implementing the layer's $4\times4$ weight matrix via the SVD architecture of Subsection 12.3.1: a $V^\dagger$ sub-mesh, a diagonal attenuation stage, and a $U$ sub-mesh. Because the processor had only one mesh, the layers of the network were executed *sequentially*: program layer 1, inject the input vector as coherent amplitudes, detect the outputs, apply the nonlinearity, then reprogram (or re-use) the mesh for layer 2 with the previous outputs re-encoded as new inputs.

---

## 12.3.2.2 The Task and the Network

The benchmark was **vowel recognition**: classify a spoken vowel from a 4-dimensional feature vector of formant frequencies (the resonances of the vocal tract) extracted from recordings of many speakers. The dataset comprised 360 examples across four vowel classes, split evenly into training and test sets. This is deliberately a small problem — four inputs, four outputs — chosen to match the four optical modes available.

The network: two fully connected $4 \times 4$ layers with a nonlinear activation between them. The activation function was *not* optical. The output powers of layer 1 were detected, a saturable-absorber-like nonlinearity was applied in electronic post-processing, and the result was re-encoded onto the optical input of the next matrix multiplication. The paper modeled this nonlinearity as the transmission of an ideal saturable absorber — a physically motivated choice intended to show that a future all-optical version could use a real absorber (Chapter 13, Section 13.2 examines why that step is harder than it sounds).

Training was performed entirely offline on a conventional computer; only inference ran on the photonic chip. The trained weight matrices were decomposed via SVD and translated into the 56 phase-shifter settings using the calibration map of each MZI ($P_\pi$ and phase offset measured individually, per Section 12.2.4).

---

## 12.3.2.3 Results

- **Digital baseline (64-bit floating point):** 91.7% correct on the test set.
- **Photonic hardware:** **76.7%** correct.

The 15-point gap is the honest headline, and the paper's most valuable contribution was diagnosing it. Shen et al. simulated the network's accuracy as a function of two error parameters: the standard deviation of the *phase encoding error* on each programmed phase shifter and the *photodetection noise* of the readout. The measured accuracy was consistent with the accumulated effect of these two error channels — with phase error the dominant contributor. Sources of phase error included finite calibration precision, thermal crosstalk between heaters (one heater's dissipated power shifts its neighbors' phases), and drift between calibration and measurement.

Two durable lessons emerged from this error analysis:

1. **Analog error compounds through depth.** Each $4\times4$ unitary had a fidelity of roughly 95% relative to target; errors from the $V^\dagger$ mesh, $\Sigma$ stage, $U$ mesh, and two layers compound multiplicatively. Scaling to the $N = 64$–1024 meshes of commercial interest would be hopeless at this per-element error level — motivating the hardware error-correction work (Bandyopadhyay et al. 2021; Hamerly et al. 2022) and the self-configuration methods (Miller 2013) discussed in Section 12.2.4.

2. **Neural network inference is unusually noise-tolerant.** A 5% matrix error caused only a 15-point accuracy loss on a 4-class task, and much of that loss is recoverable by *training with the noise in the loop* — the noise-aware training strategy that Chapter 13 (Section 13.3.3) treats in detail. Classification accuracy degrades gracefully, not catastrophically, because the argmax readout only needs the correct class score to exceed the others, not to be numerically exact.

The paper also articulated the throughput argument quantitatively: once programmed, the mesh performs its $N^2$-MAC matrix product in a single optical transit ($\sim$100 ps for a millimeter-scale mesh), and the energy per inference is dominated by lasers, detection, and electronics rather than by the multiplication itself. The projected regime of interest — forward inference at GHz vector rates with sub-picojoule-per-MAC system energy at large $N$ — became the design target for the companies discussed in the next subsection.

---

## 12.3.2.4 What the Experiment Did Not Show

A critical reading is part of the graduate curriculum, so let us be explicit about the boundaries of the demonstration:

- **Scale:** $N = 4$. The favorable $O(N^2)$-operations-per-transit scaling was argued, not demonstrated; at $N = 4$, the electronic overhead (DACs, detection, re-encoding) dwarfs any optical advantage.
- **Nonlinearity:** applied in software. No optical or even analog-electronic activation function was realized on chip.
- **Weight update speed:** thermo-optic reprogramming takes microseconds per phase and the full mesh was reconfigured on much slower laboratory timescales; the demonstration is inference-only, with weights effectively static.
- **Precision:** roughly 4–5 effective bits of matrix fidelity, adequate for this 4-class task but marginal for harder ones.
- **Energy:** no end-to-end energy advantage was measured (nor claimed for this prototype).

None of these caveats diminish the paper's historical role. It defined the architecture (mesh + SVD + electronic nonlinearity), the methodology (offline training, calibration, noise analysis), and the failure modes (phase error, thermal crosstalk, O-E-O overhead) that the subsequent decade of research — including essentially everything in Chapter 13 — has been working through.

---

## References

[1] Shen, Y., Harris, N.C., Skirlo, S., Prabhu, M., Baehr-Jones, T., Hochberg, M., Sun, X., Zhao, S., Larochelle, H., Englund, D., & Soljačić, M. (2017). "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446. [The primary source for this subsection.]

[2] Harris, N.C., et al. (2017). "Quantum transport simulations in a programmable nanophotonic processor." *Nature Photonics*, 11, 447–452. [The same 56-MZI processor used for quantum walks — companion paper illustrating the dual-use nature of programmable meshes.]

[3] Bandyopadhyay, S., Hamerly, R., & Englund, D. (2021). "Hardware error correction for programmable photonics." *Optica*, 8(10), 1247–1255. [The systematic answer to the phase-error problem identified by the 2017 experiment.]

[4] Miller, D.A.B. (2013). "Self-configuring universal linear optical component." *Photonics Research*, 1(1), 1–15. [Progressive self-alignment as an alternative to explicit calibration.]
