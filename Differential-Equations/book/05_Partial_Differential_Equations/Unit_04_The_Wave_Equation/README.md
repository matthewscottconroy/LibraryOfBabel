# Unit 4: The Wave Equation

The wave equation $u_{tt} = c^2\Delta u$ governs the propagation of disturbances through a medium: sound through air, light through space, seismic waves through the earth, vibrations of strings and membranes. Unlike the heat equation, which smooths its initial data and is irreversible, the wave equation preserves the sharpness of signals, is time-reversible, and propagates information at a finite speed $c$. These physical differences reflect deep mathematical distinctions — different characteristics, different well-posedness conditions, different long-time behavior.

## The Physical Picture

A guitar string plucked at $t=0$ vibrates back and forth indefinitely (without damping). A sound pulse emitted at a point travels outward as a spherical shell at the speed of sound. An earthquake wave detected at a seismograph carries encoded information about the distant source, arriving after a travel time $r/c$ where $r$ is the source distance and $c$ the wave speed. All of these phenomena are modeled by the wave equation, and the mathematics captures the physics with remarkable fidelity.

## Unit Overview

**Chapter 1: Derivation and Properties** derives the wave equation from Newton's second law applied to a vibrating string under tension, and discusses d'Alembert's formula — the explicit solution to the Cauchy problem in one dimension as a sum of right- and left-traveling waves. The domain of dependence and domain of influence, which encode the finite propagation speed, are analyzed.

**Chapter 2: Separation of Variables** develops the normal mode (standing wave) solutions via eigenfunction expansion. The normal modes of a string are the harmonic series of frequencies that characterize the instrument's sound. Nonhomogeneous problems (forced oscillations and resonance) are treated via Duhamel's principle.

**Chapter 3: Multidimensional Wave Equation** extends the theory to higher dimensions. The drumhead (circular membrane) and spherical waves are treated explicitly. The profound dimension-dependence of wave propagation — Huygens' principle holds in odd dimensions but not even — is explained.

**Chapter 4: Characteristics and Energy** develops the theory of characteristics for the wave equation in higher dimensions (the characteristic surfaces are light cones), proves Huygens' principle rigorously, and establishes energy conservation, which implies uniqueness and continuous dependence.

## Central Contrast with the Heat Equation

The wave equation differs from the heat equation in every qualitative respect:
- **Finite vs. infinite propagation speed.** Signals travel at speed $c$ in the wave equation; at infinite speed in the heat equation.
- **Reversibility.** The wave equation is time-reversible; the heat equation is not.
- **Regularity.** The wave equation preserves the smoothness class of initial data; the heat equation instantly smooths rough data.
- **Long-time behavior.** Solutions of the heat equation decay to zero; solutions of the wave equation in bounded domains oscillate perpetually (without dissipation).

Understanding both equations deeply, and especially their differences, is fundamental to classical mathematical physics.
