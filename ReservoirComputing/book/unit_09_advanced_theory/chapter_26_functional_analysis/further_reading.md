# Chapter 26: Further Reading

## Foundational Texts

**Boyd, S. and Chua, L.O. (1985).** "Fading Memory and the Problem of Approximating Nonlinear Operators with Volterra Series." *IEEE Transactions on Circuits and Systems*, 32(11), 1150–1161. The original paper proving what we call the Boyd-Chua theorem. Read the original: the setup is circuit-theoretic but the mathematics is clean and the proof of fading memory implying Volterra series approximability is the template for all subsequent work.

**Stone, M.H. (1937).** "Applications of the Theory of Boolean Rings to General Topology." *Transactions of the American Mathematical Society*, 41(3), 375–481. The original Stone-Weierstrass theorem. Not easy reading by modern standards, but historically important. A more accessible modern treatment is in Rudin's *Real and Complex Analysis* [Rudin1987], Chapter 5.

**Rudin, W. (1976).** *Principles of Mathematical Analysis*, 3rd edition. McGraw-Hill. The standard reference for undergraduate real analysis, including the Weierstrass theorem and its Bernstein polynomial proof. Chapter 7 covers approximation theory.

**Rudin, W. (1987).** *Real and Complex Analysis*, 3rd edition. McGraw-Hill. Graduate-level treatment. Chapter 5 gives a clean proof of Stone-Weierstrass. Chapter 11 gives the Riesz representation theorem needed for the Cybenko-style proof of neural network universality.

## Neural Network Universal Approximation

**Cybenko, G. (1989).** "Approximations by Superpositions of a Sigmoidal Function." *Mathematics of Control, Signals, and Systems*, 2(4), 303–314. The classic paper. The proof via Hahn-Banach separation and Fourier analysis is still the most elegant approach.

**Hornik, K. (1991).** "Approximation Capabilities of Multilayer Feedforward Networks." *Neural Networks*, 4(2), 251–257. Shows that the key property is the multilayer structure (the universal approximation capacity of any architecture with enough neurons and a non-constant activation).

**Leshno, M., Lin, V.Y., Pinkus, A., and Schocken, S. (1993).** "Multilayer Feedforward Networks with a Nonpolynomial Activation Function Can Approximate Any Function." *Neural Networks*, 6(6), 861–867. The definitive characterization: non-polynomial activation is necessary and sufficient.

**Barron, A.R. (1993).** "Universal Approximation Bounds for Superpositions of a Sigmoidal Function." *IEEE Transactions on Information Theory*, 39(3), 930–945. Proves the first explicit approximation rate bounds for single-hidden-layer networks: $O(1/\sqrt{N})$ error in $L^2$ norm for functions with bounded Fourier moment. A model of how to do approximation rate theory correctly.

## Approximation Theory

**DeVore, R.A. and Lorentz, G.G. (1993).** *Constructive Approximation*. Springer. The comprehensive reference on approximation theory, including rates for polynomials, splines, and other approximation schemes. Chapters 6 and 7 cover the Whitney theorem and approximation rates in Sobolev spaces.

**Pinkus, A. (1999).** "Approximation Theory of the MLP Model in Neural Networks." *Acta Numerica*, 8, 143–195. A thorough survey of neural network approximation theory up to 1999, covering both existence and rate results. Still highly relevant.

## Reservoir Computing Theory

**Gonon, L. and Ortega, J.-P. (2021).** "Reservoir Computing Universality With Stochastic Inputs." *IEEE Transactions on Neural Networks and Learning Systems*, 32(1), 100–112. Proves universality for random reservoirs driven by stochastic inputs, handling the case where inputs are random processes rather than deterministic sequences.

**Gonon, L., Grigoryeva, L., and Ortega, J.-P. (2020).** "Risk Bounds for Reservoir Computing." *Journal of Machine Learning Research*, 21, 1–61. The first rigorous sample complexity bounds for reservoir computing. Derives both approximation error and estimation error bounds, establishing the total generalization error for ridge regression readouts.

**Grigoryeva, L. and Ortega, J.-P. (2018).** "Echo State Networks Are Universal." *Neural Networks*, 108, 495–508. Proves that echo state networks (under appropriate conditions) satisfy the Boyd-Chua universality conditions. Relates the ESN architecture to the abstract fading-memory framework.

**Matthews, M.B. (1993).** "On the Uniform Approximation of Nonlinear Discrete-Time Fading-Memory Systems Using Neural Network Models." *IEEE Transactions on Circuits and Systems II*, 40(8), 490–495. Extends the Boyd-Chua framework to continuous-time and multivariable inputs, and shows that neural network reservoirs specifically satisfy the universality conditions.

## Functional Analysis Background

**Kreyszig, E. (1978).** *Introductory Functional Analysis with Applications*. Wiley. An accessible first graduate course in functional analysis. Chapters 2, 4, and 7 cover the material on Banach spaces, continuous linear functionals, and the Hahn-Banach theorem needed for this chapter.

**Conway, J.B. (1990).** *A Course in Functional Analysis*, 2nd edition. Springer. A more advanced treatment. Chapter III covers the topics on compact operators relevant to Section 26.3.

## Random Features and Kernel Methods

**Rahimi, A. and Recht, B. (2007).** "Random Features for Large-Scale Kernel Machines." *Advances in Neural Information Processing Systems*, 20. The paper that connected random features to kernel approximation. Directly relevant to the random feature perspective on reservoir computing in Section 26.5.

**Cho, Y. and Saul, L.K. (2009).** "Kernel Methods for Deep Learning." *Advances in Neural Information Processing Systems*, 22. Introduces the arc-cosine kernel, which is the kernel induced by a single layer of neural network units with ReLU activations. The infinite-width reservoir kernel is a close relative of this construction.
