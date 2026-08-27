# Exercises

Exercises marked **[carries forward]** introduce something a later chapter
assumes.

## Predicting

Write your prediction before running each one.

**7.1.**
```java
int x = 5;
int y = x;
x = 10;
System.out.println(x + " " + y);
```

**7.2.**
```java
int n = 3;
n = n + 1;
n += 2;
n++;
System.out.println(n);
```

**7.3.**
```java
int a = 5;
int b = a++;
int c = 5;
int d = ++c;
System.out.println(a + " " + b + " " + c + " " + d);
```

**7.4.**
```java
byte b = 10;
b += 300;
System.out.println(b);
```
Explain the result using Chapter 2, and say why `b = b + 300;` would not compile.

## Errors

For each, predict the exact error and whether it comes from `javac` or the JVM.

**7.5.**
```java
int x;
System.out.println(x);
```

**7.6.**
```java
for (int i = 0; i < 3; i++) { }
System.out.println(i);
```

**7.7.**
```java
final int LIMIT = 10;
LIMIT = 20;
```

**7.8.**
```java
int score = 0;
score = "hello";
```

## Reasoning

**7.9.** Explain to someone with a mathematics background why `n = n + 1` is not
a contradiction. Your explanation should mention time.

**7.10.** Why is `5 = count;` an error? Answer in terms of what each side of an
assignment must be.

**7.11. [carries forward]** Local variables must be assigned before use; fields
get a default. Explain why the rules differ, in terms of what the compiler can
prove.

**7.12.** The chapter says restricting scope restricts how much you have to think
about. Give a concrete example of a bug that a narrow scope would have prevented.

**7.13. [carries forward]** Rewrite this so no variable is assigned more than
once, then say which version you would rather debug and why:
```java
int r = 0;
r = a * 2;
r = r + b;
r = r / 3;
```

## Going further

**7.14.** Write a method containing `int counter = 0; counter++;` and call it
three times, printing `counter` each time. Explain the output in terms of
lifetime.

**7.15.** Java forbids a local variable shadowing another local, but permits a
local to shadow a field. Find out why the second is allowed, and construct an
example where it causes a bug.

**7.16.** `var` is type inference, not dynamic typing. Write two short programs
that demonstrate the difference — one that compiles and one that does not — and
explain what the compiler knew in each case.

**7.17.** The chapter claims a type is both "an agreement about bit patterns"
(Unit I) and "a promise the compiler enforces" (this chapter). Take one concrete
type error and describe it from both directions.
