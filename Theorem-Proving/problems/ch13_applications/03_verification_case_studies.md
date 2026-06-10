# Verification Case Studies

## Case Study 1: The Pentium FDIV Bug (★)

In 1994, Intel's Pentium processor had a bug in its floating-point division unit.
The lookup table used in the SRT division algorithm had a few missing entries.

**1.** Describe the Pentium FDIV bug:
  a. What went wrong mathematically?
  b. How was it discovered?
  c. What was the cost to Intel?
  d. How did this drive adoption of formal hardware verification?

**2.** Modern processors (AMD, Intel) formally verify their floating-point units using tools
like ACL2 and HOL. Look up one such verification project and summarize it.

## Case Study 2: The Therac-25 (★)

The Therac-25 radiation therapy machine had software bugs that caused fatal radiation
overdoses in 1985-1987.

**3.** Research the Therac-25 incident:
  a. What was the race condition?
  b. How could Hoare logic or model checking have caught this bug?
  c. What programming practices could have prevented it?

## Case Study 3: CompCert (★★)

**4.** CompCert is a formally verified C compiler developed by Xavier Leroy.
  a. What is verified: the compiler itself, or the output code?
  b. What proof assistant is used?
  c. What does "the compiler is correct" mean formally? State the theorem.
  d. What is NOT verified in CompCert?

## Case Study 4: Amazon AWS and TLA+ (★★)

**5.** Amazon uses TLA+ (Temporal Logic of Actions) to verify distributed systems
protocols (DynamoDB, S3, EBS).
  a. What kind of properties does TLA+ verify?
  b. What bugs has Amazon found using TLA+?
  c. Compare TLA+ to the approaches used in this textbook (Lean, Coq).

## Design Exercise (★★★)

**6.** You are designing a simple banking API with operations:
  `deposit(account, amount)`, `withdraw(account, amount)`, `transfer(from, to, amount)`

  a. State the invariants that should always hold (as FOL sentences or Hoare conditions)
  b. Write a Lean 4 or Coq specification of one operation with a correctness proof
  c. Identify a potential race condition in a concurrent implementation
