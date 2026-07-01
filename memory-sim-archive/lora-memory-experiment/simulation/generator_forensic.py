"""
Memory templates for Forensic Psychology domain.

These are scenes — moments where forensic thinking was applied,
not lectures on forensic psychology.
"""

from simulation.generator import FILLER_CONTENT

FORENSIC_TEMPLATES = [
    {
        "title_template": "The Statement That Didn't Match",
        "scene": "{person} gave a detailed account of what happened. The words were clear, the timeline made sense, and the details were specific. But something was wrong — {mismatch}. The content was technically consistent, but the {pattern_type} didn't match someone who had actually experienced what they were describing. {what_was_revealed}",
        "behaviors": ["F-001", "F-002"],
        "sensory_anchors": ["the rhythm of the words", "the pause that was too long", "the detail that was too convenient"],
        "emotions": ["the unease of something not fitting", "the click of recognition", "controlled attention"],
    },
    {
        "title_template": "Reading the Room Before It Became a Room",
        "scene": "Walked into {setting}. Within seconds, before anyone spoke: {baseline_observations}. Not because I was looking for trouble — because I always establish what normal looks like first. Later, when {deviation}, the shift was immediate and visible because the baseline was already in place. {response}",
        "behaviors": ["F-004", "P-002"],
        "sensory_anchors": ["the ambient noise level", "where people were standing", "the position of hands and eyes"],
        "emotions": ["calm vigilance", "the satisfaction of having a baseline", "the shift when something changes"],
    },
    {
        "title_template": "The Real Threat Behind the Noise",
        "scene": "Everyone was focused on {loud_threat}. The person making noise, demanding attention, being visibly aggressive. But the actual threat was {quiet_person} — not because they said anything, but because their behavior didn't match the situation. {why_they_were_dangerous}. The loud one was posturing. The quiet one was planning. {outcome}",
        "behaviors": ["F-003", "F-001"],
        "sensory_anchors": ["the loud voice drawing attention", "the stillness that didn't fit", "the hands that weren't nervous"],
        "emotions": ["recognition of misdirection", "controlled fear transmuted to focus", "the clarity of seeing the real danger"],
    },
    {
        "title_template": "The Justification That Sounded Reasonable",
        "scene": "The argument was: {justification}. On the surface, it sounded like {surface_reading}. But the structure was classic {thinking_pattern}: {pattern_breakdown}. The reasoning wasn't wrong in isolation — it was wrong in structure. It was the same cognitive architecture as {criminal_analogy}. {realization}",
        "behaviors": ["F-005", "R-004"],
        "sensory_anchors": ["the reasonable tone masking the unreasonable structure", "the familiarity of the pattern", "the recognition that this is how it starts"],
        "emotions": ["recognition of the pattern beneath the words", "unease at how reasonable it sounded", "clarity about what's actually being justified"],
    },
    {
        "title_template": "Why She Went Back",
        "scene": "Everyone asked why {person} went back. As if leaving were simple. As if {simplistic_explanation}. But trauma response doesn't work like that. {actual_explanation}. Going back wasn't weakness — it was an adaptation to a threat environment that {person} couldn't escape yet. Understanding this changed how I saw the whole situation. {shift_in_understanding}",
        "behaviors": ["F-006", "R-007"],
        "sensory_anchors": ["the weight of the question 'why didn't she just leave'", "the gap between what people expect and what actually happens", "the invisible cage of conditioning"],
        "emotions": ["frustration at the wrong question", "understanding replacing judgment", "the recognition of survival logic"],
    },
    {
        "title_template": "What Wasn't Said",
        "scene": "The statement was {statement_length}. Detailed, specific, chronological. But what mattered was what was missing: {missing_element}. Honest accounts include {expected_inclusion}. Deceptive accounts often exclude it because {why_excluded}. The absence was louder than the presence. {detection}",
        "behaviors": ["F-002", "R-006"],
        "sensory_anchors": ["the shape of the silence", "what should have been there", "the gap that speaks"],
        "emotions": ["the precision of absence", "the satisfaction of finding what's hidden in what's not there", "quiet certainty"],
    },
    {
        "title_template": "Posturing vs Planning",
        "scene": "In {situation}, {person_a} was loud, aggressive, clearly dangerous — or so it seemed. {person_b} was calm, still, watching. The instinct was to focus on {person_a}. But {person_a}'s aggression was {aggression_type} — designed to be seen, to intimidate, to create an audience. {person_b}'s stillness was {stillness_type} — the calm of someone who had already decided what they were going to do. {assessment}",
        "behaviors": ["F-003", "F-004"],
        "sensory_anchors": ["the difference between performed aggression and real threat", "the stillness that means something", "the eyes that aren't scanning — they've already chosen"],
        "emotions": ["the shift from reactive to analytical", "recognition of the real danger", "controlled awareness"],
    },
]

# Filler content for forensic psychology
FILLER_CONTENT["forensic"] = {
    "people": ["the witness", "the suspect", "the informant", "the colleague", "the interviewee", "the neighbor", "the client"],
    "mismatches": ["the emotional display didn't match the content", "the level of detail was inconsistent with the claimed emotional state", "the timeline had convenient gaps", "the denial was too specific"],
    "pattern_types": ["emotional rhythm", "level of detail", "spontaneous correction pattern", "pronoun usage"],
    "what_was_revealed": ["the rehearsed quality became visible once the mismatch was spotted", "the statement was constructed, not recalled", "the emotion was performed, not experienced"],
    "settings": ["the interview room", "the meeting", "the hallway", "the parking lot", "the kitchen", "the office"],
    "baseline_observations": ["who was standing where, who was watching who, where the exits were", "the tension level, the noise pattern, the eye contact dynamics", "body positions, breathing rates, which conversations stopped when I entered"],
    "deviation": "something shifted",
    "loud_threat": "the person shouting across the room",
    "quiet_person": "the one standing still in the corner",
    "why_they_were_dangerous": "their breathing was controlled, their eyes were tracking rather than reacting, their hands were positioned for action",
    "justification": "they had no choice because the alternative was worse",
    "surface_reading": "a reasonable person in a difficult situation making the best decision they could",
    "thinking_pattern": "minimization and justification",
    "pattern_breakdown": "minimizing the harm, justifying the action, positioning themselves as the real victim",
    "criminal_analogy": "the same pattern used by people who commit fraud — it's not stealing, it's what I'm owed",
    "simplistic_explanation": "leaving were a single decision you make once",
    "actual_explanation": "the threat environment makes leaving feel more dangerous than staying, the conditioning makes the abnormal feel normal, and the practical barriers make escape feel impossible",
    "statement_length": "comprehensive",
    "missing_element": "spontaneous corrections, first-person emotional language, or sensory details that come from experiencing rather than constructing",
    "expected_inclusion": "corrections, hedging, uncertainty, sensory details that only someone who was there would know",
    "why_excluded": "constructed statements follow a logical narrative rather than the messy reality of actual experience",
    "detection": "The absence of these elements was itself the signal",
    "aggression_type": "performative",
    "stillness_type": "preparatory",
}