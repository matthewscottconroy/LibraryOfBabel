# Drawing

Pre-made components handle most interfaces. Sometimes you need to put pixels down
yourself — a chart, a game, a diagram, a visualization of a data structure — and
the mechanism is one method.

```java
class Canvas extends JPanel {
    @Override
    protected void paintComponent(Graphics g) {
        super.paintComponent(g);              // clears the background
        g.setColor(Color.BLUE);
        g.fillRect(10, 10, 100, 50);
        g.setColor(Color.BLACK);
        g.drawString("hello", 20, 40);
    }
}
```

Override `paintComponent`, receive a `Graphics`, and draw.

## The rule that surprises everyone

**You never call `paintComponent`. The toolkit calls it, whenever it decides the
component needs redrawing** — when the window is first shown, uncovered, resized,
or when you have asked for a repaint.

If you want the display to change, you change your data and then say:

```java
repaint();
```

`repaint()` does not draw. It posts a repaint request to the event queue, which is
handled later — possibly merged with other pending requests — and eventually
results in a call to `paintComponent`.

That indirection catches people out, and it has a good reason. Several changes in
quick succession produce one redraw rather than ten, and drawing is expensive.
It also keeps all drawing on the event dispatch thread, which Section 30.1.1
established is required.

The consequences:

**`paintComponent` must be able to run at any time.** It cannot assume anything
about what has happened since the last call.

**It must be fast**, because it runs on the event loop and Section 30.1.1's rule
applies. No file reading, no computation of any size. Compute into fields, draw
from fields.

**It must be idempotent** — drawing the same state twice must produce the same
picture. A `paintComponent` that mutates state, advances an animation, or appends
to a list is broken, because the number of calls is not yours to know.

That last one is the trap. Drawing is a *function of state*, not a step in a
process, and treating it as a step produces bugs that depend on how often the
window happened to be uncovered.

## The coordinate system

The origin is the **top left**, x increases rightward, and y increases
**downward**.

That is upside down from mathematics and it catches everyone once. It comes from
raster displays, which scan top to bottom, so a memory address increasing means a
position moving down.

Every graphics system does it — HTML canvas, Android, iOS, OpenGL's window
coordinates. If your chart is mirrored vertically, this is why.

## What you can draw

```java
g.drawLine(x1, y1, x2, y2);
g.drawRect(x, y, w, h);          g.fillRect(x, y, w, h);
g.drawOval(x, y, w, h);          g.fillOval(x, y, w, h);
g.drawString("text", x, y);      // y is the baseline, not the top
g.setColor(Color.RED);
g.setFont(new Font("Sans", Font.BOLD, 14));
```

Two details that cause more trouble than they should.

**`drawString`'s y is the baseline** — the line the letters sit on — so descenders
in `g` and `y` go below it. Text positioned by its top will sit too low by the
ascent, and `FontMetrics` is how you find out by how much.

**`drawRect(x, y, w, h)` is `w + 1` pixels wide.** The outline includes both
edges. `fillRect` with the same arguments fills exactly `w`. Adjacent outlined
rectangles therefore overlap by a pixel, which is visible and mildly maddening.

Casting to `Graphics2D` gives antialiasing, transforms, stroke widths, gradients
and arbitrary shapes, and is what any real drawing code uses:

```java
Graphics2D g2 = (Graphics2D) g;
g2.setRenderingHint(RenderingHints.KEY_ANTIALIASING,
                    RenderingHints.VALUE_ANTIALIAS_ON);
```

Without that hint, diagonal lines are visibly jagged. It is one line and it should
be in essentially every `paintComponent` you write.

## Double buffering

Drawing directly to the screen produces flicker: the display is refreshed while
your picture is half drawn, and the user sees the partial state.

The fix is to draw into an off-screen image and copy it to the screen in one
operation. **Swing does this for you** — `JPanel` is double-buffered by default —
which is a real convenience and worth knowing about, because in toolkits that do
not, flicker is the first problem you hit and the technique is the answer.

## Animation

A `Timer` posting repaints:

```java
new javax.swing.Timer(16, e -> {
    model.step();
    repaint();
}).start();
```

Sixteen milliseconds is about sixty frames per second. Note the `javax.swing`
qualifier: `java.util.Timer` also exists and fires on its own thread, which
violates the single-thread rule. The Swing one fires on the event dispatch thread,
which is what you want.

And note the division of labour: the timer advances the model and *requests* a
repaint. `paintComponent` still only draws. This is Section 30.2.3's separation
appearing in the smallest possible case.

## Where this goes

Custom drawing is a small part of most applications and a large part of a few —
games, editors, visualization, instrumentation. The concepts scale:

**Retained mode** — you describe a scene as objects and the toolkit draws it. HTML
and JavaFX work this way, and so does every component you did not write.

**Immediate mode** — you issue drawing commands each frame. `paintComponent` is
this, and so are games and OpenGL.

The trade is the familiar one. Retained mode is convenient and the toolkit
optimizes for you; immediate mode is direct and everything is your problem. Most
systems are retained mode with an immediate-mode escape hatch, which is exactly
what a `JPanel` with a custom `paintComponent` is.

Next: the separation that makes any of this maintainable.
