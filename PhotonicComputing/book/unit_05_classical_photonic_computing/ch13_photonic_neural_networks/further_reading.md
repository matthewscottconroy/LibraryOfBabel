# Chapter 13: Further Reading and References

## Foundational Deep Learning

**LeCun, Y., Bengio, Y., & Hinton, G. (2015).** "Deep learning." *Nature*, 521, 436–444.
*The canonical short survey of feedforward networks, backpropagation, and representation learning. The vocabulary of this chapter, in fifteen pages.*

**Goodfellow, I., Bengio, Y., & Courville, A. (2016).** *Deep Learning*. MIT Press.
*The standard textbook. Chapters 6–8 (feedforward networks, regularization, optimization) are the background for Sections 13.1 and 13.3; freely available online.*

**Vaswani, A., Shazeer, N., Parmar, N., Uszkoreit, J., Jones, L., Gomez, A.N., Kaiser, Ł., & Polosukhin, I. (2017).** "Attention is all you need." *Advances in Neural Information Processing Systems (NeurIPS)* 30.
*The transformer paper. Defines scaled dot-product and multi-head attention — the workload dissected in Section 13.5.*

---

## Photonic Neural Network Architectures and Surveys

**Shen, Y., Harris, N.C., Skirlo, S., Prabhu, M., Baehr-Jones, T., Hochberg, M., Sun, X., Zhao, S., Larochelle, H., Englund, D., & Soljačić, M. (2017).** "Deep learning with coherent nanophotonic circuits." *Nature Photonics*, 11, 441–446.
*The field-launching coherent ONN; the mesh-plus-nonlinearity template assumed throughout the chapter and mapped onto attention projections in Section 13.5.*

**Shastri, B.J., Tait, A.N., Ferreira de Lima, T., Pernice, W.H.P., Bhaskaran, H., Wright, C.D., & Prucnal, P.R. (2021).** "Photonics for artificial intelligence and neuromorphic computing." *Nature Photonics*, 15, 102–114.
*The standard survey of the whole landscape — the best single orientation read before the primary sources.*

**Wetzstein, G., Ozcan, A., Gigan, S., Fan, S., Englund, D., Soljačić, M., Denz, C., Miller, D.A.B., & Psaltis, D. (2020).** "Inference in artificial intelligence with deep optics and photonics." *Nature*, 588, 39–47.
*A cross-community manifesto joining integrated, free-space, and computational-imaging optics for AI; frames the inference-not-training thesis of Section 13.1.*

**Bandyopadhyay, S., Sludds, A., Krishnaswamy, S., et al. (2024).** "Single-chip photonic deep neural network with forward-only training." *Nature Photonics*, 18.
*A fully integrated photonic network trained on-chip with a forward-only scheme — the state of the art for in-situ training discussed in Section 13.3.*

---

## Optical Nonlinearity and Activation

**Miller, D.A.B. (2010).** "Are optical transistors the logical next step?" *Nature Photonics*, 4, 3–5.
*The short, decisive argument on why optical devices struggle to provide gain, fan-out, and cascadability — the requirements list behind the activation problem of Section 13.2.*

**Nozaki, K., Tanabe, T., Shinya, A., Matsuo, S., Sato, T., Taniyama, H., & Notomi, M. (2010).** "Sub-femtojoule all-optical switching using a photonic-crystal nanocavity." *Nature Photonics*, 4, 477–483.
*The sub-fJ cavity switch (~0.42 fJ) that anchors the all-optical-activation energy discussion; low switching energy, but cascadability and threshold-power caveats remain.*

**Zuo, Y., Li, B., Zhao, Y., Jiang, Y., Chen, Y.-C., Chen, P., Jo, G.-B., Liu, J., & Du, S. (2019).** "All-optical neural network with nonlinear activation functions." *Optica*, 6(9), 1132–1137.
*An all-optical network using electromagnetically-induced-transparency nonlinearity in cold atoms — a physics demonstration of activation without electronics.*

**Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H., & Pernice, W.H.P. (2019).** "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569, 208–214.
*Phase-change-material neurons and synapses realizing on-chip spiking nonlinearity and plasticity.*

---

## Training Photonic Networks

**Hughes, T.W., Minkov, M., Shi, Y., & Fan, S. (2018).** "Training of photonic neural networks through in situ backpropagation and gradient measurement." *Optica*, 5(7), 864–871.
*The adjoint-method result: physical backward propagation plus local intensity measurement computes the exact gradient. The theoretical centerpiece of Section 13.3.2.*

