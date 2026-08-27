# Exercises

**23.1** Find a class in your own earlier code that is nothing but fields and
getters, with the logic that uses it living somewhere else. Move one computation
into the class. Then say what the callers no longer need to know.

**23.2** Write the `CountingHashSet` from Section 23.1.2 and confirm it reports 6
for three elements added with `addAll`. Then remove the `addAll` override and run
it again. Explain why the second version is correct today and still a bad design.

**23.3** Rewrite `CountingSet` so that it implements `Set<E>` and forwards every
method to an inner set. Count the methods you had to write. Then say what you
gained over the inheritance version, in one sentence.

**23.4** Take a class with six or more fields. For each field, list the methods
that use it. If two clusters appear, split the class along them and report the
before and after field counts.

**23.5** For each of these, name the coupling strength from Section 23.1.3's list
and suggest a weaker alternative: (a) `private ArrayList<Item> items;`
(b) `private final Logger log = new FileLogger("app.log");` (c) `class Report
extends AbstractExporter` (d) `static int totalRequests;`

**23.6** Draw a class diagram for the flashcard design of Section 23.2.2 before
reading the code again. Four boxes, the relationships between them, and the
multiplicities. Then compare with the code and note anything you got wrong.

**23.7** Add a third `Scheduler` to the flashcard program — one that doubles the
interval on GOOD and halves it on HARD. Confirm you changed exactly one file, and
that the existing output is unchanged.

**23.8** The design's weak point was that adding a timing parameter to `next`
touches four files. Implement the `ReviewEvent` record fix. Then argue, in a
paragraph, whether it should have been in the original design. There is no
required answer; the argument is the exercise.

**23.9** Write a `DeckStore` that saves a `Deck` to a text file and loads it back.
`Deck` may not change. If you find you need to add a method to `Deck`, say which
one and why the boundary was in slightly the wrong place.

**23.10** *Design.* [carries forward] Return to the library from Exercises 21.9
and 22.11 one last time. Redesign it applying this chapter: name the nouns, decide
what is a record and what is a class, identify one thing that should be an
interface because it might vary, and name three likely changes with the number of
files each touches. Write the design down before writing any code. Unit VI's
evaluator will exercise the same skill on a harder problem.

**23.11** *Longer.* Take a program you wrote for an earlier chapter that is
longer than about a hundred lines. Do not modify it. Instead, write one page: what
the classes are, where you would draw the boundaries differently now, and which of
this chapter's principles each change appeals to. Being able to criticize your own
design is the skill; rewriting it is optional.
