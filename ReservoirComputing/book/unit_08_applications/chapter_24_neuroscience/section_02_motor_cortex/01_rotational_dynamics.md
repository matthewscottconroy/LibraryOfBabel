# 24.2.1 Motor Cortex Rotational Dynamics and FORCE Learning

## Epistemic Status of This Section

- **Established fact:** The motor cortex contains neurons that fire in complex, non-monotonic patterns during movement preparation and execution. Neural recordings during reaching movements show that motor cortex population activity follows smooth, approximately low-dimensional trajectories in state space.
- **Computational finding (Churchland et al. 2012):** The dominant structure in motor cortex population dynamics during arm movements is a **rotational pattern** — the trajectories rotate in the neural state space with frequencies correlated to movement speed. This is a data analysis finding, replicated in multiple animals and experimental conditions.
- **Theoretical interpretation:** The interpretation of rotational dynamics as evidence that motor cortex functions as an oscillatory reservoir (or "dynamical system") is theoretical. Alternative explanations exist, including that the rotational structure arises from the geometry of muscle synergies rather than from intrinsic cortical dynamics.
- **Computational model (Sussillo & Abbott 2009):** FORCE learning applied to a randomly initialized RNN can generate outputs similar to EMG patterns during reaching. This is a demonstration that random recurrent networks *can* produce arm movements through trained readouts — it does not prove that the motor cortex *does* use this mechanism.

## Churchland et al. 2012: The Rotational Structure

Churchland, Cunningham, Kaufman, Foster, Nuyujukian, Ryu, and Shenoy published "Neural population dynamics during reaching" in *Nature* in 2012 [Churchland2012]. We describe what the paper found and what it claimed.

**The experiment.** The authors recorded from approximately 200 neurons simultaneously in primary motor cortex (M1) and premotor cortex (PMd) of two macaque monkeys performing center-out reaching movements to 27 target locations. Recordings were obtained using Utah arrays (silicon electrode grids implanted in cortex).

