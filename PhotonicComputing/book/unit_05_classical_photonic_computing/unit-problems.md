# Unit V Problem Set: Classical Photonic Computing

*Problems on analog optical computing, Fourier optics processing, photonic matrix-vector multiplication, photonic neural networks, and diffractive neural networks. Chapters 11–14.*

---

## Chapter 11: Fourier Optics and Analog Optical Computing

**Problem 11.1** [Easy]
Fourier transform with a lens: a thin lens of focal length $f = 10$ cm performs a 2D Fourier transform. An input pattern $E(x,y)$ at the front focal plane maps to $\tilde{E}(u,v) \propto \mathcal{F}[E](u/\lambda f, v/\lambda f)$ at the back focal plane.

(a) A rectangular aperture of width $a = 1$ mm and height $b = 2$ mm is placed at the input plane, illuminated by a plane wave at $\lambda = 633$ nm. What is the Fourier pattern in the back focal plane? Give the positions of the first zeros in $u$ and $v$ (in mm).

(b) Two point sources separated by $d = 0.5$ mm produce a fringe pattern in the Fourier plane. Find the fringe period.

(c) A spatial filter (pinhole of diameter $D = 0.1$ mm) in the Fourier plane acts as a low-pass filter. What is the spatial frequency cutoff $f_\text{cutoff}$ (in lines/mm at the input plane)?

**Problem 11.2** [Medium]
Optical correlation: the matched filter for pattern recognition. A scene $g(x,y)$ is to be searched for a template $h(x,y)$.

(a) Show that the cross-correlation $g \star h = \int g(x',y')h(x'+x,y'+y)dx'dy'$ is equivalent to a convolution with the flipped template $h(-x,-y)$.

(b) The 4-f correlator computes this as: (FT of $g$) $\times$ (conjugate FT of $h$) $\to$ (inverse FT). Describe the optical implementation.

(c) The VanderLugt filter: $H^*(u,v)$ is recorded holographically in the Fourier plane. For an input $g(x,y) = h(x-x_0, y-y_0)$ (shifted template), where does the correlation peak appear?

(d) Noise robustness: if $g(x,y) = h(x,y) + n(x,y)$ where $n$ is white Gaussian noise with PSD $N_0$, the peak SNR of the correlation is $\text{SNR} = \|h\|^2/(N_0\cdot\text{area})$. For $\|h\|^2 = 100$ mW²·μm², $N_0 = 0.01$ mW²·μm²/μm², image area $1$ mm²: compute the SNR.

**Problem 11.3** [Medium]
Acousto-optic modulator (AOM): an acoustic wave of frequency $\Omega/(2\pi) = 100$ MHz propagates in a TeO₂ crystal at acoustic velocity $v_s = 4200$ m/s. The crystal creates a moving diffraction grating.

(a) Find the acoustic wavelength $\Lambda = v_s/f_s$.

(b) Bragg diffraction condition: $\sin\theta_B = \lambda/(2\Lambda)$ for light at $\lambda = 633$ nm. Find $\theta_B$.

(c) The diffracted beam is shifted in frequency by $\pm\Omega$ (Doppler shift). For a chirped acoustic signal $\Omega(t) = \Omega_0 + \dot{\Omega}t$: the diffracted beam has a time-varying angle. This is the principle of the acousto-optic scanner. What is the angular scan rate $d\theta_B/dt$?

(d) An AOM with bandwidth $B_\text{acoustic} = 100$ MHz can modulate light at up to 100 Msamples/s. What is the time-bandwidth product $T\cdot B$ for a pulse duration $T = 1$ μs?

---

## Chapter 12: Photonic Matrix-Vector Multiplication

**Problem 12.1** [Easy]
Outer product architecture: the outer product of vectors $\mathbf{u} \in \mathbb{R}^N$ and $\mathbf{v} \in \mathbb{R}^M$ is the matrix $W = \mathbf{u}\mathbf{v}^T$ with $W_{ij} = u_i v_j$.

(a) A rank-1 matrix can be implemented optically using a laser array (encoding $\mathbf{v}$) illuminating a waveguide array (encoding $\mathbf{u}$) with a lens. Describe the optical system.

(b) A general rank-$K$ matrix can be written $W = \sum_{k=1}^K \sigma_k \mathbf{u}_k \mathbf{v}_k^T$ (SVD). How many outer product modules are needed?

(c) For $W$ an $N\times N$ matrix with rank $K \ll N$: compare the number of optical elements needed for the outer-product architecture vs. the Clements MZI mesh. Which is more efficient for low-rank matrices?

**Problem 12.2** [Medium]
MZI mesh calibration: the target unitary $U$ must be decomposed into MZI settings $\{\theta_k, \phi_k\}$. Due to thermal drift, the actual MZI settings are $\{\theta_k + \delta\theta_k, \phi_k + \delta\phi_k\}$ where $\delta\theta_k \sim \mathcal{N}(0, \sigma^2)$.

(a) For $N = 8$ (64 MZIs in the Clements mesh), phase noise $\sigma = 0.02$ rad, and input vector $\mathbf{x}$ with $\|\mathbf{x}\|_2 = 1$: estimate the rms output error $\|\delta\mathbf{y}\|_2$.

(b) An in-situ calibration algorithm measures the output power of a known test input and adjusts all $\theta_k, \phi_k$ to minimize the error. For a gradient descent calibration with step size $\eta = 0.01$ rad/step: how many calibration steps are needed to reduce the error by 10×?

(c) Compare the calibration overhead (time in units of matrix-vector products) for: (i) calibrating every 1 second if $\tau_\text{drift} = 10$ s, vs. (ii) using electro-optic phase shifters that don't drift thermally.

