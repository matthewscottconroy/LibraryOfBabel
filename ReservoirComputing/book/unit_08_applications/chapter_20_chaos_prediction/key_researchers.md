# Chapter 20 — Key Researchers

---

## Jaideep Pathak

**Affiliation:** Lawrence Berkeley National Laboratory, Computational Research Division (also previously University of Maryland)

**Contributions:** Pathak is the lead author of the two landmark papers on reservoir computing for chaotic prediction [Pathak2017, Pathak2018] that established the field as a serious approach to model-free prediction of high-dimensional spatiotemporal chaos. His 2017 *Chaos* paper demonstrated the first use of RC to replicate chaotic attractors and estimate Lyapunov exponents, while the 2018 *PRL* paper showed 8 Lyapunov times of valid prediction on the Kuramoto-Sivashinsky equation — far beyond all prior methods. Pathak subsequently developed hybrid model/machine-learning approaches that combine knowledge of the physics with reservoir learning.

**Selected publications:**
- Pathak, J., Lu, Z., Hunt, B.R., Girvan, M., & Ott, E. (2017). Using machine learning to replicate chaotic attractors and calculate Lyapunov exponents from data. *Chaos*, 27(12), 121102.
- Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data. *Physical Review Letters*, 120(2), 024102.

---

## Brian Hunt

**Affiliation:** University of Maryland, Department of Mathematics and Institute for Physical Science and Technology

**Contributions:** Hunt has been a key contributor to the mathematical understanding of chaotic prediction and data assimilation. His collaboration with Pathak, Girvan, and Ott on reservoir computing for chaos [Pathak2017, Pathak2018] brought rigorous dynamical systems analysis to the evaluation of machine learning predictions. His background in ergodic theory and strange attractors provides the theoretical foundation for interpreting VPT results in terms of attractor geometry.

---

## Michelle Girvan

**Affiliation:** University of Maryland, Department of Physics and Institute for Physical Science and Technology

**Contributions:** Girvan is a co-author of the Pathak et al. papers and a faculty member of the University of Maryland chaos group. Her research spans network science, biological physics, and nonlinear dynamics. Her contribution to the RC/chaos work includes the analysis of the reconstructed attractor and the benchmarking methodology.

---

## Edward Ott

**Affiliation:** University of Maryland, Department of Physics and Department of Electrical and Computer Engineering

**Contributions:** Ott is one of the founding figures of chaos theory, known for the Ott-Grebogi-Yorke (OGY) method for controlling chaos [Ott1990], the development of riddled basins and noise-induced chaos, and the theory of chaotic synchronization. His collaboration with Pathak et al. on RC for chaos prediction brought deep dynamical systems theory to machine learning for chaotic systems. The interpretation of VPT in terms of the attractor's Lyapunov structure (Section 20.2) reflects Ott's theoretical perspective.

**Selected publications:**
- Ott, E. (2002). *Chaos in Dynamical Systems*, 2nd ed. Cambridge University Press.
- Ott, E., Grebogi, C., & Yorke, J.A. (1990). Controlling chaos. *Physical Review Letters*, 64(11), 1196.

---

## Daniel J. Gauthier

**(See also Chapter 15)**

**Contributions to this chapter:** Gauthier's NVAR paper [Gauthier2021] provides the direct comparison between NVAR and ESN on the Lorenz system (both achieving $\approx 5$ Lyapunov times), establishing that the relevant benchmark is not which method does better on Lorenz (they are comparable) but which method scales to harder problems.

---

## Floris Takens

**Affiliation:** University of Groningen (Netherlands; 1940–2010)

**Contributions:** Takens proved the embedding theorem [Takens1981] that provides the theoretical foundation for this entire chapter. His work showed that a single scalar observation of a dynamical system is sufficient to reconstruct the full attractor topology, provided the observation dimension is large enough. This result was revolutionary: it meant that the full complexity of a chaotic system is, in principle, observable from a single measurable variable. The reservoir computing approach can be seen as a generalization of Takens' delay embedding to nonlinear, high-dimensional observation functions.
