# Appendix F: Key Researchers in Reservoir Computing

This appendix provides profiles of the major contributors to reservoir computing theory and applications. For each researcher, we note their institutional affiliation, their primary contribution to the field, their three most important papers, and their current research direction as of 2024.

---

## Herbert Jaeger

**Affiliation**: Constructor University (formerly Jacobs University), Bremen, Germany

**Role in RC Development**: Herbert Jaeger independently invented echo state networks in 2001, coining the term and developing the foundational theory. His technical report introducing ESNs [Jaeger2001] and the subsequent report on memory capacity [Jaeger2002MC] define the essential computational framework that all subsequent work builds on. Jaeger's 2004 *Science* paper with Haas [JaegerHaas2004] demonstrated ESN prediction of chaotic time series at unprecedented accuracy, bringing RC to wide attention. He also developed the concept of conceptors — algebraic structures for managing multiple dynamical patterns in a single reservoir — which represents one of the deepest extensions of the basic framework.

**Three most important papers**:
1. Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Technical Report 148*.
2. Jaeger, H. (2002). Short term memory in echo state networks. *GMD Technical Report 152*.
3. Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.

**Current research direction (2024)**: Jaeger has focused on the theoretical foundations of RC and the "conceptor" framework — a method for superimposing multiple learned patterns in a single reservoir using matrix-valued filters. He has also worked on neuro-symbolic integration and the long-term theoretical questions of what recurrent networks can and cannot compute.

---

## Wolfgang Maass

**Affiliation**: Institute of Theoretical Computer Science, Graz University of Technology, Austria

**Role in RC Development**: Wolfgang Maass independently developed the liquid state machine (LSM) framework [MaassEtAl2002] simultaneously with Jaeger's ESN work. The LSM focuses on biologically realistic spiking neurons and provides the computational neuroscience perspective on reservoir computing. Maass's key theoretical contribution is the proof that a generic LSM can approximate any filter with fading memory — the universality result that provides theoretical foundations for the entire field. He also established the connection between RC and the "separation property" and "approximation property" that characterize computationally useful liquid states.

**Three most important papers**:
1. Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.
2. Maass, W., Natschläger, T., & Markram, H. (2004). Computational models for generic cortical microcircuits. In *Computational Neuroscience: A Comprehensive Approach*, pp. 575–605.
3. Legenstein, R., & Maass, W. (2007). Edge of chaos and prediction of computational performance for neural circuit models. *Neural Networks*, 20(3), 323–334.

**Current research direction (2024)**: Maass has extended his research to the theory of learning in spiking neural networks, including reward-modulated STDP learning rules that can train reservoirs (not just readouts). His group also works on connections between RC and deep learning theory.

---

## Mantas Lukoševičius

**Affiliation**: Constructor University, Bremen, Germany

**Role in RC Development**: Mantas Lukoševičius is responsible for the most widely used practical guide to reservoir computing [Lukosevičius2012], which systematically analyzes the effects of all major ESN hyperparameters and provides hands-on recommendations that have guided thousands of practitioners. His co-authored survey with Jaeger [LukoseviciusJaeger2009] is the standard academic introduction to the field. Lukoševičius also contributed to the development of leaky-integrator ESNs and their theoretical analysis.

**Three most important papers**:
1. Lukoševičius, M. (2012). A practical guide to applying echo state networks. In *Neural Networks: Tricks of the Trade* (2nd ed.), LNCS 7700, pp. 659–686.
2. Lukoševičius, M., & Jaeger, H. (2009). Reservoir computing approaches to recurrent neural network training. *Computer Science Review*, 3(3), 127–149.
3. Jaeger, H., Lukoševičius, M., Popovici, D., & Siewert, U. (2007). Optimization and applications of echo state networks with leaky-integrator neurons. *Neural Networks*, 20(3), 335–352.

**Current research direction (2024)**: Lukoševičius continues work on practical aspects of RC including efficient hyperparameter selection, online learning variants, and applications to real-world time series problems.

---

## Benjamin Schrauwen

