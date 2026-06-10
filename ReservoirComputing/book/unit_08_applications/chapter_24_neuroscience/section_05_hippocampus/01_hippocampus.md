# Hippocampus and Temporal Sequence Memory

## Hippocampal Function: Episodic Memory and Navigation

The hippocampus plays a central role in episodic memory (memory for personally experienced events), spatial navigation, and temporal sequence learning [Squire 1992]. Its necessity for long-term memory formation is established by the profound anterograde amnesia following bilateral hippocampal lesion (as in patient H.M.) and by functional MRI evidence of hippocampal activation during memory encoding and retrieval.

The hippocampus forms part of the medial temporal lobe (MTL) memory system, which includes the entorhinal cortex (EC), perirhinal cortex, and parahippocampal cortex. The canonical flow of information is: sensory association cortex → EC → dentate gyrus (DG) → CA3 → CA1 → subiculum → EC. The EC provides the primary input and output gateway; DG and CA3 form the internal reservoirs.

## Place Cells and Grid Cells: Spatial Coding

O'Keefe & Dostrovsky [1971] discovered that hippocampal CA1 neurons ("place cells") fire preferentially when the animal occupies specific locations in an environment — the cell's "place field." Place cells collectively form a map of the environment: the population activity vector $\mathbf{r}(x, y) = [r_1(x,y), \ldots, r_N(x,y)]^\top$ at position $(x, y)$ uniquely encodes location, and different environments are represented by different ensembles of active cells.

Grid cells in the entorhinal cortex [Moser et al. 2008] fire in periodic, hexagonal patterns across the environment and are thought to provide the metric coordinate system that place cells use for navigation.

## Time Cells: Temporal Coding

MacDonald et al. [2011] discovered hippocampal neurons that fire at specific time points during a fixed-duration delay interval — "time cells." During a delayed-nonmatch-to-sample task, neurons ramp up activity, reach a peak at a specific delay time, then decline, with different cells peaking at different times. Together, time cells form a sequential "temporal barcode": the population state at delay time $t$ uniquely encodes elapsed time.

This temporal barcode is directly analogous to the reservoir state sequence: the hippocampal CA1 population at time $t$ during the delay is a high-dimensional representation of the time elapsed since the memory-encoding event, which is a computable function of the input history (the event and subsequent time). A linear readout of this representation can recover elapsed time and associated events — exactly the temporal integration function required for episodic memory [MacDonald et al. 2011].

## Reservoir Model of Time Cells

The reservoir model of time cells proposes that the CA3 recurrent network (which has strong excitatory recurrence) generates a sequence of states during the delay period, and CA1 reads out this sequence via the Schaffer collateral projection. CA3's initial state is set by the memory-encoding event (via the mossy fiber input from DG); the subsequent state evolution during the delay is driven by the CA3 internal dynamics (without external input).

Mathematically, if $\mathbf{x}_0$ is the initial CA3 state set by the encoding event and $\mathbf{W}^{\text{rec}}_{\text{CA3}}$ is the recurrent weight matrix:

$$\mathbf{x}_t = \tanh(\mathbf{W}^{\text{rec}}_{\text{CA3}} \mathbf{x}_{t-1}),$$

then the CA3 state at time $t$ is a nonlinear function of $t$ and $\mathbf{x}_0$. If $\mathbf{W}^{\text{rec}}_{\text{CA3}}$ has eigenspectrum with slow modes (eigenvalues near 1), the state evolves slowly and systematically through state space — producing the sequential activation of time cells observed by MacDonald et al. [2011].

## Theta Sequences: Compressed Replay

During active navigation, hippocampal neurons exhibit theta oscillations (4–10 Hz) modulated by gamma oscillations (30–100 Hz). Skaggs et al. [1996] discovered that within each theta cycle, place cells fire in a specific temporal sequence corresponding to the animal's current, past, and future positions — a "theta sequence." This is compressed replay: the spatial trajectory over seconds is replayed in compressed form within a single theta cycle (100–250 ms).

The reservoir interpretation: the CA3 attractor dynamics, combined with theta-phase modulation from the medial septal input, generate a forward-sweeping trajectory through the place cell representation on each theta cycle. This allows the hippocampus to pre-play future positions and post-play past positions within a single oscillation period, enabling prospective planning and retrospective evaluation [Skaggs et al. 1996].

## Pattern Completion and Pattern Separation

The hippocampal reservoir contains two functionally opposing subregions:

**CA3 (pattern completion):** CA3 has strong recurrent excitatory connections (Schaffer collaterals). This gives it attractor dynamics: a partial or noisy input pattern is completed to the stored attractor. This is the hippocampal mechanism for memory retrieval from degraded cues — the CA3 autoassociative network fills in the missing information.

**Dentate Gyrus (pattern separation):** DG has sparse connectivity and strong competitive inhibition (via mossy cell interneurons). It projects to CA3 via mossy fibers and dramatically expands the input code: $\sim 10^6$ EC cells → $\sim 10^6$ DG cells (granule cells), with extremely sparse activation ($\sim 1\%$ active simultaneously). This orthogonalizes similar input patterns, reducing interference between memories.

**Epistemic status:** Place cells and grid cells are established findings in rodents and humans [O'Keefe & Dostrovsky 1971; Moser et al. 2008]. Time cells in hippocampus are well-documented in rodents [MacDonald et al. 2011]. The reservoir/attractor interpretation of CA3 and CA1 dynamics is a productive theoretical framework with substantial computational support.

---

## References

- O'Keefe, J., & Dostrovsky, J. (1971). The hippocampus as a spatial map: Preliminary evidence from unit activity in the freely-moving rat. *Brain Research*, 34(1), 171–175.
- MacDonald, C. J., Lepage, K. Q., Eden, U. T., & Eichenbaum, H. (2011). Hippocampal "time cells" bridge the gap in memory for discontiguous events. *Neuron*, 71(4), 737–749.
- Squire, L. R. (1992). Memory and the hippocampus: A synthesis from findings with rats, monkeys, and humans. *Psychological Review*, 99(2), 195–231.
- Skaggs, W. E., McNaughton, B. L., Wilson, M. A., & Barnes, C. A. (1996). Theta phase precession in hippocampal neuronal populations and the compression of temporal sequences. *Hippocampus*, 6(2), 149–172.
