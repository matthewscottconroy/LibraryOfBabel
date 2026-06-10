# Applied Exercises

The simply typed lambda calculus and System F are not just theoretical constructs — they are the direct foundations of the type systems in OCaml, Haskell, and Scala, and the basis for the type-checking algorithms that run every time you compile a modern functional program. The exercises below connect the theory of this chapter to programming practice: implementing type inference, exploiting parametricity, encoding data structures as types, and building interpreters that are themselves type-safe by construction. Each exercise asks you to take a theoretical result from the chapter and make it concrete.

---

## Exercise B.1: Implementing Algorithm W
*Domain: Type System Implementation / Functional Programming*

**Setup:** Algorithm W is the practical realization of Hindley's principal types theorem. Given a term without type annotations, it infers the principal type by collecting type constraints and solving them via unification. This exercise guides you through implementing Algorithm W for STLC extended with let-polymorphism.

**Questions:**
1. Implement a type inference algorithm for the following small language in Haskell or OCaml:
   ```
   t ::= x                   -- variable
       | lam x t             -- lambda abstraction (no annotation)
       | app t t              -- application
       | let x = t in t      -- let binding
       | (t, t)               -- pair
       | fst t | snd t        -- projections
   ```
   Your implementation should:
   - Represent types as a data type with type variables (`TVar`), function types (`TArr`), and product types (`TProd`).
   - Implement unification: given two types, either produce a substitution that makes them equal or return an error.
   - Implement Algorithm W: given a term and an initial environment, return the principal type and accumulated substitution.
   - Implement let-generalization: at a `let x = t1 in t2`, generalize all free type variables in the type of `t1` (that don't appear in the environment) to a type scheme.

2. Test your implementation on:
   - `lam x (lam y x)` — should give `a -> b -> a` (the $K$ combinator)
   - `lam f (lam x (app f x))` — should give `(a -> b) -> a -> b` (function application)
   - `let f = lam x x in app f (app f 5)` — should give `Int` (using `5 : Int`)
   - `lam x (app x x)` — should fail (self-application requires $\alpha = \alpha \to \beta$, which has no solution)

3. Add `fix :: (a -> a) -> a` to the language (general recursion). What happens to strong normalization? Test `fix (lam f (lam n (if n == 0 then 1 else n * f (n-1))))` (factorial). Now argue: why does adding `fix` break the correspondence with logic? Specifically, what formula does the type `(a -> a) -> a` correspond to, and why is it logically problematic?

*Abstract concept illustrated: The principal types theorem; Hindley-Milner type inference; the relationship between strong normalization and logical consistency.*

---

## Exercise B.2: System F in Haskell — RankNTypes and the ST Monad
*Domain: Advanced Haskell / Type System Design*

**Setup:** Haskell's `RankNTypes` extension allows types of the form `forall a. T` to appear anywhere in a type — this is a fragment of System F. One of the most elegant applications is the `ST` monad, which uses a phantom type parameter and rank-2 types to enforce that mutable state cannot escape its scope. This exercise explores both.

**Questions:**
1. Enable `{-# LANGUAGE RankNTypes #-}` and implement the following:
   ```haskell
   -- The polymorphic identity at rank 2: you must pass a truly polymorphic function
   applyToBoth :: (forall a. a -> a) -> (b, c) -> (b, c)
   applyToBoth f (x, y) = (f x, f y)
   ```
   Why does `applyToBoth (\x -> x) (1, True)` work, but `applyToBoth (\x -> 0) (1, True)` fail? (The second function is not truly polymorphic — it ignores its input and returns a `Num a => a`, but the `forall a.` quantifier is instantiated before the caller gets to choose.) What type error does GHC report?

2. The `runST` function has type:
   ```haskell
   runST :: (forall s. ST s a) -> a
   ```
   The type variable `s` is a "phantom tag" — it never appears in values, only in types. The rank-2 quantifier ensures that the `s` in the `ST` action cannot escape. Explain in your own words why the following should (and does) fail:
   ```haskell
   badST :: STRef s Int
   badST = runST (newSTRef 42)  -- type error: s escapes
   ```
   Draw the type derivation for `runST (newSTRef 42)` and identify exactly where the type unification fails.

3. The rank-2 type `forall s. ST s a` corresponds to a System F type $\forall \alpha. T(\alpha, A)$ where $\alpha$ does not appear in $A$. What proposition does this type correspond to under the Curry-Howard correspondence? (Hint: $\forall \alpha. \alpha \to A$ is not quite right — think about what $ST$ computes.) How does the scoped-state guarantee relate to the logical principle that a universally quantified variable is "fresh"?

*Abstract concept illustrated: System F's universal types; the separation of type and term abstraction; parametricity as a safety guarantee.*

---

## Exercise B.3: Parametricity and Free Theorems
*Domain: Equational Reasoning / Haskell*

**Setup:** Reynolds's parametricity theorem says that any well-typed polymorphic function must "commute with relational liftings." In practice, this means you can derive non-trivial equations about a function purely from its type. This exercise works through several such derivations and then verifies them by testing.

**Questions:**
1. From the type `reverse :: [a] -> [a]`, the parametricity theorem guarantees:
   ```haskell
   map f (reverse xs) = reverse (map f xs)
   ```
   for any `f :: a -> b`. Explain informally why this must be true: `reverse` cannot look at the values (only their positions), so it doesn't matter whether you transform them before or after reversing. Now state the analogous free theorem for:
   - `sort :: Ord a => [a] -> [a]` (is a free theorem possible here? Why or why not?)
   - `filter :: (a -> Bool) -> [a] -> [a]`
   - `foldr :: (a -> b -> b) -> b -> [a] -> b`

2. The type `id :: forall a. a -> a` has only one inhabitant (the identity function, by parametricity). Prove this informally: any function `f :: forall a. a -> a` must satisfy `f x = x` for all `x`. (Hint: apply the relational lifting with `R = {(x, x) | x : a}` — the diagonal relation.)

3. Consider `head :: [a] -> a` (partial — undefined on empty lists). The parametricity theorem guarantees `head . map f = f . head`. Verify this in Haskell for concrete `f` and `xs`. Now consider a hypothetical `mystery :: [a] -> [a]` with the free theorem `map f . mystery = mystery . map f` for all `f`. Give three possible implementations of `mystery` consistent with this theorem. Then: is there a *unique* function satisfying this theorem for all lists and all `f`? Argue why or why not.

*Abstract concept illustrated: Reynolds's parametricity theorem; free theorems; the behavioral constraints imposed by polymorphic types.*

---

## Exercise B.4: Church Encodings and Data as Proofs
*Domain: Type Theory / Haskell with RankNTypes*

**Setup:** In System F, all basic data types can be *encoded* as polymorphic function types — the Church encodings. A Church-encoded boolean `CBool = forall a. a -> a -> a` is a function that picks one of two values; a Church-encoded natural number `CNat = forall a. (a -> a) -> a -> a` is a function that applies a given function $n$ times to a base value. These encodings reveal the deep connection between iteration, recursion, and quantification.

**Questions:**
1. In Haskell with `RankNTypes`, define:
   ```haskell
   type CBool = forall a. a -> a -> a
   type CNat  = forall a. (a -> a) -> a -> a

   ctrue  :: CBool
   cfalse :: CBool
   czero  :: CNat
   csucc  :: CNat -> CNat
   ```
   Then implement:
   - `cif :: CBool -> a -> a -> a` (Church-encoded conditional)
   - `cplus :: CNat -> CNat -> CNat` (addition of Church numerals)
   - `cmult :: CNat -> CNat -> CNat` (multiplication)
   - `toNat :: CNat -> Int` (convert to a Haskell `Int`)
   Verify `toNat (cplus (csucc czero) (csucc (csucc czero))) == 3`.

2. The Church encoding of pairs is `CPair a b = forall c. (a -> b -> c) -> c`. Define:
   ```haskell
   type CPair a b = forall c. (a -> b -> c) -> c
   cmkpair :: a -> b -> CPair a b
   cfst :: CPair a b -> a
   csnd :: CPair a b -> b
   ```
   Using pairs, define *Church predecessor* for natural numbers: `cpred :: CNat -> CNat`. (This is non-trivial — the standard trick uses pairs `(n-1, n)` updated at each step. The predecessor of zero is zero.)

3. The type `CBool = forall a. a -> a -> a` is a proposition in second-order logic. What proposition is it? (Write it in logical notation.) What does having a term of this type — say `ctrue` — tell you about the inhabitedness of that proposition? Now consider the type `forall a. a`. What proposition is this? Can you write a term of this type? What does this tell you about the logical system System F models?

*Abstract concept illustrated: Church encodings as System F terms; the connection between data and proofs in second-order logic; the Church-Rosser theorem and confluence.*

---

## Exercise B.5: A Type-Safe Interpreter Using GADTs
*Domain: Verified Software / Haskell*

**Setup:** A classic problem in writing interpreters is "type errors at runtime" — the interpreter has to check dynamically whether you're adding numbers vs. booleans. GADTs (Generalized Algebraic Data Types) let you represent the *typing derivation* as a data structure, so that a well-typed expression is literally an expression paired with a proof of its type. The interpreter then becomes a function `eval :: Expr a -> a`, which is total and type-safe.

**Questions:**
1. Enable `{-# LANGUAGE GADTs, KindSignatures #-}` and define a GADT for a simply-typed expression language:
   ```haskell
   data Expr :: * -> * where
     Lit    :: Int -> Expr Int
     BoolLit :: Bool -> Expr Bool
     Add    :: Expr Int -> Expr Int -> Expr Int
     If     :: Expr Bool -> Expr a -> Expr a -> Expr a
     Pair   :: Expr a -> Expr b -> Expr (a, b)
     Fst    :: Expr (a, b) -> Expr a
     Snd    :: Expr (a, b) -> Expr b
   ```
   Implement `eval :: Expr a -> a`. Observe that there is no case for "type mismatch" — it is impossible by construction to write `Add (BoolLit True) (Lit 5)`, because `BoolLit True :: Expr Bool` and `Add` requires `Expr Int`.

2. Add lambda abstraction and application to the language. This requires a representation of typed environments. A clean approach uses a *typed heterogeneous list*:
   ```haskell
   data Env :: [*] -> * where
     ENil  :: Env '[]
     ECons :: a -> Env as -> Env (a ': as)
   
   data Var :: [*] -> * -> * where
     VZ :: Var (a ': as) a
     VS :: Var as a -> Var (b ': as) a
   ```
   Add `Var :: Var ctx a -> Expr ctx a` and `Lam :: Expr (a ': ctx) b -> Expr ctx (a -> b)` and `App :: Expr ctx (a -> b) -> Expr ctx a -> Expr ctx b` to the GADT. Implement `eval :: Env ctx -> Expr ctx a -> a`.

3. Compare this typed interpreter to a naive untyped one:
   ```haskell
   data UExpr = ULit Int | UAdd UExpr UExpr | UIf UExpr UExpr UExpr | ...
   data Val    = VInt Int | VBool Bool | VPair Val Val | ...
   eval :: UExpr -> Either String Val  -- can fail with type error message
   ```
   What is the type-theoretic relationship between the two approaches? (Hint: the GADT `Expr a` is indexed by the type `a`, which corresponds to a type family in dependent type theory.) What would you need — beyond what Haskell offers — to also represent *dependently typed* expressions (where the type of a subexpression depends on a runtime value)?

*Abstract concept illustrated: GADTs as indexed types; the connection between typing derivations and data; the relationship between STLC and the first-order fragment of dependent type theory.*
