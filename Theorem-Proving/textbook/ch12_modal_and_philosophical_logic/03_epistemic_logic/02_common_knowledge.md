# Common Knowledge

## Beyond Mutual Knowledge

**Mutual knowledge** ($E\varphi$): everybody knows $\varphi$.
$$E\varphi = K_1\varphi \wedge K_2\varphi \wedge \cdots \wedge K_n\varphi$$

**Common knowledge** ($C\varphi$): everybody knows $\varphi$, and everybody knows that everybody knows, and everybody knows that everybody knows that everybody knows, and so on ad infinitum:
$$C\varphi = E\varphi \wedge EE\varphi \wedge EEE\varphi \wedge \cdots$$

Common knowledge is a strictly stronger condition than mutual knowledge. This distinction, subtle as it sounds, has profound implications for coordination.

## The Coordinated Attack Problem

Two armies want to attack simultaneously — only then will they win. Each can send messengers to the other. But messengers can be captured.

**Can they achieve common knowledge that both will attack at time T?**

No — not by finite rounds of messaging.

*Argument*: Suppose army A sends "Attack at dawn." If army B gets the message, army B replies "Acknowledged." But unless A gets the acknowledgment, A doesn't know B will attack. Even if A gets the acknowledgment, B doesn't know A got it — and so on forever.

No finite number of messages can create common knowledge among two agents communicating over an unreliable channel. This has direct implications for distributed systems: **consensus in the presence of message loss is impossible** (related to the FLP impossibility theorem).

## The Role of Public Announcements

Common knowledge is created by **public announcements** — statements made openly so that everyone knows everyone heard them. The father's announcement in the muddy children puzzle creates common knowledge precisely because it is public: not only does everyone hear it, but everyone knows everyone heard it, etc.

**Public Announcement Logic (PAL)**: A dynamic extension of epistemic logic with operators $[\varphi!]\psi$ — "after the public announcement of $\varphi$, $\psi$ holds." This formalizes how common knowledge changes when agents learn new facts publicly.

## Exercises
See [problems/ch12_modal_logic/03_epistemic_exercises.md](../../../problems/ch12_modal_logic/03_epistemic_exercises.md)
