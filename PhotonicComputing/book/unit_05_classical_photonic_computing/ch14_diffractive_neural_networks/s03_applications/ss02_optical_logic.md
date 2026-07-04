# Subsection 14.3.2: Optical Logic Operations

## Orientation

A diffractive network can be trained to compute Boolean logic: present the two operands as patterns of light, and the network routes energy so that an output region is bright for logic 1 and dark for logic 0. This subsection shows how a passive, linear optical network manages a nonlinear function like XOR, and draws the sharp contrast with the gain-based all-optical logic that Chapter 11 found wanting.

---

## 14.3.2.1 Encoding Logic in Light

Qian et al. (2020) trained diffractive networks to realize the full set of two-input Boolean gates — AND, OR, NOT, NAND, NOR, XOR, XNOR — at microwave frequencies. The scheme is spatial. Each input bit is encoded as the presence or absence of illumination at a designated input aperture (light on $=$ logic 1, off $=$ logic 0), the trained diffractive layers propagate and interfere the fields, and the output logic state is read as the optical intensity at a target detector region: above a threshold is 1, below is 0. Different gates are different trained masks, and cascading gates means feeding one network's output region into the next's input.

## 14.3.2.2 How a Linear Network Computes XOR

It is worth seeing why this is possible at all, since XOR is the textbook example of a function no single linear threshold can separate. The resolution is that the network is linear in the optical *field*, but the detector measures $|E|^2$ — and that square-law step is the nonlinearity. Let the field the network delivers to the output region be $E_D^{(A)}$ when only input A is on and $E_D^{(B)}$ when only input B is on. If the network is trained so that these two single-input responses are equal and opposite, $E_D^{(A)} = +a$ and $E_D^{(B)} = -a$, then by linear superposition the both-on case gives $E_D^{(A+B)} = a - a = 0$. The measured intensities are then $0$ for $(0,0)$, $|a|^2$ for $(1,0)$, $|a|^2$ for $(0,1)$, and $0$ for $(1,1)$ — exactly the XOR truth table. The gate works by destructive interference at the output, converted to a dark reading by the square-law detector.

## 14.3.2.3 Worked Example: XOR Truth Table and Contrast Ratio

The construction above makes the fabrication requirement concrete.

| $A$ | $B$ | field at $D$ | intensity | XOR |
|:---:|:---:|:---:|:---:|:---:|
| 0 | 0 | $0$ | $0$ | 0 |
| 1 | 0 | $+a$ | $\lvert a\rvert^2$ | 1 |
| 0 | 1 | $-a$ | $\lvert a\rvert^2$ | 1 |
| 1 | 1 | $a-a$ | $\approx 0$ | 0 |

The three "easy" rows are automatic; the demanding one is $(1,1)$, where both inputs are fully illuminated yet the output must go dark. Any residual imbalance — the two single-input amplitudes not perfectly equal, or their phases not perfectly $\pi$ apart, whether from training error, fabrication tolerance, or misalignment — leaves a leakage field $E_\text{leak}$ at $D$, so the "0" level becomes $|E_\text{leak}|^2$ rather than zero. The gate's reliability is set by the **contrast ratio** $\mathrm{CR} = |a|^2 / |E_\text{leak}|^2$ between the logic-1 rows and this worst-case logic-0 row. With the threshold placed at the geometric mean of the two levels, robust operation against detector noise wants CR of order 10 (10 dB) or more. A $10\%$ amplitude imbalance alone leaves $E_\text{leak} \approx 0.1\,a$, capping suppression at $|0.1a|^2 = 0.01|a|^2$, i.e. $\mathrm{CR} \approx 20$ dB — so amplitude and phase balance must be held to the few-percent level. Contrast ratio, not gate complexity, is the practical figure of merit.

## 14.3.2.4 Contrast with Chapter 11: Passivity versus Gain

Chapter 11 pursued all-optical logic in the mould of digital electronics — a gate as a nonlinear switch with gain, one gate driving the next — and ran aground on energy-per-bit, insufficient gain, and cascadability: without gain each stage loses signal, and fan-out (one output driving several inputs) is impossible. Diffractive logic sidesteps that entire framing. It is not a switch but a trained analog mapping: passive diffraction rearranges input light into an output intensity pattern, with the only nonlinearity supplied for free by the detector's square law. Its genuine advantages are passivity (no per-gate power) and parallelism (one aperture can evaluate many gates, or the same gate across much data, at once). Its limits are the mirror image: being passive it has no gain, so fan-out and deep cascading bleed signal exactly as in Chapter 11, and being fixed at fabrication (off the SLM) it cannot be reprogrammed to a new function without a new device. Diffractive logic is compelling as a parallel optical pre-processor, not as a general-purpose replacement for the transistor.

---

## References

[1] Qian, C., Lin, X., Lin, X., Xu, J., Sun, Y., Li, E., Zhang, B., & Chen, H. (2020). "Performing optical logic operations by a diffractive neural network." *Light: Science & Applications*, 9, 59. [The demonstration of trained diffractive Boolean gates and cascading — the central reference of this subsection.]

[2] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [The diffractive framework and square-law readout that make interference-based logic possible.]

[3] Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114. [Error and contrast sensitivity of diffractive mappings, which set the achievable logic contrast ratio.]
