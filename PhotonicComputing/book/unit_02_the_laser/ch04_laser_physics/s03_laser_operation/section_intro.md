# Section 4.3: Laser Operation

## From Statics to Dynamics

Sections 4.1 and 4.2 treated the laser as a steady-state system: population inversion balanced against cavity losses, gain equal to loss at threshold, output power proportional to excess current. This static picture is essential for understanding what a laser *is*, but it says nothing about what a laser *does* when you perturb it — when you switch the current, modulate it at high frequency, or ask it to produce short pulses.

Laser dynamics govern the modulation bandwidth (how fast you can encode data), the stability under feedback, the turn-on transient, the conditions under which the laser produces short pulses rather than cw output, and the noise properties that ultimately limit the signal-to-noise ratio of any photonic system using it.

This section analyzes laser operation dynamically, treating threshold behavior and slope efficiency first (the quasi-static regime), then pulse generation by Q-switching and mode-locking, and finally the fundamental noise of laser output — the Schawlow-Townes linewidth revisited from the perspective of the rate equations.

## Section Structure

- **4.3.1** — Threshold, Slope Efficiency, and Direct Modulation
- **4.3.2** — Mode-Locking
- **4.3.3** — Q-Switching
- **4.3.4** — Laser Noise and Relative Intensity Noise
