# The Toolchain

Chapter 5 introduced `javac` and `java` and deferred everything else. This
appendix is that everything else: how a program becomes runnable, where the
compiler looks for things, and what the surrounding tools are for.

It is reference material. Read the first section now and come back to the rest
when a build fails.

## What you need installed

A **JDK** — a Java Development Kit — not a JRE.

The distinction catches people, and it is the single most common reason a
beginner cannot follow Chapter 5. A JRE (Java Runtime Environment) can *run*
compiled programs and contains no compiler. A JDK contains the runtime plus
`javac`, `jar`, `javadoc`, and the rest.

The symptom is exact:

```
bash: javac: command not found
```

while `java --version` works perfectly. If that is what you are seeing, you have
a JRE and you need a JDK. Any distribution of version 17 or later runs everything
in this book: Oracle's, or an OpenJDK build from Adoptium, Amazon Corretto, Azul,
or your system's package manager.

Check what you have:

```
$ java --version
openjdk 25.0.4 2026-07-21
```

and, separately:

```
$ javac --version
```

Both must work. They are different programs and one can be present without the
other.

## The two steps

```
$ javac Hello.java     # compile: produces Hello.class
$ java Hello           # run
```

`javac` takes a **file name**, with the extension. `java` takes a **class name**,
without one. Chapter 5 explained why: the first is a file operation, the second
names a thing inside the running system.

Since Java 11 there is a shortcut for single-file programs:

```
$ java Hello.java
```

which compiles in memory and runs, leaving no `.class` file. Everything in this
book works this way, and it stops working the moment your program is two files.

## Where the compiler looks

The **classpath** is the list of places Java searches for classes. Most confusion
about "cannot find symbol" and "could not find or load main class" is classpath
confusion.

By default the classpath is the current directory. When your program is one
directory of `.java` files, nothing needs configuring.

When it is not:

```
$ javac -d out src/Hello.java          # put .class files in out/
$ java -cp out Hello                   # look for classes in out/
```

`-d` says where compiled output goes. `-cp` (or `-classpath`) says where to look.
Separate multiple entries with `:` on Linux and macOS, `;` on Windows.

To use a library:

```
$ javac -cp lib/junit.jar -d out src/*.java
$ java -cp out:lib/junit.jar MyTests
```

Note that the library must be on the classpath **both** when compiling and when
running. Forgetting it at run time gives `NoClassDefFoundError`, which is a
different error from `ClassNotFoundException` and means "it was there when you
compiled and is not there now".

## Packages

A **package** is a namespace. It groups related classes and prevents name
collisions between libraries.

```java
package com.example.app;

public class Greeter {
    public static String greet(String who) { return "Hello, " + who; }
}
```

The rule that surprises everyone: **the directory structure must mirror the
package name.** A class in package `com.example.app` must live at
`com/example/app/Greeter.java`, relative to the source root.

```
src/
  com/
    example/
      app/
        Greeter.java
```

Compile and run:

```
$ javac -d out src/com/example/app/Greeter.java
$ java -cp out com.example.app.Greeter
```

The class is now named `com.example.app.Greeter` — the package is part of its
name.

The convention of reversed domain names (`com.example`, `org.apache`) exists so
that two organizations cannot collide. You do not own a domain, and for the
exercises in this book you do not need a package at all. Use one when a project
grows past a handful of files.

## Importing

To use a class from another package:

```java
import java.util.List;
import java.util.ArrayList;

import java.util.*;          // everything in the package
```

`java.lang` — `String`, `System`, `Integer`, `Math` — is imported automatically,
which is why you have never had to import `String`.

The wildcard `*` imports one package's classes, not its subpackages:
`import java.util.*` does not bring in `java.util.regex.Pattern`.

Prefer explicit imports. Most editors write them for you, and they document what
a file depends on, which the wildcard hides.

## Jars

A **jar** is a zip file containing compiled classes, with a manifest describing
them. It is how Java code is distributed.

```
$ jar --create --file app.jar -C out .
$ jar --create --file app.jar --main-class com.example.app.Greeter -C out .
$ java -jar app.jar
```

The `--main-class` form records an entry point in the manifest, so `java -jar`
knows what to run. Without it you must name the class yourself:

```
$ java -cp app.jar com.example.app.Greeter
```

To look inside one:

```
$ jar --list --file app.jar
```

A jar is an ordinary zip file, so any zip tool will open it. That is occasionally
the quickest way to find out whether a class you expected is actually in there.

## Build tools

Beyond a few files, running `javac` by hand stops being reasonable. You have
dependencies to download, a source tree to compile in the right order, tests to
run, and a jar to assemble.

**Maven** and **Gradle** are the two in common use. Both do the same jobs:
fetch dependencies from a repository, compile, test, and package, according to a
project file you write once.

A minimal Maven project:

```
pom.xml
src/main/java/...      # your code
src/test/java/...      # your tests
```

```
$ mvn compile
$ mvn test
$ mvn package          # produces target/*.jar
```

Gradle uses `build.gradle` and the same shape of commands. Which you use is
usually decided by the project you join rather than by preference.

You do not need either for this book. You will need one for anything real, and
the concepts — a declared list of dependencies, a standard directory layout, a
repeatable build — transfer between them.

## Common failures

**`bash: javac: command not found`** — you have a JRE, not a JDK.

**`error: class Hello is public, should be declared in a file named Hello.java`**
— the file name must match the public class name.

**`Error: Could not find or load main class Hello`** — a run-time error from
`java`. Either you have not compiled, or you are in the wrong directory, or you
wrote `java Hello.java` when you meant `java Hello`, or the class is in a package
and you did not give its full name.

**`error: cannot find symbol`** — a compile error. A name that does not exist:
a typo, a missing import, or a class that is not on the classpath.

**`NoClassDefFoundError`** — it compiled and the class is not on the classpath at
run time. Almost always a missing `-cp` entry.

**`UnsupportedClassVersionError`** — compiled by a newer JDK than the one running
it. Check both versions.

## What to remember

Three things, and the rest is lookup.

**A JDK is not a JRE**, and the difference is `javac`.

**The classpath is where Java looks**, it defaults to the current directory, and
it must be right at compile time *and* at run time.

**The directory structure must mirror the package name**, because the package is
part of the class's name.