**The data analysis.** The population neural activity at each time point is a vector $\mathbf{r}(t) \in \mathbb{R}^{200}$ (200 neurons' firing rates). The authors applied **jPCA** (joint PCA, a method they developed), which finds the linear subspace of highest-variance *rotational* dynamics. The method maximizes the variance captured by a pair of axes that look most like a rotation: $\dot{\mathbf{r}} \approx M \mathbf{r}$ where $M$ is a skew-symmetric matrix.

**The finding.** In the jPCA subspace, the motor cortex population trajectories for different target conditions rotate in the plane, with periods of approximately 100–200 ms corresponding to the movement speed. The rotational structure accounted for a substantial fraction of the variance in the low-dimensional projection (Supplementary Figure 3 of [Churchland2012] shows the R² values).

This rotational structure is the **established data finding**. It has been replicated in subsequent recordings [Gallego2017], including in human cortex [Ames2019].

**The interpretation.** Churchland et al. interpret the rotational dynamics as evidence that motor cortex functions as a **pattern generator**: the rotating trajectories are proposed to be the output of a dynamical system whose internal oscillatory dynamics, modulated by the preparatory state, generate the time-varying commands to muscles.

This interpretation is *one account* of the data. **Alternative interpretations include:**
1. The rotational structure reflects the geometry of the limb's mechanical dynamics, projected back into neural space through the motor periphery [Kiehn2016 analogy].
2. Rotation arises from the competition between preparatory states for different movements, without representing oscillatory dynamics per se [Kaufman2014].
3. The rotational structure is a statistical artifact of how population activity is analyzed, rather than a fundamental computational feature [Elsayed2016].

The data *are* consistent with the reservoir/dynamical systems interpretation. Whether they *prove* it is a different question.

## Connecting to Reservoir Computing: The Pathway

The connection from rotational dynamics to reservoir computing runs through FORCE learning (Chapter 11).

**Sussillo and Abbott 2009 [Sussillo2009].** Sussillo and Abbott showed that a randomly initialized RNN, trained by FORCE learning to generate a target output signal (e.g., a time-varying motor command), develops population dynamics that resemble the rotational structure seen in motor cortex. This is a **computational demonstration**: the model produces dynamics consistent with the data.

The Sussillo-Abbott model:
1. Starts with a random recurrent network of $N = 1000$ neurons with $g > 1$ (chaotic gain regime)
2. Trains a readout $W^{out}$ by FORCE learning to produce a target EMG (electromyogram) pattern
3. After training, the network's population activity shows complex, approximately low-dimensional trajectories
4. The dominant structure in these trajectories is rotational — matching the Churchland et al. finding

This correspondence is **suggestive** that motor cortex might use a similar mechanism. However:
- The model is not uniquely consistent with the data: many other models also produce rotational dynamics
- The FORCE learning rule (Chapter 11) is not known to have a biological implementation
- The model's parameters are not fit to match motor cortex physiology

**Sussillo et al. 2015 [Sussillo2015].** In a follow-up paper in *Nature Neuroscience*, Sussillo, Churchland, Kaufman, and Shenoy trained a more constrained RNN to generate actual (not stylized) muscle activity during reaching, and compared the model's internal dynamics to the neural recordings. They found that:
1. The trained model's population dynamics match the recorded motor cortex dynamics in the jPCA subspace
2. The model's dynamical structure predicts the structure of the neural data at a quantitative level

This is a stronger result: not just "the model produces rotation" but "the model's specific rotational structure matches the data's structure." The paper concludes that **the data are consistent with a dynamical systems account of motor control** in which preparatory activity sets an initial condition and motor cortex generates movements through its trained internal dynamics. This remains an interpretation, not a proof — but it is the most quantitatively supported interpretation currently available.

**The reservoir connection:** The Sussillo-Abbott model is a trained RNN, not a reservoir computer. The connection to RC is that:
1. Before training, the random RNN is effectively a reservoir (echo state in the chaotic regime, though at $g > 1$, without full fading memory)
2. FORCE learning trains only the readout weights (Chapter 11), leaving the recurrent connections intact — this is structurally identical to reservoir training
3. The resulting computation (dynamical basis generation) is the same computation that a reservoir computer performs when generating time series by autonomous dynamics

## What Genuine Reservoir Computing Adds

The reservoir computing framework provides a language for understanding motor cortex dynamics that is absent from the raw data analysis:

1. **The separation principle.** Different preparatory states correspond to different initial conditions in the reservoir state space. The rotational dynamics that follow are the reservoir's autonomous evolution from those initial conditions.

2. **The readout principle.** The muscle commands are linear readouts of the reservoir state. The FORCE-trained readout is the learned connection from motor cortex to spinal motor neurons.

3. **The generalization principle.** The reservoir can generate movements to novel targets by interpolating between learned states. This predicts that motor cortex activity for an intermediate target should be an approximately linear interpolation of the activity for nearby trained targets — a prediction consistent with the data [Churchland2012].

---

## References

- [Churchland2012] Churchland, M.M., Cunningham, J.P., Kaufman, M.T., Foster, J.D., Nuyujukian, P., Ryu, S.I., & Shenoy, K.V. (2012). Neural population dynamics during reaching. *Nature*, 487(7405), 51–56.
- [Sussillo2009] Sussillo, D. & Abbott, L.F. (2009). Generating coherent patterns of activity from chaotic neural networks. *Neuron*, 63(4), 544–557.
- [Sussillo2015] Sussillo, D., Churchland, M.M., Kaufman, M.T., & Shenoy, K.V. (2015). A neural network that finds a naturalistic solution for the production of muscle activity. *Nature Neuroscience*, 18(7), 1025–1033.
- [Gallego2017] Gallego, J.A., Perich, M.G., Miller, L.E., & Solla, S.A. (2017). Neural manifolds for the control of movement. *Neuron*, 94(5), 978–984.
- [Kaufman2014] Kaufman, M.T., Churchland, M.M., Ryu, S.I., & Shenoy, K.V. (2014). Cortical activity in the null space: Permitting preparation without movement. *Nature Neuroscience*, 17(3), 440–448.
