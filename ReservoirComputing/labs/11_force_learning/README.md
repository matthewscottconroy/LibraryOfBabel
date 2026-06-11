**Corresponds to:** Chapter 12 — FORCE Learning: Harnessing Chaos for Pattern Generation (Unit 4)

**Prerequisites:** Chapter 5 (ESN basics), Chapter 11 (online RLS), Chapter 2 (chaotic dynamics), feedback systems

**Learning Objectives:**
- Understand FORCE learning: train the output weight vector W_out via online RLS while feeding the output back into the reservoir during training (teacher forcing)
- Appreciate why a large, chaotic reservoir (ρ > 1) is used: the freely running reservoir already produces rich high-dimensional dynamics, and FORCE shapes those dynamics to match the target
- Observe the transition from teacher-forced training to autonomous pattern generation
- Compare N=500 (large) and N=50 (small) reservoirs: how does reservoir size affect convergence speed and autonomous stability?
- Connect FORCE learning to the neuroscientific model of motor cortex (Sussillo & Abbott 2009)
