# Exercises

**33.1** Compute by hand the information in bits of: a fair coin landing heads; a
fair die showing 3; drawing the ace of spades from a shuffled deck; a specific
lottery ticket winning at odds of one in fourteen million. Show the arithmetic.

**33.2** Write a method computing the entropy of a string from its character
frequencies. Confirm it gives 0 for `"aaaa"`, 1 for `"abab"`, and 3 for a string of
eight equally frequent characters. Then run it on a paragraph of English and report
the result.

**33.3** Plot or tabulate the entropy of a biased coin for p from 0 to 1 in steps
of 0.1. Confirm the maximum is at 0.5 and explain in one sentence why the curve is
symmetric.

**33.4** A source emits four symbols with probabilities 1/2, 1/4, 1/8, 1/8.
Compute its entropy by hand. Then build the Huffman code and compute the average
code length. Explain why they are equal.

**33.5** Repeat 33.4 with probabilities 0.4, 0.3, 0.2, 0.1. Report the entropy and
the Huffman average, and confirm the gap is under one bit.

**33.6** Implement Huffman coding. Encode a paragraph of English and report the
compressed size in bits, the entropy-implied minimum, and the original ASCII size.

**33.7** Show that the code `a -> 0, b -> 1, c -> 01` is ambiguous by finding a bit
string with two decodings. Then explain what the prefix property forbids.

**33.8** Look up UTF-8's byte patterns and explain why it is a prefix code. Then
say what property of the encoding lets a decoder resynchronize after a corrupted
byte, and why a fixed-width encoding cannot.

**33.9** *Measurement.* Compress three 9,000-byte inputs with `gzip`: one repeated
character, one paragraph of English repeated, and one of random letters. Report all
three ratios alongside each input's per-character entropy, and explain why the
English row beats what its entropy alone predicts.

**33.10** *Measurement.* Compress 200 random 64-byte inputs and count how many
grew. Then explain the result using the counting argument, and say what the
compressor should have done instead.

**33.11** Prove the counting argument from Section 33.2.2 in your own words. Then
compute what fraction of 1000-bit strings can be compressed to 990 bits or fewer.

**33.12** Compress a file, then compress the result, then again. Report the three
sizes. Explain the trend in terms of what the first pass removed.

**33.13** *Design, no code.* You are storing ten million sensor readings, each an
integer between 0 and 1023, where consecutive readings usually differ by less than
4. Propose an encoding. Estimate its size in bits per reading and justify the
estimate with an entropy argument.

**33.14** *Reading, no code.* [carries forward] Section 33.1.2 claims English carries about one bit
per character. Test it on yourself: take a paragraph you have not read, cover it,
and guess it one letter at a time, counting your guesses. Shannon's experiment,
and your number will be closer to his than you expect.
