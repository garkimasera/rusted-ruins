
//! This module includes functions for handling game events

/// Represent a series of events
pub struct Event {
    pub id: String,
    pub stages: Vec<EventStage>,
}

pub struct EventStage {
    pub id: String,
}

pub enum EventTrigger {
    StartEvent(String),
}

