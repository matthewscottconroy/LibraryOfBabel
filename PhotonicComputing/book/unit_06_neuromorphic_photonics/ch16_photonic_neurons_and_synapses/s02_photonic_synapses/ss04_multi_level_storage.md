# 16.2.4 Multi-Level Analog Synaptic Storage

## An analog memory by nature

The crystalline fraction $c$ of a PCM patch is a continuous variable, so the transmission $T(c)$ it sets is continuous too (§16.2.1). A PCM synapse is therefore an **analog** memory: it stores more than one bit. If a cell can be reliably programmed and read to $N$ distinct transmission levels, its information capacity is
$$b = \log_2 N \quad \text{bits per synapse}. \tag{1}$$
Ríos et al. (2015) demonstrated multiple ($8+$) clearly separated levels in a single waveguide cell, and the regime explored by Feldmann et al. (2019, 2021) pushed multi-level storage toward roughly $5$ bits — on the order of $34$ levels — per synapse. This matters because neural-network weights rarely need more than a few bits of precision, so a single compact, non-volatile cell can hold an entire weight. The density payoff is direct: a $5$-bit analog cell replaces five binary cells, a fivefold saving in footprint that compounds across the millions of weights in a large layer — decisive when the whole point of the photonic approach is to pack a matrix onto a chip.

## Programming the levels

A target level is reached by controlling how much of the patch crystallizes, using tailored optical **pulse trains** — sequences of partial-SET pulses that incrementally nucleate and grow crystallites (or partial-RESET pulses that reduce $c$). The difficulty is that crystallization proceeds by **nucleation-and-growth kinetics** that are thermally activated and **stochastic**: nominally identical pulses do not produce identical crystalline fractions from cell to cell or shot to shot. Compounding this, the map from $c$ to $T$ is nonlinear (the exponential attenuation of §16.2.1), so equal increments of crystalline fraction are not equal increments of weight. Open-loop pulse counting is consequently imprecise.

## Non-idealities

Four effects limit how many levels can be used in practice:

- **Programming stochasticity.** Nucleation is random, spreading the achieved level around its target.
- **Level drift.** The amorphous phase relaxes structurally over time, so a cell's transmission drifts after it is written; levels that were distinct at $t=0$ can migrate.
- **Cycle-to-cycle variability.** Repeated writes of the same target land on slightly different transmissions.
- **Finite endurance.** Repeated melt-quench cycling gradually degrades the cell; endurance is on the order of ~$10^6$–$10^9$ cycles.

## Mitigations

- **Closed-loop program-and-verify.** Apply a pulse, read $T$, compare with the target, and iterate. This converges onto the desired level despite stochasticity and the nonlinear $c \to T$ map, and is the standard route to many-level programming.
- **Drift-aware encoding.** Place levels and decode thresholds so that predictable drift stays within a level's guard band, allocating wider spacing where drift is worst.
- **Periodic refresh.** Re-program cells before drift crosses a boundary. For fixed-weight inference, where weights change rarely, occasional refresh is affordable; endurance sets how often refresh or retraining can be paid for.

## Worked Example: bits and distinguishability under read noise

*Part 1 — nominal capacity.* If a cell holds $N = 34$ levels, its nominal precision from (1) is
$$b = \log_2 34 = 5.09 \approx 5\ \text{bits}.$$

*Part 2 — noise-limited capacity.* Suppose the usable transmission window runs from $T_\text{min} = 0.20$ to $T_\text{max} = 0.90$, a dynamic range $D = 0.70$, and the read (measurement) noise has standard deviation $\sigma_T = 0.005$ (a $0.5\%$ absolute, $1\sigma$ uncertainty). For a low read-error rate we require adjacent levels to be separated by at least $6\sigma_T$ (a $\pm 3\sigma$ guard band). The maximum number of reliably distinguishable levels is then
$$N_\text{dist} = 1 + \frac{D}{6\,\sigma_T} = 1 + \frac{0.70}{6 \times 0.005} = 1 + \frac{0.70}{0.030} = 1 + 23.3 \approx 24\ \text{levels},$$
for an effective precision of
$$b_\text{eff} = \log_2 24 = 4.58 \approx 4.6\ \text{bits}.$$

So although $34$ levels ($5$ bits) can be *written*, this read noise caps the *reliable* capacity near $24$ levels ($\approx 4.5$ bits). We can confirm the tension directly: spacing the full $34$ levels evenly across the window gives a step
$$\Delta T = \frac{D}{N-1} = \frac{0.70}{33} = 0.0212,$$
so the per-step signal-to-noise ratio is
$$\text{SNR} = \frac{\Delta T}{\sigma_T} = \frac{0.0212}{0.005} = 4.24 \quad (\approx 12.5\ \text{dB}).$$
A separation of only $4.2\sigma$ falls short of the $6\sigma$ target, so adjacent levels overlap unacceptably — quantitative confirmation that $34$ levels exceed what this noise floor supports. Drift over time erodes the margin further, steadily reducing $b_\text{eff}$ and making the case for drift-aware encoding and periodic refresh. The practical lesson is that a synapse's usable precision is set not by how finely it can be *written* but by how reliably it can be *read* and *held*.

---

## References

- Ríos, C., Stegmaier, M., Hosseini, P., Wang, D., Scherer, T., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2015). "Integrated all-photonic non-volatile multi-level memory." *Nature Photonics*, 9(11), 725–732.
- Feldmann, J., Youngblood, N., Wright, C.D., Bhaskaran, H. & Pernice, W.H.P. (2019). "All-optical spiking neurosynaptic networks with self-learning capabilities." *Nature*, 569(7755), 208–214.
- Feldmann, J. et al. (2021). "Parallel convolutional processing using an integrated photonic tensor core." *Nature*, 589(7840), 52–58.
