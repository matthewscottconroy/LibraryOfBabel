# 1.1.1 Coulomb, Gauss, Faraday, and Ampère: The Experimental Foundation

## The Starting Point: Electric Charge

The modern science of electricity begins with the recognition that matter can carry electric charge — a property that manifests as a force between charged objects. The ancient Greeks knew that rubbed amber attracted light objects; the word "electricity" derives from the Greek *elektron*, meaning amber. But the quantitative law governing these forces was not established until 1785, when Charles-Augustin de Coulomb published his measurement of the force between charged spheres using a torsion balance [1].

Coulomb's result is what we now call **Coulomb's law**: the force between two point charges $q_1$ and $q_2$ separated by distance $r$ is

$$\mathbf{F} = \frac{1}{4\pi\varepsilon_0} \frac{q_1 q_2}{r^2} \hat{\mathbf{r}}$$

where $\hat{\mathbf{r}}$ is the unit vector pointing from $q_2$ to $q_1$, and $\varepsilon_0 = 8.854 \times 10^{-12}$ F/m is the permittivity of free space. The constant $1/(4\pi\varepsilon_0)$ appears because of the SI unit convention; in Gaussian units it is simply 1.

Several things are worth noting about this equation:

**The inverse-square law.** The force falls as $1/r^2$. This is the same scaling as Newton's gravitational force, and it is not a coincidence — both forces arise from fields in three-dimensional space, and the $1/r^2$ dependence is the hallmark of a field that spreads over the surface area of a sphere ($4\pi r^2$) without being absorbed or created.

**Action at a distance?** Coulomb's law appears to describe an instantaneous force between two objects separated in space. This troubled physicists deeply, and rightly so. The resolution — that the force is mediated by the electric *field*, not transmitted instantaneously — was provided by Michael Faraday in the 1840s. We will return to this conceptually important point.

**The sign of the force.** Like charges repel; unlike charges attract. This is encoded in the product $q_1 q_2$: it is positive (and the force is repulsive, pointing from $q_2$ toward $q_1$) when the charges have the same sign, and negative (attractive) when they have opposite signs.

## The Electric Field

The concept of the electric **field** was introduced by Michael Faraday in the 1840s [2]. Rather than thinking of force as acting directly between distant charges, Faraday proposed that each charge creates a field in the surrounding space, and other charges respond to the field at their location. This is more than a bookkeeping device. The field is a physical entity: it carries energy, it propagates at a finite speed, and it exists independently of whether a test charge is present to feel it.

The electric field $\mathbf{E}$ at a point $\mathbf{r}$ due to a point charge $q$ at the origin is defined as the force per unit charge that a small test charge would experience at that point:

$$\mathbf{E}(\mathbf{r}) = \frac{1}{4\pi\varepsilon_0} \frac{q}{r^2} \hat{\mathbf{r}}$$

The field is a vector — it has both magnitude and direction at every point in space.

## Gauss's Law: Counting Field Lines

Carl Friedrich Gauss derived a remarkable consequence of Coulomb's law in 1835 (though it was not published until after his death). The key insight is geometric.

Imagine a closed surface $S$ surrounding a charge $q$. The electric field lines — lines drawn tangent to the field direction at each point — radiate outward from the charge in all directions. No matter what shape $S$ has, each field line must pass through it exactly once. Therefore, the total "flux" of field lines through $S$ depends only on the total charge enclosed by $S$, not on the shape of the surface or the distribution of charge inside.

Mathematically, the electric flux through a surface element $d\mathbf{A}$ (where the direction of $d\mathbf{A}$ is the outward normal to the surface) is $\mathbf{E} \cdot d\mathbf{A}$. Gauss's law states that the total flux through any closed surface equals the total enclosed charge divided by $\varepsilon_0$:

$$\oint_S \mathbf{E} \cdot d\mathbf{A} = \frac{Q_{\text{enc}}}{\varepsilon_0}$$

This is the first of Maxwell's four equations in integral form.

*Why is this law so powerful?* Because it relates a global quantity (total flux through a surface) to a local quantity (total enclosed charge) without requiring knowledge of the field everywhere. For highly symmetric situations — a sphere of charge, an infinite line of charge, an infinite plane of charge — Gauss's law can be used to find the electric field with almost no calculation. For less symmetric situations, it still constrains the field in useful ways.

**Worked example: field of a point charge.** Take $S$ to be a sphere of radius $r$ centered on a point charge $q$. By spherical symmetry, $\mathbf{E}$ must point radially and must have the same magnitude everywhere on $S$. Therefore $\oint_S \mathbf{E} \cdot d\mathbf{A} = E \cdot 4\pi r^2$. Setting this equal to $q/\varepsilon_0$ and solving, we recover Coulomb's law: $E = q/(4\pi\varepsilon_0 r^2)$.

