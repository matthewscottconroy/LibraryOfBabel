# Separating Model from View

There is a shortcut sitting right there in every event handler, and taking it is
the single most damaging thing you can do to an interactive program.

The shortcut is to keep the program's state in the components — the count in the
label's text, the selection in the list widget. It works, it is less code, and it
produces a program whose rules cannot be tested without a screen and whose logic
dies with the toolkit.

And it is the easiest mistake in the world to make, which is why it is worth a
whole lesson. The listener is right there. The field you need is a component. Every
force acting on you at the moment you write the code is pushing you toward it.

Here it is:

```java
// the thing to avoid
JLabel display = new JLabel("0");

up.addActionListener(e -> {
    int current = Integer.parseInt(display.getText());
    display.setText(String.valueOf(current + 1));
});
```

Look at where the count actually lives. It is in a label's text — which means the
authoritative record of your program's state is a piece of display formatting. To
find out the value you parse a string. To change it you format one.

Now think about where the *rules* have to go. That the count cannot go negative.
That it gets saved to a file. That it resets on Tuesdays. Every one of them has to
live inside a listener, and can only be reached by clicking something.

So that program cannot be tested without a screen. It cannot show the same value in
two places. And it cannot be reused behind any other interface, ever, because there
is no "it" — there is only the window.

## The separation

Two objects with one direction of dependency.

**The model** is what the program is about. It holds the state, enforces the
rules, and knows nothing about how it is displayed.

```java
static final class Counter {
    private int value;
    private final List<IntConsumer> observers = new ArrayList<>();

    int value() { return value; }
    void observe(IntConsumer o) { observers.add(o); }

    void add(int n) {
        if (value + n < 0) throw new IllegalArgumentException("would go negative");
        value += n;
        for (IntConsumer o : observers) o.accept(value);
    }
}
```

**The view** shows it and knows nothing about the rules.

```java
static final class TextView {
    final List<String> painted = new ArrayList<>();
    void render(int v) { painted.add("[ " + v + " ]"); }
}
```

Wiring them:

```java
Counter model = new Counter();
TextView a = new TextView(), b = new TextView();
model.observe(a::render);
model.observe(b::render);
model.add(5); model.add(3); model.add(-2);
```

Verified:

```
model  = 6
view a = [[ 5 ], [ 8 ], [ 6 ]]
view b = [[ 5 ], [ 8 ], [ 6 ]]
```

Two views, both current, neither with any idea the other exists. The model told
both of them because both had asked to be told — which is the Observer pattern
from earlier in the chapter, finally doing the job it was invented for.

And the rule held:

```
rejected: would go negative
model unchanged: 6
```

The invariant is enforced in the model, so it holds no matter which view or
listener attempts the change. In the label version it would have to be duplicated
in every handler that modifies the count.

## What the separation buys

**Testability**, and this is the one that matters most. Read that output and note
what is missing from it:

```
no view attached, value = 10
```

There was no view. None. The model was exercised with nothing attached to it at
all. Every rule the program has can be
tested in milliseconds, with no window, no clicking, and no screen — which means
those tests can run in a build, which means they will actually run.

Interface code is genuinely hard to test: it needs a display, the tests are slow,
and they break when a button moves. So **everything you move out of the view
becomes testable, and everything you leave in effectively is not.**

**Multiple views**, as above. Also a command-line interface over the same model, a
test harness, a log.

**Replaceable interface.** Swing today, JavaFX in three years, a web front end
after that. If the logic is in the model, that is a rewrite of the view. If it is
in listeners, it is a rewrite of the program.

**Comprehensibility.** Chapter 23's question was where a change lands. A change to
the rules touches the model. A change to the appearance touches the view. When
those two are mixed, every change touches everything.

## The dependency direction

One arrow, and it is the whole of the discipline:

```
view ────knows about────> model
view <───notified by ──── model
```

The view knows the model's type and calls its methods. The model does **not** know
the view's type; it holds `IntConsumer`s — Chapter 26's functional interface — and
notifies them.

That asymmetry is what allows the model to exist without any view, and it is the
concrete meaning of "the model knows nothing about the interface".

If you find yourself importing a UI class into your model, the arrow has reversed
and the benefits are gone.

## The pattern family

You will meet several names for arrangements of this idea, and they differ less
than the discourse suggests.

**MVC** — Model, View, Controller. From Smalltalk in 1979. The controller handles
input, the view renders, the model holds state.

**MVP** — Model, View, Presenter. The presenter mediates, and the view is
deliberately dumb, which makes the presenter testable.

**MVVM** — Model, View, ViewModel. The view binds declaratively to a view model.
JavaFX properties, and every modern web framework.

All three are the same claim: **state and rules in one place, presentation in
another, with a defined connection.** Which variant to use is a much smaller
question than whether you have done it at all.

## Where the line goes

This is the part that takes practice, and there is a test from Chapter 23 that
works here unchanged: *would this still make sense if the interface changed?*

**Model**: the count, the rule that it cannot go negative, the file format, the
calculation, the validation, anything about the subject matter.

**View**: the widget layout, the colors, the number formatting, the wording of
the message, the animation.

**The boundary cases**, which are where the arguments happen: input validation,
error message text, undo. The useful default is that the *rule* belongs to the
model and the *wording* to the view — the model rejects a negative count and says
so with an exception; the view decides whether that becomes a red border or a
dialogue box.

## The last connection

Every argument in this lesson has appeared before.

Chapter 19 said put a boundary around an invariant. Chapter 23 said behavior
belongs with its data, and that a class should not know about a format it merely
gets rendered into. Chapter 26 said keep the pure computation separate from the
effects at the edges.

A model and a view is the same claim with a screen attached. And the reason it
gets its own lesson is that interface code is where the principle is hardest to
follow, because the shortcut is always right there in the listener.

Chapter 31 takes the last step outward: several things happening genuinely at
once, on several processors, and on other machines.
