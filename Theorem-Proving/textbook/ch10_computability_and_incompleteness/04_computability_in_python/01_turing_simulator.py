"""
Turing Machine Simulator in Python
Chapter 10, Section 4

A simple deterministic TM simulator. Demonstrates the Church-Turing thesis
by making computation explicit and mechanical.
"""

from dataclasses import dataclass, field
from typing import Dict, Optional, Tuple


@dataclass
class TuringMachine:
    """
    Deterministic Turing Machine.
    Tape is represented as a defaultdict; head starts at position 0.
    """
    states: set
    input_alphabet: set
    tape_alphabet: set      # must include blank ('#')
    transitions: Dict[Tuple[str, str], Tuple[str, str, str]]
    start_state: str
    accept_state: str
    reject_state: str
    blank: str = '#'

    def run(self, input_string: str, max_steps: int = 10_000) -> Tuple[bool, str, list]:
        """
        Run the TM on input_string.
        Returns (accepted, final_state, trace).
        """
        tape: Dict[int, str] = {i: c for i, c in enumerate(input_string)}
        head = 0
        state = self.start_state
        steps = 0
        trace = []

        while state not in (self.accept_state, self.reject_state):
            symbol = tape.get(head, self.blank)
            key = (state, symbol)
            trace.append((state, head, dict(tape)))

            if key not in self.transitions:
                # No transition: implicit reject
                return False, self.reject_state, trace

            new_state, write_symbol, direction = self.transitions[key]
            tape[head] = write_symbol
            state = new_state
            head += 1 if direction == 'R' else -1
            steps += 1

            if steps >= max_steps:
                raise RuntimeError(f"TM exceeded {max_steps} steps — possible infinite loop")

        return state == self.accept_state, state, trace


def make_equal_01_tm():
    """
    TM for {0^n 1^n | n >= 0}.
    Strategy: repeatedly mark first 0 as 'X', scan to first 1, mark as 'Y', return.
    """
    return TuringMachine(
        states={'q0', 'q1', 'q2', 'q3', 'q4', 'qa', 'qr'},
        input_alphabet={'0', '1'},
        tape_alphabet={'0', '1', 'X', 'Y', '#'},
        transitions={
            # q0: scan for unmarked 0
            ('q0', '0'): ('q1', 'X', 'R'),   # mark 0, go right
            ('q0', 'Y'): ('q3', 'Y', 'R'),   # all 0s marked, check for remaining 1s
            ('q0', '#'): ('qa', '#', 'R'),   # empty input: accept
            # q1: scan right over 0s and Ys to find 1
            ('q1', '0'): ('q1', '0', 'R'),
            ('q1', 'Y'): ('q1', 'Y', 'R'),
            ('q1', '1'): ('q2', 'Y', 'L'),   # mark 1, go back left
            ('q1', '#'): ('qr', '#', 'R'),   # no 1 found: reject
            # q2: scan left back to X
            ('q2', '0'): ('q2', '0', 'L'),
            ('q2', 'Y'): ('q2', 'Y', 'L'),
            ('q2', 'X'): ('q0', 'X', 'R'),   # found X, go to start of remaining
            # q3: verify no 1s remain
            ('q3', 'Y'): ('q3', 'Y', 'R'),
            ('q3', '#'): ('qa', '#', 'R'),   # done, accept
            ('q3', '1'): ('qr', '1', 'R'),   # extra 1: reject
        },
        start_state='q0',
        accept_state='qa',
        reject_state='qr',
    )


if __name__ == '__main__':
    tm = make_equal_01_tm()
    test_cases = [
        ('', True),
        ('01', True),
        ('0011', True),
        ('000111', True),
        ('0001111', False),
        ('0010', False),
        ('1', False),
        ('0', False),
    ]
    print("Testing TM for {0^n 1^n}:")
    for s, expected in test_cases:
        accepted, state, _ = tm.run(s)
        status = 'PASS' if accepted == expected else 'FAIL'
        print(f"  [{status}] input='{s}' accepted={accepted} (expected {expected})")