**Affiliation**: Ghent University, Belgium (later SoftBank Robotics)

**Role in RC Development**: Benjamin Schrauwen led the Ghent University reservoir computing group and was responsible for a large fraction of the applied RC work in the 2005–2015 period, including speech processing, spoken digit recognition, and the development of the ReservoirPy precursors. His group produced systematic benchmarks comparing ESN, LSM, and delay-based reservoir approaches. Schrauwen also worked on photonic and analog hardware reservoirs.

**Three most important papers**:
1. Verstraeten, D., Schrauwen, B., d'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.
2. Schrauwen, B., Wardermann, M., Verstraeten, D., Steil, J. J., & Stroobandt, D. (2008). Improving reservoirs using intrinsic plasticity. *Neurocomputing*, 71(7–9), 1159–1171.
3. Jaeger, H., Lukoševičius, M., Popovici, D., & Siewert, U. (2007). (see above)

**Current research direction (2024)**: Schrauwen moved to industry; his group's academic work was continued by his students (Dambre, Dambreetal, etc.).

---

## Claudio Gallicchio

**Affiliation**: University of Pisa, Italy

**Role in RC Development**: Claudio Gallicchio developed the Deep Echo State Network (DeepESN) architecture [GallicchioMicheli2017], extending reservoir computing to hierarchical multi-layer models. His work provides theoretical analysis of how depth affects computational capacity and temporal processing in reservoirs. Gallicchio's group has also contributed to structured (non-random) reservoir design and theoretical analysis of reservoir dynamics via random matrix theory.

**Three most important papers**:
1. Gallicchio, C., & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.
2. Gallicchio, C., Micheli, A., & Pedrelli, L. (2018). Design of deep echo state networks. *Neural Networks*, 108, 33–47.
3. Gallicchio, C., & Micheli, A. (2020). Ring reservoir neural networks for graphs. In *IJCNN 2020*.

**Current research direction (2024)**: Extending reservoir computing to graph-structured data (graph neural networks with reservoir readouts), theoretical analysis of deep reservoirs, and connections between RC and kernel methods.

---

## Alessio Micheli

**Affiliation**: University of Pisa, Italy

**Role in RC Development**: Alessio Micheli co-developed the DeepESN with Gallicchio and has made foundational contributions to structured RC architectures, including the Echo State Network for sequences (ESS) and reservoir computing for trees and graphs (EchoST, GraphESN). His group's work on structured domains extends RC beyond time series to general relational data.

**Three most important papers**:
1. Gallicchio, C., & Micheli, A. (2017). (see above)
2. Gallicchio, C., & Micheli, A. (2010). Graph echo state networks. In *IJCNN 2010*.
3. Micheli, A. (2009). Neural network for graphs: A contextual constructive approach. *IEEE Transactions on Neural Networks*, 20(3), 498–511.

**Current research direction (2024)**: RC for structured domains (graphs, trees), theoretical analysis of network architectures, and application to chemoinformatics and bioinformatics.

---

## Daniel Brunner

**Affiliation**: FEMTO-ST Institute, CNRS/Université de Franche-Comté, France

**Role in RC Development**: Daniel Brunner is a leading figure in photonic reservoir computing, particularly delay-based (time-multiplexed) photonic reservoirs. His experimental work with optoelectronic delay systems [AppeltantEtAl2011, LargerEtAl2012] demonstrated that a single nonlinear node with feedback delay could implement a high-performance reservoir, making hardware RC practically accessible. Brunner's subsequent work has advanced photonic RC toward chip-scale integration.

**Three most important papers**:
1. Appeltant, L., Soriano, M. C., Van der Sande, G., Danckaert, J., Massar, S., Dambre, J., Schrauwen, B., Mirasso, C. R., & Fischer, I. (2011). Information processing using a single dynamical node as complex system. *Nature Communications*, 2, 468.
2. Larger, L., Soriano, M. C., Brunner, D., Appeltant, L., Gutiérrez, J. M., Pesquera, L., Mirasso, C. R., & Fischer, I. (2012). Photonic information processing beyond Turing: An optoelectronic implementation of reservoir computing. *Optics Express*, 20(3), 3241–3249.
3. Brunner, D., Soriano, M. C., Mirasso, C. R., & Fischer, I. (2013). Parallel photonic information processing at gigabyte per second data rates using transient states. *Nature Communications*, 4, 1364.

