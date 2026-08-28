# References

Two things can be equal without being the same thing, and once that sentence stops
sounding like a distinction without a difference, most of this chapter is behind
you.

Two five-pound notes are interchangeable — anyone would swap one for the other
without a thought — and they are still two notes, with different serial numbers,
and burning one does not burn the other. Programs deal with both facts constantly,
and the language spells them very similarly.

Three lessons, all of which Chapter 12 introduced and then deferred.

The reference model first — what is actually in a variable of object type, stated
once more because everything else depends on it. Then aliasing: two names for one
object, and why it produces bugs that look impossible. Then copying, and the
distinction between copying one level and copying all of them, which is where
`clone` misleads people.
