# Granular Media Reservoirs

## Granular Chains and Hertz Contact

A granular chain is a one-dimensional array of spherical particles in contact. When two elastic spheres of radii $R_1$ and $R_2$ and elastic moduli $E_1, E_2$ are pressed together with a normal force $F$, the Hertz contact law gives the contact force as a nonlinear function of the overlap $\delta$ (approach of centers):

$$F = \frac{4}{3} E^* \sqrt{R^*} \, \delta^{3/2},$$

where $E^* = [(1-\nu_1^2)/E_1 + (1-\nu_2^2)/E_2]^{-1}$ is the composite modulus and $R^* = (1/R_1 + 1/R_2)^{-1}$ is the composite radius [Nesterenko 2001]. The $\delta^{3/2}$ power law is the critical nonlinearity: unlike a linear spring ($F \propto \delta$), the Hertz contact gives a stiffness that increases with compression ($dF/d\delta \propto \sqrt{\delta}$), creating strongly nonlinear wave propagation.

This Hertz nonlinearity is the source of the reservoir's computational power. Linear granular chains would be equivalent to a chain of harmonic oscillators, producing only linear temporal filtering. The $\delta^{3/2}$ force law enables amplitude-dependent propagation speeds, harmonic generation, and solitary wave formation.

## Solitary Waves in Granular Chains

The most striking dynamical feature of granular chains is the solitary wave (soliton-like pulse). In a homogeneous, pre-compressed chain, Nesterenko [2001] showed that the equations of motion admit a traveling wave solution:

$$u_n(t) = A f(n - vt),$$

where $u_n$ is the displacement of particle $n$, $A$ is the amplitude, $v$ is the wave speed, and $f$ is a bell-shaped envelope. The wave speed depends on amplitude:

$$v \propto A^{1/4} c_0,$$

where $c_0 = [(4/3)E^* \sqrt{R^*} F_0^{1/2} / \rho V_0]^{1/2}$ is the speed at static pre-compression force $F_0$. Amplitude-dependent speed means that different-amplitude pulses travel at different speeds, providing a nonlinear separation of input signals that is the wave-based analog of nonlinear mixing in a reservoir [Nesterenko 2001].

## Nakajima et al. 2014 Demonstration

Nakajima et al. [2014] demonstrated granular chain reservoir computing for locomotion pattern generation. A chain of 10 stainless steel spheres (diameter 9.5 mm) was pre-compressed by a screw mechanism to set the static contact force $F_0$, controlling the linear propagation speed. Piezoelectric actuators at one end provided the input; piezoelectric sensors at several points along the chain read out the state.

The task was to generate the appropriate activation patterns for a hexapod robot's six legs (three pairs of legs, each pair needing coordinated two-phase activation). The granular chain's dynamics, when driven by a simple periodic input and read out at 6 sensor positions, generated the required six-channel locomotion pattern after training a linear readout with ridge regression [Nakajima et al. 2014].

Key result: the 6-sensor readout of a 10-ball chain correctly generated all three locomotion gaits (tripod, tetrapod, wave) by changing only the readout weights — demonstrating the reservoir property that a fixed physical system can compute multiple tasks through readout variation.

## Rich Spectral Content from Nonlinear Mixing

The Hertz nonlinearity generates harmonics and inter-modulation products of the input frequency. An input sinusoid at frequency $f_0$ produces output components at $f_0, 2f_0, 3f_0, \ldots$ (harmonics) as well as subharmonics in sufficiently nonlinear regimes. This rich spectral content is the granular reservoir's analog of the high-dimensional reservoir state in an ESN: the nonlinear mixing distributes energy across many frequencies, each carrying information about the input history [Nesterenko 2001].

The spectral richness can be controlled by pre-compression $F_0$: high pre-compression (strongly nonlinear regime) produces more harmonic mixing; low pre-compression (near-linear regime) reduces mixing. This provides a physically tunable nonlinearity parameter analogous to the spectral radius in a simulated ESN.

## Tuning Granular Media

The principal tuning parameters for granular reservoir performance are:

**Particle material:** Steel (high stiffness, fast dynamics), aluminum (lower stiffness, intermediate), nylon (low stiffness, slow, high damping). High damping increases fading memory rate but reduces signal amplitude.

**Particle size:** Larger particles reduce natural frequencies, shifting the reservoir's effective timescale toward lower frequencies — analogous to decreasing $\alpha$ in an ESN.

**Pre-compression $F_0$:** Controls operating point on the Hertz curve. Larger $F_0$ increases wave speed and reduces the effective nonlinearity (the system operates in the weakly nonlinear regime). Smaller $F_0$ increases nonlinearity (strongly nonlinear regime), potentially destabilizing the reservoir.

**Chain length:** More particles provides more sensor positions (more virtual nodes) and longer effective memory from wave propagation time.

## Limitations

Granular reservoirs have several practical disadvantages: (1) they are sensitive to environmental vibration, which adds uncontrolled input; (2) the dynamic range is limited by particle contact mechanics — too-large inputs cause irreversible deformation or contact loss; (3) readout is limited by sensor count and placement options; (4) operation speed is orders of magnitude slower than photonic systems. These limitations confine granular reservoirs to research demonstrations and low-frequency robotics applications [Nakajima et al. 2014].

---

## References

- Nakajima, K., Hauser, H., Kang, R., Guglielmino, E., Caldwell, D. G., & Pfeifer, R. (2014). Exploiting the dynamics of soft materials for machine learning. *Soft Robotics*, 1(4), 266–275.
- Nesterenko, V. F. (2001). *Dynamics of Heterogeneous Materials*. Springer.
