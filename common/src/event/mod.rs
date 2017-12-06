
//! This module includes functions for handling game events

/// Represent a series of events
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub stages: Vec<EventStage>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventStage {
    pub id: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum EventTrigger {
    StartEvent(String),
}

