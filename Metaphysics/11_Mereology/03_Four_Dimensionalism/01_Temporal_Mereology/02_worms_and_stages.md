# Worms and Stages

Four-dimensionalism comes in two main variants, distinguished by what they take to be the primary bearers of properties and what they take ordinary discourse to be about. Both accept temporal parts; they disagree about which entities are fundamental and how talk of persisting objects should be analyzed.

## The Worm View

Lewis (*On the Plurality of Worlds*, 1986; *Parts of Classes*, 1991) and Sider (*Four-Dimensionalism*, 2001) defend the worm view. On this account, the persisting object — a person, a table, a planet — is a four-dimensional worm, the mereological sum of all its instantaneous temporal parts. Ordinary predicates like "is sitting" or "is happy" are *temporally relativized*: to say "O is sitting at t" is to say that the temporal part of O at t is sitting simpliciter. Identity through time is *mereological*: to say "the person at t₁ is the same person as the person at t₂" is to say that the temporal parts of the worm at t₁ and t₂ are parts of the same worm.

Formally, let W(x) mean "x is a worm" and TP(s, x, t) mean "s is the temporal part of x at t":

- The property *is-sitting-at-t* applied to worm W holds iff Sitting(TP(W, t)) — the temporal part of W at t has the property *sitting* simpliciter
- W₁ = W₂ iff ∀t ∀s [TP(s, W₁, t) ↔ TP(s, W₂, t)] — worms are identical iff they share all temporal parts

The sentence "John is sitting," uttered at time t, is analyzed as: the temporal part of John-worm at t is sitting. Ordinary predications of the form "Fx" are implicitly indexed to a time: F(x) at t iff F(TP(x, t)).

## The Stage View

Katherine Hawley (*How Things Persist*, 2001) and Sider (in some writings) explore the stage view. Here the primary objects are *stages* — instantaneous or brief temporal slices. What we ordinarily call "persisting objects" are not four-dimensional worms but momentary stages. Persistence is analyzed via a *temporal counterpart relation* among stages: what makes a past stage and a present stage part of "the same person" is that they bear appropriate temporal counterpart relations to each other.

Formally, let S(x) mean "x is a stage" and TC(s, s') mean "s is the temporal counterpart of s'":

- "John was a child" (said now, of present-stage s) is analyzed as: ∃s' [TC(s', s) ∧ s' is-at-a-past-time ∧ Child(s')]
- "John will be happy" is analyzed as: ∃s'' [TC(s'', s) ∧ s'' is-at-a-future-time ∧ Happy(s'')]
- "John is the same person as the person born thirty years ago": ∃s''' [TC(s, s''') ∧ s''' is-born-thirty-years-ago]

On the stage view, there is no persisting worm that John *is*; John *is* a momentary stage. Persistence is not mereological (stages don't compose a worm that is John) but relational (stages are temporally counterpart-related to each other).

## Comparing the Views

Both share the four-dimensionalist commitment to temporal parts and both deny endurantism. Their disagreement is about what ordinary discourse is about.

On the *worm view*, "John" refers to the four-dimensional worm; "John is happy" is implicitly relativized to a time. The worm is the primary bearer; temporal parts have properties simpliciter, and these are used to analyze time-indexed worm predicates. The identity claims of ordinary discourse come out literally true: "I am the same person who was born thirty years ago" is true because the worm that is me includes the temporal part from thirty years ago.

On the *stage view*, "John" refers to the present temporal stage; "John was happy" means there is a past counterpart-related stage that is happy. Stages are the primary bearers; they have properties simpliciter without relativization. Ordinary predication is literal: "John is happy" is simply true because John (= the present stage) has the property *happy* directly. But the identity claims of ordinary discourse come out false: "I am the same person who was born thirty years ago" is strictly false on the stage view, because I (the present stage) am not identical to any stage from thirty years ago.

On the *temporary intrinsics problem*: the worm view handles straight/bent by making temporal parts the direct instantiators of these properties, used to analyze time-indexed worm predicates — perhaps inelegant. The stage view has properties had directly and simpliciter by stages, without any time-indexing.

## The Formal Relation Between the Views

The worm view and stage view are formally inter-translatable. Given a stage ontology with temporal counterpart relations, we can define worm-objects as the fusions of counterpart-related stage sequences:

W(s) = σs' [TC*(s, s')]     (the worm of stage s is the fusion of all stages temporally counterpart-related to s)

Conversely, given a worm ontology, we can define stages as the instantaneous temporal parts of worms:

Sₜ(W) = σs [TP(s, W, t)]

The two frameworks generate equivalent formal structures. The choice between them is partly a matter of which entities we take as fundamental and partly a matter of how we analyze ordinary discourse.

## Advantages and Disadvantages

The worm view's main advantage is that ordinary identity claims come out literally true; worms are the natural entities of a spacetime ontology. Its disadvantage is that ordinary predication must be time-indexed, which is a revision of ordinary semantics; and the worm is not wholly present at any time, conflicting with ordinary intuitions about what it is for an object to *exist* at a time.

The stage view's main advantage is that ordinary predication is literal and stages are wholly present at their time. It handles cases of personal identity indeterminacy more naturally: if it is unclear whether two stages are counterparts, this is a gradable uncertainty about the counterpart relation, not about the identity of a worm. Its disadvantage is that ordinary identity claims come out false, requiring revisionary semantics; and some find it intuitively odd to say that I, the person now speaking, am a momentary stage rather than a persisting thing.

## The Worm-Stage Debate and Personal Identity

The debate has special significance for personal identity. On the worm view, the question "is person A at t₁ the same person as person B at t₂?" is a mereological question: are A's t₁-stage and B's t₂-stage parts of the same worm? On the stage view, it is a question about counterpart relations: is B's t₂-stage a temporal counterpart of A's t₁-stage?

Derek Parfit's discussion in *Reasons and Persons* (1984) is relevant here. Parfit argues that what matters in survival is not strict identity but psychological continuity — which corresponds more naturally to the counterpart relation than to mereological overlap. If Parfit is right, the stage view may be better suited to the conclusion that personal identity is not what matters in survival, while the worm view is better suited to the view that it is.
