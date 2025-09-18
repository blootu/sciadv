use crate::models::*;

pub fn create_chaos_head_data() -> Vec<Route> {
    vec![
        create_silent_sky_route(),
        create_crying_sky_route(),
        create_daydream_route(),
        create_moon_and_sun_route(),
        create_bloody_contract_route(),
        create_disease_slaughter_route(),
        create_deus_ex_machina_route(),
        create_anima_archetype_route(),
        create_blue_sky_route(),
    ]
}

fn create_silent_sky_route() -> Route {
    Route {
        name: "Silent Sky".to_string(),
        description: "The first ending - complete the basic story path".to_string(),
        prerequisites: vec![],
        chapters: vec![
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "silent_sky_ch1_swimsuit".to_string(),
                        description: "Answer swimsuit questions as you'd like".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Series of YES/NO questions about school swimsuits".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 2,
                name: "Chapter Two".to_string(),
                steps: vec![
                    Step {
                        id: "silent_sky_ch2_hospital".to_string(),
                        description: "Answer hospital checklist questions as you'd like".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Checklist questions when Takumi visits hospital - results appear in TIPS menu".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 6,
                name: "Chapter Six".to_string(),
                steps: vec![
                    Step {
                        id: "silent_sky_ch6_save".to_string(),
                        description: "Save your game before O-FRONT scene".to_string(),
                        step_type: StepType::Checkpoint {
                            save_point: "Before O-FRONT YES/NO questions".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "silent_sky_ch6_bad_ending".to_string(),
                        description: "Optional: View bad ending (NO, NO, NO)".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "First O-FRONT question".to_string(), answer: false },
                                PromptChoice { question: "Second O-FRONT question".to_string(), answer: false },
                                PromptChoice { question: "Third O-FRONT question".to_string(), answer: false },
                            ],
                        },
                        completed: false,
                    },
                    Step {
                        id: "silent_sky_ch6_proceed".to_string(),
                        description: "Proceed with story (YES, YES, NO, NO, YES)".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "First O-FRONT question".to_string(), answer: true },
                                PromptChoice { question: "Second O-FRONT question".to_string(), answer: true },
                                PromptChoice { question: "Third O-FRONT question".to_string(), answer: false },
                                PromptChoice { question: "Fourth O-FRONT question".to_string(), answer: false },
                                PromptChoice { question: "Fifth O-FRONT question".to_string(), answer: true },
                            ],
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 10,
                name: "Chapter Ten".to_string(),
                steps: vec![
                    Step {
                        id: "silent_sky_ch10_ending".to_string(),
                        description: "Achieve Silent Sky ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Silent Sky ending will occur during this chapter".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_crying_sky_route() -> Route {
    Route {
        name: "Crying Sky".to_string(),
        description: "View at least 11 delusions throughout the story".to_string(),
        prerequisites: vec!["Silent Sky".to_string()],
        chapters: vec![
            Chapter {
                number: 0,
                name: "Setup".to_string(),
                steps: vec![
                    Step {
                        id: "crying_sky_setup".to_string(),
                        description: "Start new game after Silent Sky ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Use Skip mode to fast-forward through read text. Answer NO to all heroine YES/NO prompts.".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "crying_sky_dt1".to_string(),
                        description: "Delusion Trigger #1: In class, talking to Misumi - Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 1,
                            polarity: Polarity::Neutral,
                            location: "While in class, talking to Misumi".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "crying_sky_dt3".to_string(),
                        description: "Delusion Trigger #3: Yua looking at you from across the street - Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 3,
                            polarity: Polarity::Neutral,
                            location: "Yua is looking at you from across the street".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 2,
                name: "Chapter Two".to_string(),
                steps: vec![
                    Step {
                        id: "crying_sky_dt8".to_string(),
                        description: "Delusion Trigger #8: During Phantasm performance, watching FES - Positive".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 8,
                            polarity: Polarity::Positive,
                            location: "During the Phantasm performance, while watching FES".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "crying_sky_dt9".to_string(),
                        description: "Delusion Trigger #9: After Phantasm performance, playing ESO - Positive".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 9,
                            polarity: Polarity::Positive,
                            location: "After the Phantasm performance, while playing ESO".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_daydream_route() -> Route {
    Route {
        name: "Daydream".to_string(),
        description: "Nanami's route - focus on her character choices".to_string(),
        prerequisites: vec!["Crying Sky".to_string()],
        chapters: vec![
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "daydream_dt1".to_string(),
                        description: "Delusion Trigger #1: In class, talking to Misumi - Negative".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 1,
                            polarity: Polarity::Negative,
                            location: "While in class, talking to Misumi".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "daydream_dt2".to_string(),
                        description: "Delusion Trigger #2: In the Base with Nanami - Positive".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 2,
                            polarity: Polarity::Positive,
                            location: "In the Base with Nanami".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "daydream_nanami_choices".to_string(),
                        description: "Nanami relationship choices".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "I like big sister types more.".to_string(), answer: false },
                                PromptChoice { question: "I like girls with no chests.".to_string(), answer: true },
                                PromptChoice { question: "I have another 3D girlfriend besides you.".to_string(), answer: false },
                                PromptChoice { question: "You're cute, I'll admit that.".to_string(), answer: true },
                                PromptChoice { question: "I want to kiss you!".to_string(), answer: true },
                            ],
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 6,
                name: "Chapter Six".to_string(),
                steps: vec![
                    Step {
                        id: "daydream_ending".to_string(),
                        description: "Achieve Daydream ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Game will branch into Daydream ending in chapter six".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_moon_and_sun_route() -> Route {
    Route {
        name: "Moon and Sun".to_string(),
        description: "Yua's route - focus on her character choices".to_string(),
        prerequisites: vec!["Crying Sky".to_string()],
        chapters: vec![
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "moon_sun_dt1".to_string(),
                        description: "Delusion Trigger #1: In class, talking to Misumi - Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 1,
                            polarity: Polarity::Neutral,
                            location: "While in class, talking to Misumi".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "moon_sun_yua_choices".to_string(),
                        description: "Yua relationship choices".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "It's just a coincidence.".to_string(), answer: false },
                                PromptChoice { question: "I'm hallucinating.".to_string(), answer: false },
                                PromptChoice { question: "Whoever did it is inside the school.".to_string(), answer: true },
                                PromptChoice { question: "They're taunting me.".to_string(), answer: true },
                                PromptChoice { question: "I want to erase it now.".to_string(), answer: true },
                            ],
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 7,
                name: "Chapter Seven".to_string(),
                steps: vec![
                    Step {
                        id: "moon_sun_ending".to_string(),
                        description: "Achieve Moon and Sun ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Game will branch into Moon and Sun ending in chapter seven".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_bloody_contract_route() -> Route {
    Route {
        name: "A Bloody Contract for Your Sins".to_string(),
        description: "Ayase's route - focus on her character choices".to_string(),
        prerequisites: vec!["Crying Sky".to_string()],
        chapters: vec![
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "bloody_contract_dt1".to_string(),
                        description: "Delusion Trigger #1: In class, talking to Misumi - Negative".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 1,
                            polarity: Polarity::Negative,
                            location: "While in class, talking to Misumi".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "bloody_contract_dt2".to_string(),
                        description: "Delusion Trigger #2: In the Base with Nanami - Negative or Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 2,
                            polarity: Polarity::Negative,
                            location: "In the Base with Nanami".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 2,
                name: "Chapter Two".to_string(),
                steps: vec![
                    Step {
                        id: "bloody_contract_dt8".to_string(),
                        description: "Delusion Trigger #8: During Phantasm performance, watching FES - Negative".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 8,
                            polarity: Polarity::Negative,
                            location: "During the Phantasm performance, while watching FES".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "bloody_contract_ayase_choices".to_string(),
                        description: "Ayase relationship choices".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "FES wants me to save her.".to_string(), answer: true },
                                PromptChoice { question: "I want to give in to her temptation.".to_string(), answer: false },
                                PromptChoice { question: "It would feel so good to jump.".to_string(), answer: true },
                                PromptChoice { question: "I'd be willing to die if I could do it with FES.".to_string(), answer: true },
                                PromptChoice { question: "I want to be released from everything.".to_string(), answer: true },
                            ],
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 7,
                name: "Chapter Seven".to_string(),
                steps: vec![
                    Step {
                        id: "bloody_contract_ending".to_string(),
                        description: "Achieve A Bloody Contract for Your Sins ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Game will branch into A Bloody Contract for Your Sins ending in chapter seven".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_disease_slaughter_route() -> Route {
    Route {
        name: "A Disease that Leads to Slaughter".to_string(),
        description: "Kozue's route - focus on her character choices".to_string(),
        prerequisites: vec!["Crying Sky".to_string()],
        chapters: vec![
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "disease_slaughter_dt1".to_string(),
                        description: "Delusion Trigger #1: In class, talking to Misumi - Negative".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 1,
                            polarity: Polarity::Negative,
                            location: "While in class, talking to Misumi".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "disease_slaughter_dt2".to_string(),
                        description: "Delusion Trigger #2: In the Base with Nanami - Negative or Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 2,
                            polarity: Polarity::Negative,
                            location: "In the Base with Nanami".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 2,
                name: "Chapter Two".to_string(),
                steps: vec![
                    Step {
                        id: "disease_slaughter_dt8".to_string(),
                        description: "Delusion Trigger #8: During Phantasm performance, watching FES - Positive or Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 8,
                            polarity: Polarity::Positive,
                            location: "During the Phantasm performance, while watching FES".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 3,
                name: "Chapter Three".to_string(),
                steps: vec![
                    Step {
                        id: "disease_slaughter_dt10".to_string(),
                        description: "Delusion Trigger #10: At Center Street with Sena - Positive or Negative".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 10,
                            polarity: Polarity::Positive,
                            location: "At Center Street with Sena".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "disease_slaughter_dt13".to_string(),
                        description: "Delusion Trigger #13: In classroom, Kozue has joined your class - Positive".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 13,
                            polarity: Polarity::Positive,
                            location: "In the classroom, Kozue has just joined your class".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "disease_slaughter_kozue_choices".to_string(),
                        description: "Kozue relationship choices".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "She came to save me?".to_string(), answer: true },
                                PromptChoice { question: "The transfer student is a professional exorcist.".to_string(), answer: true },
                                PromptChoice { question: "Her wimpy looks are actually camouflage.".to_string(), answer: true },
                                PromptChoice { question: "She is here to kill Rimi.".to_string(), answer: true },
                                PromptChoice { question: "You two kill each other!".to_string(), answer: false },
                            ],
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 7,
                name: "Chapter Seven".to_string(),
                steps: vec![
                    Step {
                        id: "disease_slaughter_ending".to_string(),
                        description: "Achieve A Disease that Leads to Slaughter ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Game will branch into A Disease that Leads to Slaughter ending in chapter seven".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_deus_ex_machina_route() -> Route {
    Route {
        name: "Deus Ex Machina".to_string(),
        description: "Sena's route - focus on her character choices".to_string(),
        prerequisites: vec!["Crying Sky".to_string()],
        chapters: vec![
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "deus_ex_machina_dt1".to_string(),
                        description: "Delusion Trigger #1: In class, talking to Misumi - Negative".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 1,
                            polarity: Polarity::Negative,
                            location: "While in class, talking to Misumi".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "deus_ex_machina_dt2".to_string(),
                        description: "Delusion Trigger #2: In the Base with Nanami - Negative or Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 2,
                            polarity: Polarity::Negative,
                            location: "In the Base with Nanami".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 2,
                name: "Chapter Two".to_string(),
                steps: vec![
                    Step {
                        id: "deus_ex_machina_dt8".to_string(),
                        description: "Delusion Trigger #8: During Phantasm performance, watching FES - Positive or Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 8,
                            polarity: Polarity::Positive,
                            location: "During the Phantasm performance, while watching FES".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 3,
                name: "Chapter Three".to_string(),
                steps: vec![
                    Step {
                        id: "deus_ex_machina_dt10".to_string(),
                        description: "Delusion Trigger #10: At Center Street with Sena - Neutral".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 10,
                            polarity: Polarity::Neutral,
                            location: "At Center Street with Sena".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "deus_ex_machina_sena_choices".to_string(),
                        description: "Sena relationship choices".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "Look at me more.".to_string(), answer: true },
                                PromptChoice { question: "For her, I wouldn't mind begging on my knees.".to_string(), answer: true },
                                PromptChoice { question: "I wish she'd step on me, actually.".to_string(), answer: true },
                                PromptChoice { question: "That sword's going to run me through, isn't it? I can tell.".to_string(), answer: true },
                                PromptChoice { question: "She's going to kill me with that sword…".to_string(), answer: false },
                            ],
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 7,
                name: "Chapter Seven".to_string(),
                steps: vec![
                    Step {
                        id: "deus_ex_machina_ending".to_string(),
                        description: "Achieve Deus Ex Machina ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Game will branch into Deus Ex Machina ending in chapter seven".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_anima_archetype_route() -> Route {
    Route {
        name: "Anima Archetype".to_string(),
        description: "Rimi's route - focus on her character choices".to_string(),
        prerequisites: vec!["Crying Sky".to_string()],
        chapters: vec![
            Chapter {
                number: 1,
                name: "Chapter One".to_string(),
                steps: vec![
                    Step {
                        id: "anima_archetype_dt1".to_string(),
                        description: "Delusion Trigger #1: In class, talking to Misumi - Positive".to_string(),
                        step_type: StepType::DelusionTrigger {
                            trigger_number: 1,
                            polarity: Polarity::Positive,
                            location: "While in class, talking to Misumi".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "anima_archetype_rimi_choices".to_string(),
                        description: "Rimi appearance choices".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "…tied her hair with ribbons.".to_string(), answer: true },
                                PromptChoice { question: "…wore glasses.".to_string(), answer: false },
                                PromptChoice { question: "…wore over-the-knee socks.".to_string(), answer: true },
                                PromptChoice { question: "…wore a long skirt.".to_string(), answer: false },
                                PromptChoice { question: "…was posing like a soldier at attention.".to_string(), answer: true },
                            ],
                        },
                        completed: false,
                    },
                ],
            },
            Chapter {
                number: 8,
                name: "Chapter Eight".to_string(),
                steps: vec![
                    Step {
                        id: "anima_archetype_ending".to_string(),
                        description: "Achieve Anima Archetype ending".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Game will branch into Anima Archetype ending in chapter eight".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}

fn create_blue_sky_route() -> Route {
    Route {
        name: "Blue Sky".to_string(),
        description: "True ending - complete all other routes first".to_string(),
        prerequisites: vec![
            "Silent Sky".to_string(),
            "Crying Sky".to_string(),
            "Daydream".to_string(),
            "Moon and Sun".to_string(),
            "A Bloody Contract for Your Sins".to_string(),
            "A Disease that Leads to Slaughter".to_string(),
            "Deus Ex Machina".to_string(),
            "Anima Archetype".to_string(),
        ],
        chapters: vec![
            Chapter {
                number: 0,
                name: "True Ending".to_string(),
                steps: vec![
                    Step {
                        id: "blue_sky_unlock".to_string(),
                        description: "Select 'blue sky' from main menu".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "New main menu option unlocked after completing all other routes".to_string(),
                        },
                        completed: false,
                    },
                    Step {
                        id: "blue_sky_final_choices".to_string(),
                        description: "Final YES/NO sequence".to_string(),
                        step_type: StepType::YesNoPrompts {
                            prompts: vec![
                                PromptChoice { question: "Question 1".to_string(), answer: true },
                                PromptChoice { question: "Question 2".to_string(), answer: true },
                                PromptChoice { question: "Question 3".to_string(), answer: false },
                                PromptChoice { question: "Question 4".to_string(), answer: false },
                                PromptChoice { question: "Question 5".to_string(), answer: false },
                                PromptChoice { question: "Question 6".to_string(), answer: true },
                                PromptChoice { question: "Question 7".to_string(), answer: true },
                                PromptChoice { question: "Question 8".to_string(), answer: true },
                                PromptChoice { question: "Question 9".to_string(), answer: true },
                                PromptChoice { question: "Question 10".to_string(), answer: true },
                                PromptChoice { question: "Question 11".to_string(), answer: true },
                                PromptChoice { question: "Question 12".to_string(), answer: true },
                                PromptChoice { question: "Question 13".to_string(), answer: true },
                                PromptChoice { question: "Question 14".to_string(), answer: true },
                                PromptChoice { question: "Question 15".to_string(), answer: true },
                            ],
                        },
                        completed: false,
                    },
                    Step {
                        id: "blue_sky_ending".to_string(),
                        description: "Complete Chaos;Head NoAH".to_string(),
                        step_type: StepType::GeneralInstruction {
                            instruction: "Achieve Blue Sky ending and complete the game".to_string(),
                        },
                        completed: false,
                    },
                ],
            },
        ],
    }
}