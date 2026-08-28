# Building a Window

The toolkit in this section is thirty years old and no longer the one you would
choose. That is deliberate, and worth saying plainly.

Java has had three GUI libraries and the industry has largely moved to browsers
and phones. Memorising widget names is a poor use of your attention. What does not
change is the tree, the layout principle, the drawing model, and the separation at
the end — all four of which are the same in a browser, and the last of which is the
part worth keeping.

Three lessons.

Components and layout, briefly, because the concepts transfer and the API does
not. Then drawing, which is where a program stops arranging pre-made parts and
puts pixels down itself — and where the event loop's rules become visible. Then
the separation of model from view, which is the part of this chapter worth
remembering in five years.
