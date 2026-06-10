pub struct Question {
    pub unit: &'static str,
    pub topic: &'static str,
    pub text: &'static str,
    pub prompt: &'static str,
}

pub static QUESTIONS: &[Question] = &[
    // ── Unit I · First Principles ────────────────────────────────────────────
    Question {
        unit: "Unit I · First Principles",
        topic: "The Augustine Problem",
        text: "Augustine writes: \"If no one asks me, I know; if I want to explain it to one who asks, I do not know.\" What does this gap between knowing-how and knowing-what reveal about the nature of philosophical inquiry into time? Is the gap a sign that the concept is confused, or that our ordinary grasp of time outruns our ability to articulate it?",
        prompt: "Consider whether the same gap appears in your understanding of consciousness, personal identity, or color — and what distinguishes cases where it can be closed from cases where it cannot.",
    },
    Question {
        unit: "Unit I · First Principles",
        topic: "The Slow Freeze",
        text: "Imagine that every physical, biological, and mental process in the universe slows uniformly to one-millionth of its normal rate for one hour (by the clock of some hypothetical external observer), then returns to normal. Would time itself have slowed, or only the processes? What does your answer reveal about whether time is constituted by events or flows independently of them?",
        prompt: "Could there be a fact of the matter about this from inside the universe? If not, what does the undecidability tell us about the concept of absolute time?",
    },
    Question {
        unit: "Unit I · First Principles",
        topic: "Tensed versus Tenseless Language",
        text: "The B-theorist proposes to replace every tensed statement — \"the battle is over,\" \"spring is coming,\" \"now is the time to act\" — with a tenseless paraphrase using only \"earlier than,\" \"later than,\" and \"simultaneous with.\" Does the paraphrase preserve everything philosophically significant, or does something real get lost in translation?",
        prompt: "Try paraphrasing \"the best moment of my life has already passed\" in purely tenseless terms. Does the result capture what the original meant to the person who said it?",
    },
    Question {
        unit: "Unit I · First Principles",
        topic: "Reflective Equilibrium",
        text: "You hold the following commitments simultaneously: only what is present is real; you are the same person you were ten years ago; statements about the past can be true or false; special relativity entails that simultaneity is frame-relative; the past is fixed and cannot be changed; the arrow of time is mind-independent. These commitments are in tension. Using the method of reflective equilibrium, which would you revise first, and why?",
        prompt: "Is there a principled procedure for deciding which intuitions to trust over which theoretical results — or is the choice ultimately a matter of philosophical temperament?",
    },
    Question {
        unit: "Unit I · First Principles",
        topic: "Descriptive vs. Revisionary Metaphysics",
        text: "Descriptive metaphysics maps the concepts implicit in ordinary thought; revisionary metaphysics corrects them in light of theory. The B-theorist claims that the experienced flow of time is not an objective feature of reality. The A-theorist insists it must be, because it is given directly in experience. Which approach — descriptive or revisionary — is more appropriate for the philosophy of time, and what would justify that choice?",
        prompt: "Consider cases where revision is clearly warranted (the Earth does not \"rise\") and cases where the ordinary concept should be protected (the difference between deliberate action and reflex). Where does temporal passage fall?",
    },
    Question {
        unit: "Unit I · First Principles",
        topic: "The Empty Universe",
        text: "Suppose a universe exists that contains no matter, no energy, and no events of any kind — but nevertheless endures for a billion years before matter spontaneously appears. Does this scenario make sense? What would it mean for time to pass in a universe with no events? And if the scenario is incoherent, does that refute absolute time, or only our ability to verify it?",
        prompt: "The relational view says time consists in relations between events; the absolutist view says time flows regardless. Which view can make sense of this scenario, and at what philosophical cost?",
    },

    // ── Unit II · Ancient Philosophy ─────────────────────────────────────────
    Question {
        unit: "Unit II · Ancient Philosophy",
        topic: "Heraclitus and Persistence",
        text: "Heraclitus says you cannot step into the same river twice. Does this imply that identity through time is impossible, or that identity is compatible with continuous change? What must be preserved — matter, structure, causal continuity, something else — for the river, and for you, to remain \"the same\"?",
        prompt: "If the river is not \"the same\" because its water has changed, are you not \"the same\" because your cells have turned over? And if you are not the same, who is reading this question?",
    },
    Question {
        unit: "Unit II · Ancient Philosophy",
        topic: "Parmenides and the Possibility of Change",
        text: "Parmenides argues that what is real cannot come into being or pass away: coming-into-being would require something arising from nothing, which is impossible, since \"nothing\" cannot be thought or spoken. Is this argument sound? What would it take to refute it without simply asserting that change obviously exists?",
        prompt: "Plato's response in the Sophist distinguishes different senses of \"not-being.\" Is this distinction genuinely available to Parmenides's opponent, or does Parmenides have resources to block it?",
    },
    Question {
        unit: "Unit II · Ancient Philosophy",
        topic: "Zeno's Dichotomy",
        text: "Zeno's Dichotomy: before Achilles can reach his destination, he must reach the halfway point; before that, the quarter-way point; and so on indefinitely. There are infinitely many sub-tasks, and completing infinitely many tasks seems impossible. Aristotle's response distinguishes potential from actual infinity. Does this distinction genuinely dissolve the paradox, or does it relocate the problem without solving it?",
        prompt: "Consider what it would mean to \"actually\" complete an infinite series versus to traverse a distance that is \"potentially\" divisible into infinitely many parts. Are these the same claim or different ones?",
    },
    Question {
        unit: "Unit II · Ancient Philosophy",
        topic: "Plato on Time and Eternity",
        text: "Plato says time is \"a moving image of eternity, proceeding according to number\" (Timaeus 37d). What work does this metaphor do philosophically? In what sense is time like an image? And what would the \"original\" — eternity — have to be like for the comparison to hold?",
        prompt: "Is Plato's eternity better understood as an infinite duration (sempiternity) or as a genuinely timeless existence? And if timeless, how can a timeless original give rise to a temporal image?",
    },
    Question {
        unit: "Unit II · Ancient Philosophy",
        topic: "Aristotle's Definition of Time",
        text: "Aristotle defines time as \"the number of motion with respect to before and after.\" If there were no minds capable of counting, would there still be time on this account? Does Aristotle's definition entail that time is mind-dependent, or can the \"numbering\" be read as an objective relation in nature?",
        prompt: "Compare Aristotle's view with contemporary debates about whether time requires an observer. If no conscious beings had ever existed, would the Cretaceous period still have lasted a certain number of years?",
    },
    Question {
        unit: "Unit II · Ancient Philosophy",
        topic: "The Eternity of the World",
        text: "Aristotle argues the world must be eternal: every beginning requires a prior time, and every prior time contains a prior event. The argument seems valid given its premises. But if the Big Bang was the beginning of time itself — not an event within time — does cosmology refute Aristotle, vindicate him, or answer a different question than he was asking?",
        prompt: "Augustine anticipated something like the Big Bang response: God didn't act \"before\" creation because time itself began with creation. Does this dissolve Aristotle's worry or merely redescribe it?",
    },

    // ── Unit III · Hellenistic & Medieval ────────────────────────────────────
    Question {
        unit: "Unit III · Hellenistic & Medieval",
        topic: "The Epicurean Symmetry Argument",
        text: "The Epicurean argument: (1) you will not exist when you are dead; (2) what does not exist cannot be harmed; (3) therefore death cannot harm you. And Lucretius adds: you were equally non-existent before your birth, yet you are not troubled by that non-existence. Should you be equally untroubled by post-mortem non-existence? Is the asymmetry in our attitudes rational or merely psychological?",
        prompt: "Nagel replies that the asymmetry is real: you could have existed longer after death, but you could not have existed longer before birth (you could not have been born earlier without being a different person). Is this response convincing?",
    },
    Question {
        unit: "Unit III · Hellenistic & Medieval",
        topic: "Plotinus on Time as the Life of the Soul",
        text: "Plotinus argues that time is not a measure of the physical cosmos but of the Soul's unfolding activity — its \"life\" as it moves from one state to the next. Does this make time genuinely mind-dependent? And if so, whose mind? The individual soul, the World Soul, or something else?",
        prompt: "Compare Plotinus with Augustine: both locate time in the soul rather than in external reality. But do they mean the same thing by \"soul\"? And does a soul-based account of time explain time, or merely relocate the puzzle?",
    },
    Question {
        unit: "Unit III · Hellenistic & Medieval",
        topic: "Augustine on Creation and Time",
        text: "Augustine argues there was no \"before\" God's creation of the world because time itself was created. This is a philosophical argument, not merely a theological one: if time began with creation, the question \"what was God doing before the universe existed?\" contains a false presupposition. Is this response satisfying? Can the claim that \"time has a beginning\" be made coherent independently of theology?",
        prompt: "Does contemporary cosmology — which also posits a beginning of time at the Big Bang — vindicate Augustine's insight, or does it answer a different question? What would it mean for time to have a beginning that is not itself in time?",
    },
    Question {
        unit: "Unit III · Hellenistic & Medieval",
        topic: "The Distentio Animi",
        text: "Augustine argues that what we measure when we measure time is the \"impression\" (affectio) left in the mind by passing events — not the events themselves. The soul is \"stretched\" (distentio) between memory, attention, and expectation. If this is right, does it make temporal measurement merely subjective? What would Augustine say to someone who insists we measure objective durations, not mental states?",
        prompt: "When two people disagree about how long a piece of music lasted, is there a fact of the matter? If so, what determines it — the physical duration, or the more accurate distentio?",
    },
    Question {
        unit: "Unit III · Hellenistic & Medieval",
        topic: "Boethius and Divine Foreknowledge",
        text: "Boethius claims that God knows future events by perceiving them all at once in an eternal \"standing now\" (nunc stans), not by foreknowing them. If God perceives your future choices simultaneously with perceiving your present deliberation, does this preserve your freedom? Or does the perception of a determined outcome foreclose the openness of the future even if no causal compulsion is involved?",
        prompt: "Is there a difference between (a) God knowing what you will freely choose and (b) God seeing what you freely chose? If not, does the \"eternal present\" response genuinely dissolve the problem of foreknowledge?",
    },

    // ── Unit IV · Early Modern ───────────────────────────────────────────────
    Question {
        unit: "Unit IV · Early Modern",
        topic: "Leibniz and the Principle of Sufficient Reason",
        text: "Leibniz argues that absolute time is meaningless: if the universe had begun five minutes earlier than it actually did, while everything within it remained internally identical, there would be no difference whatsoever — and a difference that makes no difference is no difference at all. Is this a decisive refutation of Newton's absolute time, or can Newton respond?",
        prompt: "The argument rests on the Principle of Sufficient Reason: nothing happens without a reason. Is this principle itself well-founded? And if not, can Newton's absolute time be saved even if the PRS is false?",
    },
    Question {
        unit: "Unit IV · Early Modern",
        topic: "Newton's Bucket",
        text: "Newton argues for absolute space and time using the rotating bucket: water in a bucket that rotates with respect to absolute space curves at its surface, regardless of how it stands relative to other bodies. Mach objects that rotation is only meaningful relative to the fixed stars. If all other matter were removed from the universe, would there be a fact about whether the bucket is rotating?",
        prompt: "Einstein's general relativity partially vindicates Mach: the distribution of matter determines the local inertial frames. But does this fully capture what Mach intended, or does GR still contain a form of absolute structure?",
    },
    Question {
        unit: "Unit IV · Early Modern",
        topic: "Kant's Transcendental Aesthetic",
        text: "Kant argues that time is the form of inner sense — a structure contributed by the mind to experience, not a feature of things as they are in themselves. Time is \"transcendentally ideal\" but \"empirically real.\" What would it mean for time to be real for all possible experience while being unreal as a feature of mind-independent reality?",
        prompt: "If time is contributed by the mind, what should we make of physical theories — special relativity, general relativity — that describe time as part of the objective structure of spacetime? Is Kant's view compatible with modern physics, or does modern physics refute it?",
    },
    Question {
        unit: "Unit IV · Early Modern",
        topic: "Kant's First Antinomy",
        text: "In the First Antinomy, Kant claims that pure reason generates valid-seeming proofs both that the universe had a beginning in time and that it did not. The thesis proof: an infinite past would require traversing an infinite series, which is impossible. The antithesis proof: a beginning requires an empty time before it, and empty time contains no conditions that could initiate the universe. Which proof, if either, do you find stronger? And what does the existence of both suggest about the limits of cosmological reasoning?",
        prompt: "Can contemporary cosmology — the Big Bang, inflationary theory, quantum cosmology — decide a question that Kant thought reason alone could not? Or does cosmology presuppose rather than answer the question of temporal origins?",
    },

    // ── Unit V · Metaphysics of Temporal Ontology ────────────────────────────
    Question {
        unit: "Unit V · Metaphysics of Temporal Ontology",
        topic: "McTaggart's A/B Distinction",
        text: "McTaggart distinguishes the A-series (events ordered as past, present, and future) from the B-series (events ordered by the permanent relations \"earlier than\" and \"later than\"). He argues that the A-series is essential to time as we ordinarily understand it, but that A-properties generate a vicious regress. If the regress is genuine, what follows — that time is unreal, that A-theory must be abandoned, or something else?",
        prompt: "Prior's response: A-properties are not genuine properties but indexical expressions of a temporal perspective, like \"here\" and \"there.\" Does this dissolve the regress, or does it simply deny what needs explaining — namely, that there is an objective present?",
    },
    Question {
        unit: "Unit V · Metaphysics of Temporal Ontology",
        topic: "The Block Universe",
        text: "The block universe holds that past, present, and future are equally real, forming a static four-dimensional manifold. \"Now\" is no more objectively special than \"here.\" If this is right, in what sense is the future open or uncertain? And what becomes of the intuition that I can still affect what happens tomorrow but cannot affect what happened yesterday?",
        prompt: "Does the block universe entail determinism? A block universe with indeterministic quantum branching has been proposed. Can genuine contingency and the block universe coexist?",
    },
    Question {
        unit: "Unit V · Metaphysics of Temporal Ontology",
        topic: "The Rietdijk-Putnam Argument",
        text: "Putnam argues: if \"real = present,\" and if the present is relative to a reference frame, then there are scenarios in which two observers cannot agree on whether a distant event is real. Since an event cannot be both real and unreal, the only coherent conclusion is that all events — past, present, and future — are equally real. Is this a compelling argument for the block universe, or is there a way to resist it without abandoning special relativity?",
        prompt: "One response: accept that there is a privileged frame (neo-Lorentzianism), and ground presentism in that frame even though we cannot detect it. Does this preserve the spirit of presentism? Does it conflict with the spirit of relativity?",
    },
    Question {
        unit: "Unit V · Metaphysics of Temporal Ontology",
        topic: "The Growing Block",
        text: "The growing block theory holds that past and present are real, while the future does not yet exist. Time grows from a fixed past toward an open future. But here is a subtle problem: if you are an observer embedded in the block, how do you know you are at the leading edge — the present — rather than embedded in an interior slice of a much larger already-completed block?",
        prompt: "What epistemic resources would an observer have to determine whether they are at the edge or the interior? If none, is this an insoluble problem for growing block theory, or merely an interesting feature of it?",
    },
    Question {
        unit: "Unit V · Metaphysics of Temporal Ontology",
        topic: "The Moving Spotlight",
        text: "The moving spotlight theory holds that all times exist (as in the block universe), but an objective \"present\" moves through them, illuminating each in turn. Critics ask: at what rate does the spotlight move? \"One second per second\" seems trivially true but empty. And moving through what? The answer seems to require a second, \"hyper-time\" against which the spotlight's motion is measured. Is this regress devastating to the view?",
        prompt: "Is the rate-of-passage objection a genuine problem for the moving spotlight, or is it a misconception — a demand for an explanation of temporal motion using temporal concepts that is bound to seem circular?",
    },
    Question {
        unit: "Unit V · Metaphysics of Temporal Ontology",
        topic: "Presentism and the Past",
        text: "Presentism holds that only the present moment exists. But if the past is unreal, what makes claims about it true? \"Caesar crossed the Rubicon\" seems clearly true, yet on presentism there is no Caesar, no Rubicon-crossing, and no past event for the statement to be about. How should the presentist respond to this challenge?",
        prompt: "One response: present facts about the present world (causal traces, records, memories) make past-tense statements true without requiring the past entities to exist. Does this response capture what we mean when we say the past is real, or does it change the subject?",
    },

    // ── Unit VI · Topology and Direction ─────────────────────────────────────
    Question {
        unit: "Unit VI · Topology and Direction",
        topic: "Loschmidt's Reversibility Objection",
        text: "The fundamental laws of physics (with minor exceptions in weak-force interactions) are time-reversal symmetric: for every entropy-increasing evolution permitted by the laws, there is an equally lawful entropy-decreasing time-reverse of it. Yet we only ever observe entropy increasing. Where does the asymmetry come from if not from the laws themselves?",
        prompt: "Does the answer lie in the laws, in the initial conditions of the universe, in the structure of observation, or in something else? And does the answer explain the arrow of time or merely push the question back one step?",
    },
    Question {
        unit: "Unit VI · Topology and Direction",
        topic: "The Past Hypothesis",
        text: "Albert's Past Hypothesis: the universe began in an extraordinarily low-entropy state. Combined with statistical mechanics, this explains why entropy increases toward the future and not toward the past. But why was the initial state so special — so improbably low in entropy? Penrose estimates the probability at 1 in 10^(10^123). Is this a question that deserves a physical, metaphysical, or theological answer — or is it a brute fact about which no further explanation is possible?",
        prompt: "Boltzmann originally proposed that we are in a local entropy fluctuation within an eternal equilibrium universe. This avoids positing a special initial state — but it generates the Boltzmann Brain problem: it's overwhelmingly more likely that only a single fluctuating brain with false memories exists than that an entire ordered universe does. Does the Past Hypothesis escape this problem? At what cost?",
    },
    Question {
        unit: "Unit VI · Topology and Direction",
        topic: "The Causal Theory of Temporal Direction",
        text: "Reichenbach and others propose that the direction of time is grounded in causal asymmetry: causes precede their effects. Price objects that this is circular — our concept of causation already presupposes temporal direction, so we cannot use causation to ground it. Is Price's circularity objection decisive? Can a causal theory of temporal direction be given that avoids it?",
        prompt: "Consider whether causal asymmetry might be grounded in thermodynamic asymmetry, which is in turn grounded in the Past Hypothesis. Does this chain of reductions escape the circularity, or just make the circle larger?",
    },
    Question {
        unit: "Unit VI · Topology and Direction",
        topic: "Circular Time",
        text: "Could time be topologically circular — such that the \"end\" of the universe's history is identical with its beginning, looping back on itself? General relativity permits spacetimes with unusual global topologies, and some cosmological models suggest cyclic cosmologies. What would circular time mean for the arrow of time? And what would it mean to say that a given event is both \"before\" and \"after\" another?",
        prompt: "If time is circular, the Past Hypothesis might be trivially explained: the \"initial\" low-entropy state is the same as the \"final\" state, and the loop explains itself. But would this be an explanation, or an elegant redescription of the mystery?",
    },
    Question {
        unit: "Unit VI · Topology and Direction",
        topic: "The Psychological Arrow",
        text: "We remember the past and anticipate the future, not the reverse. Is this asymmetry in our psychological relationship to time explained by the physical thermodynamic arrow — our memories are records that require low-entropy past states to be causally formed — or is the psychological arrow independent, and perhaps more fundamental?",
        prompt: "Prior observes that there is something we are glad is over: a past suffering is relieved when over in a way that a future suffering, equally certain to occur, is not. Is this asymmetry fully explained by the physical arrow? Or does it reveal something about agency and time that physics has not captured?",
    },

    // ── Unit VII · Time and Physics ───────────────────────────────────────────
    Question {
        unit: "Unit VII · Time and Physics",
        topic: "The Relativity of Simultaneity",
        text: "Special relativity entails that whether two spatially separated events are simultaneous is not an objective matter but depends on the reference frame of the observer. Does this mean \"the present moment\" has no objective physical meaning, making presentism untenable? Or can the presentist find a privileged frame — a \"real\" present — without contradicting the physics?",
        prompt: "The neo-Lorentzian strategy: keep a privileged frame but admit we cannot detect it, since the Lorentz symmetry perfectly conceals it. Is this scientifically honest? Is it philosophically motivated, or is it an ad hoc rescue of a pre-relativistic metaphysics?",
    },
    Question {
        unit: "Unit VII · Time and Physics",
        topic: "The Twin Paradox",
        text: "One twin travels at near-light speed for what feels to her like five years, then returns to find her twin has aged thirty. Both twins claim, rightly, that the other was moving relative to them. Why does the traveler age less? What does the resolution of this paradox reveal about the nature of time?",
        prompt: "The key is that the traveling twin accelerates, and acceleration — unlike uniform motion — is not symmetric. Does this show that absolute acceleration (not just relative velocity) is physically real? And what does that imply for Mach's relational program?",
    },
    Question {
        unit: "Unit VII · Time and Physics",
        topic: "Time as Dynamical",
        text: "In general relativity, time is not a fixed background but a dynamic participant: mass curves spacetime, and the geometry of time — how fast clocks tick, how events are causally connected — depends on the distribution of matter and energy. GPS satellites must correct for this effect to remain accurate. What does it mean for time to be \"dynamical\" rather than a fixed container? Does this vindicate or undermine the philosophical picture of time as a dimension of reality?",
        prompt: "Newton thought time flows equably, indifferently to what happens within it. GR shows this is false. Does dynamical time support the relational view — that time is not a substance but a feature of relations between events — or is it still a \"container\" view, just a curved one?",
    },
    Question {
        unit: "Unit VII · Time and Physics",
        topic: "The Wheeler-DeWitt Equation",
        text: "The Wheeler-DeWitt equation, the leading candidate for a quantum theory of gravity, contains no time variable. If the fundamental equation of physics describing the universe as a whole has no time parameter, does this mean time is not fundamental — that it is emergent from deeper timeless structure? Or does the absence of explicit time in the equation reflect a feature of the formalism, not of reality?",
        prompt: "Rovelli proposes that time emerges from thermodynamics: \"thermal time\" is what clocks measure when a system is in a thermal state. Is emergent time still real time? Or is it a useful approximation that breaks down at the quantum gravity scale?",
    },
    Question {
        unit: "Unit VII · Time and Physics",
        topic: "Boltzmann Brains",
        text: "In a universe that reaches maximum entropy and lasts for infinite time, random quantum or thermal fluctuations would occasionally produce, purely by chance, a brain with false memories of an ordered past — a Boltzmann Brain. Such brains are vastly more probable than the entire ordered universe you seem to perceive. Why does this possibility constitute a serious problem for purely statistical explanations of the arrow of time?",
        prompt: "Does the Boltzmann Brain problem show that the Past Hypothesis is not a contingent feature of our universe but a necessary component of any adequate explanation of temporal asymmetry? Or is the problem generated by an illegitimate application of probability theory to cosmological questions?",
    },
    Question {
        unit: "Unit VII · Time and Physics",
        topic: "Noether's Theorem and Time",
        text: "Noether's theorem establishes that every continuous symmetry of the laws of physics corresponds to a conserved quantity. Symmetry under time-translation corresponds to conservation of energy. In certain cosmological models — for instance, when the universe is expanding — energy is not conserved. Does this mean that the laws of physics are not time-translation symmetric at the cosmological scale, and that there is something objectively special about some times compared to others?",
        prompt: "If the universe's expansion breaks time-translation symmetry, does this give a cosmological grounding for the distinction between past and future that the local, time-symmetric laws seem to lack?",
    },

    // ── Unit VIII · Phenomenology and Psychology ──────────────────────────────
    Question {
        unit: "Unit VIII · Phenomenology and Psychology",
        topic: "Husserl's Retention and Protention",
        text: "Husserl argues that to hear a melody as a melody — rather than as a series of isolated, disconnected tones — the mind must actively \"retain\" the just-passed notes while \"protending\" toward the notes that are about to come. What is the phenomenological structure that makes this possible? And how does this structure relate to the mathematical \"now\" of physical time, which is a durationless instant?",
        prompt: "Is Husserl's account of temporal consciousness merely a psychological observation, or does it have implications for the metaphysics of time? Can the phenomenological structure of retention-primal impression-protention be identified with any physical or neural process?",
    },
    Question {
        unit: "Unit VIII · Phenomenology and Psychology",
        topic: "The Specious Present",
        text: "The specious present — the psychologically extended \"now\" of roughly two to three seconds — does not correspond to any single physical instant. Yet it is the phenomenological unit within which experience is unified. How should we reconcile the psychologically real present with the durationless mathematical instant of physics? Which should constrain our metaphysics of the present?",
        prompt: "If the specious present is the phenomenologically real present, does presentism need to be a doctrine about the specious present rather than about the mathematical instant? And if so, what becomes of the argument that special relativity refutes presentism?",
    },
    Question {
        unit: "Unit VIII · Phenomenology and Psychology",
        topic: "Heidegger on Authentic Temporality",
        text: "Heidegger argues that authentic temporality — genuinely confronting one's finitude, taking responsibility for one's thrown existence — is constitutively different from the \"vulgar\" conception of time as an infinite sequence of now-points. Is there a sense in which anxiety about death is constitutive of temporal experience itself, or is this an existential projection onto something more neutral?",
        prompt: "Can a being that does not know it will die have a genuinely temporal experience in Heidegger's sense? What would a creature with no sense of finitude experience — an eternal present, a mere succession of instants, or something we have no concept for?",
    },
    Question {
        unit: "Unit VIII · Phenomenology and Psychology",
        topic: "Time Perception and Attention",
        text: "Studies in interval timing show that our subjective experience of duration is affected by attention, emotional arousal, temperature, dopamine levels, and whether we are watching the clock or absorbed in an activity. Does this variability suggest that subjective time is merely a cognitive construction — a useful fiction — or that it is a genuine form of temporal perception that is sensitive to real features of the world in a way that differs from how a clock is sensitive to them?",
        prompt: "The visual system distorts color perception in systematic ways (e.g., color constancy), yet we say we genuinely perceive colors. Is the variability of duration perception analogous — a distortion of genuine time perception — or is it evidence that subjective duration has no objective referent?",
    },
    Question {
        unit: "Unit VIII · Phenomenology and Psychology",
        topic: "Mental Time Travel",
        text: "Tulving's concept of \"episodic memory\" and \"mental time travel\" describes the human capacity to mentally re-experience the past and pre-experience the future. This capacity is claimed to be uniquely or specially developed in humans. If mental time travel is genuinely a form of temporal cognition — a way of accessing the past rather than merely representing it — what are the implications for the metaphysics of time? Does the past need to be, in some sense, accessible?",
        prompt: "Is mental time travel best understood as a form of perception (you are in some sense \"in\" the past moment), a form of imagination (you construct a representation of what the past was like), or something else? Does the answer matter for the metaphysics of memory?",
    },

    // ── Unit IX · Identity and Persistence ───────────────────────────────────
    Question {
        unit: "Unit IX · Identity and Persistence",
        topic: "Endurance vs. Perdurance",
        text: "Endurantism holds that you are wholly present at each moment of your existence — the very same thing at age ten and age forty. Perdurantism holds that you persist by having temporal parts at different times, as a road has spatial parts at different locations. Which view better captures what it means to be the same person across time? And can the debate be settled, or does it turn on a choice between frameworks?",
        prompt: "If you are a four-dimensional worm extending through time, your past self is a distinct temporal part of you — not you, but part of you. Is the relationship between you and your past self more like the relationship between you and your arm, or between you and your twin?",
    },
    Question {
        unit: "Unit IX · Identity and Persistence",
        topic: "Parfit on What Matters",
        text: "Parfit argues that personal identity is not what matters in survival. What matters is psychological continuity and connectedness — the preservation of memories, beliefs, desires, personality — which can hold to various degrees. If this is right, the question \"Will I survive?\" may have no determinate answer in certain cases, and that may not matter. Does Parfit's view dissolve or merely relocate our concern for our own futures?",
        prompt: "In Parfit's fission case, your brain is divided and each half transplanted into a different body; both resulting persons have your memories and personality. Is there a fact about which one is you? If not, what follows for the rationality of self-concern?",
    },
    Question {
        unit: "Unit IX · Identity and Persistence",
        topic: "Locke's Memory Theory",
        text: "Locke holds that you are the same person as a past person if and only if you can remember being that person. Reid objects: if a general remembers the brave officer, and the officer remembered the young soldier, but the general cannot remember the young soldier, then by transitivity the general is and is not the same person as the young soldier — a contradiction. Can the memory theory be repaired, or does this objection reveal a fundamental flaw?",
        prompt: "The obvious repair: replace direct memory with overlapping chains of memory connections. Does this repair succeed? And does it preserve what Locke cared about, namely that personal identity is constituted by consciousness rather than by bodily or metaphysical continuity?",
    },
    Question {
        unit: "Unit IX · Identity and Persistence",
        topic: "The Block Universe and the Self",
        text: "If the block universe is correct, your entire life exists as a completed four-dimensional object. In what sense is it meaningful to say that you are currently experiencing \"this\" moment of your life rather than any other? What distinguishes the \"now\" of your experience from the other temporal parts of the four-dimensional worm that is you?",
        prompt: "Can the block universe account for the indexical character of \"now\" — the fact that \"now\" always picks out the moment of utterance — without reintroducing an objective, moving present? Or does the persistence of the indexical reveal that the block universe has not captured something real?",
    },
    Question {
        unit: "Unit IX · Identity and Persistence",
        topic: "The Ship of Theseus",
        text: "The Ship of Theseus has every plank gradually replaced until not one original component remains. Is it still the same ship? Now suppose the original planks are preserved and reassembled into a second ship. Which is the \"real\" Ship of Theseus? This puzzle about spatial parts seems to have a direct analogue for temporal persistence: as your cells, memories, and beliefs change over time, what preserves your identity?",
        prompt: "Does the answer depend on causal continuity (the gradual replacement preserves the ship; the reassembled ship is a replica), or on something else? And does the same criterion apply to persons?",
    },

    // ── Unit X · Ethics and Existence ────────────────────────────────────────
    Question {
        unit: "Unit X · Ethics and Existence",
        topic: "The Deprivation Account of Death",
        text: "The deprivation account says death is bad for the person who dies because it deprives them of the goods they would otherwise have experienced. But if the person does not exist after death, there is no subject who is deprived — no one for whom death is bad at any time. Can this challenge to the deprivation account be met? And if so, when exactly is death bad for the person who dies?",
        prompt: "Consider the \"location problem\": is death bad ante-mortem (because of the anticipation of non-existence), at the moment of death, or posthumously? Each answer faces serious objections. Is the location problem a genuine philosophical problem or a pseudo-problem generated by misapplying the concept of \"when\" to states that lack temporal location?",
    },
    Question {
        unit: "Unit X · Ethics and Existence",
        topic: "The Non-Identity Problem",
        text: "Parfit's non-identity problem: a policy decision will bring into existence different people than would otherwise exist. The people who come to exist cannot be said to have been harmed by the policy that produced them — if the policy had not been enacted, they would not have existed at all. Does this mean we have no moral obligations to future generations whose very existence we determine? Or does the problem reveal a defect in our concept of harm?",
        prompt: "One response: shift from person-affecting views (harm requires a specific harmed person) to impersonal views (outcomes can be bad even if no specific person is worse off). Does this response preserve what we care about in obligations to future generations, or does it change the moral landscape in troubling ways?",
    },
    Question {
        unit: "Unit X · Ethics and Existence",
        topic: "Fatalism and the Block Universe",
        text: "If the block universe is correct and the future already exists as determinate reality, does this give us reasons to believe in fatalism — the view that what will happen cannot be prevented? Or is fatalism a logical error: even in a block universe, what will happen is what will happen as a result of causes, including our deliberate choices?",
        prompt: "Aristotle's sea-battle argument: if it is already true that there will be a sea-battle tomorrow, then it cannot be prevented; and if it is already true that there will not be, it cannot be forced to occur. Does this argument establish fatalism? And does the block universe make it valid, or was it valid (or invalid) all along?",
    },
    Question {
        unit: "Unit X · Ethics and Existence",
        topic: "Obligations to Future Generations",
        text: "Future generations cannot advocate for themselves, cannot vote, and cannot bring legal claims. Yet the decisions we make today — about climate change, nuclear waste storage, constitutional design, resource extraction — will profoundly shape the conditions of their existence. What obligations, if any, do we have to people who do not yet exist? And is it defensible to discount their interests on the grounds that they are temporally distant from us?",
        prompt: "Temporal discounting in economics applies to future money flows; philosophers debate whether it should also apply to future welfare. Is discounting future persons' interests analogous to discounting distant persons' interests — which most find morally objectionable — or is it different in kind?",
    },

    // ── Unit XI · Time Travel ────────────────────────────────────────────────
    Question {
        unit: "Unit XI · Time Travel",
        topic: "Lewis on Time Travel",
        text: "Lewis defines time travel as a discrepancy between personal time — measured by the traveler's biological and psychological processes — and external time — measured by the rest of the world. Is this definition adequate? Does it require the existence of a fixed, objective external time? And is personal time genuinely a form of time, or merely a convenient label for a causal-biographical trajectory?",
        prompt: "Could a presentist coherently believe in time travel? If only the present exists, what does the traveler travel to? And if the past is unreal, what are they doing when they arrive there?",
    },
    Question {
        unit: "Unit XI · Time Travel",
        topic: "The Grandfather Paradox",
        text: "A time traveler goes back and kills their grandfather before the traveler's parent is conceived. This seems to imply both that the traveler exists (they traveled back) and that they do not (the grandfather died). Lewis's response: the traveler \"can\" kill the grandfather in one sense — they have the skills, the weapon, the opportunity — but \"cannot\" in another sense, because the grandfather was not killed. Is Lewis's response genuinely satisfying, or is it simply an assertion that the paradox will not occur?",
        prompt: "Does Lewis's response rely on a coherent distinction between two senses of \"can\" — ability and compossibility — or does it make the future's influence over the past seem like a mysterious external force constraining the traveler's freedom?",
    },
    Question {
        unit: "Unit XI · Time Travel",
        topic: "Closed Timelike Curves",
        text: "General relativity permits spacetimes containing closed timelike curves — paths through spacetime that loop back to their own past. If you were traveling along such a curve, would you notice? What would consciousness be like for a being traversing a CTC? And does the existence of such a path in a solution to Einstein's equations mean that time travel is physically possible, or merely mathematically representable?",
        prompt: "Consider whether the \"direction\" of experience — the felt sense that time moves forward — is itself a physical fact about the traveler's thermodynamic state, or a phenomenological datum that physics cannot explain. Could a mind traverse a CTC and feel nothing unusual?",
    },
    Question {
        unit: "Unit XI · Time Travel",
        topic: "The Chronology Protection Conjecture",
        text: "Hawking proposed that the laws of physics conspire to prevent the formation of closed timelike curves and thus protect the chronological order of events. This conjecture remains unproved, and some physicists doubt it. Is chronological protection a scientific hypothesis — something that could in principle be falsified — or a philosophical principle about what kinds of causal structures are permissible? And why do we feel that chronological order must be preserved?",
        prompt: "Is the desire for chronological protection a physically motivated requirement, or does it reflect an intuition — that causes must precede their effects — that physics may simply not be obligated to honor? What would the world have to be like for time travel to be both physically possible and causally coherent?",
    },

    // ── Unit XII · Cutting-Edge Questions ────────────────────────────────────
    Question {
        unit: "Unit XII · Cutting-Edge Questions",
        topic: "The Emergence of Time",
        text: "Rovelli, Barbour, and others argue that time does not exist at the fundamental level of quantum gravity: the Wheeler-DeWitt equation is timeless, and \"time\" emerges only at scales where thermodynamic gradients exist. If this is correct, what survives of the ordinary concept of time? Is the time we experience a real feature of the world, an emergent but genuine quantity, or a useful approximation that breaks down at the deepest level of description?",
        prompt: "Compare: temperature is not a fundamental quantity in microphysics but is perfectly real at the thermodynamic level. Is \"emergent time\" real in the same sense? Or is there something special about time — as the condition of all experience — that makes its emergence philosophically more troubling than the emergence of temperature?",
    },
    Question {
        unit: "Unit XII · Cutting-Edge Questions",
        topic: "The Hard Problem of Temporal Experience",
        text: "Even if neuroscience gave us a complete account of the brain mechanisms underlying time perception — the dopaminergic interval timer, the neural oscillators, the cortical dynamics of duration estimation — would we have explained why those processes feel like anything? Why is there \"something it is like\" to wait, to anticipate, to remember? Or is the phenomenology of time fully exhausted by its functional and neural basis?",
        prompt: "Is the hard problem of temporal experience a special instance of the general hard problem of consciousness? Or does temporal experience have features — the lived sense of passage, the felt openness of the future — that make it especially resistant to third-person explanation?",
    },
    Question {
        unit: "Unit XII · Cutting-Edge Questions",
        topic: "Longtermism and Temporal Impartiality",
        text: "Longtermists argue that because future populations could be astronomically large and long-lived, the expected moral value of reducing existential risk vastly outweighs any other moral consideration. Critics charge that this is \"fanatical\" — it would justify enormous present sacrifices for small probabilities of astronomical distant gains. Does longtermist reasoning rest on temporal impartiality (all times count equally) in a way that is philosophically defensible, or does it distort practical reason by making the present morally negligible?",
        prompt: "Suppose you can either prevent 10,000 deaths now with certainty, or take a 1-in-a-billion chance of preventing 100 trillion deaths 10,000 years from now. What does expected value theory say? Do you trust that answer? And if not, why not?",
    },
    Question {
        unit: "Unit XII · Cutting-Edge Questions",
        topic: "Cross-Cultural Philosophy of Time",
        text: "Nāgārjuna's Madhyamaka Buddhism argues that nothing — including time itself — has intrinsic existence; all temporal relations are dependently arisen and ultimately empty (śūnya) of inherent being. How does this position compare with analytic debates between presentism and eternalism? Does the Madhyamaka framework offer philosophical resources that Western analytic philosophy lacks, or are the frameworks too different to allow genuine comparison?",
        prompt: "Śaṅkara's Advaita Vedānta holds that time is a feature of māyā — appearance, not ultimate reality. Is this structurally similar to McTaggart's conclusion that time is unreal? Or are the motivations, methods, and conclusions different enough that comparison distorts both traditions?",
    },
    Question {
        unit: "Unit XII · Cutting-Edge Questions",
        topic: "The Passage Problem",
        text: "Why does time seem to flow? The phenomenological passage problem asks why we experience time as moving; the metaphysical passage problem asks whether anything in the objective structure of the world corresponds to this experience. A satisfying answer to one problem might not answer the other. Which do you find more tractable? And could a complete neuroscience of time perception, however detailed, settle the metaphysical question — or would it always leave the metaphysical question open?",
        prompt: "Suppose we discovered that the felt sense of passage is a systematic illusion produced by a specific neural mechanism. Would this show that time doesn't really flow, or only that our neural machinery generates an inaccurate representation of something that does?",
    },
    Question {
        unit: "Unit XII · Cutting-Edge Questions",
        topic: "Is Time Real?",
        text: "After traversing the full landscape of the philosophy of time — Augustine's distentio animi, Aristotle's numbered motion, Kant's form of inner sense, Einstein's relativistic spacetime, the Wheeler-DeWitt equation, Husserl's retention and protention — has your answer to the question \"Is time real?\" changed? What would it take to convince you that time is not real? And what would be lost — philosophically, practically, existentially — if the answer were no?",
        prompt: "McTaggart, Barbour, and Rovelli all argue that time is, in some sense, not fundamental or not real. Yet the experience of time — of passage, anticipation, regret, hope — is perhaps the most inescapable feature of conscious existence. Can both be true? What would it mean for something to be experientially inescapable but metaphysically absent?",
    },
];
