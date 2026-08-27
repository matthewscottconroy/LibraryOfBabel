# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

Do the conversions by hand. You will not do this again after this chapter, which
is exactly why it is worth doing now.

## Conversions

**2.1.** Convert to decimal: `1010`, `11111`, `10000000`, `01010101`.

**2.2.** Convert to 8-bit binary: 3, 31, 100, 127, 128, 255.

**2.3. [carries forward]** Convert to hexadecimal by regrouping, not by arithmetic:
`11001010`, `1111000010100101`, `00010010001101000101011001111000`.

**2.4.** Convert to binary: `0x2A`, `0xFF`, `0x100`, `0xDEAD`.

**2.5.** Without converting to decimal, say which is larger: `0x7F` or
`0b10000000`. Explain how you knew.

## Arithmetic

**2.6. [carries forward]** Add these 8-bit patterns by hand, showing carries. State the result in
binary and decimal, and say whether it overflowed under an unsigned reading.
- `00101101 + 00010011`
- `10000000 + 10000000`
- `11111111 + 00000001`

**2.7. [carries forward]** Give the 8-bit two's complement pattern for −1, −2,
−64, −127, −128. What do you notice about the pattern for −1, and can you say why
it must be that?

**2.8.** Use flip-and-add-one to negate `00010110`. Then negate your answer and
confirm you get back where you started.

**2.9.** Compute 20 − 35 in 8-bit two's complement by adding 20 to the pattern
for −35. Show the discarded carry. Confirm the result reads as −15.

**2.10.** Under an 8-bit *signed* reading, what is `10000000 + 11111111`? Did
signed overflow occur? Apply the same-sign rule from the chapter.

## Reasoning

**2.11.** The chapter said signed overflow can only happen when adding two
numbers of the same sign. Prove it. (Hint: if the operands have opposite signs,
where must the true result lie relative to them?)

**2.12. [carries forward]** In 8-bit two's complement, what does multiplying a
pattern by 2 do to its bits? What does dividing by 2 do? Try it with +6 and with
−6, and say what has to happen at the leftmost bit for division to work for
negatives.

**2.13.** Explain why `low + (high - low) / 2` cannot overflow when `low` and
`high` are both non-negative and `low <= high`, while `(low + high) / 2` can.

**2.14.** A programmer proposes detecting overflow in `a + b` by checking whether
the result is smaller than `a`. For which of unsigned and signed arithmetic does
this work? Give a counterexample for the one where it fails.

## Going further

**2.15.** Design a 4-bit encoding for the numbers −4 through +4 inclusive. You
have sixteen patterns and nine values to place. Say what you do with the leftover
seven patterns, and defend the choice — every real format has had to answer this
question.

**2.16.** Two's complement makes subtraction into addition. Is there an analogous
trick that makes division into multiplication? Consider what you would need, and
why it is harder. (You are not expected to succeed; the point is to see what the
obstacle is.)

**2.17.** The 2038 problem arises from a signed 32-bit seconds counter starting
in 1970. If the counter were unsigned instead, when would it wrap? If it were
signed 64-bit, roughly how long would it last? Is there any practical difference
between the last answer and "forever"?

**2.18.** Find out what balanced ternary is, and work out how it represents
negative numbers. Then say, in two or three sentences, what it gains over two's
complement and what it gives up. (Chapter 1's profile of Brusentsov is a starting
point.)
