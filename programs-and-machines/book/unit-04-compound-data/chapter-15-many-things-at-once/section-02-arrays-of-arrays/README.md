# Arrays of Arrays

A chessboard, a spreadsheet, an image, a table of results. Grids are everywhere,
and Java has no grid.

What it has is arrays of arrays, which is *nearly* the same thing — and the places
where it is not the same thing are the source of every two-dimensional array bug
worth knowing about.

Three lessons on two-dimensional data.

The first observes that Java has no two-dimensional arrays at all — what it has is
arrays whose elements are arrays, which behaves almost the same and differs in
ways worth knowing. The second is the consequence: rows may have different
lengths, which is occasionally useful and occasionally a trap. The third is
traversal, and the surprising fact that the order in which you walk a grid can
change how fast your program runs by a large factor.
