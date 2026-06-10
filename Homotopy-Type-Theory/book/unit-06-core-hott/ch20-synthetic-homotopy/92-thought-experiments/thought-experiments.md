# Thought Experiments: Synthetic Homotopy Theory

## Thought Experiment 1: The Covering Space Bureaucracy

Imagine you are trying to prove that the loop space of the circle is equivalent to the integers. The classical route through covering spaces requires you to:

1. Define the universal cover R → S¹.
2. Prove R is contractible (which requires ε-δ analysis).
3. Prove the covering map is a fibration (which requires the theory of fiber bundles).
4. Apply the long exact sequence of homotopy groups of the fibration.
5. Conclude π₁(S¹) = Z.

Each step is entirely correct. But each step is also an *import* — you are importing a piece of mathematical machinery (covering spaces, ε-δ topology, fiber bundles, long exact sequences) that was built to solve a different problem and adapted to this one.

Now consider the HoTT proof. You define code : S¹ → Type by code(base) = Z and ap_code(loop) = ua(succ). This single definition says: "as you go around the loop, the counting type transforms by the successor function." Everything that follows — the encode function, the decode function, the round-trip proofs — is just unpacking what this definition already contains.

The question: what is the relationship between these two proofs? They prove the same theorem. But are they the same proof?

One answer: no, they are genuinely different. The classical proof uses real analysis and general topology. The HoTT proof uses only type theory. The HoTT proof is shorter, more general, and constructive. They are different proofs of the same fact.

Another answer: they are the same proof at different levels of abstraction. The covering space R → S¹ is the total space of the code family. Transport along loop is the deck transformation of the cover. The monodromy of the covering space is exactly what the code family computes. The two proofs are the same mathematical object, expressed in different languages.

Which answer is right? Probably both, at different levels. But thinking through this equivalence reveals something important: the HoTT proof is not a *simplification* of the classical proof. It is the classical proof's mathematical core, stripped of its set-theoretic implementation overhead and re-expressed in a language where the core is the entire proof.

## Thought Experiment 2: What Makes a Good Code Family?

The encode-decode method depends entirely on finding the right code family: a type family code : X → Type such that code(basepoint) is the group you want and transport around each generator does what the generator does.

Suppose you wanted to compute π₁(RP²) = Z/2Z using encode-decode directly (without van Kampen). How would you define the code family?

RP² has the HIT presentation with base : RP², loop : base = base, and surf : loop · loop = refl (the "double loop is trivial" relation). A code family code : RP² → Type must satisfy:
- code(base) = Z/2Z (the group we want)
- transport^code(loop) = some automorphism of Z/2Z
- transport^code(loop · loop) = transport^code(refl) = id

The last condition says the automorphism must square to the identity. The only non-trivial automorphism of Z/2Z is the swap 0 ↔ 1. Does swapping square to the identity? Yes: swap ∘ swap = id. So: code(base) = Z/2Z and transport^code(loop) = swap.

This works. The encode-decode method would then prove π₁(RP²) = Z/2Z synthetically, without van Kampen.

The lesson: finding the right code family *is* the mathematical content of the theorem. Once you have the right code, the proof is mechanical. The creative act is recognizing which automorphism of which type family captures the topology of the loop.

This is a genuine reorientation. In the classical proof, the mathematical content is in the long exact sequence and the covering space construction. In the HoTT proof, the mathematical content is in the code family. Same theorem; different locations of the mathematical intelligence.

## Thought Experiment 3: The Algebraist's Objection to van Kampen

A pure algebraist might object to the HoTT van Kampen theorem as follows: "You say the fundamental group of a pushout is the amalgamated free product of the fundamental groups. But that's just the universal property of the amalgamated free product — it follows immediately from the universal property of the pushout. You haven't proved anything; you've just observed that two universal properties agree."

Engage with this objection. Is it right?

In a sense, yes. The HoTT proof of van Kampen is indeed the observation that two universal properties agree: maps from π₁(P) to a group G (pushout of groups = amalgamated free product) correspond to group homomorphisms from π₁(A) and π₁(B) that agree on π₁(C). This is a consequence of the universal property of the pushout P and the universal property of the amalgamated free product.

But this is not "not proving anything." Observing that two universal properties agree *is* the theorem. The classical proof of van Kampen spends 10 pages constructing the amalgamated free product directly, using explicit word representations and cancellation lemmas. The HoTT proof takes 3 paragraphs to note that the universal properties agree and concludes immediately.

The HoTT proof is shorter because it uses more powerful tools — specifically, the fact that HITs come with universal properties by definition. The classical proof must construct the universal property from the ground up because the classical spaces don't come equipped with it.

The algebraist's objection reveals something true: van Kampen is fundamentally a statement about universal properties. The HoTT proof makes this manifest. The classical proof obscures it behind geometric constructions that are ultimately serving to establish a universal property that could have been stated directly.

## Thought Experiment 4: Stable Homotopy and the Illusion of Complexity

The table of homotopy groups of spheres seems to show a bewildering variety of groups: Z, Z/2Z, Z/12Z, Z/2Z, Z/120Z, ... The pattern seems chaotic, without rhyme or reason.

