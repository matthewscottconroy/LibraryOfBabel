**Corresponds to:** Chapter 3 — The Fading Memory Property (Unit 2)

**Prerequisites:** Chapter 1 (dynamical systems basics), Chapter 2 (fixed points and stability), basic NumPy and matplotlib

**Learning Objectives:**
- Understand what fading memory means: a reservoir's current state is more strongly influenced by recent inputs than by distant past inputs
- Visualise how the influence of past inputs decays as a function of lag for different spectral radii ρ
- Quantify "effective memory time" as the lag at which cross-correlation drops below 1/e of its peak
- Discover the core ρ → memory tradeoff: larger ρ gives longer memory at the cost of approaching instability
- Verify the formal fading memory property: two sequences that agree after time k produce nearly identical states, with the difference vanishing as k increases
