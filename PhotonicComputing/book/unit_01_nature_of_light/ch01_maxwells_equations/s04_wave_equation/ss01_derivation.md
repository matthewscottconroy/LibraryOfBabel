# 1.4.1 Deriving the Wave Equation from Maxwell's Equations

## Setup: Free Space

In free space, there are no charges ($\rho = 0$) and no currents ($\mathbf{J} = 0$). Maxwell's equations in differential form simplify to:

$$\nabla \cdot \mathbf{E} = 0 \tag{1}$$
$$\nabla \cdot \mathbf{B} = 0 \tag{2}$$
$$\nabla \times \mathbf{E} = -\frac{\partial \mathbf{B}}{\partial t} \tag{3}$$
$$\nabla \times \mathbf{B} = \mu_0\varepsilon_0 \frac{\partial \mathbf{E}}{\partial t} \tag{4}$$

We want to find what field configurations are consistent with all four equations simultaneously.

## Step 1: Eliminate $\mathbf{B}$

Take the curl of equation (3):

$$\nabla \times (\nabla \times \mathbf{E}) = -\frac{\partial}{\partial t}(\nabla \times \mathbf{B})$$

On the right side, substitute equation (4):

$$\nabla \times (\nabla \times \mathbf{E}) = -\mu_0\varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial t^2}$$

## Step 2: Apply the Vector Identity

Use the vector identity:
$$\nabla \times (\nabla \times \mathbf{E}) = \nabla(\nabla \cdot \mathbf{E}) - \nabla^2 \mathbf{E}$$

Since $\nabla \cdot \mathbf{E} = 0$ in free space (equation 1), the first term vanishes:

$$-\nabla^2 \mathbf{E} = -\mu_0\varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial t^2}$$

## Step 3: The Wave Equation

Rearranging:

$$\boxed{\nabla^2 \mathbf{E} = \mu_0\varepsilon_0 \frac{\partial^2 \mathbf{E}}{\partial t^2}}$$

This is **the electromagnetic wave equation**. An identical equation holds for $\mathbf{B}$, derived by taking the curl of equation (4) instead.

## What the Wave Equation Says

The wave equation has the form:

$$\nabla^2 \mathbf{E} = \frac{1}{v^2}\frac{\partial^2 \mathbf{E}}{\partial t^2}$$

where $v = 1/\sqrt{\mu_0\varepsilon_0}$ is the wave speed. This is exactly the form of a wave equation in classical mechanics (for waves on a string, sound waves, etc.), but here the "wave" is the electromagnetic field itself propagating through free space.

## The Speed of Light

The wave speed is:
$$c = \frac{1}{\sqrt{\mu_0\varepsilon_0}}$$

Let us compute this. In SI units:
$$c = \frac{1}{\sqrt{(4\pi \times 10^{-7}\ \text{H/m})(8.854 \times 10^{-12}\ \text{F/m})}}$$

Working through the units:
$$[\mu_0\varepsilon_0] = \frac{\text{H}}{\text{m}} \cdot \frac{\text{F}}{\text{m}} = \frac{V\cdot s}{A\cdot m} \cdot \frac{A\cdot s}{V\cdot m} = \frac{s^2}{m^2}$$

(using $H = V\cdot s/A$ and $F = A\cdot s/V$), so $1/\sqrt{\mu_0\varepsilon_0}$ has units of m/s. Numerically:

$$c = \frac{1}{\sqrt{1.113 \times 10^{-17}\ \text{s}^2/\text{m}^2}} = \frac{1}{1.055 \times 10^{-9}\ \text{s/m}} \approx 2.998 \times 10^8\ \text{m/s}$$

This is the speed of light, measured by Rømer (from Jupiter's moons, 1676), by Fizeau (rotating gear method, 1849), and others. Maxwell knew this value. He recognized immediately what his calculation meant.

## Maxwell's Own Words

From the 1865 paper [1]:

> "The velocity of transverse undulations in our hypothetical medium, calculated from the electro-magnetic experiments of MM. Kohlrausch and Weber, agrees so exactly with the velocity of light calculated from the optical experiments of M. Fizeau, that we can scarcely avoid the inference that light consists in the transverse undulations of the same medium which is the cause of electric and magnetic phenomena."

The "medium" Maxwell refers to is what he and his contemporaries called the ether — a medium he believed must exist for waves to propagate through. Within decades, the Michelson-Morley experiment (1887) would definitively show that no such medium exists, and Einstein's special relativity (1905) would explain why: the electromagnetic field does not require a medium. But Maxwell's identification of light as electromagnetic waves was completely correct, and it stands as one of the greatest theoretical discoveries in the history of physics.

## The Wave Equation in a Dielectric Medium

In a linear dielectric with permittivity $\varepsilon = \varepsilon_r \varepsilon_0$ and permeability $\mu_0$ (for non-magnetic materials), the wave equation becomes:

$$\nabla^2 \mathbf{E} = \mu_0\varepsilon \frac{\partial^2 \mathbf{E}}{\partial t^2} = \frac{n^2}{c^2}\frac{\partial^2 \mathbf{E}}{\partial t^2}$$

where $n = \sqrt{\varepsilon_r}$ is the refractive index. The wave speed in the medium is $v = c/n$, and the wavelength at frequency $f$ is $\lambda = v/f = \lambda_0/n$, where $\lambda_0 = c/f$ is the free-space wavelength.

**Example**: 1550 nm light in a silicon waveguide ($n \approx 3.47$):
- Wavelength in silicon: $\lambda_{Si} = 1550/3.47 \approx 447$ nm
- The mode is physically smaller inside silicon, which is why silicon waveguides can be so compact — this physical compression is what enables the tight bends and dense integration of silicon photonic circuits.

---

## References

[1] Maxwell, J.C. (1865). "A dynamical theory of the electromagnetic field." *Philosophical Transactions of the Royal Society of London*, 155, 459–512.

[2] Born, M., & Wolf, E. (1999). *Principles of Optics*, 7th ed. Cambridge University Press. Ch. 1. [Standard reference for the derivation of the wave equation in isotropic media.]
