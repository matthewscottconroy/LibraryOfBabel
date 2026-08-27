# Exercises

**28.1** Write a `parseOrMinusOne` and demonstrate the sentinel problem: find two
inputs, one valid and one invalid, that produce the same result. Then rewrite it
to return an `Optional<Integer>` and say what the caller can now distinguish.

**28.2** Write a chain of four methods where the innermost fails. Implement error
reporting twice — once with return codes checked at every level, once with an
exception — and compare the line counts of the two middle methods.

**28.3** Write a custom exception carrying a field, throw it, and have the handler
use the field rather than parsing the message. Then write the version that parses
the message and say what breaks when the message is reworded.

**28.4** Write a method with a `try` that returns a value and a `finally` that
prints. Confirm the order. Then add a `return` inside the `finally` and observe
what happens to the value the `try` returned.

**28.5** Write a class implementing `AutoCloseable` that prints on close. Use it
in a try-with-resources block whose body throws. Confirm that close runs before
the handler.

**28.6** Open two resources in one try-with-resources statement and confirm the
close order. Then make the first one's `close` throw and check that the second
still closes.

**28.7** Write code where the body of a try-with-resources throws and `close` also
throws. Print `getSuppressed()`. Explain which exception propagated and why that
is the right choice.

**28.8** Catch a `NumberFormatException` and rethrow as an `IllegalStateException`,
once with the cause and once without. Print both stack traces and describe the
difference from an operator's point of view.

**28.9** Find an empty catch block in code you have written or in an open-source
project. Describe a specific scenario in which it would hide a real failure, and
say what it should do instead.

**28.10** *Measurement.* Benchmark one million throw-and-catch cycles against one
million calls returning a sentinel. Then repeat with a pre-allocated exception
whose `fillInStackTrace` is overridden to do nothing. Report all three and say
where the cost is.

**28.11** Write a retry loop with a bound, an exponential backoff, and a rethrow
of the final failure with its cause. Then name an operation for which your loop
would be dangerous, and say what would have to change.

**28.12** *Design, no code.* You are writing a batch job that processes a hundred
thousand records. Decide where the `try/catch` goes, what happens to a record that
fails, what the job's exit status should be if 5 records failed, and what an
operator needs to see. Justify each decision against Section 28.2.1's test.

**28.13** [carries forward] Add error handling to Chapter 25's interpreter so that
a parse failure reports the token position and an evaluation failure reports which
statement was running. Keep the interpreter; Chapter 29 gives it the ability to
read a program from a file, at which point the file may not exist.
