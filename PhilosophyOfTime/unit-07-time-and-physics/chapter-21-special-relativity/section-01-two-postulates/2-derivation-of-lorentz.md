# Derivation of the Lorentz Transformations

## Setting Up the Problem

The Lorentz transformations are the coordinate transformations that relate the spacetime coordinates of one inertial reference frame to those of another, when the two postulates of special relativity hold. They replace the Galilean transformations of Newtonian mechanics and encode the strange but well-confirmed consequences of Einstein's theory.

Let us set up the problem carefully. Suppose we have two inertial observers, Alice and Bob. Alice is at rest in frame *S* and uses coordinates *(t, x)*. Bob is moving to the right at constant velocity *v* relative to Alice, and uses coordinates *(t', x')*. We set things up so that at *t = t' = 0*, the origins of both frames coincide.

In Newtonian mechanics (Galilean transformation), the relationship is simple:
- *x' = x - vt*
- *t' = t*

The second equation embodies absolute time: both observers use the same time coordinate. The Galilean transformation is Newtonian absolutism written in mathematics.

Special relativity replaces this with the Lorentz transformation. We will not reproduce the full algebraic derivation here (which can be found in any introductory relativity textbook), but the key step is to demand that the speed of light is the same in both frames.

## The Lorentz Transformation

If a light pulse is emitted at the origin when *t = 0*, its location is given by *x = ct* in Alice's frame. In Bob's frame, the same pulse must satisfy *x' = ct'* (by Postulate 2). We seek the most general linear transformation of coordinates that satisfies this condition (we assume linearity because we want uniform motion to transform to uniform motion).

The result is the Lorentz transformation:

*x' = γ(x - vt)*
*t' = γ(t - vx/c²)*

where *γ* (the Lorentz factor) is:

*γ = 1 / √(1 - v²/c²)*

This factor *γ* is always greater than or equal to 1, and becomes very large as *v* approaches *c*. At ordinary speeds (*v* much less than *c*), *γ* is so close to 1 that the Lorentz transformation is indistinguishable from the Galilean transformation — which is why Newtonian mechanics worked so well for centuries.

Notice the transformation for *t'*: it includes a term *-vx/c²* that was not present in the Galilean transformation. This term depends on the spatial coordinate *x* — meaning that what counts as "the same time" in Bob's frame depends on *where* as well as *when* an event occurs in Alice's frame. This is the source of all the surprising consequences of special relativity.

## Relativity of Simultaneity

The most philosophically important consequence follows immediately. Suppose two events, E₁ and E₂, are simultaneous in Alice's frame: they both occur at time *t = 0*, but at different locations *x₁* and *x₂*. What does Bob say?

Using the Lorentz transformation, the time of E₁ in Bob's frame is *t₁' = γ(0 - vx₁/c²) = -γvx₁/c²*, and the time of E₂ is *t₂' = -γvx₂/c²*. If *x₁ ≠ x₂* (the events are at different locations) and *v ≠ 0* (Bob is actually moving), then *t₁' ≠ t₂'*: the events are *not* simultaneous in Bob's frame.

This is the relativity of simultaneity: whether two spatially separated events are simultaneous is not an absolute fact. It depends on the reference frame of the observer. Different inertial observers in relative motion will disagree about which events are simultaneous.

This is not a matter of perception or measurement error. It is a fact about the structure of spacetime. There is no "true" fact about whether two spatially separated events are simultaneous — the question is reference-frame dependent.

## Time Dilation

Another key consequence: moving clocks run slow. Suppose Bob is carrying a clock that ticks at regular intervals. Between two ticks of Bob's clock, the proper time elapsed (as measured by Bob) is *Δt'* = the interval between ticks. In Alice's frame, the same interval of proper time corresponds to a coordinate time interval *Δt = γΔt'*, which is larger than *Δt'* because *γ ≥ 1*.

From Alice's perspective, Bob's clock is running slow by a factor of *γ*. The faster Bob moves, the larger *γ* and the slower his clock runs. At very high speeds (*v → c*), *γ → ∞* and Bob's clock (from Alice's perspective) barely ticks at all.

This is *time dilation*: moving clocks run slow. The effect is symmetric — from Bob's perspective, it is Alice's clock that is running slow. This apparent paradox (each says the other's clock is slow) is resolved by noting that there is no absolute way to say whose clock is "really" running slow when both are in inertial motion; asymmetry only appears when one observer accelerates (as in the twin paradox, discussed in Section 3).

## Length Contraction

A related consequence: moving objects are contracted in the direction of motion. An object at rest in Bob's frame with length *L₀* (its proper length) has a shorter length *L = L₀/γ* when measured in Alice's frame. The faster the object moves, the more it is contracted.

Like time dilation, length contraction is a real physical effect, not an illusion. It has observable consequences: relativistic particle beams, for instance, appear contracted along the direction of motion when observed in the lab frame.

## What This Means

It is worth pausing to absorb how radical these results are. Newton's mechanics assumed — and common sense still suggests — that time intervals and length intervals are objective facts about the world, the same for all observers. Special relativity shows that this is simply wrong. A duration of one second between two events is not an objective, frame-independent fact; it depends on the reference frame of the observer. A length of one meter is not objective; it depends on the frame. What *is* frame-independent is the spacetime interval between two events — a quantity that combines temporal and spatial separation in a specific way (as Minkowski showed, and as we will examine in Section 4).

The philosophical implications are deep. If temporal duration is frame-relative, what happens to the notion of a "present moment" shared by all observers? And if there is no shared present, what happens to our ordinary conception of what exists? These are the questions we take up in the sections that follow.

**References**

Einstein, Albert. 1905/1923. "On the Electrodynamics of Moving Bodies." In *The Principle of Relativity*, translated by W. Perrett and G. B. Jeffery. London: Methuen.

Taylor, Edwin F., and John Archibald Wheeler. 1992. *Spacetime Physics: Introduction to Special Relativity*. 2nd ed. New York: W. H. Freeman.

Mermin, N. David. 2005. *It's About Time: Understanding Einstein's Relativity*. Princeton: Princeton University Press.
