-- Lambda calculus in Haskell
-- Haskell is essentially typed lambda calculus + laziness + IO

module LambdaCalculus where

-- =====================================================
-- Lambda calculus is Haskell's foundation
-- =====================================================
-- Every Haskell function is a lambda:
--   f x = x + 1  is sugar for  f = \x -> x + 1

-- The identity combinator (SKI: I)
i :: a -> a
i = \x -> x

-- The constant combinator (SKI: K)
k :: a -> b -> a
k = \x _ -> x

-- The S combinator: S f g x = f x (g x)
s :: (a -> b -> c) -> (a -> b) -> a -> c
s = \f g x -> f x (g x)

-- SKI completeness: any lambda term can be expressed with S, K, I
-- For example: I = S K K
i' :: a -> a
i' = s k k  -- (S K K) x = K x (K x) = x ✓

-- =====================================================
-- Church Encodings
-- =====================================================
-- Natural numbers as iteration counts

type Nat = forall a. (a -> a) -> a -> a

zero :: Nat
zero = \f x -> x

one :: Nat
one = \f x -> f x

two :: Nat
two = \f x -> f (f x)

succ' :: Nat -> Nat
succ' n = \f x -> f (n f x)

add :: Nat -> Nat -> Nat
add m n = \f x -> m f (n f x)

mul :: Nat -> Nat -> Nat
mul m n = \f -> m (n f)

-- Convert to Int for display
toInt :: Nat -> Int
toInt n = n (+1) 0

-- Church booleans
type Bool' = forall a. a -> a -> a

true' :: Bool'
true' = \t _ -> t

false' :: Bool'
false' = \_ f -> f

ifThenElse :: Bool' -> a -> a -> a
ifThenElse b t f = b t f

-- Church pairs
type Pair a b = forall c. (a -> b -> c) -> c

pair :: a -> b -> Pair a b
pair x y = \f -> f x y

fst' :: Pair a b -> a
fst' p = p (\x _ -> x)

snd' :: Pair a b -> b
snd' p = p (\_ y -> y)

-- =====================================================
-- Reduction strategies (approximated in Haskell)
-- =====================================================

-- Haskell uses call-by-need (lazy): arguments evaluated only when needed
-- This is normal-order reduction (outermost first) with sharing

-- Infinite structures are possible due to laziness:
nats :: [Int]
nats = [0..]

-- fibs: works because Haskell is lazy (call-by-need)
fibs :: [Integer]
fibs = 0 : 1 : zipWith (+) fibs (tail fibs)

first10Fibs :: [Integer]
first10Fibs = take 10 fibs  -- [0,1,1,2,3,5,8,13,21,34]

-- =====================================================
-- Fixed points and the Y combinator
-- =====================================================

-- In a lazy language, we can define the Y combinator directly:
-- (In strict languages, Y diverges due to eager evaluation)

fix :: (a -> a) -> a
fix f = let x = f x in x  -- Haskell's laziness makes this work

-- Define factorial via fix (no explicit recursion)
factorial :: Int -> Int
factorial = fix (\self n -> if n <= 0 then 1 else n * self (n-1))

-- =====================================================
-- Curry-Howard in Haskell types
-- =====================================================

-- A -> B is a proof that "given A, we can produce B"
-- Product type (a, b) is a proof of A ∧ B
-- Sum type Either a b is a proof of A ∨ B
-- Void (empty type) is ⊥

-- Modus ponens: (A -> B) -> A -> B
modusPonens :: (a -> b) -> a -> b
modusPonens f x = f x  -- function application!

-- Conjunction introduction: A -> B -> A ∧ B
conjIntro :: a -> b -> (a, b)
conjIntro x y = (x, y)

-- Disjunction introduction: A -> A ∨ B
disjIntroL :: a -> Either a b
disjIntroL = Left

-- Hypothetical syllogism (transitivity): (A→B) -> (B→C) -> (A→C)
hypoSyll :: (a -> b) -> (b -> c) -> (a -> c)
hypoSyll = flip (.)  -- function composition!
