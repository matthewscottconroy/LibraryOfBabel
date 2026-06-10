-- Functions as mathematical objects in Haskell
-- Haskell is a pure functional language; all functions are mathematical functions

module FunctionsInHaskell where

import Data.List (nub, sort)
import Data.Maybe (fromJust)

-- Function composition: (g . f) x = g (f x)
-- The (.) operator is built-in in Haskell
compose :: (b -> c) -> (a -> b) -> (a -> c)
compose g f = \x -> g (f x)

-- Identity function
identity :: a -> a
identity x = x

-- Demonstrating f . id = f and id . f = f
prop_id_left :: (a -> b) -> a -> b
prop_id_left f x = (identity . f) x  -- = f x

prop_id_right :: (a -> b) -> a -> b
prop_id_right f x = (f . identity) x  -- = f x

-- Injective (one-to-one): different inputs give different outputs
-- Can only check on finite domains
isInjective :: (Eq a, Eq b) => [a] -> (a -> b) -> Bool
isInjective domain f =
  let outputs = map f domain
  in length outputs == length (nub outputs)

-- Surjective (onto): every codomain element is hit
-- Requires a finite codomain
isSurjective :: (Eq b) => [a] -> [b] -> (a -> b) -> Bool
isSurjective domain codomain f =
  all (\y -> y `elem` map f domain) codomain

-- Bijective: both injective and surjective
isBijective :: (Eq a, Eq b) => [a] -> [b] -> (a -> b) -> Bool
isBijective domain codomain f =
  isInjective domain f && isSurjective domain codomain f

-- Example: check doubling on {0,1,2,3,4} -> {0,1,2,3,4}
example1 :: Bool
example1 = isInjective [0..4] (\x -> x * 2 `mod` 5)
-- True: x |-> 2x mod 5 is a bijection on Z/5Z

-- Inverse of a bijection (on finite domain)
inverseOf :: (Eq b) => [a] -> (a -> b) -> (b -> a)
inverseOf domain f y = fromJust $ lookup y [(f x, x) | x <- domain]

-- Fixed points: x such that f(x) = x
fixedPoints :: (Eq a) => [a] -> (a -> a) -> [a]
fixedPoints domain f = filter (\x -> f x == x) domain

-- Cantor's diagonal: a function cannot enumerate all functions Nat -> Bool
-- This shows the idea (on finite prefixes)
diagonal :: (Int -> (Int -> Bool)) -> (Int -> Bool)
diagonal enum n = not (enum n n)

-- The diagonal function differs from every enum n at position n
-- demonstrating uncountability of {0,1}^ℕ

-- Higher-order functions as function-space inhabitants
-- (A -> B) -> (B -> C) -> (A -> C) is composition
-- This is a map on function spaces
applyTwice :: (a -> a) -> a -> a
applyTwice f = f . f

-- Church encoding of natural numbers (the lambda-calculus view)
-- n is represented as \f x -> f^n x
type Church = forall a. (a -> a) -> a -> a

zero :: Church
zero = \_ x -> x

one :: Church
one = \f x -> f x

succ' :: Church -> Church
succ' n = \f x -> f (n f x)

add :: Church -> Church -> Church
add m n = \f x -> m f (n f x)

-- Convert Church numeral to Int
toInt :: Church -> Int
toInt n = n (+1) 0
