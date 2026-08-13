# Chapter 72 — The People

**This chapter has no mechanism to attribute**, and its people are the ones who wrote about how
engineering decisions are actually made — **which is a smaller literature than it should be, and
most of it is from outside networking.**

**Priscilla Oppenheimer.** ***Top-Down Network Design*** **(1998, and three editions since).**

**The book that established that network design begins with requirements**, and its title is its
argument: **start at the application and the business, and derive downward** — **rather than
starting with a catalogue of equipment and working up.**

> **Which sounds obvious and was not.** **The prevailing practice in the 1990s was
> vendor-led — a design was a bill of materials with a topology attached** — **and Oppenheimer's
> contribution was the insistence that a design is a derivation.**

**Her characterisation methodology** — **document the existing network, characterise the traffic,
then design** — **is §72.2's derivation with the survey stage added**, and the traffic
characterisation chapters remain the best available treatment.

**Russ White, Denise Donohue and the "art of network architecture" material.**

**White's contribution is the framing that design is about trade-offs that cannot be
eliminated**, and his formulation is worth carrying:

> **"Complexity, optimisation and surface." You may reduce complexity, optimise for a specific
> outcome, or reduce the interaction surface between components — and improving any two worsens
> the third.**

**Which is a genuine engineering law rather than an aphorism**, and **it explains why every
design that removes complexity does so by giving up optimisation** (a simpler routing protocol
that converges more slowly) **or by increasing the surface** (a flatter network with more
components able to affect each other).

**White is also unusually direct that most networks are over-designed**, and **his recurring
question — "what problem does this solve?"** — **is §72.4's over-engineering test.**

**And *The Art of Network Architecture* (White and Donohue, 2014)** is **the closest thing to a
book about how to reason about network design rather than about what to build**, which is why
this chapter exists in the shape it does.

**Christopher Alexander (1936–2022).** **Architecture, not networking — and the source of an idea
that transferred.**

***A Pattern Language* (1977)** and ***Notes on the Synthesis of Form* (1964)** established
**that a design problem is a set of conflicting forces, and that a good design resolves them
rather than choosing between them.**

**Two of his arguments transfer directly:**

> **A design cannot be evaluated except against the forces it was resolving** — **which is
> §72.4's traceability requirement, and the reason a decision record is necessary rather than
> merely tidy.**

**And his observation about "unselfconscious" versus "selfconscious" design:** **traditional
craft evolves solutions over generations without anyone articulating why; modern design must
articulate, because the pace of change removes the time for evolution.**

> **Which is exactly this book's argument for history.** **A mechanism whose reason has been
> forgotten is one that cannot be evaluated when the constraint changes**, and **network
> engineering has a great deal of unselfconscious inheritance.**

**Alexander's pattern language also produced the software design patterns movement**, and its
lesson there was instructive: **the patterns were adopted and the underlying argument — that a
pattern is a resolution of forces in a context — was largely lost**, producing a catalogue where
a method was intended. **The same risk applies to reference architectures.**

**Herbert Simon (1916–2001).** ***The Sciences of the Artificial* (1969)**, and **satisficing.**

**Simon's argument is that a designer does not optimise, because the search space is too large
and the criteria conflict.** **They satisfice** — **find a solution that is good enough against
the criteria that matter** — **and the skill is in choosing the criteria and the threshold.**

> **Which is a more accurate description of network design than "find the best design", and it
> reframes §72.4's defensibility:** **you cannot defend a design as optimal, and you can defend
> it as satisfying stated requirements at a stated cost, with the alternatives considered.**

**Simon also gave the argument for hierarchy** — **that complex systems that work are almost
always hierarchical, because hierarchy is what permits a component to be understood without
understanding the whole** — **which is Chapter 21's layering argument, Chapter 27's address plan,
and Chapter 67's fabric, each independently.**

**David Parnas (b. 1941)**, for information hiding and for a specific piece of honesty.

**Parnas's 1972 paper on modular decomposition established that modules should be defined by what
they hide rather than by what they do** — **which is Chapter 21 §21.1's layering argument, in
software, five years earlier.**

**And his 1986 paper "A Rational Design Process: How and Why to Fake It" is the one that belongs
here:**

> **The argument is that a rational, top-down design process is never actually followed** —
> **requirements change, understanding develops, mistakes are made** — **and that the
> documentation should nonetheless present the design as though it had been.**

**Not as dishonesty**, but because **the reader needs the rational structure to understand the
result**, and **the actual history of false starts is not useful to them.**

> **Which is a precise description of what §72.4's design document is.** **The derivation in
> §72.2 is not how the design was arrived at — nobody derives a topology in a straight line —
> and it is how it should be presented**, because the reader needs to know whether it follows
> from the requirements, not how many attempts it took.

**And Parnas's honesty about this is worth emulating.** **A design document is a reconstruction,
and saying so is better than pretending the process was linear.**

## What this chapter's history establishes

**Three observations to close on.**

**The design literature is thin and mostly borrowed.** **Networking has an enormous mechanism
literature and very little on how to choose among mechanisms** — **and the useful material comes
from architecture, from systems theory and from software engineering.**

**The recurring finding is that the trade-offs cannot be removed.** **White's complexity
triangle, Simon's satisficing, Alexander's conflicting forces** — **all say the same thing: a
design is a resolution rather than an optimisation**, and **an engineer looking for the right
answer is looking for something that does not exist.**

**And the documentation is a reconstruction.** **Parnas's point, and it removes a great deal of
anxiety:** **the design you defend was not arrived at the way you present it, and presenting it
rationally is the correct thing to do.**

> **Which is where seventy-two chapters end.** **Not with a mechanism, but with the observation
> that every mechanism in the book was someone's resolution of conflicting forces under
> constraints that have since changed** — **and that your job is to do the same thing, for your
> constraints, and to be able to say why.**
