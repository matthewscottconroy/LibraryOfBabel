# Chapter 13 — Further Reading and References

---

## Essential References

### [Gallicchio2017a] — The Foundational Theory Paper

**Gallicchio, C. & Micheli, A. (2017). Echo state property of deep reservoir computing networks. *Cognitive Computation*, 9(3), 337–350.**

The paper that established deep ESNs as a theoretically principled architecture. Gallicchio and Micheli prove the layerwise echo state property, derive the sufficient conditions, and characterize the timescale hierarchy analytically. The proof of the effective memory time constant at each layer is presented here. Essential reading before working with deep reservoirs.

### [Gallicchio2017b] — The Critical Experimental Analysis

**Gallicchio, C., Micheli, A., & Pedrelli, L. (2017). Deep reservoir computing: A critical experimental analysis. *Neurocomputing*, 268, 87–99.**

The companion empirical paper. Tests deep ESNs against shallow baselines on 8 benchmark tasks including speech recognition, sequential MNIST, and polyphonic music prediction. Shows that the concatenated readout outperforms any single layer, that the timescale hierarchy is measurably present, and that depth helps most for tasks with genuine multi-scale structure. Includes a careful ablation study.

---

## Graph Reservoir Computing

### [Gallicchio2010]

**Gallicchio, C. & Micheli, A. (2010). Graph echo state networks. In *Proceedings of IJCNN 2010*. IEEE.**

The original Graph ESN paper. Proposes the architecture, establishes the ESP condition for graphs, and applies it to structured-data classification tasks.

### [Gallicchio2020]

**Gallicchio, C. & Micheli, A. (2020). Fast and deep graph neural networks. In *Proceedings of AAAI 2020*, 34(04), 3898–3905.**

A more recent treatment combining deep reservoir computing with graph neural network architectures. Shows that random, fixed encoders with linear readouts can be competitive with trained GNNs on several molecular property prediction benchmarks.

### [Micheli2009]

**Micheli, A. (2009). Neural network for graphs: A contextual constructive approach. *IEEE Transactions on Neural Networks*, 20(3), 498–511.**

Micheli's foundational work on GNNs, which provides the context for the Graph ESN architecture. Understanding the original constructive GNN approach clarifies why the reservoir version (fixed weights, linear readout) is a natural simplification.

---

## Background: Depth in Neural Networks

### [LeCun2015]

**LeCun, Y., Bengio, Y., & Hinton, G. (2015). Deep learning. *Nature*, 521(7553), 436–444.**

The review that popularized deep learning. Section on temporal models is directly relevant. Understanding why depth matters for feedforward networks motivates the analogous question for reservoirs.

### [Pascanu2013]

**Pascanu, R., Mikolov, T., & Bengio, Y. (2013). On the difficulty of training recurrent neural networks. In *Proceedings of ICML*. PMLR, 1310–1318.**

Analyzes the exploding/vanishing gradient problem in deep RNNs. The timescale analysis in Chapter 13 can be read as the reservoir-computing response to the challenges identified here: by fixing the recurrent weights, we avoid the optimization problem while retaining the representational benefits of depth.

---

## Timescale Analysis and Liquid State Machines

### [Maass2002]

**Maass, W., Natschläger, T., & Markram, H. (2002). Real-time computing without stable states: A new framework for neural computation based on perturbations. *Neural Computation*, 14(11), 2531–2560.**

The liquid state machine paper. Introduces the separation property and approximation property, which are the LSM analogs of the ESP and universal approximation results for ESNs. The temporal kernel analysis in this paper motivates the timescale hierarchy discussion in Section 13.2.

### [Verstraeten2007]

**Verstraeten, D., Schrauwen, B., D'Haene, M., & Stroobandt, D. (2007). An experimental unification of reservoir computing methods. *Neural Networks*, 20(3), 391–403.**

Compares ESNs, LSMs, and backpropagation-decorrelated learning machines on a common benchmark suite. The discussion of timescales and memory capacity provides useful background for understanding why deep architectures extend the useful operating range.

---

## Advanced: Functional Analysis of Deep Reservoirs

### [Gonon2020]

**Gonon, L. & Ortega, J.P. (2020). Reservoir computing universality with stochastic inputs. *IEEE Transactions on Neural Networks and Learning Systems*, 31(1), 100–112.**

Proves universal approximation for random reservoirs with stochastic inputs, extending the Boyd-Chua theory. The deep case is discussed as a special instance where the effective approximation class grows with depth.

### [Grigoryeva2018]

**Grigoryeva, L. & Ortega, J.P. (2018). Echo state networks are universal. *Neural Networks*, 108, 495–508.**

A rigorous proof that ESNs with polynomial readouts are universal approximators for fading-memory functionals. The extension to deep networks follows from the layerwise ESP argument of Chapter 13.
