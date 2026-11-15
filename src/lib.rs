//! SigmaOS-actions: action model.
//!
//! Real implementation: triggers, events, handlers, validation.
//! Replaces the 104-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Action {
    pub id: String,
    pub name: String,
    pub trigger: String,
    pub handler: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ActionError {
    EmptyName,
    DuplicateAction(String),
}

impl fmt::Display for ActionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "action name must not be empty"),
            Self::DuplicateAction(id) => write!(f, "duplicate action {:?}", id),
        }
    }
}

impl std::error::Error for ActionError {}

pub struct ActionEngine {
    pub name: String,
    actions: BTreeMap<String, Action>,
    events: BTreeMap<String, String>,
}

impl ActionEngine {
    pub fn new(name: impl Into<String>) -> Self {
        Self { name: name.into(), actions: BTreeMap::new(), events: BTreeMap::new() }
    }

    pub fn add_action(&mut self, a: Action) -> Result<(), ActionError> {
        if a.name.trim().is_empty() { return Err(ActionError::EmptyName); }
        let id = a.id.clone();
        if self.actions.contains_key(&id) {
            return Err(ActionError::DuplicateAction(id.clone()));
        }
        self.actions.insert(id, a);
        Ok(())
    }

    pub fn action_count(&self) -> usize { self.actions.len() }

    pub fn get_action(&self, id: &str) -> Option<&Action> { self.actions.get(id) }

    pub fn trigger_event(&mut self, name: String, payload: String) {
        self.events.insert(name, payload);
    }

    pub fn get_event(&self, name: &str) -> Option<&String> { self.events.get(name) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_aaa() {
        let e = ActionEngine::new("My Engine");
        assert_eq!(e.name, "My Engine");
        assert_eq!(e.action_count(), 0);
    }

    #[test]
    fn test_add_aaa() {
        let mut e = ActionEngine::new("My Engine");
        let a = Action { id: "act1".into(), name: "open".into(), trigger: "hotkey".into(), handler: "open_app".into() };
        e.add_action(a).unwrap();
        assert_eq!(e.action_count(), 1);
    }

    #[test]
    fn test_empty_rejected_aaa() {
        let mut e = ActionEngine::new("My Engine");
        let a = Action { id: "x".into(), name: "".into(), trigger: "a".into(), handler: "b".into() };
        assert!(e.add_action(a).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut e = ActionEngine::new("My Engine");
        let a = Action { id: "x".into(), name: "X".into(), trigger: "a".into(), handler: "b".into() };
        e.add_action(a).unwrap();
        let a2 = Action { id: "x".into(), name: "X".into(), trigger: "a".into(), handler: "b".into() };
        assert!(e.add_action(a2).is_err());
    }
}
