# Chapter Notes — Chapter 32

## On Descriptive Set Theory

The standard reference is Kechris's *Classical Descriptive Set Theory* (Springer Graduate Texts in Mathematics, 1995). It is a masterfully organized book — the first three parts cover Polish spaces, Borel sets, and analytic sets with clear motivation throughout. The later chapters on Borel equivalence relations are more demanding but essential for understanding the classification problem.

Gao's *Invariant Descriptive Set Theory* (CRC Press, 2009) covers the Borel equivalence relation theory and classification problems in the style more directly relevant to dynamics. If your primary interest is the Foreman-Rudolph-Weiss theorem and the unclassifiability of ergodic systems, Gao's book is the more direct route.

## On Turbulence

Hjorth's turbulence theory is developed in *Classification and Orbit Equivalence Relations* (AMS Mathematical Surveys and Monographs, 2000). The book is somewhat terse; supplementing it with Hjorth's later expository articles (available on his webpage) makes the main ideas more accessible.

The key example to internalize before reading Hjorth: the action of $\ell^2(\mathbb{N})$ (an infinite-dimensional Polish group, with addition) on itself is turbulent — every orbit is dense, and every orbit is meager. Any classification of this action by a Borel function would have to separate elements of $\ell^2$ that are "irrationally" related, which the Borel hierarchy can't do. This example motivates the general theory.

## On the Foreman-Rudolph-Weiss Theorem

The Foreman-Rudolph-Weiss theorem is in *The conjugacy problem in ergodic theory* (Annals of Mathematics, 2011). This is a long and technically demanding paper; the introduction is worth reading carefully even if you don't follow the proofs. Foreman and Weiss also wrote an accessible survey: *An Anti-Classification Theorem for Ergodic Measure Preserving Transformations* (J. European Math. Soc., 2004), which explains the main ideas without the full technical machinery.

## On Generic Properties

The computation that ergodic MPTs form a dense $G_\delta$ in all MPTs is classical — it follows from Rokhlin's theorem (every aperiodic MPT is a limit of periodic ones in the weak topology). The result that strongly mixing is meager (while weakly mixing is generic) is due to Halmos and Rohlin; see Halmos's *Lectures on Ergodic Theory* for the proofs.

The distinction between generic (in the Baire category sense) and "typical" (in any measure-theoretic sense on the space of systems) is philosophically important and often confusing. Glasner and Weiss's *A simple characterization of the set of $\mu$-entropy pairs* (1994) and subsequent work explores when these notions align and when they diverge.

## On the Axiom of Determinacy

The connection between AD and the regularity of all subsets of Polish spaces is developed in Kanamori's *The Higher Infinite* (Springer, 2003) and Jech's *Set Theory* (Springer, 2003). The descriptive set-theoretic consequence — that AD implies all sets have the Baire property and are measurable — follows from the Martin-Steel theorem on projective determinacy (provable from large cardinals).

For dynamics, the relevance is primarily philosophical: it tells us that any dynamical system that can arise from an explicit mathematical construction has the Baire property and is measurable, so the pathologies (Vitali sets, Bernstein sets) are genuinely outside the realm of practice.

## On the Connection to Chapter 27

The arithmetic hierarchy of Chapter 27 and the Borel hierarchy of Chapter 32 are deeply connected. Both classify sets (or properties) by their logical complexity. The arithmetic hierarchy lives in the world of computability (sets definable by Turing machine computations), while the Borel hierarchy lives in the world of topology (sets definable by topological operations). For subsets of $\omega^\omega$ (Baire space), these coincide at finite levels: $\Sigma^0_n$ in the Borel sense equals $\Sigma_n^0$ in the arithmetic sense, for $n < \omega$. Beyond $\omega$, the Borel hierarchy extends further than the arithmetic hierarchy, and the projective hierarchy goes further still. The full picture is the *descriptive complexity theory* of dynamical properties — a subject that remains active.