**Current research direction (2024)**: Integrated photonic reservoir computing on chip, neuromorphic photonics, and hybrid electro-photonic computing systems.

---

## Guy Van der Sande

**Affiliation**: Vrije Universiteit Brussel, Belgium

**Role in RC Development**: Guy Van der Sande has contributed to photonic and optoelectronic reservoir computing, focusing on semiconductor laser-based implementations. He co-authored the Appeltant et al. Nature Communications paper [AppeltantEtAl2011] and has continued work on laser-based RC systems including VCSEL (Vertical Cavity Surface Emitting Laser) reservoirs.

**Three most important papers**:
1. Appeltant, L., et al. (2011). (see above)
2. Vatin, J., Rontani, D., & Sciamanna, M. (2019). Experimental reservoir computing using VCSEL polarization dynamics. *Optics Express*, 27(13), 18579–18584.
3. Van der Sande, G., Brunner, D., & Soriano, M. C. (2017). Advances in photonic reservoir computing. *Nanophotonics*, 6(3), 561–576.

**Current research direction (2024)**: All-optical reservoir computing, photonic neural networks for edge AI, and spiking photonic devices.

---

## Ingo Fischer

**Affiliation**: Instituto de Física Interdisciplinar y Sistemas Complejos (IFISC), UIB-CSIC, Spain

**Role in RC Development**: Ingo Fischer led the experimental side of the photonic RC program, co-authoring the foundational Nature Communications paper and developing the optoelectronic delay-based reservoir concept. His group at IFISC has systematically studied the dynamics of delay-coupled semiconductor lasers and their information-processing properties.

**Three most important papers**:
1. Appeltant, L., et al. (2011). (see above)
2. Larger, L., et al. (2012). (see above)
3. Brunner, D., et al. (2013). (see above)

**Current research direction (2024)**: Physical reservoir computing with delay systems, neuromorphic photonics, and the theory of computing with dynamical systems.

---

## Miguel C. Soriano

**Affiliation**: Instituto de Física Interdisciplinar y Sistemas Complejos (IFISC), UIB-CSIC, Spain

**Role in RC Development**: Miguel Soriano has contributed to both the theoretical and experimental sides of photonic RC, co-authoring numerous foundational papers. He has also worked on reservoir computing for time-series analysis, including ECG processing and financial time series, and on the theoretical analysis of reservoir dynamics using tools from dynamical systems theory.

**Three most important papers**:
1. Appeltant, L., et al. (2011). (see above)
2. Van der Sande, G., Brunner, D., & Soriano, M. C. (2017). (see above)
3. Soriano, M. C., Ortín, S., Brunner, D., Larger, L., Mirasso, C. R., Fischer, I., & Pesquera, L. (2013). Optoelectronic reservoir computing: Tackling noise-induced performance degradation. *Optics Express*, 21(1), 12–20.

**Current research direction (2024)**: Photonic neuromorphic computing, reservoir computing for signal processing in communication systems, and the physics of delay-coupled nonlinear oscillators.

---

## Kohei Nakajima

**Affiliation**: University of Tokyo, Japan

**Role in RC Development**: Kohei Nakajima is the central figure in physical reservoir computing in the mechanical/soft-body domain and has also co-developed quantum reservoir computing. His work on octopus-arm-inspired soft robots [NakajimaEtAl2013], granular media [NakajimaEtAl2015Granular], and the general theory of physical reservoirs has established the subfield. He edited (with Ingo Fischer) the comprehensive Springer volume on reservoir computing [NakajimaFischer2021].

