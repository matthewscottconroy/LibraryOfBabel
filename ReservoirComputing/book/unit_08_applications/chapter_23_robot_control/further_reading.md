# Chapter 23: Further Reading

## CPGs and Locomotion Control

**Ijspeert, A. J., Crespi, A., Ryczko, D., & Cabelguen, J. M. (2007).** From swimming to walking with a salamander robot driven by a spinal cord model. *Science*, 315(5817), 1416–1420.
The landmark CPG locomotion paper. Demonstrates that a biologically inspired CPG model can generate both swimming and walking gaits through modulation of drive signal amplitude — a direct precursor to reservoir-based locomotion.

**Ijspeert, A. J. (2008).** Central pattern generators for locomotion control in animals and robots: A review. *Neural Networks*, 21(4), 642–653.
Comprehensive review of CPG models and their application to robot locomotion. Covers both biological evidence for CPGs and engineering applications.

**Grillner, S., & Wallén, P. (1985).** Central pattern generators for locomotion, with special reference to vertebrates. *Annual Review of Neuroscience*, 8(1), 233–261.
Classic neuroscience paper establishing the CPG concept from biological evidence. The theoretical backdrop for reservoir CPG approaches.

## Reinforcement Learning

**Williams, R. J. (1992).** Simple statistical gradient-following algorithms for connectionist reinforcement learning. *Machine Learning*, 8(3–4), 229–256.
The original REINFORCE paper. Contains the derivation of the policy gradient theorem and the REINFORCE update rule used throughout this chapter.

**Salimans, T., Ho, J., Chen, X., Sidor, S., & Sutskever, I. (2017).** Evolution strategies as a scalable alternative to reinforcement learning. *arXiv:1703.03864*.
Demonstrates ES as competitive with deep RL on Mujoco locomotion benchmarks. Directly relevant to ES-based reservoir policy training.

**Peters, J., & Schaal, S. (2008).** Natural actor-critic. *Neurocomputing*, 71(7–9), 1180–1190.
The natural policy gradient approach, providing superior convergence compared to vanilla REINFORCE. Applicable to reservoir policies.

**Sutton, R. S., & Barto, A. G. (2018).** *Reinforcement Learning: An Introduction* (2nd ed.). MIT Press.
The standard RL textbook. Required background for Sections 23.2–23.3.

## Robot Learning

**Kober, J., Bagnell, J. A., & Peters, J. (2013).** Reinforcement learning in robotics: A survey. *International Journal of Robotics Research*, 32(11), 1238–1274.
Comprehensive survey of RL methods for robot control, providing the competitive context for reservoir policy approaches.

**Conti, E., Madhavan, V., Such, F. P., Lehman, J., Stanley, K. O., & Clune, J. (2018).** Improving exploration in evolution strategies for deep reinforcement learning via a population of novelty-seeking agents. In *NeurIPS 2018*.
Extends ES with novelty search, directly relevant to reservoir+ES locomotion training.

## Reservoir Computing for Robot Control

**Nakajima, K. (2020).** Physical reservoir computing — an introductory perspective. *Japanese Journal of Applied Physics*, 59(6), 060501.
Overview paper connecting physical reservoir computing (including mechanical systems, Chapter 18) to robot control applications.

**Hauser, H., Füchslin, R. M., & Pfeifer, R. (Eds.) (2014).** *Opinions and Outlooks on Morphological Computation*. E-book, http://www.morphcomp.org.
Collection of essays on morphological computation from leading researchers, with multiple chapters on robot control and reservoir computing.
