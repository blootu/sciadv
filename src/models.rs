use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Route {
    pub name: String,
    pub description: String,
    pub chapters: Vec<Chapter>,
    pub prerequisites: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Chapter {
    pub number: u32,
    pub name: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub id: String,
    pub description: String,
    pub step_type: StepType,
    pub completed: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum StepType {
    DelusionTrigger {
        trigger_number: u32,
        polarity: Polarity,
        location: String,
    },
    YesNoPrompts {
        prompts: Vec<PromptChoice>,
    },
    GeneralInstruction {
        instruction: String,
    },
    Checkpoint {
        save_point: String,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Polarity {
    Positive,
    Negative,
    Neutral,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PromptChoice {
    pub question: String,
    pub answer: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    pub routes: Vec<Route>,
    pub current_route: Option<usize>,
    pub progress: HashMap<String, bool>,
}

impl GameData {
    pub fn new() -> Self {
        Self {
            routes: Vec::new(),
            current_route: None,
            progress: HashMap::new(),
        }
    }

    pub fn mark_step_completed(&mut self, step_id: &str, completed: bool) {
        self.progress.insert(step_id.to_string(), completed);
        
        if let Some(route_idx) = self.current_route {
            if let Some(route) = self.routes.get_mut(route_idx) {
                for chapter in &mut route.chapters {
                    for step in &mut chapter.steps {
                        if step.id == step_id {
                            step.completed = completed;
                            break;
                        }
                    }
                }
            }
        }
    }

    pub fn get_completion_percentage(&self, route_idx: usize) -> f32 {
        if let Some(route) = self.routes.get(route_idx) {
            let total_steps: usize = route.chapters.iter()
                .map(|chapter| chapter.steps.len())
                .sum();
            
            let completed_steps: usize = route.chapters.iter()
                .flat_map(|chapter| &chapter.steps)
                .filter(|step| step.completed)
                .count();
            
            if total_steps == 0 {
                0.0
            } else {
                (completed_steps as f32 / total_steps as f32) * 100.0
            }
        } else {
            0.0
        }
    }
}