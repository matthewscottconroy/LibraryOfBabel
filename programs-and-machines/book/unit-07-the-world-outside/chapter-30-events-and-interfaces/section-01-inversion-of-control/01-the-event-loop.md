# The Event Loop

A user interface has to cope with a program that does not know what happens next.
The answer, in every toolkit ever built, is the same twelve lines:

```java
while (running) {
    Event e = queue.take();          // wait for something to happen
    dispatch(e);                     // give it to whoever registered
}
```

That is the **event loop**. It waits, it takes the next event, it delivers it, it
waits again. Every graphical program you have used is running one right now.

## A working one

Here is the same thing with the parts filled in:

```java
static final class Loop {
    private final Deque<Event> queue = new ArrayDeque<>();
    private final Map<Class<?>, List<Listener>> listeners = new LinkedHashMap<>();
    private boolean running = true;

    void post(Event e) { queue.addLast(e); }

    void on(Class<? extends Event> kind, Listener l) {
        listeners.computeIfAbsent(kind, k -> new ArrayList<>()).add(l);
    }

    void run() {
        while (running) {
            Event e = queue.pollFirst();
            if (e == null) break;
            for (Listener l : listeners.getOrDefault(e.getClass(), List.of()))
                l.handle(e);
            if (e instanceof Quit) running = false;
        }
    }
}
```

Thirty lines, and it is a real event loop. Registering handlers and posting
events:

```java
loop.on(Click.class, e -> System.out.println("click at " + ...));
loop.on(Key.class,   e -> typed.append(((Key) e).c()));
loop.on(Quit.class,  e -> System.out.println("quitting"));

loop.post(new Click(10, 20));
loop.post(new Key('h'));
loop.post(new Key('i'));
loop.post(new Quit());
loop.post(new Click(99, 99));     // posted after Quit
loop.run();
```

Verified:

```
click at 10,20
key h
key i
quitting
typed = hi, dispatched 4 events
```

Five events posted, four dispatched. The `Quit` stopped the loop and the click
behind it was never delivered — which is exactly what happens when you close a
window with a click still in the queue.

The events are Chapter 22's sealed interface and records again. An event is a
value: it has a type and some data, it is created when something happens, and it
is delivered later. That "later" is the whole idea — an event is a fact that has
been *recorded* rather than acted upon immediately.

## Why a queue

Because events arrive when they arrive, and handling takes time.

The operating system detects a click and must not wait for your program to finish
thinking about the previous one. So it appends to a queue and returns. Your
program drains the queue at its own pace, and the ordering is preserved.

Two consequences worth naming.

**Events are handled in order, one at a time.** No two handlers run
simultaneously, which means handler code needs no locks and Chapter 31's problems
do not arise inside it. That is a large simplification and it is why the design
has survived fifty years.

**A slow handler delays everything behind it.** Which is the next section.

## The one rule

**Do not block the event loop.**

Verified, with a handler that sleeps for 300 milliseconds:

```
slow handler done
the key event waited 300 ms behind the click
```

The key event was posted immediately after the click and was delivered 300
milliseconds late, because the loop was inside the click handler and there is only
one loop.

In a real application, that is the interface freezing. Nothing repaints — repaint
is an event too — no button responds, and the operating system eventually offers
to kill your program. Every unresponsive application you have used was a handler
that took too long.

What counts as blocking:

- reading a file, especially over a network
- an HTTP request
- a database query
- `Thread.sleep`
- any loop over a large collection
- waiting for a lock

Anything over about 100 milliseconds is noticeable; anything over a second reads
as broken.

## What to do instead

The standard pattern, and it is the same in every toolkit:

**Start the work elsewhere. Post the result back.**

```java
button.addActionListener(e -> {
    setStatus("Loading...");
    new Thread(() -> {
        String data = fetchFromNetwork();          // off the loop
        SwingUtilities.invokeLater(() -> {         // back on the loop
            setStatus("Done");
            display(data);
        });
    }).start();
});
```

Three phases: update the interface immediately so the user knows something is
happening, do the slow work on another thread, and post the result back to the
loop for display.

That last step is required, not optional, and the reason is the next section's
subject.

## The single-thread rule

Interface toolkits are single-threaded. Components may only be touched from the
event loop's own thread.

Verified:

```
main thread name        : main
main is the EDT?        : false
invokeAndWait ran on    : AWT-EventQueue-0
and it is the EDT?      : true
```

In Swing that thread is the **Event Dispatch Thread**, and `main` is not it.
`SwingUtilities.invokeLater` and `invokeAndWait` are how other threads get code
onto it — they post a `Runnable` to the same queue the clicks go into.

The rule is absolute and the reason is Chapter 31's: making a whole widget
hierarchy thread-safe would require locking on every access, would be slow, and
would deadlock — a repaint holding a lock while a handler waits for one is a
classic. Every toolkit tried it in the 1990s and every toolkit gave up. Swing's
predecessor AWT was thread-safe and it is why Swing is not.

Touching a component from the wrong thread usually appears to work, which is the
worst possible outcome. It fails intermittently, under load, in ways that do not
reproduce.

**Slow work off the loop. All interface updates on it.** That sentence is most of
what makes a graphical program correct.

Next: how your code gets called.
