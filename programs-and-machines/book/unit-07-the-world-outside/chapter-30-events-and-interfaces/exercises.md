# Exercises

**30.1** Implement the event loop from Section 30.1.1 with your own event types.
Post five events including a `Quit` in the middle, and confirm that events behind
the `Quit` are never dispatched.

**30.2** Register two listeners for the same event type and confirm they both run,
in registration order. Then have the first one throw and observe what happens to
the second. Say what a real toolkit should do here.

**30.3** Add a handler to your loop that sleeps for 500 milliseconds. Post a
second event behind it and measure how long that event waited. Relate the number
to what a user would experience.

**30.4** Write a Swing window with a button that sleeps for three seconds in its
listener. Click it and try to move or resize the window. Describe exactly what
happens. Then fix it with a background thread and `invokeLater`, and describe the
difference.

**30.5** Print `SwingUtilities.isEventDispatchThread()` from `main` and from
inside `invokeLater`. Report both. Then explain why `main` building the interface
directly is a bug that usually appears to work.

**30.6** Build a window with a `BorderLayout`: a label in the north, a custom
panel in the center, and two buttons in a `FlowLayout` panel in the south. Resize
the window and describe which parts change size and which do not.

**30.7** Write a `paintComponent` that draws a bar chart from an `int[]` field.
Then add a call that increments a counter inside `paintComponent`, print it, and
resize the window several times. Explain why this is a bug.

**30.8** Draw two adjacent rectangles with `drawRect` at x and x+w. Zoom in and
count the pixels between them. Explain the result using Section 30.2.2.

**30.9** Turn antialiasing on and off in a `paintComponent` that draws diagonal
lines and text. Describe the difference and say whether you would ever leave it
off.

**30.10** Write a `Counter` model with an invariant and an observer list, plus two
different views. Confirm both update. Then write a test for the invariant with no
view attached at all, and note how long the test takes to run.

**30.11** Take a small interactive program you or someone else has written where
state lives in a component. Refactor the state into a model. List what became
testable.

**30.12** *Design, no code.* A user clicks Save. The file write takes four
seconds. Describe exactly what should happen in the interface: which thread does
what, what the user sees at each moment, what happens if they click Save again
during the write, and what happens if the write fails.

**30.13** *Longer.* [carries forward] Build a window over Chapter 25's
interpreter: a text area for the program, a Run button, and an output area. The
interpreter must not be modified, the evaluation must not block the event loop,
and an `EvalError` must be reported in the output area rather than crashing. Keep
it; Chapter 31 makes it serve other machines.
