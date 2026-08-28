# Listeners and Callbacks

You do not call the toolkit. The toolkit calls you.

That sentence sounds like a slogan and it has a consequence you will feel
immediately: after the interface is built, `main` does nothing at all. There is no
line of code anywhere describing the order things happen in, because the order is
not yours to decide.

You do not call the toolkit. The toolkit calls you.

```java
button.addActionListener(e -> save());
```

You hand over a piece of behavior, and something else decides when to run it.
Chapter 26 called this passing behavior as a value; here it is the basis of an
entire program's structure.

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

That is the general contract: **many listeners per event, called in the order
registered, each independent.** No listener knows the others exist, and the source
knows nothing about any of them beyond the interface.

That decoupling is the point. A button knows how to detect a press and nothing
about saving files; the code that saves files knows nothing about buttons. Neither
would have to change if the other were replaced.

The pattern's name is **Observer**, it is Chapter 23's design-patterns material,
and it is the one that shows up most often outside user interfaces — in message
queues, in change notifications, in Section 30.2.3's model.

## What a listener is

An interface with one method — Chapter 26's functional interface:

```java
interface Listener { void handle(Event e); }
```

So a lambda satisfies it, which is why modern interface code reads as it does.
Before Java 8 the same thing was:

```java
button.addActionListener(new ActionListener() {
    public void actionPerformed(ActionEvent e) { save(); }
});
```

Five lines for one call. Multiply by every control in an application and you have
the reason lambdas were added; interface code was the motivating use case, and
`ActionListener` had been a functional interface for sixteen years before there
was a notation for it.

## Inversion of control

Step back and notice what has happened to the shape of the program.

A batch program's `main` reads top to bottom, and you can follow it. An
event-driven program's `main` is:

```java
public static void main(String[] args) {
    buildInterface();       // create components, register listeners
    loop.run();             // and then nothing else ever
}
```

After `run()`, `main` does nothing until the program exits. All the behavior is in
handlers, called from outside, in an order nobody wrote down.

This is **inversion of control**, sometimes stated as the Hollywood principle:
*don't call us, we'll call you.* Section 22.1.2's template method was a small
version — a parent calling into a subclass's steps — and this is the same idea
governing a whole program.

Three things change, and they are worth anticipating.

**There is no single flow to read.** Understanding what happens when a button is
pressed means finding its listener, and understanding the program means
understanding a set of handlers and the state they share.

**State becomes the connection between handlers.** One handler sets something, a
later one reads it. That shared state is the program's real structure, and it is
Chapter 19's argument arriving with force — if the state is scattered across
handlers, nothing can be reasoned about.

**Order is not yours.** A handler cannot assume another has run. Anything one
handler needs must either be established at construction or be checked.

## Handlers should be small

Which gives the rule that keeps event-driven code readable:

**A handler should decide what to do and delegate.**

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

The listener translates a user action into a call on the model, and reports the
outcome. The saving logic is in `Document`, where it can be tested without a
window.

The alternative — a hundred lines of file handling inside a listener — is the
commonest failure mode in interface code, and it produces a program whose logic
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
