# Version Control

Chapter 10 mentioned bisecting through history and deferred the mechanics. This
appendix is the mechanics, for someone who has never used version control.

## What problem it solves

You have a working program. You change it to add a feature. It stops working, and
you no longer remember exactly what you changed.

Everyone's first solution is copies: `project`, `project-backup`,
`project-backup-2`, `project-final`, `project-final-ACTUAL`. This works badly. The
copies take space, nothing records *why* each was made, and merging two people's
changes means comparing directories by eye.

**Git** records the history of a directory: what changed, when, by whom, and —
because you write it down — why. It also lets you return to any earlier state
exactly, and combine changes made by several people.

## The model

Three places a file can be, and understanding this settles most confusion.

```
  working directory  ──git add──▶  staging area  ──git commit──▶  repository
   (your files)                     (what will be                 (permanent
                                     in the next commit)            history)
```

**The working directory** is your files as they are now.

**The staging area** — also called the index — is what you have marked for the
next commit. This intermediate step is what people find odd at first, and its
purpose is to let you commit *some* of your changes: you fixed a bug and also
corrected a typo, and those are two commits.

**The repository** is the permanent history, in the hidden `.git` directory.

A **commit** is a snapshot of the whole project plus a message, an author, a time,
and a pointer to the commit before it.

## Starting

```
$ git init
$ git config user.name "Your Name"
$ git config user.email "you@example.com"
```

`git init` creates `.git` in the current directory. The two `config` lines can be
set globally with `--global` so you do the once.

## The daily loop

```
$ git status
$ git add Hello.java
$ git commit -m "Add the greeting program"
```

Real output, in order. Before adding:

```
$ git status --short
?? Hello.java
```

`??` means untracked — git does not know about this file. After adding:

```
$ git status --short
A  Hello.java
```

`A` means added, staged for the next commit. Then:

```
$ git commit -m "Add Hello"
$ git log --oneline
ec4b6f8 Add Hello
```

That hexadecimal is the commit's identifier — a hash of its contents, so it is
unique and unforgeable. You will refer to commits by the first seven characters.

## Seeing what changed

Change the file, and:

```
$ git diff --stat
 Hello.java | 2 +-
 1 file changed, 1 insertion(+), 1 deletion(-)
```

and in full:

```
$ git diff
-public class Hello { ... "Hi" ... }
+public class Hello { ... "Hello" ... }
```

Lines beginning `-` were removed, `+` added. `git diff` shows unstaged changes;
`git diff --staged` shows what is staged.

**Read the diff before every commit.** It is the cheapest review available and it
catches debugging statements you meant to delete, files added by accident, and
changes you forgot you made.

## The commands worth knowing

```
git status                    what is changed, staged, untracked
git add <file>                stage a file
git add -p                    stage selected hunks, interactively
git commit -m "message"       commit what is staged
git log --oneline             history, one line each
git diff                      unstaged changes
git diff --staged             staged changes
git show <hash>               one commit in full
git restore <file>            discard changes to a file
git restore --staged <file>   unstage without discarding
```

`git restore <file>` **throws work away**. It is the one command in that list that
loses something, and there is no undo.

## Writing a message

The message is the only part a future reader gets for free, and most of them are
wasted.

```
bad:   fixed stuff
       update
       asdf

good:  Fix off-by-one in largest() for single-element arrays
       Use StringBuilder in the report loop; the concatenation was quadratic
```

The convention: a short summary line in the imperative — *Fix*, not *Fixed* —
under about fifty characters, and if more is needed a blank line and a paragraph
explaining **why**. The *what* is in the diff. The *why* is only in your head
until you write it down.

## Branches

A **branch** is a movable pointer to a commit, so you can work on something
without disturbing the main line.

```
$ git switch -c add-sorting     # create and switch to it
   ... work, commit ...
$ git switch main
$ git merge add-sorting
```

If both branches changed the same lines, git cannot decide and reports a
**conflict**, marking the file:

```
<<<<<<< HEAD
the version on main
=======
the version on the branch
>>>>>>> add-sorting
```

Edit the file so it says what you want, delete the markers, `git add` it, and
commit. Conflicts feel alarming the first time and are ordinary afterwards.

## Working with others

```
$ git clone <url>          copy a repository
$ git pull                 fetch others' changes and merge them in
$ git push                 send yours
```

`git pull` before you start and before you push. Most collaboration trouble is
someone working for a day on a stale copy.

## What not to commit

**Compiled output.** `.class` files, `target/`, `out/`. They are derived from the
source and they cause conflicts constantly.

**Secrets.** Passwords, API keys, tokens. Removing one from history is genuinely
difficult, and anything pushed to a public repository must be treated as
compromised regardless.

**Large binaries** that change often.

A `.gitignore` file lists what to skip:

```
*.class
out/
target/
.DS_Store
```

## Bisecting

Chapter 10's promise. If a program worked at some old commit and fails now:

```
$ git bisect start
$ git bisect bad                  # the current state is broken
$ git bisect good ec4b6f8         # this old one worked
```

Git checks out a commit halfway between and asks. You test it and answer
`git bisect good` or `git bisect bad`, and it halves again. A hundred commits
take about seven answers, because this is Chapter 10's bisection with the
bookkeeping automated.

```
$ git bisect reset                # when finished
```

## The habit worth forming

**Commit small and often.** A commit that changes one thing can be understood,
reverted, and bisected to. A commit containing a day's work can do none of those.

The test: can you describe it in one line without "and"? That is Chapter 11's
one-job rule, applied to history.