**Problem 12.3** [Medium]
Energy efficiency of optical vs. digital MVM: compare a photonic accelerator to an NVIDIA A100 GPU.

(a) The A100 has peak throughput 312 TeraFLOPS (BF16) and thermal design power 400 W. Compute energy per FLOP.

(b) A photonic MVM system (Clements mesh, $N = 256$, 1 nm TFLN EO phase shifters) operates at 100 GHz modulation rate. The energy per symbol is $E_s = C V_\pi^2 / 2$ for capacitance $C = 10$ fF and $V_\pi = 1$ V. There are $N(N-1)/2$ phase shifters and each is updated for every matrix-vector product. Compute the total energy per MVM.

(c) The number of MACs per MVM is $N^2$. Compute energy per MAC and compare to the GPU.

(d) The photonic system has latency 10 ps (propagation through the chip) vs. the GPU's $\sim 10$ μs for the same $N = 256$ operation. For latency-sensitive applications (real-time inference at $10^9$ images/s), which system is preferred? What throughput is achievable with each?

**Problem 12.4** [Hard]
*Hint: Use the singular value decomposition of the weight matrix and analyze how weight quantization affects the singular values.*

Precision requirements for photonic neural networks: a fully-connected neural network layer has weight matrix $W$ of size $64\times64$. The weights are implemented on a photonic chip with finite phase precision: each MZI angle is quantized to $B$ bits (smallest step $\Delta\theta = \pi/2^B$).

(a) The quantization noise variance in each MZI angle is $\sigma_\text{quant}^2 = (\Delta\theta)^2/12$. For $B = 6$ bits, compute $\sigma_\text{quant}$.

(b) Using the noise propagation result $\langle\|\delta\mathbf{y}\|^2\rangle \approx K\sigma^2\|\mathbf{x}\|^2$ from the chapter: estimate the output error for $K = N(N-1)/2$ MZIs and $N = 64$.

(c) A ResNet-50 model achieves 76% top-1 accuracy on ImageNet with 32-bit float weights, and 74% with 6-bit quantization. Estimate the accuracy loss from the photonic phase quantization noise (assume the accuracy degrades proportionally to output error).

(d) Noise-aware training: if the network is retrained with the quantization noise modeled as additive Gaussian noise during training, the accuracy loss is typically halved. What minimum $B$ is needed to stay within 1% of the baseline accuracy?

---

## Chapter 13 & 14: Photonic and Diffractive Neural Networks

**Problem 13.1** [Easy]
Diffractive deep neural network (D²NN): a series of diffractive layers each implementing a complex transmission mask $t_m(x,y) = |t_m|\exp(i\phi_m(x,y))$.

(a) For a layer with transmission amplitude $|t_m| = 1$ (phase-only), what is the maximum number of independent control parameters per $1\times1$ mm² layer with $\lambda = 0.4$ mm (THz) and pixel pitch $= \lambda/2$?

(b) The diffraction between layers is computed by the Rayleigh-Sommerfeld diffraction integral. For a layer spacing $z = 5$ mm, $\lambda = 0.4$ mm: estimate the spatial resolution (minimum spot size) at the next layer.

(c) Compare the inference latency of a D²NN (speed of light propagation) to a GPU (electronic) for a 5-layer network.

**Problem 13.2** [Medium]
Reservoir computing with a photonic system: an optical reservoir uses a nonlinear node (semiconductor optical amplifier, SOA) with $N_\text{virtual} = 400$ virtual nodes, mask period $T_\theta = 25$ ps, input mask duration $T = N_\theta\cdot\tau = 10$ ns.

(a) What is the effective reservoir update rate in patterns/second?

(b) The readout layer is trained by ridge regression. For input dimension $N_\text{in} = 1$ (scalar time series) and reservoir size $N = 400$: the readout weight vector $\mathbf{w}_\text{out}$ has 400 elements. What is the training complexity?

(c) For NARMA-10 prediction (a standard benchmark), the normalized RMSE of a well-tuned photonic reservoir is $\approx 0.2$. Compare this to a digital LSTM of equivalent size.

**Problem 13.3** [Hard]
*Hint: The gradient of the loss with respect to weights in a photonic network must be computed using the transfer matrix formalism, and can be done via the adjoint method (backpropagation through the optical system).*

In-situ training of an MZI mesh: a photonic neural network has $L$ layers each implemented as an MZI Clements mesh. The forward pass $\mathbf{y} = f(W(\boldsymbol{\theta})\mathbf{x} + \mathbf{b})$ is computed optically. Training requires computing $\partial\mathcal{L}/\partial\theta_k$ for each MZI angle.

(a) The "direct feedback alignment" approach: compute gradients approximately using random fixed feedback matrices $B_l$ (not the transpose of $W_l$). Show that the weight update still reduces the loss in expectation (hint: argue from the inner product $\langle\nabla_W\mathcal{L}, \mathbf{x}\mathbf{e}^T B^T\rangle$).

(b) An in-situ method: for each parameter $\theta_k$, perturb it by $+\delta$ and $-\delta$, measure outputs, and estimate $\partial\mathcal{L}/\partial\theta_k \approx [\mathcal{L}(\theta_k+\delta) - \mathcal{L}(\theta_k-\delta)]/(2\delta)$. For $K = N(N-1)/2$ parameters and $N = 32$: how many forward passes are needed per gradient estimate? Compare to backpropagation.

(c) For a batch size of 32 images and 10,000 training steps: compare the total training time (in wall-clock seconds) for in-situ gradient estimation vs. digital training + weight transfer, given that one forward pass takes 10 ps (optical) vs. 1 μs (digital).
