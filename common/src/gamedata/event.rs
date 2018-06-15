
//! This module includes functions for handling game events.
//!
//! An event consists of its identifier (name + idx) and current stage name.
//! The progress of events are represented by their stage name.
//! EventTrigger is used to change the stages.

use std::collections::BTreeMap;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EventHolder {
    /// Ongoing events
    pub ongoing: BTreeMap<EventId, Event>,
}

#[derive(Clone, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
pub enum EventId {
    Scenario {
        name: String,
    },
    FreeMission,
}

/// Represent an ongoing event
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Event {
    pub current_stage: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "kind")]
pub enum EventTrigger {
    Start {
        id: EventId,
        first_stage: String,
    },
    ProceedStage {
        id: EventId,
        name: String,
    },
    End {
        id: EventId,
    },
}

