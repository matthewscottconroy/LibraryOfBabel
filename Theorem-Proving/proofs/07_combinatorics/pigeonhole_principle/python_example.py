"""Pigeonhole Principle: demonstrations in Python."""

from itertools import combinations
from collections import Counter


def birthday_pigeonhole(n_people: int) -> bool:
    """Return True if n_people definitely share a birthday (n_people > 366)."""
    return n_people > 366


def suit_pigeonhole(hand):
    """Among 5+ cards, show two cards share a suit."""
    suits = [card[-1] for card in hand]  # e.g. 'H', 'S', 'D', 'C'
    counts = Counter(suits)
    return {suit: cnt for suit, cnt in counts.items() if cnt >= 2}


# Demo: 5 integers, two have same remainder mod 4
numbers = [3, 7, 11, 15, 2]  # remainders: 3,3,3,3,2 — obvious collision
remainders = {n: n % 4 for n in numbers}
print("Numbers:", numbers)
print("Remainders mod 4:", remainders)
# Find collision
rev = Counter(remainders.values())
print("Repeated remainders:", {r: c for r, c in rev.items() if c > 1})
print()

# Card hand example
hand = ['2H', '5H', 'KS', '7D', 'AC', 'QH']
print(f"Hand: {hand}")
print(f"Suits with 2+ cards: {suit_pigeonhole(hand)}")
print()

# Birthday: minimum group for guaranteed shared birthday
print(f"Need > 366 people for guaranteed shared birthday: 367+ people needed")
print(f"366 people: {birthday_pigeonhole(366)}  (NOT guaranteed)")
print(f"367 people: {birthday_pigeonhole(367)}  (guaranteed)")
