# Components and Layout

Drag a window's corner and everything inside rearranges itself. Nobody wrote code
for your particular window size.

The obvious way to place things — this button at pixel 10, 40 — cannot do that, and
it also breaks when the font is larger, when the text is translated into German,
and when the display has different pixel density. All four happen, which is why
every toolkit ever built has the same answer.

Open any application you like and look at it as a structure rather than a picture.

A window is a tree.

```
JFrame
└── JPanel                      (content)
    ├── JLabel   "Count:"
    ├── JLabel   "0"
    └── JPanel                  (buttons)
        ├── JButton "+"
        └── JButton "-"
```

Each node is a **component** — it knows its own size and position, it can draw
itself, and it can be sent events. Some of them, the **containers**, hold others.

And that is the model in every graphical toolkit anybody has built. The DOM in a
browser is this tree. Android's view hierarchy is this tree. Swap the vocabulary
and the structure survives intact, which is the first hint that learning this one
was not wasted.

## A minimal window

```java
public static void main(String[] args) {
    SwingUtilities.invokeLater(() -> {
        JFrame frame = new JFrame("Counter");
        frame.setDefaultCloseOperation(JFrame.EXIT_ON_CLOSE);

        JLabel display = new JLabel("0");
        JButton up = new JButton("+");
        up.addActionListener(e -> ...);

        JPanel panel = new JPanel();
        panel.add(display);
        panel.add(up);

        frame.setContentPane(panel);
        frame.pack();
        frame.setVisible(true);
    });
}
```

Most of that is construction. One line is doing something you would never guess
at, so find it before reading on: which line is load-bearing in a way the others
are not?

It is `SwingUtilities.invokeLater`, and it is not decoration.

Components have to be built and touched on the event dispatch thread, and `main` is
not that thread. Build your interface directly in `main` and it will usually work —
which is precisely the problem. A thing that usually works is the worst kind of bug
there is, because it passes every test you run and fails on somebody else's
machine.

`pack()` asks the tree to size itself from its contents. `setVisible(true)` shows
it and — importantly — returns immediately. `main` then ends, and the program
keeps running, because the event dispatch thread is still going. That surprises
people the first time.

## Layout

The hard part of interface construction is not creating components; it is
deciding where they go when the window is resized.

So toolkits use **layout managers**: an object attached to a container that
computes its children's positions from a policy.

| manager | policy |
|---|---|
| `FlowLayout` | left to right, wrapping. The default for `JPanel`. |
| `BorderLayout` | five regions: north, south, east, west, center. |
| `GridLayout` | equal-sized cells in a grid. |
| `BoxLayout` | a single row or column. |
| `GridBagLayout` | a constraint-based grid. Powerful and unpleasant. |

Real layouts nest them: a `BorderLayout` frame with a toolbar in the north, a
drawing area in the center, and a `FlowLayout` panel of buttons in the south.

The general principle, and it is the transferable one:

> **Describe the relationships, not the positions.** Let the layout compute the
> pixels.

That is exactly what CSS flexbox and grid do, what Android's `ConstraintLayout`
does, and what SwiftUI and every declarative toolkit does. Absolute positioning is
wrong everywhere for the same reasons.

## Sizing

Three sizes per component: minimum, preferred, and maximum. A layout manager reads
them, applies its policy, and assigns actual bounds — which may be none of the
three, because the container may not have room.

Notice that this is a negotiation and not a command. Your component does not
*have* a size; it has an opinion about its size, which the parent may overrule. And
that is the source of nearly all layout frustration, including the kind that makes
people give up on layout managers entirely and start positioning things by pixel.

So when something will not shrink, go looking for a minimum size somewhere in the
tree. When something will not grow, go looking for a maximum.

Which gives you the one piece of advice that will save you the most time here:
**when a layout misbehaves, look at the container rather than the component.** The
child only asked. The parent decided.

## Toolkits, briefly

Java has had three, and knowing which you are looking at saves confusion.

**AWT** (1995) wrapped the operating system's own widgets. Applications looked
native and behaved differently on each platform, which defeated the purpose.

**Swing** (1998) draws everything itself. Identical everywhere, pluggable look and
feel, and it is in every JDK. It is also visibly of its era and no longer actively
developed.

**JavaFX** (2008) is the modern one, with CSS styling, a scene graph, and
data binding. It is a separate dependency since Java 11.

For learning the concepts, Swing is fine and requires no setup. For a real desktop
application today you would use JavaFX, or more likely not build a desktop
application at all.

That last observation is worth stating plainly rather than implying. Most
interfaces are now web pages or mobile applications, and the desktop toolkit you
learn is unlikely to be the one you use. What survives the move is the tree, the
layout principle, the event loop, and the separation in Section 30.2.3 — all four
of which are the same in a browser.

Next: drawing.
