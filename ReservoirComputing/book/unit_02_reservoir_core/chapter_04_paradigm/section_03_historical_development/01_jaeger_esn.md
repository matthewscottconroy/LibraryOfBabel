# 4.3.1 Herbert Jaeger and the Echo State Network

## The 2001 Technical Report

On August 2, 2001, Herbert Jaeger submitted a 48-page technical report to the internal publication series of the GMD — the German National Research Center for Information Technology in Bremen, Germany. The report was numbered GMD Report 148 and titled "The 'Echo State' Approach to Analysing and Training Recurrent Neural Networks." It was not, initially, published in a peer-reviewed journal. It was a technical report, the kind of document that circulates quietly in research communities before (and sometimes instead of) formal publication.

And yet this document launched a new field.

The report [Jaeger2001] introduced two ideas: the **echo state property** and the **echo state network**. The echo state property is a formal condition on a recurrent network that guarantees a certain kind of stability and input-driven behavior. The echo state network is a practical architecture built around this property: a large, randomly-connected recurrent network (the reservoir) with a trained linear readout.

What made the report remarkable — and what ensured that it would be read and built upon despite its non-journal status — was its combination of theoretical clarity, empirical demonstration, and engineering pragmatism. Jaeger did not just introduce the concept; he tested it, characterized it, and showed it working on non-trivial tasks.

## Jaeger's Background and Motivation

Herbert Jaeger came to the echo state idea through a combination of interests that, in retrospect, seems perfectly positioned for the discovery: he was interested in dynamical systems, in the theoretical foundations of neural computation, and in the practical problems of training recurrent networks.

Jaeger had been frustrated — as most researchers were — by the difficulty of training RNNs with gradient-based methods. The vanishing gradient problem was well known. LSTM, while a genuine advance, was complex and still required careful training. Jaeger was looking for something simpler.

His key insight, as he has described it in subsequent accounts [Jaeger2007interview], was the realization that the difficult part of recurrent network training — learning the recurrent weights — might be unnecessary. If you could simply *read out* what you needed from the network's natural dynamics, without having to teach the dynamics themselves, training would collapse to a simple linear regression problem.

The conceptual move is simple: instead of asking "what recurrent weights should the network have?", ask "given some fixed random recurrent weights, what linear readout extracts the desired output?"

## The Echo State Property

The central concept of the paper is the **echo state property** (ESP). Informally: a network has the echo state property if, for any input sequence, the state of the network converges to a unique trajectory that depends only on the input, not on the initial condition.

Formally: a driven recurrent network $\mathbf{x}_{t+1} = F(\mathbf{x}_t, \mathbf{u}_t)$ has the echo state property with respect to an input set $\mathcal{U}$ if there exists a null set $C \subset \mathbb{R}^N$ such that for any two initial conditions $\mathbf{a}, \mathbf{b} \notin C$ and any input sequence $(\mathbf{u}_t)_{t \leq 0}$:

$$\lim_{t \to -\infty} \|\mathbf{x}_t^{(\mathbf{a})} - \mathbf{x}_t^{(\mathbf{b})}\| = 0$$

where $\mathbf{x}_t^{(\mathbf{a})}$ denotes the state trajectory starting from initial condition $\mathbf{a}$.

The echo state property is the discrete-time analog of the "synchronization" property in coupled dynamical systems, and it is closely related to the concept of **fading memory**: if the network has the ESP, then its current state is entirely determined by the input history, not by the arbitrary initial condition. The initial condition "washes out."

**A necessary condition:** For the ESP to hold, it is necessary that $\rho(W^{rec}) < 1$ (for the linear approximation to the full nonlinear system). This is not sufficient in general — the full nonlinear system requires more careful analysis — but in practice, $\rho(W^{rec}) < 1$ is both necessary and nearly sufficient for well-behaved reservoirs with bounded nonlinearities like $\tanh$.

**A sufficient condition [Jaeger2001]:** The network has the echo state property if $\|W^{rec}\|_2 < 1$ (the spectral norm is less than 1). This is a stronger condition than $\rho(W^{rec}) < 1$, but easier to verify.

Jaeger showed that reservoirs with spectral radius slightly below 1 typically satisfy the ESP and produce rich, useful state trajectories, while reservoirs with spectral radius much below 1 produce states that decay too quickly (poor memory) and reservoirs above 1 can produce divergent behavior.

## The Echo State Network

Given the echo state property, the architecture follows naturally:

