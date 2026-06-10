# Chapter 30: Further Reading

## State Space Models

**Gu, A., Goel, K., and Ré, C. (2022).** "Efficiently Modeling Long Sequences with Structured State Spaces." *International Conference on Learning Representations (ICLR 2022)*. The S4 paper. Introduces the DPLR structure, the HiPPO initialization connection, and the efficient Cauchy kernel computation. Essential reading for Sections 30.2.3–30.2.5.

**Gu, A., Johnson, I., Goel, K., et al. (2020).** "HiPPO: Recurrent Memory with Optimal Polynomial Projections." *Advances in Neural Information Processing Systems (NeurIPS 2020)*. Introduces the HiPPO framework. Derives the optimal polynomial approximation interpretation of recurrent memory and the specific ODE matrices for different memory measures.

**Gu, A. and Dao, T. (2023).** "Mamba: Linear-Time Sequence Modeling with Selective State Spaces." *arXiv:2312.00752*. The Mamba paper. Introduces selective (input-dependent) state spaces and the hardware-efficient parallel scan algorithm. Shows state-of-the-art performance on language modeling benchmarks while being more efficient than transformers for long sequences.

**Gu, A., Goel, K., Gupta, A., and Ré, C. (2022).** "On the Parameterization and Initialization of Diagonal State Space Models." *Advances in Neural Information Processing Systems (NeurIPS 2022)*. Introduces S4D, a simplified diagonal SSM. Shows that the DPLR structure can be simplified to a diagonal $A$ matrix with appropriate initialization, significantly reducing implementation complexity.

## Liquid Neural Networks and CfC

**Hasani, R., Lechner, M., Amini, A., Rus, D., and Grosu, R. (2021).** "Liquid Time-Constant Networks." *Proceedings of the AAAI Conference on Artificial Intelligence*, 35. The original LNN paper. Derives the time-constant neuron model from Hodgkin-Huxley conductance dynamics and demonstrates performance on autonomous driving tasks with very small networks.

**Hasani, R., Lechner, M., Amini, A., Liebenwein, L., Ray, A., Tschaikowski, M., Tanner, G., and Rus, D. (2022).** "Closed-Form Continuous-Time Neural Networks." *Nature Machine Intelligence*, 4, 992–1003. The CfC paper. Derives the closed-form approximation to LNN dynamics, connects it to GRUs, and demonstrates competitive performance with much reduced computational cost.

## Recurrent-Transformer Hybrids

**De, S., Smith, S., Fernando, A., et al. (2024).** "Griffin: Mixing Gated Linear Recurrences with Local Attention for Efficient Language Models." *arXiv:2402.19427*. A principled hybrid of recurrent (reservoir-like) and local attention components. Achieves transformer-level performance with improved efficiency at long context.

**Bulatov, A., Kuratov, Y., and Burtsev, M.S. (2022).** "Recurrent Memory Transformer." *Advances in Neural Information Processing Systems (NeurIPS 2022)*. Uses recurrent token memories to extend transformer context. The recurrent memories play the role of a reservoir state summarizing long-past context.

**Sun, Y., Dong, L., Huang, S., et al. (2023).** "Retentive Network: A Successor to Transformer for Large Language Models." *arXiv:2307.08621*. Proposes the "retention" mechanism as a recurrent alternative to attention, with explicit connections to state space models. Another example of the convergence between RC ideas and transformer architectures.

## Linear Attention and Reservoir Connections

**Katharopoulos, A., Vyas, A., Pappas, N., and Fleuret, F. (2020).** "Transformers are RNNs: Fast Autoregressive Transformers with Linear Attention." *Proceedings of the International Conference on Machine Learning (ICML 2020)*. Shows that the softmax attention can be approximated by a kernel function, yielding a linear-time recurrent model. The resulting recurrence is equivalent to a linear reservoir with a kernel feature map.

## Foundation Models as Feature Extractors

**Bommasani, R., Hudson, D.A., Aditi, E., et al. (2021).** "On the Opportunities and Risks of Foundation Models." *Stanford CRFM Technical Report*. A comprehensive survey of large pretrained models. Section 4 discusses linear probing and representation quality, relevant to the "LLM as reservoir" paradigm.

**Tay, Y., Dehghani, M., Abnar, S., et al. (2021).** "Long Range Arena: A Benchmark for Efficient Transformers." *International Conference on Learning Representations (ICLR 2021)*. The benchmark used to evaluate S4, Mamba, and related models. Understanding the tasks helps calibrate when state space models (and by extension, reservoirs) outperform transformers.
