# Subsection 14.4.3: The Non-Negativity Constraint

## Orientation

The last and most fundamental limit is set by the readout. A photodetector responds to optical power, not field: it measures $|U|^2 \ge 0$. The output of a diffractive network is therefore intrinsically non-negative, and any detection placed *between* layers would additionally destroy the phase that carries the computation. This single fact explains three things at once — why a passive D2NN can classify at all despite being almost entirely linear, why its outputs cannot natively represent signed quantities, and why the standard remedy, class-specific differential detection, works. We take them in turn.

## 14.4.3.1 The Complex-Linear Interior and the $|\cdot|^2$ Nonlinearity

In a passive diffractive stack the field remains complex from input to output; no photodetector intervenes until the final plane. As shown in Subsection 14.4.1, the interior is therefore one complex-linear operator $M = H_d\,\mathrm{diag}(t^L)\cdots H_d\,\mathrm{diag}(t^1)$ acting on the input field. The *only* nonlinearity in the entire network is the terminal magnitude-squared imposed by the detectors, $I = |M\,U^{\text{in}}|^2$.

This is simultaneously the reason the architecture needs no optical nonlinearity — Lin et al. (2018) built a working classifier out of nothing but phase plates and free space, because $|\cdot|^2$ supplies the required nonlinear decision surface — and the reason its capacity is bounded. A complex-linear map followed by intensity detection is, in machine-learning terms, close to a complex linear classifier with a quadratic readout: powerful enough to separate many classes, but far from a deep nonlinear network. Kulce et al. (2021) quantify precisely how much a cascade of such surfaces can compute. The absence of inter-layer nonlinearity is not an oversight to be corrected but the defining property that makes the network passive — and the source of its expressivity ceiling.

## 14.4.3.2 The Loss of the Sign Axis

The complex-linear interior notwithstanding, negative effective weights are perfectly available *inside* the network: destructive interference subtracts amplitudes, so the effective transform $M$ has complex — hence signed — entries realized by phase, not by any negative intensity. The problem is not the computation but its *readout*. Whatever signed, complex quantity the field encodes at the output plane, the detector collapses it to a non-negative real number $I=|U|^2$. A task whose natural output is signed — a regression onto $[-1,1]$, a difference of two evidence terms, a decision variable that should be free to go negative — cannot be represented directly on a single intensity region.

Classification hides this by convention: assign each class a spatial region and pick the region with the most energy (an argmax over non-negative scores). It works, but it wastes the sign axis and, as Li et al. (2019) showed, leaves accuracy on the table, because the network can only accumulate evidence *for* a class, never subtract evidence against it.

## 14.4.3.3 Differential Detection and Complex-Field Readout

The clean fix restores the sign by subtraction. **Class-specific differential detection** (Li et al. 2019) assigns each class *two* detector regions, a positive and a negative, and defines the class score as their difference,

$$s_c = I_c^{+} - I_c^{-} = |U_c^{+}|^2 - |U_c^{-}|^2.$$

Each $I \ge 0$, but their difference spans the whole real line: the network can now learn to route energy into $D_c^-$ to *penalize* a class, and the decision $\hat c = \arg\max_c s_c$ operates on genuinely signed evidence. Li et al. demonstrated that this improves blind-testing accuracy across the MNIST, Fashion-MNIST, and CIFAR-10 image datasets, at the modest cost of doubling the number of output regions (and detectors) from $C$ to $2C$.

**Example (two-class differential readout).** For a binary task, place two detectors $D^+$ and $D^-$ and form the single decision variable

$$y = I_{+} - I_{-}.$$

Suppose an input of class A produces $(I_+, I_-) = (0.7,\,0.2)$ in normalized units and an input of class B produces $(0.2,\,0.6)$. The raw intensities are non-negative and, taken singly, ambiguous; the differences are $y_A = +0.5$ and $y_B = -0.4$. A single signed threshold at $y=0$ separates them — the sign of $y$ *is* the class. No single detector could have produced a negative decision value; the pair recovers the full real axis at the price of a second photodiode. This is exactly the balanced-detection trick of Chapter 12, where a microring's drop and through ports feed a balanced photodiode pair to synthesize a signed weight $w\in[-1,1]$ from two non-negative transmissions. Differential detection is that same idea moved from the weights to the network output.

Two further routes exist. Keeping the computation **all-optical and complex until a single final detection** — never detecting between layers — preserves the phase information the interior needs, and is the default in every passive D2NN. And **phase-sensitive (homodyne) detection**, in which the output field is beat against a coherent reference, recovers the full complex field — sign and phase together — at the cost of supplying and stabilizing that reference (Chapter 12). Each route buys back part of what the bare $|\cdot|^2$ discards; none is free.

## References

[1] Li, J., Mengu, D., Luo, Y., Rivenson, Y., & Ozcan, A. (2019). "Class-specific differential detection in diffractive optical neural networks improves inference accuracy." *Advanced Photonics*, 1(4), 046001. [The differential-detection method: paired positive and negative detector regions restore signed class scores; the central source of §14.4.3.3.]

[2] Lin, X., Rivenson, Y., Yardimci, N.T., Veli, M., Luo, Y., Jarrahi, M., & Ozcan, A. (2018). "All-optical machine learning using diffractive deep neural networks." *Science*, 361(6406), 1004–1008. [Establishes that the terminal $|\cdot|^2$ readout suffices to classify with an otherwise linear, passive optical stack.]

[3] Kulce, O., Mengu, D., Rivenson, Y., & Ozcan, A. (2021). "All-optical information-processing capacity of diffractive surfaces." *Light: Science & Applications*, 10, 25. [Quantifies how much a cascade of complex-linear diffractive surfaces can compute before the terminal nonlinearity — the capacity context for the expressivity ceiling.]