**Pai, S., Sun, Z., Hughes, T.W., Park, T., Bartlett, B., Williamson, I.A.D., Minkov, M., Milanizadeh, M., Abebe, N., Morichetti, F., Melloni, A., Fan, S., Solgaard, O., & Miller, D.A.B. (2023).** "Experimentally realized in situ backpropagation for deep learning in photonic neural networks." *Science*, 380(6643), 398–404.
*The experimental realization on silicon meshes with integrated gradient readout — backpropagation running in the optics.*

**Wright, L.G., Onodera, T., Stein, M.M., Wang, T., Schachter, D.T., Hu, Z., & McMahon, P.L. (2022).** "Deep physical neural networks trained with backpropagation." *Nature*, 601, 549–555.
*Physics-aware training: a hardware forward pass with a differentiable digital model for the backward pass, closing the sim-to-real gap across diverse physical systems.*

**Spall, J., Guo, X., & Lvovsky, A.I. (2022).** "Hybrid training of optical neural networks." *Optica*, 9(7), 803–811.
*An optical forward pass combined with a digital backward pass — the practical bridge between offline and fully in-situ training.*

---

## Reservoir Computing

**Appeltant, L., Soriano, M.C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., Schrauwen, B., Mirasso, C.R., & Fischer, I. (2011).** "Information processing using a single dynamical node as complex system." *Nature Communications*, 2, 468.
*The time-multiplexing insight: one nonlinear node plus delayed feedback emulates a network of virtual nodes. The basis of Section 13.4.2.*

**Brunner, D., Soriano, M.C., Mirasso, C.R., & Fischer, I. (2013).** "Parallel photonic information processing at gigabyte per second data rates using transient states." *Nature Communications*, 4, 1364.
*Gb/s photonic reservoir processing with a semiconductor-laser node — the demonstration of raw speed.*

**Larger, L., Baylón-Fuentes, A., Martinenghi, R., Udaltsov, V.S., Chembo, Y.K., & Jacquot, M. (2017).** "High-speed photonic reservoir computing using a time-delay-based architecture: Million words per second classification." *Physical Review X*, 7, 011015.
*Delay-based optoelectronic reservoir classifying spoken digits at ~1 million words/s — the headline throughput result.*

**Vandoorne, K., Mechet, P., Van Vaerenbergh, T., Fiers, M., Morthier, G., Verstraeten, D., Schrauwen, B., Dambre, J., & Bienstman, P. (2014).** "Experimental demonstration of reservoir computing on a silicon photonics chip." *Nature Communications*, 5, 3541.
*A passive integrated silicon photonic reservoir — a spatial network of waveguides rather than a delay line. The integrated counterpoint of Section 13.4.3.*

**Van der Sande, G., Brunner, D., & Soriano, M.C. (2017).** "Advances in photonic reservoir computing." *Nanophotonics*, 6(3), 561–576.
*The review that ties the delay-based and integrated schools together; the recommended survey for Section 13.4.*

---

## Optical Transformers and Energy Analysis

**Anderson, M.G., Ma, S.-Y., Wang, T., Wright, L.G., & McMahon, P.L. (2024).** "Optical transformers." *Transactions on Machine Learning Research (TMLR)*. arXiv:2302.10360.
*The energy-scaling analysis of transformer inference on optical hardware: the optical MVM advantage grows with model dimension. The primary source for Section 13.5.2.*

**Hamerly, R., Bernstein, L., Sludds, A., Soljačić, M., & Englund, D. (2019).** "Large-scale optical neural networks based on photoelectric multiplication." *Physical Review X*, 9, 021032.
*Coherent stream-versus-stream multiplication scaling to $N \sim 10^6$ at sub-attojoule optical energy per MAC — the natural accelerator for attention's activation-by-activation products.*

**Nahmias, M.A., Ferreira de Lima, T., Tait, A.N., Peng, H.-T., Shastri, B.J., & Prucnal, P.R. (2020).** "Photonic multiply-accumulate operations for neural networks." *IEEE Journal of Selected Topics in Quantum Electronics*, 26(1), 7701518.
*The careful MAC-energy accounting including all conversion overheads — the antidote to headline TOPS figures and the source of the fJ/MAC system numbers.*

**McMahon, P.L. (2023).** "The physics of optical computing." *Nature Reviews Physics*, 5, 717–734.
*A rigorous, skeptical assessment of where optical computing's advantages are physically real; the recommended capstone reading for the chapter.*
