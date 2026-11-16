// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-systemd-generator: systemd generator component model.
//!
//! Real implementation: generators, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generator {
    pub id: String,
    pub name: String,
    pub kind: String,
    pub detail: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Event {
    pub id: String,
    pub subject: String,
    pub detail: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SystemdGeneratorError {
    EmptyGeneratorName,
    DuplicateGenerator(String),
    EmptyEventSubject,
}

impl fmt::Display for SystemdGeneratorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyGeneratorName => write!(f, "generator name must not be empty"),
            Self::DuplicateGenerator(id) => write!(f, "duplicate generator {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for SystemdGeneratorError {}

pub struct SystemdGenerator {
    pub name: String,
    generators: BTreeMap<String, Generator>,
    events: BTreeMap<String, Event>,
}

impl SystemdGenerator {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            generators: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_generator(&mut self, item: Generator) -> Result<(), SystemdGeneratorError> {
        if item.name.trim().is_empty() {
            return Err(SystemdGeneratorError::EmptyGeneratorName);
        }
        let id = item.id.clone();
        if self.generators.contains_key(&id) {
            return Err(SystemdGeneratorError::DuplicateGenerator(id.clone()));
        }
        self.generators.insert(id, item);
        Ok(())
    }

    pub fn generator_count(&self) -> usize {
        self.generators.len()
    }

    pub fn get_generator(&self, id: &str) -> Option<&Generator> {
        self.generators.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), SystemdGeneratorError> {
        if event.subject.trim().is_empty() {
            return Err(SystemdGeneratorError::EmptyEventSubject);
        }
        self.events.insert(event.id.clone(), event);
        Ok(())
    }

    pub fn event_count(&self) -> usize {
        self.events.len()
    }

    pub fn get_event(&self, id: &str) -> Option<&Event> {
        self.events.get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_generator(id: &str, name: &str) -> Generator {
        Generator {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = SystemdGenerator::new("SystemdGenerator");
        assert_eq!(m.name, "SystemdGenerator");
        assert_eq!(m.generator_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_generator() {
        let mut m = SystemdGenerator::new("SystemdGenerator");
        m.add_generator(sample_generator("a1", "Alpha")).unwrap();
        assert_eq!(m.generator_count(), 1);
        assert!(m.get_generator("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = SystemdGenerator::new("SystemdGenerator");
        assert!(m.add_generator(sample_generator("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = SystemdGenerator::new("SystemdGenerator");
        m.add_generator(sample_generator("x", "One")).unwrap();
        assert!(m.add_generator(sample_generator("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = SystemdGenerator::new("SystemdGenerator");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
