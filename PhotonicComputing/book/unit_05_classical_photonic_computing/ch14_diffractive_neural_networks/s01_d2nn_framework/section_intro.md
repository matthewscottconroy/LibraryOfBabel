# Section 14.1: The D2NN Framework

## What This Section Is About

A diffractive deep neural network turns a familiar abstraction — a trained cascade of linear layers — into a piece of shaped matter. This section builds that abstraction from the physics up and shows exactly where the "neural network" lives in a stack of transparent plates.

**14.1.1 — The physical principle.** Each neuron is a single pixel on a thin layer that imprints a trainable phase (and optionally amplitude) on the light passing through it. Free-space diffraction does the wiring: every neuron radiates as a Huygens secondary source, so its light reaches *every* neuron of the next layer. The all-to-all fan-in that an electronic layer must route explicitly is here supplied, for free, by propagation.

**14.1.2 — The mathematical model.** Rayleigh–Sommerfeld propagation between layers is a dense linear operator; the layer update is a convolution by the propagation kernel followed by a diagonal modulation — a fully-connected layer with *structured* weights. We give the operator form, the fast angular-spectrum (FFT) computation, and the far-field link to the Fourier optics of Chapter 11.

**14.1.3 — Training.** With the diffraction model made differentiable, the trainable parameters are the per-pixel phases. A softmax over the integrated intensities of the output detector regions supplies the loss, and backpropagation runs through the propagator — the simulated twin of the physical in-situ backpropagation of Subsection 13.3.2.

**14.1.4 — The founding experiment.** Lin et al.'s 2018 terahertz network — five 3D-printed layers, roughly 0.2 million neurons, MNIST classified at the speed of light — made the framework physical and set the agenda for the rest of the chapter.

The organizing point to carry through all four: a D2NN is a physical deep *linear* network whose weights are the propagation-plus-modulation operators, with nonlinearity supplied, at minimum, by intensity detection at the output plane.