**Three most important papers**:
1. Nakajima, K., Hauser, H., Li, T., & Pfeifer, R. (2015). Information processing via physical soft body. *Scientific Reports*, 5, 10487.
2. Fujii, K., & Nakajima, K. (2017). Harnessing disordered-ensemble quantum dynamics for machine learning. *Physical Review Applied*, 8(2), 024030.
3. Nakajima, K., & Fischer, I. (Eds.) (2021). *Reservoir Computing: Theory, Physical Implementations, and Applications*. Springer.

**Current research direction (2024)**: Physical RC in soft robotics, quantum RC, and the information-theoretic foundations of physical computing.

---

## David Sussillo

**Affiliation**: Google DeepMind / Stanford University

**Role in RC Development**: David Sussillo developed the FORCE learning algorithm [SussilloAbbott2009], which trains both the reservoir weights and readout simultaneously using an online update rule based on recursive least squares. FORCE learning enables reservoirs to generate target trajectories (not just classify inputs), opening up applications in motor control and pattern generation. This work bridges the standard RC framework (fixed reservoir) with fully trained RNNs.

**Three most important papers**:
1. Sussillo, D., & Abbott, L. F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
2. Sussillo, D. (2014). Neural circuits as computational dynamical systems. *Current Opinion in Neurobiology*, 25, 156–163.
3. Sussillo, D., Churchland, M. M., Kaufman, M. T., & Shenoy, K. V. (2015). A neural network that finds a naturalistic solution for the production of muscle activity. *Nature Neuroscience*, 18(7), 1025–1033.

**Current research direction (2024)**: Interpretability of recurrent neural networks, the geometry of neural population dynamics, and brain-computer interfaces.

---

## Surya Ganguli

**Affiliation**: Stanford University

**Role in RC Development**: Surya Ganguli has contributed theoretical analyses connecting reservoir computing to statistical mechanics and random matrix theory. His work on the eigenspectrum of random neural networks [GanguliSompolinsky2010] provides the theoretical tools for understanding how reservoir size, spectral radius, and connectivity affect computational capacity. His broader research program on the statistical physics of learning informs the theoretical foundations of RC.

**Three most important papers**:
1. Ganguli, S., Huh, D., & Sompolinsky, H. (2008). Memory traces in dynamical systems. *Proceedings of the National Academy of Sciences*, 105(48), 18970–18975.
2. Ganguli, S., & Sompolinsky, H. (2010). Short-term memory in neuronal networks through dynamical compressed sensing. In *NIPS 2010*.
3. Saxe, A. M., McClelland, J. L., & Ganguli, S. (2014). Exact solutions to the nonlinear dynamics of learning in deep linear neural networks. In *ICLR 2014*.

**Current research direction (2024)**: Statistical physics of deep learning, the geometry of high-dimensional learning dynamics, and theoretical neuroscience.

---

## Daniel Gauthier

**Affiliation**: Ohio State University

**Role in RC Development**: Daniel Gauthier has made important contributions to photonic and electronic reservoir computing, including experimental demonstrations of next-generation reservoir computing concepts. His work with Pathak et al. on Lorenz system prediction [PathakEtAl2018] is one of the most widely cited results in modern RC. He has also worked on hybrid "next-generation" RC (NGRC) that incorporates known physical structure into the readout layer.

**Three most important papers**:
1. Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). Model-free prediction of large spatiotemporally chaotic systems from data: A reservoir computing approach. *Physical Review Letters*, 120(2), 024102.
2. Gauthier, D. J., Bollt, E., Griffith, A., & Barbosa, W. A. S. (2021). Next generation reservoir computing. *Nature Communications*, 12, 5564.
3. Lu, Z., Hunt, B. R., & Ott, E. (2018). Attractor reconstruction by machine learning. *Chaos*, 28(6), 061104.

**Current research direction (2024)**: Next-generation RC (incorporating physics-informed features), quantum RC, and photonic computing for edge applications.

---

## Jaideep Pathak

**Affiliation**: Lawrence Berkeley National Laboratory / NVIDIA Research

