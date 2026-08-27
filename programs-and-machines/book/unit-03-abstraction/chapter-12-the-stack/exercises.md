# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Frames

**12.1.** Draw the stack at the moment `sum` is executing, for
`main` → `report` → `mean` → `sum`. Label which frame is running and which are
suspended.

**12.2.** Explain, in terms of frames, why this prints 1 every time it is called:
```java
static void count() { int n = 0; n++; System.out.println(n); }
```

**12.3. [carries forward]** Why can a local variable not outlive its method?
Answer in terms of what happens to the frame, and say what Java prevents that
would otherwise make this dangerous.

**12.4.** A method creates an object and returns it. The frame is discarded on
return. Explain why the object survives.

**12.5.** Given this trace, say which method was executing, which were suspended,
and in what order they will finish:
```
	at App.parse(App.java:41)
	at App.load(App.java:20)
	at App.main(App.java:8)
```

## Stack overflow

**12.6.** Write a method that recurses without a base case, catch the resulting
`StackOverflowError`, and print the depth reached. Run it three times — is the
number stable?

**12.7.** Name three distinct causes of an unintended infinite recursion, and say
what each looks like in a stack trace.

**12.8.** Why is `StackOverflowError` an `Error` rather than an `Exception`?
Answer in terms of what recovery would require.

**12.9.** A correct recursion is too deep for the default stack. Give three
responses and say when each is appropriate.

## Pass by value

For 12.10–12.13, predict the output before running.

**12.10.**
```java
static void change(int n) { n = 99; }
int x = 5; change(x); System.out.println(x);
```

**12.11.**
```java
static void change(int[] a) { a[0] = 99; }
int[] arr = {1,2,3}; change(arr); System.out.println(arr[0]);
```

**12.12.**
```java
static void change(int[] a) { a = new int[]{9,9,9}; }
int[] arr = {1,2,3}; change(arr); System.out.println(arr[0]);
```

**12.13. [carries forward]**
```java
static void change(int[] a) { a[0] = 50; a = new int[]{9,9,9}; a[0] = 99; }
int[] arr = {1,2,3}; change(arr); System.out.println(arr[0]);
```

**12.14.** Using 12.10–12.13, state in one sentence the rule that explains all
four. Then explain why "objects are passed by reference" gets 12.12 wrong.

**12.15.** Explain why you cannot write a Java method that swaps two `int`
variables belonging to the caller. Give two things you could do instead.

## Overloading

**12.16.** Predict which overload each call selects, given `f(int)`, `f(long)`,
`f(double)`, `f(Object)`:
```java
f(1);  f(1L);  f(1.0);  f('a');  f(1.0f);  f(true);
```

**12.17.** Why is the return type not part of a method's signature? Give the call
that would be ambiguous if it were.

**12.18.** `list.remove(1)` and `list.remove(Integer.valueOf(1))` do different
things. Explain, and say what property of overload resolution causes it.

**12.19.** Given `f(String)` and `f(Integer)`, why does `f(null)` fail to
compile, and how would you fix the call?

## Going further

**12.20.** Section 12.1.2 claims the stack discipline is "not a design choice but
a recognition". Argue for that claim, then construct a scenario in which calls
would *not* nest and say what structure would be needed instead. (Hint: Chapter
31.)

**12.21.** Java does not eliminate tail calls. Find out what tail-call
elimination is, state the argument for Java's decision, and say what it costs.

**12.22.** Section 12.1.3 lists four instances of one pattern — a fixed-size
region buying speed and excluding what does not fit. Name them, and predict where
the fifth appears.
