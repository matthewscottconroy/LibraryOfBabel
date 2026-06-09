# Chapter 4: Applications of Second-Order Linear Equations

The theory developed in the preceding chapters finds its most natural expression in two closely parallel physical models: the mechanical vibration of a spring-mass system and the electrical oscillation of an RLC circuit. Both are governed by second-order linear constant-coefficient equations; the mathematical content is identical while the physical interpretation differs. This chapter develops both models in the full range of their behavior.

## The Spring-Mass-Dashpot System

A mass $m$ on a spring with stiffness constant $k$ and a dashpot (damper) with damping coefficient $\gamma$ satisfies Newton's second law:

$$m\ddot{x} + \gamma\dot{x} + kx = F(t),$$

where $x(t)$ is displacement from equilibrium and $F(t)$ is any external forcing. This single equation, with $m, \gamma, k > 0$, encompasses all the phenomena of linear vibration theory: free undamped oscillation, free damped oscillation in three regimes, and forced oscillation with the possibility of resonance.

## Chapter Contents

The chapter is organized into five sections. The first develops free (unforced) undamped oscillation: the ideal spring with $\gamma = 0$ and $F = 0$, giving pure sinusoidal motion at the natural frequency $\omega_0 = \sqrt{k/m}$. The second adds damping and analyzes the three regimes (underdamped, critically damped, overdamped) in terms of the discriminant. The third introduces forcing $F(t)$ and studies the particular solution (steady-state response). The fourth analyzes resonance (when the driving frequency equals the natural frequency) and beats (when frequencies are close but not equal). The fifth develops the exact mathematical analogy with the RLC electrical circuit.

## Mathematical Parallels

The analogy between mechanical and electrical systems is one of the most fertile in all of applied mathematics:

| Mechanical | Electrical |
|---|---|
| Mass $m$ | Inductance $L$ |
| Damping $\gamma$ | Resistance $R$ |
| Spring stiffness $k$ | $1/C$ (reciprocal capacitance) |
| Displacement $x$ | Charge $q$ |
| Force $F$ | Voltage $E$ |

The governing equations $m\ddot{x} + \gamma\dot{x} + kx = F(t)$ and $L\ddot{q} + R\dot{q} + q/C = E(t)$ are mathematically identical. Physical insight from one domain translates directly to the other. The analogy suggests, for example, that electrical resonance in an RLC circuit is exactly analogous to mechanical resonance in a spring-mass system, down to the formula for the resonant frequency.
