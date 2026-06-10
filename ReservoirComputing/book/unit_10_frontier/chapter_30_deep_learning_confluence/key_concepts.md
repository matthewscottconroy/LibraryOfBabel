# Chapter 30: Key Concepts

**State Space Model (SSM).** A linear dynamical system $\dot{x} = Ax + Bu$, $y = Cx + Du$ with learned parameters. The continuous-time linear reservoir. Discretized with step size $\Delta$ to give $x_k = \bar{A}x_{k-1} + \bar{B}u_k$, $y_k = Cx_k$, which is exactly a linear ESN.

**S4 (Structured State Space Sequence Model).** [GuGoel2022] A deep learning sequence model based on the state space framework, with the system matrix $A$ initialized using the HiPPO-LegS scheme and parameterized with DPLR structure for computational efficiency. Combines principled fading-memory initialization with end-to-end gradient training.

**Mamba.** [GuDao2023] An extension of S4 where the state space parameters $B$, $C$, and $\Delta$ are input-dependent (computed from the current input by learned projections). Makes the SSM selective: the model can dynamically allocate memory to relevant context. The continuous-time analogue of the Gated Recurrent Unit.

**HiPPO (High-Order Polynomial Projection Operators).** [GuHasani2020] A framework for deriving optimal online polynomial approximation of input histories. The HiPPO-LegS operator compresses the input history $u_{[0,t]}$ into the Legendre polynomial coefficients, and the compression evolves as a linear ODE with specific matrices $A, B$.

**HiPPO-LegS Matrix.** The specific lower-triangular matrix $A$ and vector $B$ such that $\dot{c} = Ac + Bu$ maintains the Legendre polynomial coefficients of the scaled history $u_{[0,t]}$ with sliding window measure. Provides the ideal fading memory initialization for SSMs.

**Zero-Order Hold (ZOH) Discretization.** A method for converting a continuous-time SSM to discrete time: $\bar{A} = e^{\Delta A}$, $\bar{B} = A^{-1}(e^{\Delta A} - I)B$. Exact for piecewise constant inputs; widely used in S4 and related models.

**Convolution View of SSM.** The discrete SSM output can be written as $y = \bar{K} * u$ where $\bar{K}_j = C\bar{A}^{j-1}\bar{B}$ is the impulse response. This convolution can be computed in $O(L \log L)$ via FFT, providing a parallelizable alternative to the sequential recurrence.

**DPLR Structure.** Diagonal Plus Low-Rank: $A = \Lambda - PQ^*$ where $\Lambda$ is diagonal and $P, Q \in \mathbb{R}^{N \times r}$ with $r \ll N$. The DPLR structure enables efficient computation of the S4 convolution kernel via Cauchy matrix evaluations.

**Liquid Neural Network (LNN).** [HasaniLechner2021] A continuous-time RNN where the effective time constants of neurons adapt to the current input: $\dot{x} = -x/\tau_{\text{eff}}(x,u) + \text{input-dependent term}$. Implements adaptive fading memory: the memory horizon is determined by input content.

**Liquid Time Constant.** $\tau_{\text{eff}} = \tau / (1 + \tau \cdot g(x,u))$ where $g(x,u)$ is an input-modulated gate. When the input is strong ($g$ large), $\tau_{\text{eff}}$ is small (fast dynamics, short memory). When the input is weak ($g$ small), $\tau_{\text{eff}} \approx \tau$ (slow dynamics, long memory). The neuron dynamically adjusts its timescale.

**Closed-Form Continuous-Time (CfC) Network.** [HasaniLechner2022] An approximation to the LNN that avoids numerical ODE integration by treating the gate $g$ as constant over each time step, yielding a closed-form update. Produces an update rule resembling a Gated Recurrent Unit with biologically motivated gating.

**Linear Probing.** Using a frozen pretrained model (e.g., an LLM) as a feature extractor and training a linear readout on the extracted features. Equivalent to reservoir computing with a deep network as the reservoir. Often achieves strong performance with very few trainable parameters.

**KV Cache.** In transformer language models, the key-value cache stores past token representations for efficient autoregressive generation. Grows linearly with context length, creating a memory bottleneck. Reservoir states provide a fixed-size alternative: compress the far-past context into a reservoir state, use full attention only for the recent context.

**Recurrent Memory Transformer.** A hybrid architecture combining a recurrent cell (analogous to a reservoir) for compressing long-range context with a transformer block for processing recent context. Examples include Recurrent Memory Transformer [BulatovKuratov2022] and Griffin [DeHoog2024].

**Long Range Arena.** [TayDehghani2021] A benchmark for evaluating sequence models on tasks requiring long-range dependencies, including classification of byte-level text, image recognition at the pixel level, and mathematical language understanding. Standard benchmark for evaluating S4, Mamba, and related models.
