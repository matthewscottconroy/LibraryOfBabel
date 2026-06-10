# Deontic Logic: Obligation and Permission

## The Logic of Norms

**Deontic logic** applies modal operators to *normative* contexts — what is obligatory, permitted, or forbidden:
- $O\varphi$ (Obligatory): $\varphi$ is obligatory — you must do it
- $P\varphi$ (Permitted): $\varphi$ is permitted — you may do it
- $F\varphi$ (Forbidden): $\varphi$ is forbidden — $F\varphi = O\neg\varphi$

The standard system **KD** (Kripke + Seriality) adds:
- $O\varphi \to P\varphi$ (if $\varphi$ is obligatory, it is permitted) — from $D$: $\square\varphi \to \Diamond\varphi$

The **ideal world semantics**: $O\varphi$ means "$\varphi$ holds in all ideal (deontically accessible) worlds." An obligation says what would be true if everyone behaved as they should.

## Applications

**Legal reasoning**: Contracts, laws, and regulations can be represented as deontic formulas. Automatic consistency checking: does a set of laws contain contradictions ($O\varphi \wedge O\neg\varphi$)?

**Software specifications**: Access control policies — "users without admin rights may not delete system files" — are deontic statements.

**Ethics**: Deontic logic provides formal machinery for exploring ethical theories (utilitarian, deontological) and their consistency.

## The Paradoxes of Deontic Logic

Deontic logic is plagued by fascinating paradoxes:

**Ross's paradox**: If "Post the letter" is obligatory ($O(P)$), then "Post the letter or burn it" ($O(P \vee B)$) also seems obligatory — but this seems wrong. (Standard deontic logic validates this by distribution.)

**Good Samaritan paradox**: "You ought to help the robbery victim" implies "There ought to be a robbery victim" in KD. This counterintuitive result arises from the interaction of obligation and existential import.

**Chisholm's paradox**: Contrary-to-duty obligations (what you ought to do given that you violated an obligation) are very difficult to represent consistently in standard deontic logic.

These paradoxes motivate various non-standard deontic systems — an active area of philosophical logic research.

## Exercises
See [problems/ch12_modal_logic/04_deontic_exercises.md](../../../problems/ch12_modal_logic/04_deontic_exercises.md)
