# Key Researchers: Chapter 33 — Reservoir Computing for Scientific Discovery

## PDE Surrogate Modeling

**Jaideep Pathak**: Physicist and machine learning researcher; with Hunt, Girvan, Lu, and Ott at the University of Maryland, introduced the parallel reservoir architecture for spatiotemporally chaotic systems [Pathak et al. 2018]. Demonstrated that reservoir computing could predict Kuramoto-Sivashinsky dynamics for multiple Lyapunov times, establishing reservoir surrogates as a serious tool for scientific computing. Subsequently moved to NVIDIA to work on AI-based weather prediction.

**Edward Ott** (1941–): Distinguished University Professor at the University of Maryland; pioneer of chaos theory (Ott-Grebogi-Yorke control method) and nonlinear dynamics. Together with Girvan, Hunt, Pathak, and Lu, produced a series of influential papers applying reservoir computing to chaotic system prediction and surrogate modeling [Ott et al. 2018 series].

**Michelle Girvan**: Physicist at the University of Maryland; works on complex networks, chaos, and reservoir computing. Contributed to the parallel reservoir architecture for spatiotemporal chaos and to reservoir-based observer problems.

**Zhixin Lu**: Contributed to reservoir-based observer problems and the Lyapunov regularization approach for improving the long-term stability of reservoir surrogates [Lu et al. 2018].

## Data Assimilation

**Julien Brajard**: Researcher at the Nansen Environmental and Remote Sensing Center (NERSC), Bergen, Norway. With Carrassi, Bocquet, and Bertino, developed the reservoir computing + nudging approach for data assimilation [Brajard et al. 2020]. Work represents a significant advance in model-free data assimilation.

**Marc Bocquet**: Researcher at École des Ponts ParisTech; specialist in data assimilation theory and inverse problems. Co-developed the reservoir-based DA framework and the machine learning approach to inferring dynamical models from sparse observations [Bocquet et al. 2019, 2020].

**Alberto Carrassi**: Professor at the University of Reading; data assimilation specialist. With Bocquet and Brajard, developed the theoretical framework for reservoir-based DA and analyzed its relationship to classical filter theory.

**Geir Evensen** (1959–): Norwegian researcher; inventor of the ensemble Kalman filter [Evensen 1994]. The EnKF revolutionized operational data assimilation and is the baseline against which reservoir-based DA is compared. Author of the definitive monograph on the EnKF [Evensen 2009].

**Eugenia Kalnay** (1942–): Distinguished University Professor at the University of Maryland; developed key methods for atmospheric data assimilation and authored the standard textbook [Kalnay 2003]. Not directly involved in reservoir DA, but her frameworks are the target that reservoir methods aim to replace or augment.

## Conservation Laws and Physics-Constrained ML

**George Karniadakis** (1960–): Professor at Brown University; pioneer of physics-informed neural networks (PINNs) and scientific machine learning. Works on integrating physical constraints into neural network training. The PINN framework [Raissi et al. 2019] for enforcing PDEs as soft constraints is directly relevant to conservation-law-preserving reservoirs.

**Miles Cranmer**: Astrophysicist and machine learning researcher at Cambridge; developed Lagrangian neural networks [Cranmer et al. 2020] and symbolic regression tools for scientific discovery. His approach of learning the Lagrangian rather than the equations of motion represents a physics-first perspective on surrogate modeling.

**Samuel Greydanus**: AI researcher; with Dzamba and Yosinski, developed Hamiltonian neural networks [Greydanus et al. 2019], the feedforward precursor to the symplectic reservoir idea.

**Pengzhan Jin**: With Karniadakis, Zhang, Zhu, and Tang, developed SympNets [Jin et al. 2020] for exactly symplectic neural networks. These represent the state of the art in architecture-level conservation law enforcement.

## High-Energy Physics

**Yann Coadou**: Physicist at Aix-Marseille University; with collaborators, proposed photonic reservoir computing for LHC trigger applications [Coadou et al. 2022]. Work represents the most concrete proposal for physical RC in particle physics.

**Javier Duarte**: Particle physicist at UC San Diego; co-developed hls4ml [Duarte et al. 2018], the standard framework for deploying machine learning on FPGAs for LHC trigger applications. hls4ml is the computational benchmark against which physical RC proposals are compared.

**Ekaterina Govorkova**: Particle physicist; contributed to autoencoder-based anomaly detection for LHC triggers [Govorkova et al. 2022], providing the unsupervised learning baseline for RC-based anomaly detection proposals.

## Equation of State

**Tobias Désert**: Physicist; with Clérouin, Recoules, and Becker, applied reservoir computing to dense plasma EOS inference [Désert et al. 2022]. This paper represents the first systematic application of RC to equation-of-state modeling in the warm dense matter regime.

**Attila Becker**: Physicist at the University of Rostock; with collaborators, developed machine learning methods for dense plasma EOS calculation [Becker et al. 2020], providing the GP-based baseline against which reservoir approaches are compared.

## References

- Brajard, J., Carrassi, A., Bocquet, M., and Bertino, L. (2020). Combining data assimilation and machine learning. *Journal of Computational Science*, 44, 101171.
- Coadou, Y. et al. (2022). Reservoir computing for fast jet classification at the LHC. *JINST*, 17, P08022.
- Cranmer, M. et al. (2020). Lagrangian neural networks. *ICLR Workshop*.
- Désert, T. et al. (2022). EOS of hot dense matter with reservoir computing. *Phys. Rev. E*, 105, 025210.
- Duarte, J. et al. (2018). Fast inference of deep neural networks in FPGAs. *JINST*, 13, P07027.
- Evensen, G. (1994). Sequential data assimilation. *J. Geophys. Res.*, 99(C5), 10143.
- Govorkova, E. et al. (2022). Autoencoders on FPGAs for anomaly detection at 40 MHz. *Nature Machine Intelligence*, 4, 154–161.
- Greydanus, S., Dzamba, M., and Yosinski, J. (2019). Hamiltonian neural networks. *NeurIPS*, 32.
- Jin, P. et al. (2020). SympNets. *Neural Networks*, 132, 166–179.
- Kalnay, E. (2003). *Atmospheric Modeling, Data Assimilation and Predictability*. Cambridge.
- Pathak, J. et al. (2018). Model-free prediction of spatiotemporally chaotic systems. *PRL*, 120, 024102.
- Raissi, M., Perdikaris, P., and Karniadakis, G. E. (2019). Physics-informed neural networks. *J. Comput. Phys.*, 378, 686–707.
