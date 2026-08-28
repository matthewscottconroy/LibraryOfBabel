# Listeners and Callbacks

You do not call the toolkit. The toolkit calls you.

That sentence sounds like a slogan and it has a consequence you will feel
immediately: after the interface is built, `main` does nothing at all. There is no
line of code anywhere describing the order things happen in, because the order is
not yours to decide.

It starts with one line:

```java
button.addActionListener(e -> save());
```

You hand over a piece of behavior and walk away. Something else will decide when
it runs, and how often, and whether at all. Chapter 26 called that passing
behavior as a value; here it is holding up an entire program.

## Registration

```java
loop.on(Click.class, e -> System.out.println("click at " + ...));
loop.on(Key.class,   e -> typed.append(((Key) e).c()));
loop.on(Key.class,   e -> System.out.println("key " + ((Key) e).c()));
```

Two handlers on `Key`, and verified, both ran, in registration order:

```
key h
key i
typed = hi
```

So the contract is: **as many listeners per event as you like, called in
registration order, each one independent.** No listener knows the others are
there. The button knows nothing about any of them beyond the interface.

Sit with how thoroughly those two ends have been separated. A button knows how to
notice a press and nothing whatsoever about saving files. The file-saving code has
never heard of buttons. Replace either one and the other does not change — which is
why the same button class can serve every application ever written with it.

The pattern's name is **Observer**, it is Chapter 23's design-patterns material,
and of all the patterns in that book it is the one you will meet most often away
from user interfaces — in message queues, in change notifications, and in the model
at the end of this chapter.

## What a listener is

An interface with one method — Chapter 26's functional interface:

```java
interface Listener { void handle(Event e); }
```

One method, so a lambda satisfies it — which is why the code above is one line.

It was not always. Here is the same registration, written the only way it could be
written before 2014:

```java
button.addActionListener(new ActionListener() {
    public void actionPerformed(ActionEvent e) { save(); }
});
```

Five lines to say `save()`. Now multiply that by every button, menu item, slider
and checkbox in an application, and you have a fair share of why lambdas were added
to the language at all.

And here is the part I find genuinely funny. `ActionListener` had been a
functional interface since 1998. Nothing about it needed to change. Java spent
sixteen years with the concept fully in place and no notation short enough to use
it comfortably.

## What has happened to your program

Step back from the syntax for a moment and look at the shape of the thing you are
building, because it has quietly turned inside out.

You know how a program reads. `main` starts at the top, goes to the bottom, and if
you are patient you can follow every step. Here is `main` in an application with a
window:

```java
public static void main(String[] args) {
    buildInterface();       // create components, register listeners
    loop.run();             // and then nothing else ever
}
```

That is the whole of it. After `run()`, `main` does nothing for the rest of the
program's life. Every behavior your application has lives in a handler, called
from outside, in an order that appears nowhere in your source.

This is **inversion of control**, and it has a nickname — the Hollywood principle:
*don't call us, we'll call you.* You met a small version of it in Chapter 22, where
a parent class called down into a subclass's steps. This is the same idea, scaled
up until it governs everything.

Three things change once you are living here, and it is worth knowing them in
advance rather than discovering them.

**There is no single flow to read.** Understanding what happens when a button is
pressed means finding its listener, and understanding the program means
understanding a set of handlers and the state they share.

**State becomes the wiring.** One handler sets something; a later one reads it.
That shared state is now the real structure of your program — it is what connects
behaviors that never call each other. Which makes Chapter 19's argument urgent
rather than tidy: scatter that state across a dozen handlers and there is nothing
left to reason about.

**Order is not yours.** A handler cannot assume another has run. Anything one
handler needs must either be established at construction or be checked.

## Keep handlers small

All of which gives one rule, and following it is most of what keeps event-driven
code readable:

**A handler decides what to do, and then delegates.**

```java
saveButton.addActionListener(e -> {                     // good
    try {
        document.saveTo(chooseFile());
        status.setText("Saved");
    } catch (IOException ex) {
        status.setText("Could not save: " + ex.getMessage());
    }
});
```

The listener does two things: turn a click into a call, and report what happened.
The actual work of saving lives in `Document`, where you can test it without ever
opening a window.

The alternative — a hundred lines of file handling inside the listener — is the
single commonest failure in interface code, and it produces a program whose logic
cannot be tested at all, because reaching it requires a click.

Note also that the handler catches. It is the boundary Section 28.2.1 described:
the user's action is a natural place to handle failure, because it is the place
where there is someone to tell.

## Callbacks elsewhere

The shape is not specific to interfaces, and recognizing it elsewhere is most of
this lesson's value.

**Web servers.** You register a handler per route; the server's loop calls it per
request. Same structure, same rule about not blocking.

**JavaScript.** Every browser API is a callback, and its single-threaded event
loop is exactly Section 30.1.1's — which is why a long computation freezes a web
page in the same way it freezes a window.

**Message consumers.** A handler per message type, called by a framework's loop.

**Timers and schedulers.** `Timer.schedule(task, delay)` is a callback with the
event supplied by a clock.

**Chapter 26's higher-order methods.** `forEach(this::process)` is the same
inversion at the smallest scale — you hand over behavior and something else
decides when to run it.

## The costs

Two, and both are real.

**Debugging.** A stack trace inside a handler shows the toolkit's dispatch
machinery, not the code that registered the handler. The frames between `main` and
your code belong to somebody else, and the interesting question — *why was this
called* — is usually not answerable from the trace.

**Callback nesting.** A handler that starts work whose completion callback starts
more work produces the shape JavaScript called *callback hell*. Java's `Future`
and `CompletableFuture`, and other languages' async/await, exist to flatten it.
Chapter 31 touches on this.

Neither outweighs the benefit, which is that a program can respond to things
happening in an order nobody predicted. That is not achievable any other way.

Next: putting something on the screen.
