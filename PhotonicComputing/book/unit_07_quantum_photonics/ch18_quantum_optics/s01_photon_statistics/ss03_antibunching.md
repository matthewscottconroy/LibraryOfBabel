# 18.1.3 Photon Antibunching and Non-Classicality

## The One Signature Classical Light Cannot Fake

Bunching ($g^{(2)}(0)>1$) and randomness ($=1$) both live inside the classical world. **Antibunching** — $g^{(2)}(0)<1$ — does not. Section 18.1.1 proved the Cauchy-Schwarz bound $g^{(2)}(0)\ge 1$ for every field describable by a non-negative classical intensity distribution; equivalently, in the Glauber-Sudarshan representation $\rho=\int P(\alpha)|\alpha\rangle\langle\alpha|\,d^2\alpha$, antibunching requires $P(\alpha)$ to go negative and cease to be a probability density. Antibunching is thus a **strict witness of non-classicality**: a single number below unity that no laser, no lamp, no thermal source, and no classically fluctuating combination of them can ever produce. It is the experimental definition of "quantum light," and unlike Wigner negativity (Section 17.3.4) it is measured with nothing more than two detectors and a coincidence counter.

## Why a Single Emitter Antibunches

The physical origin is almost embarrassingly simple. A single two-level emitter — an atom, an ion, a quantum dot, a color center — carries exactly one quantum of excitation. When it fluoresces it drops to the ground state and *must be re-excited before it can emit again*. Two photons cannot leave simultaneously because there is only ever one photon's worth of excitation to give. The emitted stream is therefore antibunched, and for an ideal single emitter $g^{(2)}(0)=0$.

The correlation function makes the timescale explicit. For a resonantly driven two-level atom with excited-state decay rate $\Gamma$, in the weak-drive limit

$$g^{(2)}(\tau) = \left(1 - e^{-\Gamma\tau/2}\right)^2,$$

which is $0$ at $\tau=0$ and recovers to the uncorrelated value $1$ over the fluorescence lifetime $\sim 1/\Gamma$: the emitter needs one lifetime to "reload." At stronger drive, damped **Rabi oscillations** appear in $g^{(2)}(\tau)$, which can transiently exceed $1$ at the Rabi period — the emitter, re-pumped, is *most* likely to emit again half a Rabi cycle later. Observing that structure is direct evidence of a single quantum emitter driven coherently.

## Kimble, Dagenais, and Mandel, 1977

The first observation of antibunching was made by H. Jeff Kimble, Mario Dagenais, and Leonard Mandel at Rochester in 1977, in the resonance fluorescence of a dilute sodium atomic beam (Kimble, Dagenais & Mandel, 1977). Watching the light scattered by (on average) a fraction of an atom in the observation volume, they recorded $g^{(2)}(\tau)$ rising from a sub-unity value at $\tau=0$ — the hallmark dip — the first light ever shown to be more orderly than a laser. The measurement was subtle: atomic-number fluctuations and transit-time effects pushed the raw $g^{(2)}(0)$ back up, and only careful accounting exposed the genuine antibunching of the single-atom contribution. The experiment inaugurated single-emitter quantum optics and, through Mandel, connects directly to the Hong-Ou-Mandel work a decade later (Section 18.2.2).

## Grangier-Roger-Aspect: One Photon, One Detector

Where Kimble-Dagenais-Mandel probed the *emission* statistics, Philippe Grangier, Gérard Roger, and Alain Aspect (1986) attacked the *particle* question head-on: does a single photon, sent onto a beam splitter, ever trigger both detectors at once? A classical wave divides its energy and can; a single indivisible photon cannot.

**Worked example.** *The anticorrelation parameter and its classical bound.*

Their source produced heralded single photons (a photon from an atomic cascade announced its partner). The heralded photon struck a 50/50 beam splitter feeding detectors in the two output ports, $c$ and $d$. Define the **anticorrelation parameter**

$$\alpha = \frac{P_{cd}}{P_c\,P_d},$$

the probability $P_{cd}$ of a joint $c$-and-$d$ click divided by the product of the single-detector probabilities (all conditioned on a herald). This is precisely a heralded $g^{(2)}(0)$ written in click probabilities. For the single-photon input the beam-splitter transformation of Section 18.2.1 gives

$$|1,0\rangle \to \tfrac{1}{\sqrt2}\big(|1,0\rangle + i\,|0,1\rangle\big),$$

a superposition in which exactly one detector fires: $P_{cd}=0$, hence $\alpha=0$. A classical field, by contrast, obeys $\alpha\ge 1$ (the Cauchy-Schwarz inequality again). GRA measured $\alpha\approx 0.18$ — many standard deviations below the classical floor of $1$ — the cleanest possible demonstration that light is delivered in indivisible quanta on a beam splitter. Careful modern reproductions of the experiment reach $\alpha\lesssim 0.02$. Crucially, the *same* heralded photon that gives $\alpha=0$ (particle-like anticorrelation) rebuilds full interference fringes when the two paths are recombined in a Mach-Zehnder — wave and particle behavior in one apparatus, mediated entirely by whether the paths are read out or interfered.

## The Modern Benchmark

Antibunching graduated from a physics curiosity to an engineering specification. A single-photon source for quantum computing must reach $g^{(2)}(0)<0.01$ — meaning multi-photon emission below one percent — and the best solid-state emitters now achieve $g^{(2)}(0)$ in the $10^{-4}$ range (Chapter 19). Every such number is an antibunching measurement in the HBT geometry of Section 18.1.2, and every one is a laboratory statement that the light in question has crossed the classical boundary this subsection defines.

## Measuring It Honestly

Two subtleties govern how the celebrated $g^{(2)}(0)<0.01$ numbers are obtained. First, finite detector timing resolution convolves the true antibunching dip with the instrument response, so a directly binned $g^{(2)}(0)$ is generally *higher* than the intrinsic value; the smallest reported numbers come from fitting the temporal profile and extrapolating, or from integrating the vanishing central peak of a pulsed source against its side peaks. Second, antibunching ($g^{(2)}(0)<1$, a statement about arrival *timing*) and sub-Poissonian statistics ($Q<0$, a statement about photon *number* in a mode) are distinct properties that happen to coincide for a single stationary mode but can diverge for pulsed or multimode emission. A well-characterized single-photon source is expected to be both, and reporting $g^{(2)}(0)$ from a Hanbury Brown-Twiss measurement (Section 18.1.2) has become the community's standard, detector-agnostic way to certify it.

## Why It Matters

Antibunching is the acceptance test of the photonic quantum era. It certifies that a source emits *one* photon and not a faint classical pulse — the difference between a genuine qubit carrier and a weak coherent decoy (Chapter 22), and the difference between a scalable quantum computer and an expensive lamp. $g^{(2)}(0)<1$ is, in a single inequality, the boundary between the classical optics of the first six units and the quantum optics of this one.
