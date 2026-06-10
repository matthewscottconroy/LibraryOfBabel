# Chapter 9: Further Reading

## Primary Sources

**Rodan, A., & Tino, P. (2011). Minimum complexity echo state network. *IEEE Transactions on Neural Networks*, 22(1), 131–144.**
[RodanTino2011]

The SCR paper. Introduces the Simple Cycle Reservoir and several related minimum-complexity architectures, proves the memory capacity bound $MC = N$ for the linear SCR, and provides extensive empirical comparisons with random ESNs. The theoretical analysis is rigorous and the empirical results are convincing. Essential reading for anyone interested in structured reservoirs.

**Triesch, J. (2005). A gradient rule for the plasticity of a neuron's intrinsic excitability. In *Proceedings of the International Conference on Artificial Neural Networks* (ICANN 2005), pp. 65–70. Springer.**
[Triesch2005]

The original IP paper. Compact (6 pages) and mathematically precise. Derives the update rules for gain and bias from the infomax principle for both logistic and tanh activations. The convergence analysis is informal but the empirical demonstrations are clear. Read in conjunction with the longer follow-up paper:

**Triesch, J. (2007). Synergies between intrinsic and synaptic plasticity mechanisms. *Neural Computation*, 19(4), 885–909.**
[Triesch2007]

An extended analysis of IP and its interaction with Hebbian synaptic plasticity. More mathematically thorough than the conference paper.

## Background and Extensions

**Schrauwen, B., Wardermann, M., Verstraeten, D., Steil, J. J., & Stroobandt, D. (2008). Improving reservoirs using intrinsic plasticity. *Neurocomputing*, 71(7–9), 1159–1171.**
[Schrauwen2008]

A systematic empirical study of IP applied to echo state networks. Tests IP on several benchmark tasks and analyzes the effect of the target distribution parameter $\mu$ on performance. Confirms that IP improves performance on tasks with complex nonlinear structure and provides guidelines for setting IP hyperparameters.

**Tino, P., & Rodan, A. (2013). Short-term memory in input-driven linear dynamical systems. *Neurocomputing*, 112, 58–63.**
[Tino2013]

A theoretical follow-up to the SCR paper that analyzes memory capacity in linear reservoirs with arbitrary structure. Shows that the memory capacity depends on the eigenspectrum of $W$ in a precise way: $MC = \sum_i (1-|\lambda_i|^2)|\lambda_i|^2 / (|\lambda_i|^2 \text{ variance terms})$, and derives conditions under which the SCR's uniform spectrum is optimal.

**Linsker, R. (1988). Self-organization in a perceptual network. *Computer*, 21(3), 105–117.**
[Linsker1988]

The original infomax paper in neural networks. While predating reservoir computing, this is essential context for understanding the theoretical foundations of intrinsic plasticity.

**Jaeger, H., Lukosevicius, M., Popovici, D., & Siewert, U. (2007). Optimization and applications of echo state networks with leaky integrator neurons. *Neural Networks*, 20(3), 335–352.**
[Jaeger2007leaky]

Introduces and analyzes leaky integrator ESNs, closely related to the material in Chapter 8.4 but relevant here because it discusses initialization strategies for leaky reservoirs.