The Freudenthal theorem reveals that much of this complexity is illusory. For each fixed k, the groups πₙ₊ₖ(Sⁿ) stabilize as n → ∞. The "stable" groups are:

- π₁ˢ = Z/2Z
- π₂ˢ = Z/2Z
- π₃ˢ = Z/24Z
- π₄ˢ = 0
- π₅ˢ = 0

The individual groups πₙ₊ₖ(Sⁿ) are not chaotic — they are all the same group, once n is large enough.

Now think about what Freudenthal's theorem is really saying. The suspension functor Σ takes an n-sphere to an (n+1)-sphere. The theorem says: in a range that grows with n, the map πₖ(Sⁿ) → πₖ₊₁(Sⁿ⁺¹) induced by suspension is an isomorphism.

So the stable groups are not groups of specific spheres — they are groups of "spheres in the limit." A stable homotopy class is not a map between two specific spheres; it is a consistent system of maps between spheres of all dimensions, all related by suspension.

The stable homotopy groups are thus more fundamental than the unstable groups. They describe the intrinsic topology of the sphere-in-the-limit, not the accidental features of any particular sphere.

Question: is this a simplification or a complication? In one sense, simplification: instead of infinitely many groups (one for each sphere), you have one stable group (the same group for all spheres in the stable range). In another sense, complication: computing the stable groups requires understanding all spheres simultaneously, not just one.

The Freudenthal theorem in HoTT makes this precise: it gives an explicit range in which the suspension map is an isomorphism. This range is computable from the connectivity of the sphere. The HoTT proof thus makes Freudenthal not just a theorem but a computation.

## Thought Experiment 5: What Is the Brunerie Number, Really?

Guillaume Brunerie proved that π₄(S³) = Z/|β|Z for some integer β, and then showed (by computer) that β = 2. The computation required running a Cubical Agda program.

What is the Brunerie number β, intuitively?

β is the image of the generator of π₄(S³) under a specific map related to the Hopf invariant. More precisely: the Hopf fibration gives a map η : S³ → S², and the suspension of η gives Ση : S⁴ → S³. The Brunerie number is related to how many times you need to "go around" the fiber of Ση before you get a nullhomotopic map.

But there is something strange here. The proof *in HoTT* established that π₄(S³) = Z/|β|Z for *some* integer β, but couldn't determine β without external computation. This means: the type-theoretic proof was incomplete without the computer verification.

Is this a deficiency of the proof? Or is it a feature?

One perspective: it is a feature. The proof is modular. The mathematical core (π₄(S³) = Z/|β|Z) is proved by a purely mathematical argument. The computational core (β = 2) is verified by a separate computation. Separating the mathematical and computational content is good engineering.

Another perspective: it is surprising that a purely mathematical proof could fail to determine a specific integer. In classical mathematics, if you prove X = Z/|β|Z, you usually determine β along the way. The fact that HoTT can prove the structure (cyclic group) without determining the order is unusual. It reflects the fact that some type expressions in HoTT are well-typed and well-specified, but not *normalizable* without substantial computation.

The resolution: HoTT is a computational system, and some computations are hard. The Brunerie number β is a specific type expression that normalizes to 2 — but normalization requires many reduction steps. Brunerie's human proof established the structure; the computer's verification established the value. Together, they give a complete formally verified proof.

## Thought Experiment 6: When Is a Synthetic Proof Better Than a Classical One?

We have seen several cases where the HoTT proof is simpler, shorter, or more general than the classical proof:
- π₁(S¹) = Z: 3 pages in HoTT vs 50+ pages in classical topology.
- Van Kampen: Universal property argument vs. geometric path construction.
- Freudenthal/Blakers-Massey: Constructive inductive argument vs. spectral sequences.

But is the HoTT proof always better? Consider: the HoTT proof requires building all the infrastructure of HoTT — identity types, transport, HITs, the Univalence Axiom. A student who knows classical topology might find the classical proof more transparent, because it uses more familiar tools.

Here is the question: for a given theorem, what makes one proof *better* than another?

Some criteria:
1. **Length**: The HoTT proofs are often shorter.
2. **Generality**: The HoTT proofs often work in more general settings (any ∞-topos, not just spaces).
3. **Constructivity**: The HoTT proofs are often constructive (give computable witnesses).
4. **Formalizability**: The HoTT proofs can be directly checked by a proof assistant.
5. **Transparency**: The HoTT proofs make the mathematical structure more visible.
6. **Accessibility**: The HoTT proofs require different prerequisites than classical proofs.

On criteria 1-5, HoTT often wins. On criterion 6, it depends on what the reader already knows.

The deeper question: is there a mathematical fact that the HoTT proof is *wrong about* in some sense? Probably not — if both proofs are correct, they prove the same theorem. But one proof might give more *insight* into why the theorem is true.

The claim of this chapter is that the synthetic proofs give more insight, because they force the mathematical content into the foreground. When you define code(base) = Z and transport^code(loop) = succ, you are not just *finding* the right structure — you are *stating* the theorem in its own language. The proof is the theorem, expressed in the right notation.

Whether that is better or worse depends on what you think proofs are for. If they are for convincing skeptics, length might not matter. If they are for understanding, the more transparent proof wins. If they are for computer verification, formalizability wins. The HoTT proofs score well on all three.
