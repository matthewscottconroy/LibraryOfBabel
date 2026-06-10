# Two-Dimensional Semantics

Here is a puzzle that Kripke's work left unresolved. "Water is H₂O" is necessary — there is no possible world where water is not H₂O — yet it is known a posteriori, through empirical investigation rather than armchair reflection. "The standard meter bar is one meter long" is known a priori — it is true by definition — yet it seems contingent, since the bar could have been shorter. How can necessity be divorced from the a priori, and the a priori from necessity? Two-dimensional semantics, developed by Robert Stalnaker, Martin Davies, Lloyd Humberstone, David Chalmers, and Frank Jackson, provides the most systematic framework for answering this question.

## The Two Intensions

The central insight is that expressions have two distinct meaning functions, both mapping possible worlds to extensions, but differing in how the world parameter is treated.

The **primary intension** of an expression specifies what it picks out in world w when w is considered as actual — when w is treated as the world the speaker inhabits. It answers: if w were the actual world, what would this term refer to? For "water," the primary intension picks out whatever is the watery stuff of the context world. In our world, that is H₂O; in a Twin Earth world where XYZ fills the oceans and comes out of taps, the primary intension of "water" as used there picks out XYZ.

The **secondary intension** specifies what the expression picks out in world w when w is considered as counterfactual — when the actual world has already been fixed and we ask what the expression refers to across circumstances. The secondary intension is fixed by actuality. For "water," since water is actually H₂O, the secondary intension rigidly picks out H₂O at all counterfactual worlds, including ones where XYZ fills the oceans.

Formally, a two-dimensional intension is a function from ordered pairs ⟨context, circumstance⟩ to extensions, where both coordinates range over possible worlds. The primary intension is the diagonal — the function from contexts to the extension the term has when that context is both actual and evaluated; the secondary intension is a row — the function from circumstances to extensions, with the actual context held fixed.

The framework is often represented as a matrix:

| Primary / Secondary | w₁ | w₂ | w₃ |
|---|---|---|---|
| w₁ (actual) | P(w₁,w₁) | P(w₁,w₂) | P(w₁,w₃) |
| w₂ | P(w₂,w₁) | P(w₂,w₂) | P(w₂,w₃) |
| w₃ | P(w₃,w₁) | P(w₃,w₂) | P(w₃,w₃) |

Each cell P(c,w) gives the truth value of a proposition P when c is the context world and w is the circumstance of evaluation.

## Resolving the Kripkean Puzzles

With this machinery, the puzzles dissolve. "Water is H₂O" is necessary in the secondary sense: given that water is actually H₂O, the secondary intension of "water" rigidly picks out H₂O at every counterfactual world, so there is no circumstance where water fails to be H₂O. This accounts for the necessity. But "water is H₂O" is not a priori in the primary sense: the primary intension of "water" picks out whatever is the watery stuff of the context world, and that could be XYZ rather than H₂O. Whether water is H₂O depends on which world is actual — a matter knowable only empirically. This accounts for the a posteriori status.

"The standard meter bar is one meter long" runs in the opposite direction. The primary intension is: at every context world, the meter bar of that world is one meter long by stipulation — the definition of "meter" in terms of that bar makes this true at every context. The claim is a priori in the primary sense. But the secondary intension fixes the actual length of the bar and evaluates it across circumstances; since the bar could have been manufactured differently, the claim is false at some counterfactual circumstances. Hence: a priori but contingent.

## Chalmers and the Philosophy of Mind

Chalmers deploys the framework in his argument against physicalism. Consider the primary intension of "pain": in any context world, it picks out whatever plays the pain role in that world — the functional state associated with damage-signaling and aversive behavior. Consider the primary intension of "C-fiber firing": in any context world, it picks out C-fiber firing (a natural kind term with rigid reference, or close to it). Now consider a zombie world: a world physically and functionally identical to ours but where there is nothing it is like to be in any mental state. Is such a world conceivable?

Chalmers argues that primary conceivability entails primary metaphysical possibility: if there is no a priori contradiction in the description of a zombie world — no contradiction in the description of a world where pain's functional role is realized without any phenomenal character — then the zombie world is possible in the primary sense. And since the zombie world is one where the physical facts are identical to ours but the phenomenal facts differ, physicalism fails: phenomenal consciousness is not a priori entailed by the physical.

Physicalist critics respond that primary possibility does not entail secondary possibility. The zombie world may be primarily possible (no a priori contradiction at the level of primary intensions) while being secondarily impossible: metaphysically, given what pain actually is, it is necessarily realized by physical processes, even if this is not knowable a priori. The two-dimensional framework generates the tools for both sides to state their positions precisely, but it does not resolve the dispute — it relocates it to the question of whether primary possibility is sufficient for the metaphysical possibility that the zombie argument requires.

## Jackson and the A Priori Entailment Project

Jackson uses the framework in defense of what he calls the a priori entailment project: the thesis that, for any true physical description D of a world and any proposition P, the conditional D → P is knowable a priori if P is true at that world. The primary intension of "good" or "right" picks out, at any context, whatever plays the goodness or rightness role in that context. Since primary intensions are a priori accessible — they capture the functional role a concept plays in our reasoning — moral truths are a priori entailed by full physical descriptions plus an account of which world is actual. This allows Jackson to maintain moral supervenience on natural properties while avoiding reduction of moral concepts to natural ones.

## Assessment

Two-dimensional semantics is the most systematic framework currently available for understanding the relationship between meaning, necessity, and the a priori. Its central achievement is showing how an expression can have both a priori and a posteriori aspects without contradiction: the primary intension is the a priori grip on a concept, the secondary intension is the metaphysically rigid designation fixed by what the world turns out to contain. Whether the framework solves the problems it addresses or merely provides precise vocabulary for restating them is a question on which philosophers remain divided. What is not in dispute is that two-dimensional semantics has become a standard framework for modal semantics, philosophy of language, and the metaphysics of mind, reshaping how these debates are conducted across analytic philosophy.
