# Chapter 23: Key Researchers

## Auke Jan Ijspeert (EPFL)

Auke Ijspeert is the leading researcher on CPG-based locomotion control and its connection to reservoir computing. His work on salamander locomotion [IjspeertEtAl2007] demonstrated how a CPG network can generate both swimming and walking gaits through modulation of drive signals, with direct parallels to reservoir computing principles. Ijspeert's group has developed the Webots robotics simulator and has conducted landmark experiments on CPG-controlled robotic locomotion.

**Representative works**: [IjspeertEtAl2007], [IjspeertEtAl2014], [RoemscheidEtAl2016]

## Florentin Wörgötter (University of Göttingen)

Florentin Wörgötter has worked on the intersection of reservoir computing and robot control, particularly on learning sensorimotor loops and adaptive locomotion. His work on "regularized" reservoir computing for robot control addresses the sample efficiency problem and online adaptation.

**Representative works**: [WorgotterEtAl2013], [WorgotterEtAl2014]

## Jan Peters (TU Darmstadt / MPI Intelligent Systems)

Jan Peters is a leading researcher on robot reinforcement learning, particularly policy gradient methods and their application to robot motor skills. His work on REINFORCE, Natural Policy Gradient, and REPS provides the theoretical foundation for the policy gradient methods used with reservoir policies.

**Representative works**: [PetersSchaal2008], [PetersEtAl2010]

## Tim Salimans (OpenAI)

Tim Salimans co-authored the OpenAI ES paper [SalimansEtAl2017] that demonstrated evolution strategies as a competitive alternative to deep RL for locomotion tasks. This work directly enabled the ES + reservoir policy combination and showed that simple parameter-perturbation search can solve complex locomotion problems.

**Representative works**: [SalimansEtAl2017]

## Konrad Rawlik / Sethu Vijayakumar (University of Edinburgh)

Sethu Vijayakumar's group has studied reservoir computing for learning motor skills in humanoid robots, building on the Hierarchical Motor Learning framework. Their work connects the biomechanics of human movement (compliance, redundancy, impedance) with the reservoir computing framework.

**Representative works**: [VijayakumarEtAl2005], [RawlikEtAl2013]

## Marc Toussaint (TU Berlin)

Marc Toussaint's work on probabilistic trajectory optimization and model-based RL has informed reservoir-based approaches to robot manipulation, particularly in the treatment of uncertainty and contact dynamics.

**Representative works**: [ToussaintEtAl2018], [ToussaintEtAl2020]

## Jens Kober (TU Delft)

Jens Kober has conducted extensive empirical studies comparing RL algorithms for robot motor skill learning, with work that is directly relevant to positioning reservoir policies in the landscape of robot learning methods.

**Representative works**: [KoberEtAl2013]

## Wolfgang Maass / Robert Legenstein (Graz University of Technology)

In addition to liquid state machine theory, Maass and Legenstein have worked on reward-modulated learning in spiking neural networks [LegensteinEtAl2010] — a biologically motivated approach to reservoir RL that uses STDP-like rules to train the reservoir (not just the readout). This goes beyond the standard reservoir computing paradigm and represents a bridge to biologically realistic motor learning.

**Representative works**: [MaassEtAl2002], [LegensteinEtAl2010], [MaassEtAl2014]
