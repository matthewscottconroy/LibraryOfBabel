# Multivariate Time Series with Reservoir Computing

## The Multivariate Setting

Many real-world time series are inherently multivariate: multiple correlated signals measured simultaneously, with cross-channel dependencies that carry information beyond what any single channel contains. Examples include multi-electrode EEG recordings, multi-variate climate indices (temperature, pressure, humidity), financial baskets (returns of correlated assets), and multi-joint robot kinematics.

In the reservoir computing framework, multivariate inputs are handled naturally by concatenating all channels into a single input vector $\mathbf{u}_t \in \mathbb{R}^M$, where $M$ is the number of channels. The reservoir maps this $M$-dimensional input stream into an $N$-dimensional state sequence, and the readout maps state sequences to outputs. No architectural changes are needed — the standard ESN framework handles multivariate inputs transparently [Jaeger 2001].

## Single Reservoir for Cross-Channel Mixing

A key advantage of the reservoir architecture for multivariate inputs is that the recurrent connections perform automatic cross-channel mixing. Consider two input channels $u_t^{(1)}$ and $u_t^{(2)}$ entering the reservoir through input weights $\mathbf{w}_1^{\text{in}}$ and $\mathbf{w}_2^{\text{in}}$. After one time step, the reservoir state is:

$$\mathbf{x}_t = \tanh(\mathbf{W}^{\text{rec}}\mathbf{x}_{t-1} + \mathbf{w}_1^{\text{in}} u_t^{(1)} + \mathbf{w}_2^{\text{in}} u_t^{(2)}).$$

The contribution of channel 1 to neuron $i$ is $(W_{ij}^{\text{in}}) u_t^{(1)}$, and similarly for channel 2. Through the recurrent dynamics, information from both channels is mixed across all neurons after multiple steps, creating features that depend on nonlinear interactions between channels [Jaeger 2001].

This cross-channel mixing is particularly valuable when the predictive information lies in cross-channel correlations or lagged cross-correlations — for example, when temperature at location A predicts pressure at location B two hours later. A single reservoir with joint input from both channels can capture this dependency through its recurrent dynamics, without any explicit cross-correlation preprocessing.

## Multiple-Output Readout

For multivariate prediction (predicting all $M$ channels simultaneously), the readout is:

$$\hat{\mathbf{y}}_t = \mathbf{W}^{\text{out}} \mathbf{x}_t, \quad \mathbf{W}^{\text{out}} \in \mathbb{R}^{M \times N}.$$

All $M$ readout vectors are trained simultaneously by multivariate ridge regression:

$$\hat{\mathbf{W}}^{\text{out}} = \mathbf{Y}^{*} \mathbf{X}^\top (\mathbf{X}\mathbf{X}^\top + \lambda \mathbf{I})^{-1},$$

where $\mathbf{Y}^* \in \mathbb{R}^{M \times T}$ is the target matrix and $\mathbf{X} \in \mathbb{R}^{N \times T}$ is the state matrix. The training cost is $O(N^3 + MTN)$, identical to $M$ independent ridge regressions.

## When Dimensionality Helps

Higher input dimensionality (more channels) is generally beneficial for reservoir computing when the channels carry complementary information. The reservoir state becomes richer as more channels are added: with $M$ input channels, the reservoir's attractor covers a higher-dimensional space, providing more diverse features for the readout.

Formally, the information processing capacity (IPC) of the reservoir is bounded by $N$ regardless of $M$ (since the state dimension is $N$). However, the IPC realized on a specific task generally increases with $M$ when additional channels provide novel information. The eigenspectrum of the cross-channel input correlation matrix $\boldsymbol{\Sigma}_{uu} = \mathbb{E}[\mathbf{u}_t\mathbf{u}_t^\top] \in \mathbb{R}^{M \times M}$ determines how many independent dimensions the input contributes: channels with high mutual correlation contribute less than channels with low correlation [Dambre et al. 2012].

## Echo-State Analysis for Multivariate Input

The echo state property for multivariate inputs requires the same condition as for scalar inputs: spectral radius $\rho(\mathbf{W}^{\text{rec}}) < 1$ for the standard sufficient condition. The multivariate input structure affects the transient behavior but not the ESP condition, since the ESP depends only on the contraction of the autonomous reservoir map $\mathbf{x} \mapsto \tanh(\mathbf{W}^{\text{rec}}\mathbf{x})$.

The eigenspectrum of $\boldsymbol{\Sigma}_{uu}$ does affect the effective timescale of the reservoir's response to multivariate inputs: channels with high variance dominate the input projection and drive the reservoir more strongly. Normalizing inputs to unit variance ensures all channels contribute equally to the reservoir dynamics [Dambre et al. 2012].

## Applications: Climate Variable Prediction and EEG

**Climate:** Multivariate climate prediction using ESNs processes simultaneous time series of temperature, pressure, humidity, wind speed, and oceanic variables (SST, sea ice extent). Cross-channel correlations (e.g., temperature and humidity correlate with future precipitation) are captured by the reservoir without explicit feature engineering.

**EEG multichannel processing:** EEG recordings from $M = 64$–$128$ electrodes provide spatially distributed neural activity. Reservoir-based brain-computer interface (BCI) applications use the full $M$-channel EEG as input, relying on the reservoir to mix spatial and temporal information. The spatial mixing through the random input weight matrix $\mathbf{W}^{\text{in}} \in \mathbb{R}^{N \times M}$ provides an initial dimensionality reduction from $M$ spatial channels to $N$ mixed features, after which the recurrent dynamics extract temporal structure.

---

## References

- Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*, German National Research Center for Information Technology.
- Dambre, J., Verstraeten, D., Schrauwen, B., & Massar, S. (2012). Information processing capacity of dynamical systems. *Scientific Reports*, 2(1), 514.