**Role in RC Development**: Jaideep Pathak produced the landmark result [PathakEtAl2018] demonstrating that reservoir computing can predict the full spatiotemporal dynamics of a high-dimensional chaotic system (Kuramoto-Sivashinsky equation, 1000+ dimensional) with valid prediction times far exceeding previous methods. This paper demonstrated that RC scales to large, realistic systems and can match or exceed model-based prediction at comparable computational cost. Pathak also contributed to conceptual framework for "model-free prediction."

**Three most important papers**:
1. Pathak, J., Hunt, B., Girvan, M., Lu, Z., & Ott, E. (2018). (see above)
2. Pathak, J., Lu, Z., Hunt, B. R., Girvan, M., & Ott, E. (2017). Using machine learning to replicate chaotic attractors and calculate Lyapunov exponents from data. *Chaos*, 27(12), 121102.
3. Pathak, J., Wikner, A., Fussell, R., Chandra, S., Hunt, B. R., Girvan, M., & Ott, E. (2018). Hybrid forecasting of chaotic processes: Using machine learning in conjunction with a knowledge-based model. *Chaos*, 28(4), 041101.

**Current research direction (2024)**: Climate and weather prediction using ML, physics-informed ML, and data-driven turbulence modeling.

---

## Julie Grollier

**Affiliation**: CNRS / Thales Physics Unit, Palaiseau, France

(See detailed profile in Chapter 19 Key Researchers.)

**Three most important papers**:
1. Torrejon, J., et al. (2017). Neuromorphic computing with nanoscale spintronic oscillators. *Nature*, 547, 428–431.
2. Grollier, J., Querlioz, D., Camsari, K. Y., Everschor-Sitte, K., Fukami, S., & Stiles, M. D. (2020). Neuromorphic spintronics. *Nature Electronics*, 3(7), 360–370.
3. Riou, M., et al. (2019). Temporal pattern recognition with delayed-feedback spin-torque nano-oscillators. *Physical Review Applied*, 12(2), 024049.

**Current research direction (2024)**: Neuromorphic spintronics chip development, hybrid CMOS-spintronic systems, and AI accelerators based on physical dynamics.

---

## Peter Tino

**Affiliation**: University of Birmingham, UK

**Role in RC Development**: Peter Tino has contributed extensively to the theory of recurrent network computation, particularly regarding what formal languages and temporal patterns can be represented. His work connects RC theory to formal language theory, statistical learning theory, and the mathematics of iterated function systems. Tino's collaboration with various groups has produced important results on the capacity limits of reservoir systems.

**Three most important papers**:
1. Tino, P., & Kotismannis, M. (2010). Architectural bias in recurrent neural networks: Fractal analysis. *Neural Computation*, 22(7), 1673–1712.
2. Tino, P., Cernansky, M., & Benuskova, L. (2004). Markovian architectural bias of recurrent neural networks. *IEEE Transactions on Neural Networks*, 15(1), 6–15.
3. Hammer, B., Tiño, P., & Micheli, A. (2004). A novel approach to learning recurrent networks. *International Journal of Neural Systems*, 14(5), 303–319.

**Current research direction (2024)**: Theoretical analysis of RC capacity, probabilistic RC models, and connections to kernel methods and Gaussian processes.

---

## David Verstraeten

**Affiliation**: Ghent University (PhD); later industry

**Role in RC Development**: David Verstraeten produced the most comprehensive empirical benchmarking study of RC methods [Verstraeten2009], comparing ESN and LSM approaches on speech, robot localization, and time series tasks. He also developed the concept of the "information saturation" threshold in reservoirs — the idea that reservoir states become information-saturated when the reservoir is too small relative to the complexity of the task.

**Three most important papers**:
1. Verstraeten, D., Schrauwen, B., d'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.
2. Verstraeten, D., & Schrauwen, B. (2010). On the quantification of dynamics in reservoir computing. In *ICANN 2010*.
3. Verstraeten, D. (2009). *Reservoir Computing: Computation with Dynamical Systems*. PhD thesis, Ghent University.

**Current research direction (2024)**: Industry (AI/ML applications); academic contributions have been taken forward by former lab members.
