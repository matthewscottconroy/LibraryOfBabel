# Compression

English carries about one bit per character and is stored at eight. That gap is
not an inefficiency anyone overlooked; it is what a fixed-width encoding costs, and
Chapter 1 predicted it without being able to say how much.

This section closes the gap, and then proves a limit on how far anyone can.

Two lessons.

Variable-length codes: giving frequent symbols short codes, the prefix property
that makes them decodable, and Huffman's algorithm, which is optimal and which
hits the entropy bound exactly in a case we can check. Then the proof that no
compressor shrinks everything — three lines of counting, and 200 out of 200
verified.
