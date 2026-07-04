# Chapter 14: Further Reading and References

## Foundational D2NN

**Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018).** "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008.
*The paper that founded the field: five 3D-printed phase layers at 0.4 THz, trained by differentiable diffraction, classifying MNIST at 91.75%. Every section of this chapter refers back to it; read it first.*

**Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019).** "Analysis of diffractive optical neural networks and their integration with electronic neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 3700114.
*The quantitative companion to Lin 2018: how accuracy scales with layers and neurons, why depth helps with diminishing returns, and how diffractive front-ends pair with electronic back-ends. The analytical backbone of Section 14.4.*

---

## Analysis, Capacity, and Extensions

**Kulce, O., Mengu, D., Rivenson, Y., & Ozcan, A. (2021).** "All-optical information-processing capacity of diffractive surfaces." *Light: Science & Applications*, 10, 25.
*Derives how many independent input–output connections a diffractive surface, or a cascade of them, can realize. The formal basis for the space-bandwidth-product ceiling of Subsection 14.4.1.*

**Mengu, D., Zhao, Y., Yardimci, N.T., Rivenson, Y., Jarrahi, M., & Ozcan, A. (2020).** "Misalignment resilient diffractive optical networks." *Nanophotonics*, 9(13), 4207–4219.
*Introduces "vaccination": training with modeled layer displacements to build networks that tolerate real-world misalignment. The primary reference for Subsection 14.4.2.*

**Rahman, M.S.S., Li, J., Mengu, D., Rivenson, Y., & Ozcan, A. (2021).** "Ensemble learning of diffractive optical networks." *Light: Science & Applications*, 10, 14.
*Shows that ensembles of diffractive networks recover much of the accuracy gap to digital models, an architectural route around the single-network expressivity ceiling.*

---

## Implementations

**Zhou, T., Lin, X., Wu, J., Chen, Y., Xie, H., Li, Y., Fan, J., Wu, H., Fang, L., & Dai, Q. (2021).** "Large-scale neuromorphic optoelectronic computing with a reconfigurable diffractive processing unit." *Nature Photonics*, 15, 367–373.
*The reconfigurable DPU: SLM-programmable diffractive layers with electronic nonlinearity, scaling diffractive computing to millions of neurons and multiple tasks. The counterpoint to fixed printed stacks.*

**Fu, T., Zang, Y., Huang, Y., Du, Z., Huang, H., Hu, C., Chen, M., Yang, S., & Chen, H. (2023).** "Photonic machine learning with on-chip diffractive optics." *Nature Communications*, 14, 70.
*Brings diffraction onto an integrated chip, trading free-space aperture for a compact planar slab — the on-chip route referenced throughout Section 14.2.*

**Yu, N., & Capasso, F. (2014).** "Flat optics with designer metasurfaces." *Nature Materials*, 13, 139–150.
*The metasurface toolkit: subwavelength scatterers that impose arbitrary phase profiles on a beam. The enabling technology for visible-wavelength, nanometer-scale diffractive layers.*

---

## Applications

**Li, J., Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019).** "Class-specific differential detection in diffractive optical neural networks improves inference accuracy." *Advanced Photonics*, 1(4), 046001.
*Solves the non-negativity problem of Subsection 14.4.3 with paired positive/negative detector regions, recovering signed class scores and measurably higher accuracy.*

**Qian, C., Lin, X., Lin, X., Xu, J., Sun, Y., Li, E., Zhang, B., & Chen, H. (2020).** "Performing optical logic operations by a diffractive neural network." *Light: Science & Applications*, 9, 59.
*Diffractive networks configured to compute Boolean logic on optical inputs, extending the paradigm from classification toward general optical computing.*

**Luo, Y., Mengu, D., Yardimci, N.T., Rivenson, Y., Veli, M., Jarrahi, M., & Ozcan, A. (2019).** "Design of task-specific optical systems using broadband diffractive neural networks." *Light: Science & Applications*, 8, 112.
*Extends the single-wavelength model to broadband operation, letting one diffractive system route and process many wavelengths simultaneously.*

**Veli, M., Mengu, D., Yardimci, N.T., Luo, Y., Li, J., Rivenson, Y., Jarrahi, M., & Ozcan, A. (2021).** "Terahertz pulse shaping using diffractive surfaces." *Nature Communications*, 12, 37.
*Applies diffractive design to synthesize target terahertz temporal waveforms — a striking demonstration that the same physics shapes pulses, not just images.*

---

## Background — Diffraction and Optical AI

**Goodman, J.W. (2017).** *Introduction to Fourier Optics* (4th ed.). W.H. Freeman.
*The standard text for the scalar diffraction theory — Huygens–Fresnel, Rayleigh–Sommerfeld, the Fresnel and Fraunhofer regimes — that underlies the entire D2NN forward model. Essential background for Section 14.1 and Exercise 14.1.*

**Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020).** "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47.
*The field-defining perspective that situates diffractive networks within the broader program of optics-for-AI. The best single orientation read for where this chapter sits in the landscape.*
