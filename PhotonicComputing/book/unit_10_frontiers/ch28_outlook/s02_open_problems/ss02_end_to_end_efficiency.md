# 28.2.2 End-to-End Energy Efficiency

## The Accounting Problem

The single most misused number in photonic computing is the energy per multiply-accumulate (MAC) measured *in the optical domain*. It can be made to look spectacular — femtojoules, attojoules, "essentially free once the light is on" — and, taken alone, it is close to meaningless. An optical multiply-accumulate does not happen in isolation; it sits inside a system that must generate light, imprint numbers on it, detect it, and turn the result back into bits, and every one of those steps costs energy that the optical-domain figure omits. The right question, as Chapter 25 argued, is not "how little energy does the multiply take" but "what is the minimum energy of the *whole system* — lasers, converters, and calibration included — and which algorithm-hardware co-design minimizes it."

## The Overheads the Optical Number Omits

Accounted honestly, a photonic matrix-vector engine pays, per operation or amortized across the array:

- **Laser wall-plug inefficiency.** Semiconductor lasers convert electrical to optical power at roughly 10–30% wall-plug efficiency, and much of the delivered light is split, weighted, and lost before it reaches a detector; the laser draws its power whether or not the array is busy.
- **DAC energy.** Each input value and each programmable weight must be produced by a digital-to-analog converter driving a modulator; fast, multi-bit DACs are a dominant and famously non-scaling cost.
- **Modulator drive energy.** Imprinting an analog value on the optical carrier costs energy that grows with speed and with the required extinction and precision.
- **Photodetection, TIA, and ADC.** The optical result is a photocurrent that a transimpedance amplifier must amplify and an analog-to-digital converter must digitize — and ADCs at the speed and bit depth of a useful accelerator are among the hungriest circuits on the die.
- **Calibration and thermal control.** Continuous closed-loop tuning to hold the mesh at its programmed operating point (see Section 28.2's engineering challenges) draws standby power indefinitely.

The recurring empirical finding — quantified across the benchmarking literature [Miller, *Journal of Lightwave Technology*, 2017; Nahmias et al., *IEEE JSTQE*, 2020; Tait, *Physical Review Applied*, 2022] — is that for realistic silicon-photonic neural networks these electronic interface and control costs frequently *dominate* the total. A system whose optical core is a thousand times more efficient than a GPU may be, end to end, no better and possibly worse. This is not a claim that photonics loses; it is a claim that the optical-core number does not decide the contest. Chapter 25 is the chapter that decides it.

## The One Thing That Does Amortize: The $1/N$ Argument

There is nonetheless a real and important structural reason photonics can win, and it is worth stating with precision. In a broadcast-and-weight or WDM architecture, a single modulated input drives many parallel multiplications at once: one input vector of length $N$, encoded a single time, is dotted against many weight rows simultaneously in the optical domain. The *input conversion cost is therefore shared* across all the MACs it feeds, so the per-MAC contribution of that conversion scales as $\sim 1/N$. Hamerly and colleagues made this quantitative for coherent, photoelectric-multiplication networks and showed that in the large-$N$ limit the optical multiply energy can fall below the thermal-noise-limited energy of a single electronic MAC [Hamerly et al., *Physical Review X*, 2019]. This is the real physics behind the "photonics for large matrices" thesis:

> **The larger the linear-algebra operation, the more MACs each expensive conversion is amortized over — so large, dense matrices favor optics.**

But read the argument to its end. The $1/N$ amortization applies to the *input* encoding and to the shared laser — the costs spread across the fan-out. It does *not* apply to the conversions that scale with the *output* dimension: every output element still needs its own detector, transimpedance amplifier, and ADC, once per inference. Those per-output, per-conversion costs do not amortize away; they set a floor. The binding constraint of the whole enterprise is therefore the **data-conversion wall** — the ADC/DAC boundary — not the multiply. You can make the matrix bigger to dilute the input and laser overhead, but you cannot make the matrix bigger to escape needing to digitize the answer.

## Why This Points to Interconnect, Not Compute

This accounting is the quantitative core of the interconnect-first thesis of Chapter 26. If the dominant, non-amortizable cost is moving analog values across the electronic-photonic boundary, then the application where photonics wins first is the one whose *entire purpose* is to move data — communication — where the "conversion" is not overhead but the product itself, and is tolerated at a bit-error rate rather than held to an arithmetic precision bound. Co-packaged optics and optical I/O (Ayar Labs; Lightmatter's Passage — Chapter 26) win precisely because they do not fight the conversion wall; they *are* the conversion, performed efficiently and sold as bandwidth. Optical computation, which must pay the conversion tax on top of doing arithmetic, is later in the order of battle for exactly this reason.

## What Remains Open

Several first-order questions are genuinely unresolved and together define the research program:

- **The minimum achievable system energy.** No one has established the true end-to-end floor for a photonic accelerator on a realistic workload, converters and calibration included. The fundamental limits of Section 28.1 bound it from below, but the achievable engineering optimum is open.
- **Keeping data in the analog/optical domain longer.** Every avoided conversion is pure profit. Architectures that chain multiple optical operations — several matrix layers, or an intervening optical nonlinearity (Section 28.2.1) — before digitizing could amortize the ADC across more computation, but no general, precision-stable way to do this at depth yet exists.
- **Which workloads have the arithmetic intensity to pay.** Photonics rewards a high ratio of arithmetic to input/output. Identifying and co-designing the algorithms and models whose structure keeps the optics busy between conversions — large dense layers, specific transforms, high-reuse kernels — is an open algorithm-hardware co-design problem, and probably the most consequential one in the field.

The disciplined summary [McMahon, *Nature Reviews Physics*, 2023; Shastri et al., *Nature Photonics*, 2021] is this: the optical-domain energy advantage is real and, for large linear operations, provably large; whether it survives end to end depends entirely on the interfaces; and closing that gap is a systems problem, not a physics one.