1. **Construct a large, sparse random recurrent matrix** $W^{rec}$ with spectral radius $\rho < 1$ (typically $\rho \approx 0.9$ to $0.99$).
2. **Construct a random input matrix** $W^{in}$ (no constraints beyond appropriate scaling).
3. **Run the network** on the input sequence, discarding the first $T_w$ steps (washout).
4. **Collect the states** $\mathbf{x}_{T_w+1}, \ldots, \mathbf{x}_{T_{train}}$ into a matrix.
5. **Solve the linear system**: find $W^{out}$ by least-squares to match the target outputs.

Steps 1–2 are performed once. Steps 3–5 are the "training," but only step 5 involves any parameter adjustment.

Jaeger named the network after its defining property: the network "echoes" the input, producing a state that is determined entirely by the history of the input signal. The state is an "echo" of what the network has seen.

## Empirical Demonstrations

The 2001 report contained several demonstrations that established the ESN's practical value.

**Signal generation:** Jaeger demonstrated that an ESN could learn to generate various periodic waveforms (sine waves of different frequencies, Lissajous figures) in autonomous (feedback) mode — running the output back into the input to produce a self-sustaining oscillation. The network learned a stable attractor whose output matched the target waveform.

**Nonlinear system identification:** An ESN was used to learn the inverse model of a nonlinear filter, demonstrating that the reservoir could handle nonlinear input-output relationships. The task required the network to learn a mapping with nonlinear temporal dependencies.

**Channel equalization:** The 2001 report showed that ESNs could learn to compensate for nonlinear channel distortion in a communications problem — a task with practical applications and a well-established benchmark.

These demonstrations were important not because they broke records (the tasks were chosen to be illustrative, not to maximize performance) but because they showed the approach working on qualitatively different problems, with minimal tuning.

## The 2004 *Science* Paper

The reservoir computing paradigm reached a much wider audience with Jaeger's 2004 *Science* paper "Harnessing Nonlinearity: Predicting Chaotic Systems and Saving Energy in Wireless Communication" [Jaeger2004], co-authored with Harald Haas. This paper demonstrated that an ESN could achieve state-of-the-art results on a difficult and well-known benchmark: predicting a chaotic dynamical system (the Mackey-Glass system) many steps into the future. The ESN's performance was, at the time, an order of magnitude better than any previous method — a striking result that attracted attention from researchers who might have dismissed the technical report.

The *Science* paper also demonstrated autonomous generation of speech-like sequences and introduced the "teacher forcing" training technique for generative tasks: during training, the target output is fed back into the network as if it were the network's own output, creating a closed loop that stabilizes the learning of attractors.

## Reception and Impact

The initial reception of Jaeger's work was mixed in the typical way of paradigm-shifting ideas: some researchers immediately grasped its significance, while others were skeptical. The skepticism centered on two points:

**"Aren't you just throwing away the most powerful part of the network?"** This objection misses the point: the fixed random weights are not a limitation, they are a different kind of representation. The power of the reservoir lies in its high-dimensional dynamics, not in the precise values of its weights.

**"Won't this fail on hard tasks?"** The fair response is: yes, on some tasks, a trained RNN will outperform a reservoir of comparable size. But the question is always about the tradeoff: training cost, generalization, stability, and the practical difficulty of gradient-based optimization. For many tasks, the reservoir's performance is competitive with or better than trained RNNs — and the training cost is vastly lower.

The impact of the ESN framework on subsequent research has been substantial. It provided a vocabulary (echo state property, spectral radius, washout), a practical toolkit (the training procedure is simple enough to implement in a dozen lines of code), and a theoretical framework (the connection between the ESP and fading memory) that structured the field. As of 2025, the GMD Report 148 has been cited over 4,000 times — an extraordinary impact for a technical report.

---

## References

- [Jaeger2001] Jaeger, H. (2001). The "echo state" approach to analysing and training recurrent neural networks. *GMD Report 148*, German National Research Center for Information Technology.
- [Jaeger2004] Jaeger, H., & Haas, H. (2004). Harnessing nonlinearity: Predicting chaotic systems and saving energy in wireless communication. *Science*, 304(5667), 78–80.
- [Jaeger2007interview] Jaeger, H. (2007). Echo state network. *Scholarpedia*, 2(9), 2330.
- [Lukoševičius2012] Lukoševičius, M., & Jaeger, H. (2009). Reservoir computing approaches to recurrent neural network training. *Computer Science Review*, 3(3), 127–149.
