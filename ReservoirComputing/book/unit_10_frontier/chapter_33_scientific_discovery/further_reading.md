# Further Reading: Chapter 33 — Reservoir Computing for Scientific Discovery

## Scientific Machine Learning: General Texts and Reviews

**Brunton, S. L. and Kutz, J. N. (2022). *Data-Driven Science and Engineering: Machine Learning, Dynamical Systems, and Control*. 2nd ed. Cambridge University Press.**
The most accessible and comprehensive treatment of machine learning methods for dynamical systems, including sparse regression, Koopman operators, neural ODEs, and reservoir computing. Chapters 6–8 on reduced-order modeling and Chapter 12 on deep learning for dynamics are directly relevant. Highly recommended as a companion to this chapter.

**Karniadakis, G. E., Kevrekidis, I. G., Lu, L., Perdikaris, P., Wang, S., and Yang, L. (2021). Physics-informed machine learning. *Nature Reviews Physics*, 3(6), 422–440.**
Review of physics-informed neural networks and related approaches for integrating physical knowledge into machine learning. Covers soft constraints, hard constraints, and architecture-level physics enforcement. Essential background for Section 33.4.

**Willard, J., Jia, X., Xu, S., Steinbach, M., and Kumar, V. (2022). Integrating scientific knowledge with machine learning for engineering and environmental systems. *ACM Computing Surveys*, 55(4), 1–37.**
Comprehensive survey of physics-constrained machine learning across domains. Taxonomy of constraint types (physical laws, symmetries, conservation laws) and enforcement strategies (penalties, projections, architectures) is useful for organizing the material in this chapter.

## PDE Surrogate Modeling

**Pathak, J., Hunt, B., Girvan, M., Lu, Z., and Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120, 024102.**
The foundational paper for reservoir surrogates of spatiotemporally chaotic PDEs. Demonstrates parallel reservoir architecture on the Kuramoto-Sivashinsky equation. Accessible to physicists; good entry point.

**Vlachas, P. R., Byeon, W., Wan, Z. Y., Sapsis, T. P., and Koumoutsakos, P. (2020). Backpropagation algorithms and reservoir computing in recurrent neural networks for the forecasting of complex spatiotemporal dynamics. *Neural Networks*, 126, 191–217.**
Systematic comparison of LSTM, GRU, and reservoir computing for spatiotemporal PDE prediction. Finds that reservoir computing is competitive with trained RNNs at lower computational cost. Important reference for performance comparisons.

**Raissi, M., Perdikaris, P., and Karniadakis, G. E. (2019). Physics-informed neural networks: A deep learning framework for solving forward and inverse problems involving nonlinear PDEs. *Journal of Computational Physics*, 378, 686–707.**
The original PINN paper. Uses differential equation residuals as soft constraints in the training loss. The methodology for enforcing physical constraints developed here applies directly to reservoir surrogates (Section 33.4).

**Sanchez-Gonzalez, A., Godwin, J., Pfaff, T., Ying, R., Leskovec, J., and Battaglia, P. (2020). Learning to simulate complex physics with graph networks. *ICML*, 8459–8468.**
Graph neural network approach to PDE surrogate modeling on irregular meshes. Conceptually related to the parallel reservoir approach but using learned message-passing rather than fixed random connections.

## Data Assimilation

**Kalnay, E. (2003). *Atmospheric Modeling, Data Assimilation and Predictability*. Cambridge University Press.**
The standard textbook for atmospheric data assimilation. Chapters 5–6 on Kalman filtering and ensemble methods are required background for reservoir-based DA. Clear exposition of the EnKF suitable for non-specialists.

**Evensen, G. (2009). *Data Assimilation: The Ensemble Kalman Filter*. 2nd ed. Springer.**
The definitive reference on ensemble Kalman filtering by its inventor. Covers all variants of the EnKF, its relationship to optimal estimation, and applications in geophysics. Chapter 14 on nonlinear extensions is the starting point for understanding where the EnKF fails and reservoir DA might succeed.

