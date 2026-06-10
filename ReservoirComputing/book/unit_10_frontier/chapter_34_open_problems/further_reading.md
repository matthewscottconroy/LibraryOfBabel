# Chapter 34: Further Reading

## Approximation Rate Theory

**Gonon, L., Grigoryeva, L., and Ortega, J.-P. (2020).** "Risk Bounds for Reservoir Computing." *Journal of Machine Learning Research*, 21(240), 1–61. The most rigorous treatment of RC approximation and generalization bounds. Directly addresses Problem 34.1.1, though does not fully resolve it.

**Gonon, L. and Ortega, J.-P. (2021).** "Reservoir Computing Universality With Stochastic Inputs." *IEEE Transactions on Neural Networks and Learning Systems*, 32(1), 100–112. Extends Boyd-Chua universality to stochastic inputs using ergodic theory. Proves approximation rates under specific conditions on input statistics.

**Pinkus, A. (1985).** *$n$-Widths in Approximation Theory*. Springer. The standard reference on Kolmogorov $n$-widths. Essential background for the optimal reservoir design problem (Problem 34.1.2). Chapters 1-3 give the theory; Chapter 6 covers widths of Sobolev spaces.

**DeVore, R.A. (1998).** "Nonlinear Approximation." *Acta Numerica*, 7, 51–150. The comprehensive survey of nonlinear approximation theory, including $n$-term approximation (analogous to using an $N$-unit reservoir). Reviews rates for wavelet and neural network approximation, with implications for RC.

**Barron, A.R. (1993).** "Universal Approximation Bounds for Superpositions of a Sigmoidal Function." *IEEE Transactions on Information Theory*, 39(3), 930–945. The model for how to prove tight approximation rate bounds for neural networks. The techniques here should be adapted for reservoir computing.

## Online Learning and FORCE

**Sussillo, D. and Abbott, L.F. (2009).** "Generating Coherent Patterns of Activity from Chaotic Neural Networks." *Neuron*, 63(4), 544–557. The original FORCE paper. Essential reading for Problem 34.1.5.

**Cesa-Bianchi, N. and Lugosi, G. (2006).** *Prediction, Learning, and Games*. Cambridge University Press. The comprehensive reference on online learning theory, including regret bounds for online convex optimization. Chapters 1-3 provide the framework needed for online reservoir readout learning.

**Orabona, F. (2019).** "A Modern Introduction to Online Learning." *arXiv:1912.13213*. A self-contained modern treatment of online learning theory. More accessible than Cesa-Bianchi and Lugosi for readers new to the area.

**Ziemann, I. and Sandberg, H. (2022).** "Regret Bounds for Adaptive Nonlinear Control." *SIAM Journal on Control and Optimization*, 60(3), 1343–1370. Convergence theory for online system identification in the presence of temporal correlations. The most directly relevant recent work on Problem 34.1.3.

## Optimal Reservoir Design

**Gu, A., Johnson, I., Timalsina, A., et al. (2023).** "How to Train Your HiPPO: State Spaces, Recurrent Memory, and Generalization." *ICLR 2023*. Extends the HiPPO analysis and provides guidance for training SSMs from different initializations. The most recent and practical contribution to the optimal reservoir design question.

**Tishby, N. and Zaslavsky, N. (2015).** "Deep Learning and the Information Bottleneck Principle." *IEEE Information Theory Workshop*. The information bottleneck perspective on representation learning. Provides a framework for asking: what information about the input history should the reservoir state retain? Relevant to Problem 34.1.2.

## ESP-Task Performance Gap

**Bertschinger, N. and Maass, T. (2004).** "Real-Time Computation at the Edge of Chaos in Recurrent Neural Networks." *Neural Computation*, 16(7), 1413–1436. Seminal paper on the edge of chaos. Defines the problem rigorously and provides theoretical and empirical evidence for the peak-at-edge-of-chaos hypothesis.

**Toyoizumi, T. and Abbott, L.F. (2011).** "Beyond the Edge of Chaos: Amplification and Temporal Integration by Recurrent Networks in the Chaotic Regime." *Physical Review E*, 84(5), 051908. Challenges the simple edge-of-chaos story and shows good performance can occur in the mildly chaotic regime.

**Legenstein, R. and Maass, W. (2007).** "Edge of Chaos and Prediction of Computational Performance for Neural Circuit Models." *Neural Networks*, 20(3), 323–334. Empirical and theoretical study of the relationship between ESP, edge of chaos, and computational performance. Directly relevant to Problem 34.1.4.

## General Reservoir Computing Theory Surveys

**Lukosevicius, M. and Jaeger, H. (2009).** "Reservoir Computing Approaches to Recurrent Neural Network Training." *Computer Science Review*, 3(3), 127–149. The most widely cited survey of reservoir computing theory and practice. Still relevant for understanding the state of the field before the recent theoretical advances.

**Nakajima, K. and Fischer, I. (Eds., 2021).** *Reservoir Computing: Theory, Physical Implementations, and Applications*. Springer. A comprehensive edited volume covering theory, physical implementations (including quantum, biological, and photonic), and applications. Chapter 2 (theoretical foundations) and Chapter 8 (open problems) are most relevant to Chapter 34.
