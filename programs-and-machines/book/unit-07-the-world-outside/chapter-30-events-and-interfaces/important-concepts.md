# Important Concepts

**Inversion of control** — you do not call the toolkit; the toolkit calls you.
After the interface is built, `main` does nothing and all behavior lives in
handlers, called in an order nobody wrote down.

**The event loop** — take the next event from a queue, dispatch it to whoever
registered, repeat. Twelve lines, and every graphical program is running one.

**An event is a value** — a type and some data, recorded when something happens
and delivered later. Chapter 22's sealed interface and records.

**Why a queue** — the operating system must not wait for your program. Events are
handled in order, one at a time, so handler code needs no locks; and a slow
handler delays everything behind it.

**Do not block the event loop** — measured: a 300 ms handler delayed the next
event by 300 ms. In a real application that is the interface freezing, because
repaint is an event too.

**What counts as blocking** — file reads, network requests, database queries,
sleeps, large loops, waiting for a lock. Over 100 ms is noticeable, over a second
reads as broken.

**Start the work elsewhere, post the result back** — update the interface
immediately, do the slow work on another thread, and use `invokeLater` to return.

**The single-thread rule** — components may only be touched from the event
dispatch thread. Absolute, because a thread-safe widget hierarchy would need
locking everywhere and would deadlock. Every toolkit tried it and gave up; AWT was
thread-safe, which is why Swing is not.

**Touching a component from the wrong thread usually appears to work** — which is
the worst outcome, since it fails intermittently and does not reproduce.

**Listener** — an interface with one method, so a lambda satisfies it. Many per
event, called in registration order, each independent.

**Observer** — the pattern. A source knows nothing about its listeners beyond the
interface, so neither has to change when the other is replaced.

**Lambdas were motivated by this** — `ActionListener` had been a functional
interface for sixteen years before there was a notation for it.

**State connects handlers** — one sets, a later one reads. That shared state is
the program's real structure, and scattering it across handlers makes the program
unreasonable about.

**Handlers should decide and delegate** — translate a user action into a call on
the model and report the outcome. A hundred lines of logic in a listener cannot be
tested, because reaching it requires a click.

**A handler is an error boundary** — Section 28.2.1's rule: the user's action is
where there is someone to tell.

**The costs** — stack traces show dispatch machinery rather than the registration
site, and nested callbacks produce the shape that motivated futures and
async/await.

**A window is a tree** — components that know their size and can draw themselves,
with containers holding others. The DOM and Android's view hierarchy are the same
structure.

**Layout managers** — describe the relationships, not the positions. Absolute
positioning breaks on resize, larger fonts, translation, and different pixel
densities, and it is wrong in every toolkit for the same reasons.

**Sizing is a negotiation** — minimum, preferred and maximum are requests, and the
container decides. When a layout misbehaves, look at the container.

**You never call paintComponent** — the toolkit calls it. `repaint()` posts a
request, which may be merged with others and handled later.

**Drawing is a function of state** — `paintComponent` must be fast, must run at
any time, and must be idempotent. One that mutates state is broken, because the
number of calls is not yours to know.

**The origin is top left and y increases downward** — from raster scan order, and
true in every graphics system.

**drawString's y is the baseline**, and `drawRect` draws `w + 1` pixels wide.

**Antialiasing is off by default** and should be on in essentially every
`paintComponent`.

**Double buffering** — draw off-screen and copy in one operation, to avoid
flicker. Swing does it for you.

**Retained versus immediate mode** — describing a scene the toolkit draws, against
issuing commands each frame. Most systems are the first with an escape hatch to
the second.

**Model and view** — state and rules in one object, presentation in another, with
the view depending on the model and never the reverse.

**The invariant belongs to the model** — so it holds no matter which view or
listener attempts the change, rather than being duplicated in every handler.

**What the separation buys** — testability above all: the model was exercised with
no view, in milliseconds, which means the tests run in a build. Also multiple
views, a replaceable interface, and changes that land in one place.

**MVC, MVP, MVVM** — three names for the same claim. Which variant matters far
less than whether the separation exists.

**Where the line goes** — the rule belongs to the model, the wording to the view.
Section 23.1.1's test applies: would this still make sense if the interface
changed?
