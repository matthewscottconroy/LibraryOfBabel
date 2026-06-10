# Proof: The Free Group Satisfies the Universal Property

## Statement

Let S be a set. The *free group* F(S) is defined by generators S and no relations. We show F(S) satisfies the universal property: for any group G and any function f : S → G, there exists a unique group homomorphism f̄ : F(S) → G with f̄ ∘ ι = f (where ι : S → F(S) is the inclusion).

## Construction

Elements of F(S) are *reduced words* over S ∪ S⁻¹: finite sequences of symbols s and s⁻¹ (s ∈ S) with no consecutive pair s, s⁻¹ or s⁻¹, s.

Group operations:
- **Multiplication**: concatenation followed by reduction (cancel adjacent inverses)
- **Identity**: the empty word ε
- **Inverse**: reverse the word and negate each symbol

## Proof of Universal Property

**Existence**: Define f̄ on generators by f̄(s) = f(s) and f̄(s⁻¹) = f(s)⁻¹, extended to words by:
```
f̄(x₁ x₂ ... xₙ) = f̄(x₁) · f̄(x₂) · ... · f̄(xₙ)
```

This is well-defined on reduced words (the image of a reduced word is the same regardless of intermediate cancellation).

f̄ is a homomorphism: f̄(w₁ · w₂) = f̄(reduce(w₁w₂)) = f̄(w₁) · f̄(w₂) (since reduction doesn't change the group element represented).

**Uniqueness**: Any homomorphism h : F(S) → G with h ∘ ι = f must satisfy:
- h(s) = f(s) for all s ∈ S (from the condition)
- h(s⁻¹) = h(s)⁻¹ = f(s)⁻¹ (homomorphism property)
- h(x₁...xₙ) = h(x₁)·...·h(xₙ) (homomorphism on words)

So h = f̄. Uniqueness follows. ∎

## Categorical Formulation

The universal property says: the inclusion ι : S → F(S) is initial in the category of pairs (G, f : S → G). This is exactly the statement that F is left adjoint to the forgetful functor U : **Grp** → **Set**.

The unit of the adjunction is ι itself. The free group construction is the *canonical* way to turn a set into a group with no additional structure beyond what the universal property forces.
