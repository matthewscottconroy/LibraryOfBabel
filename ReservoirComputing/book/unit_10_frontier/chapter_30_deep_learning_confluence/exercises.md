# Chapter 30 Exercises

**Exercise 30.1.** *(S4 as ESN)* Implement a single S4 layer in Python (using PyTorch) with the HiPPO-LegS initialization. Freeze $A$ and $B$ (as in a reservoir) and train only $C$ and $D$ by linear regression on the NARMA-10 task. Compare the performance to: (a) a random reservoir with the same state dimension, (b) an S4 layer with all parameters trained end-to-end.

**Exercise 30.2.** *(HiPPO impulse response)* Compute the impulse response $\bar{K}_j = C\bar{A}^j\bar{B}$ for the HiPPO-LegS S4 layer with $N = 64$, $\Delta = 0.01$. Plot $\bar{K}_j$ as a function of $j$. How does it compare to the impulse response of a random ESN with the same state dimension and the same target spectral radius?

**Exercise 30.3.** *(Convolution perspective)* Show that the discrete SSM with parameters $(\bar{A}, \bar{B}, C)$ and input sequence $(u_1, \ldots, u_L)$ computes the output sequence $(y_1, \ldots, y_L)$ as a convolution $y = \bar{K} * u$ where $\bar{K}_j = C\bar{A}^{j-1}\bar{B}$. Implement this convolution using FFT and verify it gives the same result as the recurrence $x_k = \bar{A}x_{k-1} + \bar{B}u_k$, $y_k = Cx_k$.

**Exercise 30.4.** *(Liquid time constants)* For the scalar LNN equation $\dot{x} = -(1/\tau + \sigma(wx + w_{\text{in}}u + b)) x + A\sigma(wx + w_{\text{in}}u + b)$, find all fixed points as a function of $u$ (treating $u$ as constant). How does the number of fixed points depend on the parameters? For what parameter values does the system have a unique stable fixed point for all constant inputs?

**Exercise 30.5.** *(CfC derivation)* Derive the CfC update equation from the exact ODE solution by treating the gate $g_k = \sigma(w^\top x(t_k) + w_{\text{in}}u(t_k) + b)$ as constant over the interval $[t_k, t_{k+1}]$. Show that the resulting update has the form of a gated recurrent unit. What approximation error does this introduce?

**Exercise 30.6.** *(Hybrid architecture design)* You are given the task of classifying long biomedical time series (electroencephalography recordings, length $T = 10,000$ samples, sampling rate 256 Hz) into two classes (seizure vs. non-seizure). Design a principled hybrid architecture combining reservoir computing and attention, following the design principles of Section 30.6.5. Specify:
(a) What component handles what timescale?
(b) How large is each component?
(c) What is trained and what is frozen?
(d) How is the final classification decision made?

**Exercise 30.7.** *(Foundation model as reservoir)* Take a pretrained BERT-base model (frozen) as a reservoir. For each input sequence, extract the final hidden state $h \in \mathbb{R}^{768}$ as the reservoir state. Train a linear readout for sentiment classification on the SST-2 dataset by ridge regression. Compare the accuracy to: (a) a random ESN with 768 units, (b) full fine-tuning of BERT-base. Discuss the accuracy/efficiency tradeoff.

**Exercise 30.8.** *(Mamba vs. ESN for long-range dependencies)* On the Long Range Arena benchmark [TayDehghani2021], compare: (a) a standard ESN with spectral radius 0.99, (b) a Mamba model with the same state dimension. For which tasks does Mamba outperform ESN? Can you identify a theoretical reason based on the selective state space mechanism?

**Exercise 30.9.** *(Research problem)* The HiPPO framework provides optimal polynomial approximation of the input history. However, polynomial approximation may not be optimal for all tasks. Design an alternative initialization framework that optimizes for a different approximation criterion — for example, optimal representation of periodic inputs (relevant for audio processing) or optimal representation of inputs with known power spectral density. Derive the corresponding ODE matrices and compare empirically to HiPPO.
