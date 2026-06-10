# Chapter 18: Key Concepts

## Morphological Computation

The hypothesis that the physical structure of a body (its geometry, compliance, mass distribution) performs computations that would otherwise need to be implemented explicitly in a neural controller. Introduced by Pfeifer and Iida [PfeiferIida2004] in the context of embodied cognition, formalized within the reservoir computing framework by Hauser et al. [HauserEtAl2011]. The key claim: computation can be *harvested* from physical dynamics rather than designed into a substrate.

## Compliant Mechanism

A mechanism that achieves its motion through elastic deformation of its members rather than through rigid joints. Compliant mechanisms are intrinsically nonlinear (due to geometric nonlinearity at large deformations), have distributed parameter dynamics, and exhibit fading memory due to elastic restoring forces and viscous damping. These properties make them natural reservoir substrates.

## Echo State Property (Physical Systems)

In the context of physical reservoirs, the echo state property states that for any bounded input signal $u(t)$, the physical state $\mathbf{x}(t)$ of the reservoir is asymptotically independent of initial conditions. For mechanical systems with positive-definite damping, this follows from the contractivity of the flow. The forgetting timescale $\tau_{\text{mem}}$ is governed by the smallest eigenvalue of the damping matrix: $\tau_{\text{mem}} \sim d_{\min}^{-1}$.

## Participation Ratio

A measure of the effective dimensionality of the reservoir state trajectory, defined as:

$$d_{\text{eff}} = \frac{\left(\sum_i \sigma_i^2\right)^2}{\sum_i \sigma_i^4}$$

where $\sigma_i$ are the singular values of the state matrix $X = [\mathbf{x}(t_1), \ldots, \mathbf{x}(t_T)]$. Ranges from 1 (all information in one direction) to $\min(N, T)$ (uniformly distributed). Higher values indicate richer, more diverse reservoir dynamics.

## Tensegrity

A structural principle in which rigid compression members (struts) are suspended in a network of tension-only elements (cables), achieving global stability through the balance of tension and compression without direct contact between rigid members. Coined by Buckminster Fuller [Fuller1962]. Tensegrity structures are natural reservoir substrates because their cable elements act as ReLU nonlinearities (active only under tension), creating piecewise-nonlinear dynamics.

## Granular Medium

An assembly of discrete macroscopic particles interacting through contact forces. Granular media are dissipative (inelastic collisions), nonlinear (Hertzian contact law $F \propto \delta^{3/2}$), and history-dependent (force chain geometry). Demonstrated as a viable reservoir substrate by Nakajima et al. [NakajimaEtAl2015Granular], achieving NARMA-10 performance comparable to software ESNs with 16 surface displacement sensors.

## Stiffness-Memory Trade-off

The inverse relationship between a compliant reservoir's elastic stiffness and its memory depth. High stiffness $\to$ fast restoring forces $\to$ short memory. Low stiffness $\to$ slow restoring forces $\to$ long memory but reduced nonlinearity. Optimal task performance is achieved at intermediate stiffness values, analogous to the critical spectral radius in ESNs.

## Distributed Sensing

The practice of measuring physical state at multiple spatially distributed locations rather than at a single point. In compliant arm reservoirs, distributed strain sensing along the arm's length provides access to multiple bending modes (spatial Fourier components), each corresponding to a different temporal frequency of the reservoir's response. Distributed sensing is a key enabler of high reservoir capacity in physical systems.

## Geometric Nonlinearity

Nonlinearity that arises from the geometry of large deformations, as opposed to material nonlinearity (nonlinear stress-strain relations). A beam that bends significantly exhibits geometric nonlinearity because the relationship between forces and displacements becomes nonlinear even if the material itself is linearly elastic. Geometric nonlinearity is responsible for much of the nonlinear capacity of soft-body reservoirs.

## Physical Echo State Network

The functional analog of an echo state network (Chapter 5) implemented in a physical substrate. The input signal drives the physical system (plays the role of input weights), the physical dynamics evolve the state (play the role of reservoir weights), and sensors observe the state (provide the readout features). Training occurs only at the readout stage, exactly as in a software ESN. The term was introduced by Hauser et al. [HauserEtAl2011].

## Memory Capacity (Mechanical)

The total linear memory of a mechanical reservoir, computed as:

$$\text{MC} = \sum_{k=1}^{\infty} r_k^2(u(t-k), \hat{y}_k(t))$$

where $r_k^2$ is the squared correlation between the best linear readout of the reservoir state and the $k$-step delayed input. For a compliant arm with memory timescale $\tau_{\text{mem}}$, MC scales approximately as $\tau_{\text{mem}} / \Delta t$ where $\Delta t$ is the sampling interval.