**Brajard, J., Carrassi, A., Bocquet, M., and Bertino, L. (2020). Combining data assimilation and machine learning to emulate a dynamical model from sparse and noisy observations. *Journal of Computational Science*, 44, 101171.**
The key paper for reservoir-based DA. Demonstrates the nudging approach on Lorenz-63 and discusses the connection to classical DA theory. Accessible and well-written.

**Bocquet, M., Brajard, J., Carrassi, A., and Bertino, L. (2019). Data assimilation as a learning tool to infer ordinary differential equation representations of dynamical models. *Nonlinear Processes in Geophysics*, 26(3), 143–162.**
Extends the reservoir DA framework to learning the equations of motion from observations. Connects DA with system identification and machine learning in a unified framework.

## Conservation Laws and Hamiltonian/Lagrangian Networks

**Greydanus, S., Dzamba, M., and Yosinski, J. (2019). Hamiltonian neural networks. *NeurIPS*, 32.**
Introduced architecture-level energy conservation for neural ODEs. Demonstrates that enforcing Hamiltonian structure improves long-term stability dramatically. Clear exposition; good starting point.

**Jin, P., Zhang, Z., Zhu, A., Tang, Y., and Karniadakis, G. E. (2020). SympNets: Intrinsic structure-preserving symplectic networks for identifying Hamiltonian systems. *Neural Networks*, 132, 166–179.**
The definitive reference for symplectic neural networks. Proves that SympNets are universal approximators for symplectic maps. The mathematical development of the shear map composition is thorough.

**Cranmer, M., Greydanus, S., Hoyer, S., Battaglia, P., Spergel, D., and Ho, S. (2020). Lagrangian neural networks. *ICLR Workshop on Integration of Deep Neural Models and Differential Equations*.**
Learns the Lagrangian $\mathcal{L}(q, \dot{q})$ rather than the equations of motion directly. Conservation laws are automatically satisfied via the Euler-Lagrange equations. Complements the Hamiltonian approach.

## High-Energy Physics

**Coadou, Y., Fontaine, G., Lugard, A., Miagkikh, V., Nass, K., and Womersley, R. (2022). Reservoir computing for fast jet classification at the LHC. *Journal of Instrumentation*, 17, P08022.**
The key paper for reservoir computing in particle physics. Proposes photonic RC for LHC trigger, with simulation results demonstrating competitive performance vs. deep networks at much lower latency. Accessible to physicists; good entry point.

**Duarte, J., et al. (2018). Fast inference of deep neural networks in FPGAs for particle physics. *Journal of Instrumentation*, 13, P07027.**
Introduces hls4ml, the FPGA-based machine learning framework for HEP triggers. The performance benchmark against which physical RC proposals are compared. Required reading for understanding the current state of the art in HEP trigger ML.

**Salam, G. P. (2010). Towards jetography. *European Physical Journal C*, 67(3), 637–686.**
Comprehensive review of jet physics and jet algorithms. Essential background for understanding the jet classification problem addressed in Section 33.5.

**Guest, D., Cranmer, K., and Whiteson, D. (2018). Deep learning and its application to LHC physics. *Annual Review of Nuclear and Particle Science*, 68, 161–181.**
Broad review of deep learning in particle physics, covering jet classification, anomaly detection, and event generation. Provides context for the reservoir computing proposals in Section 33.5.

## Equation of State

**Désert, T., Clérouin, J., Recoules, V., and Becker, A. (2022). Equation of state of hot dense matter with reservoir computing. *Physical Review E*, 105, 025210.**
The key application paper for reservoir EOS inference. Demonstrates nonequilibrium EOS learning for warm dense matter.

**Rasmussen, C. E. and Williams, C. K. I. (2006). *Gaussian Processes for Machine Learning*. MIT Press.**
Standard reference for GP regression. Chapter 5 (model selection and adaptation) and Chapter 9 (GP classification) are relevant for understanding the Bayesian alternative to reservoir EOS methods.
