# Category Theory Exercises

## Basic Definitions

1. Verify the category axioms for:
   a. **Rel**: objects = sets, morphisms = binary relations (composition = relational composition)
   b. **Mat_k**: objects = natural numbers, Hom(m,n) = n×m matrices over field k
   c. A poset (P, ≤) viewed as a category: is there at most one morphism between any two objects?

2. In a category, prove that identity morphisms are unique. (Hint: use unitality twice.)

3. Prove that isomorphisms in **Set** are exactly bijections.

## Functors

4. Check the functor laws for:
   a. The power set functor 𝒫: **Set** → **Set** (sends f : A → B to the image function f_* : 𝒫(A) → 𝒫(B))
   b. The forgetful functor U : **Grp** → **Set**

5. Is there a functor F : **Set** → **Set** with F(∅) = {∗} and F({∗}) = ∅? If so, construct it. If not, explain why.

6. Natural transformations: construct a natural transformation η : id_**Set** → 𝒫 (the singleton map A → 𝒫(A), a ↦ {a}). Verify naturality.

## Limits and Colimits

7. Compute the following in **Set**:
   a. Product A × B (as a limit)
   b. Coproduct A + B (as a colimit)
   c. Equalizer of f, g : A → B
   d. Pushout of f : C → A and g : C → B

8. Show that the terminal object in **Set** is any one-element set, and the initial object is ∅.

## Adjunctions

9. The free-forgetful adjunction: F ⊣ U where F : **Set** → **Grp** (free group) and U : **Grp** → **Set** (forgetful).
   a. Describe the unit η_A : A → UFА.
   b. Describe the counit ε_G : FUG → G.
   c. Verify the triangle identities.

10. Show that right adjoints preserve limits and left adjoints preserve colimits. (Prove for products/coproducts.)

## Monads

11. The List monad in **Set**: T(A) = the set of finite lists over A.
    - Define the unit η_A : A → List(A) and multiplication μ_A : List(List(A)) → List(A).
    - Verify the monad laws.

12. **Bonus**: Show that every adjunction F ⊣ G gives rise to a monad T = G ∘ F. Identify the unit and multiplication in terms of the adjunction's unit and counit.