## Gauss's Law for Magnetism: No Monopoles

Magnets have two poles — north and south. Unlike electric charges, magnetic poles cannot be separated: every magnet, no matter how small, has both a north and a south pole. If you cut a bar magnet in half, you get two smaller bar magnets, each with two poles.

This observation — never yet contradicted by experiment — implies that there are no "magnetic charges" (magnetic monopoles) from which magnetic field lines could originate. Therefore the total magnetic flux through any closed surface must be zero:

$$\oint_S \mathbf{B} \cdot d\mathbf{A} = 0$$

This is the second of Maxwell's four equations. Its physical content is: **magnetic field lines form closed loops** — they never begin or end. Somewhere in the universe there may yet be a magnetic monopole (the theory of grand unification predicts their existence [3]), but none has ever been observed [4], and Maxwell's second equation expresses this empirical fact.

## Faraday's Law: Changing Magnetism Creates Electricity

In 1831, Michael Faraday made one of the most important experimental discoveries in the history of physics: a changing magnetic field induces an electric field [5]. He demonstrated this in several ways: moving a magnet toward a wire loop induced a current in the loop; changing the current in one coil induced a current in a nearby coil; rotating a copper disk in a magnetic field generated a continuous current.

The quantitative law governing these observations is **Faraday's law of induction**:

$$\oint_C \mathbf{E} \cdot d\boldsymbol{\ell} = -\frac{d}{dt} \int_S \mathbf{B} \cdot d\mathbf{A}$$

The left side is the *electromotive force* (EMF) — the work done per unit charge around a closed loop $C$. The right side is the negative rate of change of the magnetic flux through any surface $S$ bounded by $C$.

The negative sign is the mathematical expression of Lenz's law: the induced current opposes the change that creates it. If you push a magnet toward a loop, the induced current creates a magnetic field that repels the magnet. This is energy conservation in action — you must do work to push the magnet, and that work appears as electrical energy in the circuit.

Faraday's law is the third of Maxwell's four equations, and it is crucial for photonic computing: it is the equation that couples electric and magnetic fields and allows them to sustain each other as they propagate — which is what an electromagnetic wave is.

## Ampère's Law: Steady Currents Create Magnetism

Hans Christian Ørsted discovered in 1820 that a current-carrying wire deflected a nearby compass needle — electric current creates a magnetic field [6]. André-Marie Ampère developed the quantitative law in the following years. For a steady current, Ampère's law states:

$$\oint_C \mathbf{B} \cdot d\boldsymbol{\ell} = \mu_0 I_{\text{enc}}$$

where $\mu_0 = 4\pi \times 10^{-7}$ H/m is the permeability of free space and $I_{\text{enc}}$ is the total current passing through any surface bounded by the loop $C$.

This law is powerful for symmetric geometries. For an infinite straight wire carrying current $I$, the magnetic field at distance $r$ from the wire is $B = \mu_0 I / (2\pi r)$, circling the wire according to the right-hand rule.

But Ampère's law, as stated here, has a fatal flaw that Maxwell would identify. It applies only to **steady** currents. For time-varying currents — and for electromagnetic waves — it is incomplete. This is the problem that Maxwell solved, and solving it led directly to the prediction of electromagnetic waves.

---

## Summary

The empirical program from Coulomb through Ampère established four facts about electromagnetic fields:
1. Electric charges create electric fields (Gauss/Coulomb)
2. Magnetic field lines form closed loops — no monopoles (Gauss)
3. Changing magnetic fields create electric fields (Faraday)
4. Steady electric currents create magnetic fields (Ampère)

What was missing was the symmetry principle: **do changing electric fields create magnetic fields?** That was Maxwell's question, and answering it completed the theory.

---

## References

[1] Coulomb, C.A. de (1785). "Premier Mémoire sur l'Électricité et le Magnétisme." *Histoire de l'Académie Royale des Sciences*, 569–577. [The original measurement paper.]

[2] Faraday, M. (1844). *Experimental Researches in Electricity*, Vol. II. London: Richard and John Edward Taylor. [Faraday's development of the field concept.]

[3] 't Hooft, G. (1974). "Magnetic monopoles in unified gauge theories." *Nuclear Physics B*, 79(2), 276–284. [Theoretical prediction of monopoles from grand unified theories.]

[4] Cabrera, B. (1982). "First results from a superconductive detector for moving magnetic monopoles." *Physical Review Letters*, 48(20), 1378. [The famous (and controversial) single candidate event for magnetic monopoles.]

[5] Faraday, M. (1832). "Experimental researches in electricity." *Philosophical Transactions of the Royal Society*, 122, 125–162. [The original discovery of electromagnetic induction.]

[6] Ørsted, H.C. (1820). "Experiments on the effect of a current of electricity on the magnetic needle." *Annals of Philosophy*, 16, 273–276. [Translation of the original discovery.]
