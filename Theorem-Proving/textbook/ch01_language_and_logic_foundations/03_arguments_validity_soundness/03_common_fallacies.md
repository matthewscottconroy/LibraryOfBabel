# Common Fallacies

## Overview
A **fallacy** is an error in reasoning. **Formal fallacies** violate the structural rules of
logic. **Informal fallacies** involve irrelevant premises, ambiguous language, or faulty
assumptions. Recognizing fallacies is as important as constructing valid proofs.

## Learning Objectives
- Identify major formal fallacies (affirming the consequent, denying the antecedent)
- Identify major informal fallacies (ad hominem, strawman, false dichotomy)
- Explain *why* each fallacy fails

## Formal Fallacies

**Affirming the consequent**: P→Q, Q ∴ P
This is invalid — Q might be true for reasons unrelated to P.

**Denying the antecedent**: P→Q, ¬P ∴ ¬Q
Invalid — Q might still hold via other means.

**Undistributed middle**: All A are B, All C are B ∴ All A are C
Invalid — B might be large enough to contain both A and C without them overlapping.

## Informal Fallacies (Selection)
- **Ad hominem**: attacking the person rather than the argument
- **Strawman**: misrepresenting an opponent's position to make it easier to attack
- **False dichotomy**: presenting only two options when more exist
- **Begging the question**: using the conclusion as a premise (circular reasoning)
- **Appeal to authority**: citing authority as proof rather than evidence

## Tool Connections
- **Lean 4 / Coq**: formal fallacies simply fail to type-check — the proof assistant refuses them
- **Python**: automated argument mining tools attempt to detect fallacy patterns in text

## Exercises
See `problems/ch01_language_and_logic_foundations/03_validity_soundness_challenges.md`
