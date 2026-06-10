-- Recursion and Structural Induction in Haskell
-- Chapter 7, Section 4

module Recursion where

-- ── Lists ─────────────────────────────────────────────────────

-- Structural recursion over lists mirrors structural induction
myLength :: [a] -> Int
myLength []     = 0
myLength (_:xs) = 1 + myLength xs

myAppend :: [a] -> [a] -> [a]
myAppend []     ys = ys
myAppend (x:xs) ys = x : myAppend xs ys

myReverse :: [a] -> [a]
myReverse []     = []
myReverse (x:xs) = myReverse xs ++ [x]

-- Property: length (xs ++ ys) = length xs + length ys
-- Proof by structural induction on xs:
-- Base: length ([] ++ ys) = length ys = 0 + length ys = length [] + length ys
-- Step: length ((x:xs) ++ ys) = length (x : (xs ++ ys))
--     = 1 + length (xs ++ ys)
--     = 1 + length xs + length ys  [IH]
--     = length (x:xs) + length ys  QED

-- ── Binary trees ──────────────────────────────────────────────

data Tree a = Leaf | Node (Tree a) a (Tree a) deriving (Show, Eq)

height :: Tree a -> Int
height Leaf         = 0
height (Node l _ r) = 1 + max (height l) (height r)

size :: Tree a -> Int
size Leaf         = 0
size (Node l _ r) = 1 + size l + size r

-- Property: size t < 2^(height t + 1)
-- Proved by structural induction (exercise below)

insert :: Ord a => a -> Tree a -> Tree a
insert x Leaf = Node Leaf x Leaf
insert x (Node l v r)
  | x < v    = Node (insert x l) v r
  | x > v    = Node l v (insert x r)
  | otherwise = Node l v r

inorder :: Tree a -> [a]
inorder Leaf         = []
inorder (Node l v r) = inorder l ++ [v] ++ inorder r

-- ── Natural numbers ───────────────────────────────────────────

-- Peano naturals (illustrative; use Int/Integer in practice)
data Peano = Zero | Succ Peano deriving (Show)

peanoAdd :: Peano -> Peano -> Peano
peanoAdd Zero     m = m
peanoAdd (Succ n) m = Succ (peanoAdd n m)

peanoToInt :: Peano -> Int
peanoToInt Zero     = 0
peanoToInt (Succ n) = 1 + peanoToInt n

-- ── Exercises ─────────────────────────────────────────────────

-- 1. Prove (by structural induction, written as a comment):
--    length (myReverse xs) = length xs

-- 2. Implement `flatten :: Tree a -> [a]` and prove (comment):
--    length (flatten t) = size t

-- 3. Implement `mapTree :: (a -> b) -> Tree a -> Tree b` and state
--    the functor laws as properties to verify.
