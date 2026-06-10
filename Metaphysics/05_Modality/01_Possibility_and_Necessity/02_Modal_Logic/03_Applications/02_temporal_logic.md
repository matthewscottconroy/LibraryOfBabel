# Temporal Logic

Time, like modality, involves claims about what holds always, what will hold at some point, and what has held. Arthur Prior recognized in the 1950s that these temporal operators — "always," "sometimes," "will be," "has been" — have exactly the logical structure of modal operators, and that the framework of possible-worlds semantics applies directly. Temporal logic treats moments or intervals of time as the "worlds" and temporal precedence as the "accessibility relation." The result is a powerful formal framework with deep connections to both the metaphysics of time and the logic of computer systems.

## Temporal Logic as Modal Logic

The key operators come in two pairs. The future operators are G ("it will always be the case that") and F ("it will at some point be the case that"). The past operators are H ("it has always been the case that") and P ("it was at some point the case that"). In modal terms:

- G = □_future (necessity in the future direction)
- F = ◇_future (possibility in the future direction)
- H = □_past (necessity in the past direction)
- P = ◇_past (possibility in the past direction)

The accessibility relations: for G and F, t R_future t' iff t' is strictly future relative to t; for H and P, t R_past t' iff t' is strictly past relative to t. The combination of past and future operators gives rich expressive power: "P(φ) ∧ F(ψ)" says it was the case that φ, and it will be the case that ψ.

## Linear Versus Branching Time

Two main varieties of temporal logic differ on the structure of time itself. Linear temporal logic (LTL) assumes time is a single sequence of moments — a line stretching in both directions, with each moment having a unique past and a unique future. This is appropriate for deterministic sequential processes, where there is one history and one future.

Branching temporal logic (CTL, CTL*) represents time as a tree: at any moment, multiple futures are open, representing genuine indeterminism or undecidedness about which future will be realized. Two families of branching-time operators combine path quantifiers (A: for all future paths; E: there exists a future path) with path operators (X: next moment, G: always on this path, F: eventually on this path, U: until). This gives sentences like AG(φ) — "on all future paths, always φ" — and EF(ψ) — "there exists a future path on which eventually ψ."

The branching-time framework is more appropriate for reasoning about genuine possibilities. "It is inevitable that the system will terminate" is A-F(terminated); "it is possible that the system loops forever" is E-G(¬terminated). The modal structure of temporal uncertainty is directly expressed.

## Computer Science Applications

Temporal logic has had enormous practical impact in computer science through model checking — the algorithmic verification of concurrent and reactive systems against temporal specifications. A specification like "whenever a request is made, it will eventually be granted" is expressed as G(request → F(grant)) and can be automatically verified against a formal model of the system. Clarke, Emerson, and Sifakis received the Turing Award in 2007 for this work — one of the major success stories of applying formal logic to engineering.

Standard specification patterns include:

- Safety: "Nothing bad ever happens" — G(¬bad)
- Liveness: "Something good eventually happens" — F(good) or G(request → F(response))
- Fairness: "If a process is repeatedly enabled, it eventually runs" — G(F(enabled) → F(running))

## Philosophical Significance

Temporal logic connects to several deep philosophical questions. Consider fatalism: if G(A) holds — A is true at all future times — does this make A metaphysically necessary? The fatalist argues yes; opponents argue that temporal necessity ("will always be") is weaker than metaphysical necessity ("cannot be otherwise"). Making this distinction precise requires exactly the formal resources temporal logic provides.

The problem of future contingents — Aristotle's sea-battle, discussed in *De Interpretatione* ch. 9 — finds a natural formal home in branching-time logic. On the branching-time view, "there will be a sea battle tomorrow" is true on some branches, false on others, and no single branch is yet "the" actual future. This models the view that future contingent statements are neither true nor false — or rather, not determinately either — without the paradoxes that arise in linear-time frameworks.

The asymmetry between past and future — the past is fixed, the future is open — is represented formally by the difference between the past operators (which typically range over a unique, determinate history) and the future operators (which range over branching possibilities). This formal asymmetry may represent the fundamental asymmetry of causation: causes precede effects, and the past cannot be changed. The direction of time and the direction of causation are connected through the modal structure of temporal logic, and understanding that connection is a significant philosophical payoff of the formal framework.
