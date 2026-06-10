# Chapter 19: Further Reading

## Memristors

**Strukov, D. B., Snider, G. S., Stewart, D. R., & Williams, R. S. (2008).** The missing memristor found. *Nature*, 453(7191), 80–83.
The landmark experimental paper reporting the first physical memristor. The model developed here ($dw/dt = f(w,I)$, $V = R(w) \cdot I$) is the standard HP memristor model used throughout this chapter.

**Chua, L. O. (1971).** Memristor — The missing circuit element. *IEEE Transactions on Circuit Theory*, 18(5), 507–519.
Chua's original prediction of the memristor from circuit-theoretic symmetry. Remarkable for predicting a physical device solely from mathematical elegance, 37 years before its experimental realization.

**Yang, J. J., Strukov, D. B., & Stewart, D. R. (2013).** Memristive devices for computing. *Nature Nanotechnology*, 8(1), 13–24.
A comprehensive review of memristive device physics and computing applications, covering both memory and logic functions. Essential background for reservoir computing with memristors.

**Prezioso, M., Merrikh-Bayat, F., Hoskins, B. D., Adam, G. C., Likharev, K. K., & Strukov, D. B. (2015).** Training and operation of an integrated neuromorphic network based on metal-oxide memristors. *Nature*, 521(7550), 61–64.
The first experimental demonstration of a memristive crossbar trained on a simple classification task. Directly relevant to the hardware reservoir computing implementation.

## Spintronics and STNOs

**Grollier, J., Querlioz, D., Camsari, K. Y., Everschor-Sitte, K., Fukami, S., & Stiles, M. D. (2020).** Neuromorphic spintronics. *Nature Electronics*, 3(7), 360–370.
The definitive review of spintronic neuromorphic computing, including reservoir computing with STNOs. Required reading for anyone working on spintronic RC. Beautifully written and comprehensive.

**Torrejon, J., Riou, M., Araujo, F. A., Tsunegi, S., Khalsa, G., Querlioz, D., ... & Grollier, J. (2017).** Neuromorphic computing with nanoscale spintronic oscillators. *Nature*, 547(7664), 428–431.
The first experimental demonstration of STNO-based reservoir computing for spoken digit recognition. This paper established STNOs as a serious neuromorphic platform and is essential reading.

**Slonczewski, J. C. (1996).** Current-driven excitation of magnetic multilayers. *Journal of Magnetism and Magnetic Materials*, 159(1–2), L1–L7.
The theoretical paper predicting spin-transfer torque, the mechanism underlying STNO operation. The Slonczewski STT term is the starting point for all STNO modeling.

**Riou, M., Torrejon, J., Garitaine, B., Araujo, F. A., Bortolotti, P., Cros, V., ... & Grollier, J. (2019).** Temporal pattern recognition with delayed-feedback spin-torque nano-oscillators. *Physical Review Applied*, 12(2), 024049.
Extends STNO reservoir computing to temporal pattern recognition using delay-feedback virtual nodes. Contains detailed modeling and experimental validation.

**Kiselev, S. I., Sankey, J. C., Krivorotov, I. N., Emley, N. C., Schoelkopf, R. J., Buhrman, R. A., & Ralph, D. C. (2003).** Microwave oscillations of a nanomagnet driven by a spin-polarized current. *Nature*, 425(6956), 380–383.
The first experimental demonstration of STNO oscillations. The foundational experimental paper establishing that STT can drive sustained GHz oscillations.

## Magnetic Skyrmions

**Fert, A., Cros, V., & Sampaio, J. (2013).** Skyrmions on the track. *Nature Nanotechnology*, 8(3), 152–156.
Proposes skyrmion racetrack memory and surveys the physics of skyrmions relevant to computing applications.

**Pierangeli, D., Marcucci, G., & Conti, C. (2019).** Large-scale photonic Ising machine by spatial light modulation. *Physical Review Letters*, 122(21), 213902.
While focused on photonics, this paper's analysis of topologically structured reservoirs provides relevant context for skyrmion reservoir proposals.

**Jiang, W., Zhang, X., Yu, G., Zhang, W., Wang, X., Jungfleisch, M. B., ... & Hoffmann, A. (2017).** Direct observation of the skyrmion Hall effect. *Nature Physics*, 13(2), 162–169.
Characterizes the skyrmion Hall effect (motion perpendicular to applied force) that enhances coupling dimensionality in skyrmion reservoirs.

## Quantum Reservoir Computing

**Fujii, K., & Nakajima, K. (2017).** Harnessing disordered-ensemble quantum dynamics for machine learning. *Physical Review Applied*, 8(2), 024030.
The foundational paper on quantum reservoir computing. Establishes the theoretical framework, proves echo state property conditions for open quantum systems, and provides initial benchmarks.

**Ghosh, S., Opala, A., Matuszewski, M., Paterek, T., & Liew, T. C. (2019).** Quantum reservoir processing. *npj Quantum Information*, 5(1), 35.
Demonstrates quantum advantage for specific tasks in reservoir computing using a driven-dissipative quantum system (polariton condensate).

**Mujal, P., Martínez-Peña, R., Nokkala, J., García-Beni, J., Giorgi, G. L., Soriano, M. C., & Zambrini, R. (2021).** Opportunities in quantum reservoir computing and extreme learning machines. *Advanced Quantum Technologies*, 4(8), 2100027.
A comprehensive review of quantum RC and its relationship to extreme learning machines, covering both theoretical opportunities and practical challenges.

## Neuromorphic Hardware Reviews

**Schuman, C. D., Potok, T. E., Patton, R. M., Birdwell, J. D., Dean, M. E., Rose, G. S., & Plank, J. S. (2017).** A survey of neuromorphic computing and neural networks in hardware. *arXiv:1705.06963*.
A broad survey of neuromorphic hardware approaches, providing context for memristive and spintronic RC within the larger neuromorphic computing landscape.

**Christensen, D. V., et al. (2022).** 2022 roadmap on neuromorphic computing and engineering. *Neuromorphic Computing and Engineering*, 2(2), 022501.
A community roadmap signed by dozens of researchers, covering both biological inspiration and hardware implementation targets for neuromorphic computing through 2030.
