# Key Concepts

**Boolean value.** One of exactly two things, true or false. Chapter 1's bit, used
for logic rather than arithmetic.

**Truth table.** A complete specification of an operator, listing every input
combination. Complete because the inputs are finite — which is what makes
verification by exhaustion possible here and impossible for programs generally.

**AND, OR, NOT, XOR.** True when both; true when at least one; reversal; true
when the inputs differ. Java writes them `&&`, `||`, `!`, `^`. Note that OR is
*inclusive*, unlike much English usage.

**Sixteen operators.** Four input rows, each independently true or false, gives
$2^{4}$ possible two-input operators. That the set can be counted is Chapter 1's
counting argument applied to functions.

**Functional completeness.** NAND alone can express every boolean function:
`NOT A` is `A NAND A`, and AND and OR follow. This is why a manufacturer able to
make one reliable gate type can build any circuit.

**Gate.** A physical arrangement implementing a truth table. Transistors in
series give AND; in parallel, OR.

**Shannon's 1937 observation.** Boole's algebra of logic describes switching
circuits exactly. This is what made circuits designable — calculable, provable,
derivable from a specification — rather than merely buildable.

**The half adder.** For one column of binary addition, the sum column of the
truth table is XOR and the carry column is AND. Two gates. Chapter 2's arithmetic
derived from logic.

**The full adder.** Three inputs including carry-in.
`sum = (A XOR B) XOR carryIn`;
`carryOut = (A AND B) OR ((A XOR B) AND carryIn)`.
Chain 32 of them for 32-bit addition — and the thirty-second carry-out having
nowhere to go is exactly why `int` arithmetic wraps.

**Multiplexer and latch.** `(S && B) || (!S && A)` is `if` in hardware.
Cross-coupled NOR gates hold a bit, which is how a circuit remembers, which is
how Chapter 6's state physically exists.

**Boolean algebra.** Commutative, associative, distributive (both ways),
identity, annihilation, double negation, idempotence, complement. Each provable
by exhausting the truth table.

**De Morgan's laws.** `!(A && B)` is `!A || !B`; `!(A || B)` is `!A && !B`. The
negation moves inward and flips the operator. The practical use is turning an
unreadable negated conjunction into something a person would say aloud.

**Negating a comparison inverts it.** The opposite of `>=` is `<`, not `<=`.
Getting this wrong is an off-by-one that fires on exactly one input.

**`if` requires a boolean.** Java will not accept a number, which makes
`if (x = 5)` a compile error. The protection lapses when the variable is itself
`boolean`.

**`else if` is not a construct.** It is an `if` inside an `else` with the braces
omitted, which is why branches are tested in order and why later conditions may
assume earlier ones were false.

**Always use braces.** A brace-less conditional makes adding a second statement
change control flow invisibly — the shape of Apple's 2014 "goto fail" defect.

**Guard clauses.** Inverting conditions and returning early flattens nested
conditionals and puts the main path at the left margin.

**Short-circuit evaluation.** `&&` stops if the left is false; `||` stops if the
left is true. `&` and `|` always evaluate both. This is what lets a left operand
*guard* a right one, as in `s != null && s.length() > 0`, and the ordering is
required rather than stylistic.

**`&&` is control flow.** Because evaluation can stop partway, `A && B` and
`B && A` may differ in behavior even though boolean algebra says they are equal.
The laws hold for expressions that terminate normally.

**`switch`.** One value, many cases. The arrow form is an expression, allows
several labels per arm, and does not fall through. The older colon form falls
through unless you write `break` — a default that is usually wrong, which is a
design error the arrow form corrects.

**Exhaustiveness.** A `switch` over an enum needs no `default`, and stops
compiling if a new constant is added. That is a failure delivered at exactly the
right time.

**Jump tables.** Integer `switch` can compile to `tableswitch` or `lookupswitch`,
selecting an arm in constant time. Chapter 6's "consult state to choose a
continuation", implemented as arithmetic on the program counter.
