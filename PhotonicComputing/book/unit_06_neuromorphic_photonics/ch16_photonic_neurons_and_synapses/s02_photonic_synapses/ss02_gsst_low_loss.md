# 16.2.2 GSST: A Low-Loss Phase-Change Material

The absorption contrast that makes GST such a convenient synapse is, at the network scale, also its greatest liability. A synapse that stores its weight by *absorbing* light necessarily throws away optical power, and in a network signals must pass through many synapses in series. This section explains why GST's crystalline loss caps the depth of a photonic network, and how a selenium-alloyed phase-change material, **GSST**, largely removes the cap.

## The cascadability problem with GST

Recall from §16.2.1 that in GST the crystalline phase is strongly absorbing at 1550 nm; a cell programmed to a large crystalline fraction attenuates the guided mode significantly. Two consequences follow for a network:

1. **Cumulative loss.** Decibels add along a path. If each synapse in a cascade removes even a fraction of a decibel to a few decibels, then after a few tens of cells the signal is exhausted and can no longer trigger a downstream neuron — the useful depth of the network is bounded by an optical power budget, not by the logic.
2. **Weight–loss entanglement.** Because absorption *is* the weighting mechanism, a small weight (highly crystalline, absorbing) and a large weight (amorphous, transparent) impose very different insertion losses. The total path loss then depends on the *pattern* of weights, which makes power budgeting and gain provisioning awkward.

For a single memory cell, high contrast is a virtue; for a deeply cascaded synaptic fabric, it is the dominant constraint.

## Enter GSST

Zhang et al. (2019) addressed this by alloying selenium into the Ge–Sb–Te system to obtain GSST, $\mathrm{Ge_2Sb_2Se_4Te_1}$. The alloy exhibits **broadband transparency**: its extinction coefficient $k$ at 1550 nm is drastically lower than GST's in **both** the amorphous and the crystalline states. Crucially, GSST retains a large *real*-index contrast between the two phases — a refractive-index change $\Delta n$ on the order of ~2 — even though its *imaginary*-index (loss) contrast is small.

The physical consequence is a shift in the modulation mechanism. GST modulates primarily the imaginary part of $\tilde n = n + ik$, so it is an **absorption-dominant** device: switching changes how much light is *lost*. GSST modulates primarily the real part $n$, so it is a **phase-dominant** device: switching changes the optical path length — the *phase* light accumulates — with little accompanying loss. A phase-only weight can, in principle, be nearly lossless, and it is naturally deployed inside interferometers (Mach–Zehnder arms) or resonators (microrings), where a low-loss phase shift is converted to an amplitude weight while preserving extinction ratio and finesse.

Because the residual loss of a GSST cell is small and, being phase-based, largely *independent of the programmed weight*, both problems above are relaxed at once: cascades can run far deeper, and the path-loss budget no longer depends on the weight pattern.

| Property | GST ($\mathrm{Ge_2Sb_2Te_5}$) | GSST ($\mathrm{Ge_2Sb_2Se_4Te_1}$) |
|---|---|---|
| Extinction coefficient $k$ at 1550 nm | low (amorphous), high (crystalline) | low in **both** states |
| Dominant modulation | absorption (imaginary index) | phase (real index), $\Delta n \sim 2$ |
| Loss coupled to weight value? | yes (weight–loss entangled) | largely no (phase-dominant) |
| Cascadability | limited (few cells) | deep cascades |
| Non-volatile | yes | yes |

## Worked Example: cumulative loss of a synaptic cascade

*Take representative per-cell insertion losses in a mid-to-high-weight (partly crystalline) state: $\ell_\mathrm{GST} \approx 1.5$ dB/cell and $\ell_\mathrm{GSST} \approx 0.05$ dB/cell. How many synapses can be cascaded, and what is the transmission of a 20-synapse chain, for each material?*

Because losses in decibels add along a series path, the total loss of $N$ identical cells is $\ell_\text{total} = N\,\ell_\text{cell}$. Suppose we can tolerate a $6$ dB budget for the synaptic path — the point at which only $10^{-6/10} = 25\%$ of the input power survives. The maximum cascade depth is then
$$N_\text{max} = \frac{6\ \text{dB}}{\ell_\text{cell}}.$$
For the two materials:
$$N_\text{max}^\mathrm{GST} = \frac{6}{1.5} = 4\ \text{cells}, \qquad N_\text{max}^\mathrm{GSST} = \frac{6}{0.05} = 120\ \text{cells}.$$

Now compare a concrete chain of $N = 20$ synapses. The total loss and surviving transmission $T = 10^{-\ell_\text{total}/10}$ are

- **GST:** $\ell_\text{total} = 20 \times 1.5 = 30$ dB $\Rightarrow T = 10^{-3} = 0.001$. Only one part in a thousand of the light survives — the signal is effectively gone.
- **GSST:** $\ell_\text{total} = 20 \times 0.05 = 1.0$ dB $\Rightarrow T = 10^{-0.1} = 0.79$. Nearly $80\%$ of the light survives.

Under identical assumptions GSST therefore supports cascades roughly $30\times$ deeper than GST. The per-cell figures here are illustrative — actual values depend on cell length, evanescent overlap, and the programmed crystalline fraction, with the worst case being a highly crystalline (low-weight) GST cell — but the scaling is the essential point: an absorption-dominant weight cannot be cascaded far, whereas a phase-dominant, low-loss weight can, which is precisely what a many-layer photonic synaptic network demands.

---

## References

- Zhang, Y., Chou, J.B., Li, J. et al. (2019). "Broadband transparent optical phase change materials for high-performance nonvolatile photonics." *Nature Communications*, 10, 4279.
- Ríos, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732.
