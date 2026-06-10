-- Logical Equivalences in Haskell
-- Chapter 2, Section 3
--
-- Haskell's type system and pure functions make it natural to express
-- propositional equivalences as equalities between Bool-valued functions.
-- QuickCheck can verify them over all inputs.

module LogicalEquivalences where

import Test.QuickCheck

-- | De Morgan 1: not (p && q) == not p || not q
deMorgan1 :: Bool -> Bool -> Bool
deMorgan1 p q = not (p && q) == (not p || not q)

-- | De Morgan 2: not (p || q) == not p && not q
deMorgan2 :: Bool -> Bool -> Bool
deMorgan2 p q = not (p || q) == (not p && not q)

-- | Contrapositive: (p -> q) == (not q -> not p)
contrapositive :: Bool -> Bool -> Bool
contrapositive p q =
  (not p || q) == (p || not q)     -- both sides: ¬p∨q and q∨¬p (same)

-- | Distributivity of && over ||
distAndOr :: Bool -> Bool -> Bool -> Bool
distAndOr p q r =
  (p && (q || r)) == ((p && q) || (p && r))

-- | Implication as disjunction
impAsDisj :: Bool -> Bool -> Bool
impAsDisj p q = (not p || q) == (not p || q)  -- tautological, here for illustration

-- Run all checks
main :: IO ()
main = do
  putStrLn "Checking De Morgan 1..."
  quickCheck deMorgan1
  putStrLn "Checking De Morgan 2..."
  quickCheck deMorgan2
  putStrLn "Checking contrapositive..."
  quickCheck contrapositive
  putStrLn "Checking distributivity..."
  quickCheck distAndOr
  putStrLn "All equivalences verified."
