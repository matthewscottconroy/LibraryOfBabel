# Growth

Your program handles today's three thousand records in forty milliseconds. Next
year there will be a hundred thousand.

How long will it take then? No measurement you can make today answers that, because
a timing tells you about one input and says nothing about the shape of the curve
it sits on. What you need is a way to talk about how cost *grows* — and it turns
out you can often work that out by reading the code rather than running it.

Three lessons.

Counting operations rather than timing them, and why that is the right unit. Then
big-O, which is a precise statement about growth and is routinely used as though
it meant something else. Then the growth classes themselves, with measured
demonstrations of what each one does to you.
