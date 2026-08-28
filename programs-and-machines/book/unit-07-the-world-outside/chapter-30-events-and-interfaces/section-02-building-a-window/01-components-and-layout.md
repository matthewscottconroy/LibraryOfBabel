# Components and Layout

Drag a window's corner and everything inside rearranges itself. Nobody wrote code
for your particular window size.

The obvious way to place things — this button at pixel 10, 40 — cannot do that, and
it also breaks when the font is larger, when the text is translated into German,
and when the display has different pixel density. All four happen, which is why
every toolkit ever built has the same answer.

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

Each node is a **component**: it knows its size and position, it can draw itself,
and it can receive events. Some components — **containers** — hold others.

That is the model in every toolkit. HTML's DOM is this tree. Android's view
hierarchy is this tree. The names differ and the structure does not.

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

Six lines of construction and one of them is the important one.

`SwingUtilities.invokeLater` wraps the whole thing, and it is not decoration.
Components must be built and touched on the event dispatch thread, and
`main` is not it. Building the interface directly in `main` usually works and
occasionally does not, which is the worst kind of bug.

`pack()` asks the tree to size itself from its contents. `setVisible(true)` shows
it and — importantly — returns immediately. `main` then ends, and the program
keeps running, because the event dispatch thread is still going. That surprises
people the first time.

## Layout

The hard part of interface construction is not creating components; it is
deciding where they go when the window is resized.

The naive approach is absolute positioning: `setBounds(10, 10, 100, 30)`. It works
until the window is resized, or the font is larger, or the text is translated into
German, or the display has a different pixel density. All four happen.

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

This is a negotiation rather than a command, and it is the source of most layout
frustration. A component that will not shrink usually has a minimum size somewhere
in the tree; a component that will not grow usually has a maximum.

The practical advice: **when a layout misbehaves, look at the container, not the
component.** The child asked; the parent decided.

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
