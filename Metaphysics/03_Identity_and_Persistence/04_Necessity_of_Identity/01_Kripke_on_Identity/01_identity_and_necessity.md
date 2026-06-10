# Identity and Necessity

*Why identity statements, if true, are necessarily true.*

---

"Hesperus is Phosphorus." This was an empirical discovery — Babylonian astronomers learned it by observation, and it was not logically obvious in the way that "Hesperus is Hesperus" is logically obvious. Yet Kripke argues that it is *necessarily* true: there is no possible world in which Hesperus fails to be Phosphorus. How can an empirical discovery be a necessary truth? The answer transforms our understanding of the relationship between necessity and apriority.

Kripke's argument for the necessity of identity proceeds from the concept of rigid designation. Names, on his view, are rigid designators: they pick out the same individual in every possible world where that individual exists, regardless of what properties the individual has there. "Hesperus" rigidly designates Venus; "Phosphorus" rigidly designates Venus. In any possible world where Venus exists, both names refer to Venus. Since a = a (reflexivity of identity) is necessarily true, and since "a" and "b" are both rigid designators of the same thing, "a = b" is necessary wherever it is true.

The argument can be formalized in S5 modal logic:

- **(1)** a = b. [Assumption: the identity holds in the actual world]
- **(2)** □(a = a). [Necessity of self-identity: everything is necessarily identical with itself]
- **(3)** a = b → (□(a = a) → □(a = b)). [By Leibniz's Law: if a = b, a has a property iff b has it; a has □(x = a); so b has □(x = b) ↔ □(a = b)]
- **(4)** □(a = b). [From (1), (2), (3)]

## The Necessary A Posteriori

This result establishes what Kant declared impossible: necessary truths that are knowable only through experience. The *necessary a posteriori*. Kripke distinguishes necessity (a metaphysical notion: true in all possible worlds) from apriority (an epistemological notion: knowable independently of experience). These are independent dimensions:

|  | Necessary | Contingent |
|---|---|---|
| **A priori** | "All bachelors are unmarried" | "The meter stick is one meter long" |
| **A posteriori** | "Hesperus = Phosphorus" | "Water boils at 100°C at sea level" |

"Hesperus = Phosphorus" is necessary (if true, necessarily true) but a posteriori (knowable only through observation). The appearance of contingency — the sense that it might have turned out otherwise — arises from conflating the epistemic question ("how did we discover the identity?") with the metaphysical question ("could the identity have failed to hold?").

We can imagine a scenario in which ancient astronomers who observed Hesperus and those who observed Phosphorus believed they were tracking different planets. Is there a possible world in which Hesperus ≠ Phosphorus? Kripke's answer: no. The *planet Venus* could not have been two planets. The apparent possibility of Hesperus ≠ Phosphorus corresponds to a world in which the *descriptions* "morning star" and "evening star" apply to different planets — but in such a world, the rigid names "Hesperus" and "Phosphorus" (if they rigidly designate Venus) would not apply at all to the imagined scenario. Epistemic imaginability is not metaphysical possibility.

## Implications for Philosophy of Mind

The necessity of identity has significant implications for the philosophy of mind. Type identity theory holds that mental state types are identical with physical state types: pain = C-fiber firing. If this identity holds, it holds necessarily — in every possible world where pain exists, it is C-fiber firing.

But we can conceive of pain without C-fiber firing and C-fiber firing without pain. This conceivability suggests the possibility of pain ≠ C-fiber firing — which, if actual, would mean the identity does not hold at all (since if it held, it would hold necessarily). The type identity theorist must explain away the appearance of contingency.

For "Hesperus = Phosphorus," the appearance of contingency can be explained by different modes of presentation: the morning-appearance and the evening-appearance of Venus are different ways of accessing the same planet, and in worlds where those appearances diverge, the descriptions apply elsewhere. But for "pain = C-fiber firing," no analogous explanation is available. The phenomenal character of pain is essential to pain; there is no non-phenomenal mode of presentation of pain that explains why we can conceive of pain without C-fiber firing. Hence the apparent contingency of "pain = C-fiber firing" is genuine contingency — evidence that the identity does not hold. This argument against type identity theory is one of Kripke's most controversial legacies, generating decades of debate in philosophy of mind.
