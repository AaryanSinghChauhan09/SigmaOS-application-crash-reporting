// SPDX-License-Identifier: Apache-2.0
//! SigmaOS-application-crash-reporting: application crash reporting component model.
//!
//! Real implementation: reportings, events, validation, deterministic storage.
//! Replaces the placeholder stub.

use std::collections::BTreeMap;
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Reporting {
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
pub enum ApplicationCrashReportingError {
    EmptyReportingName,
    DuplicateReporting(String),
    EmptyEventSubject,
}

impl fmt::Display for ApplicationCrashReportingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyReportingName => write!(f, "reporting name must not be empty"),
            Self::DuplicateReporting(id) => write!(f, "duplicate reporting {:?}", id),
            Self::EmptyEventSubject => write!(f, "event subject must not be empty"),
        }
    }
}

impl std::error::Error for ApplicationCrashReportingError {}

pub struct ApplicationCrashReporting {
    pub name: String,
    reportings: BTreeMap<String, Reporting>,
    events: BTreeMap<String, Event>,
}

impl ApplicationCrashReporting {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            reportings: BTreeMap::new(),
            events: BTreeMap::new(),
        }
    }

    pub fn add_reporting(&mut self, item: Reporting) -> Result<(), ApplicationCrashReportingError> {
        if item.name.trim().is_empty() {
            return Err(ApplicationCrashReportingError::EmptyReportingName);
        }
        let id = item.id.clone();
        if self.reportings.contains_key(&id) {
            return Err(ApplicationCrashReportingError::DuplicateReporting(id.clone()));
        }
        self.reportings.insert(id, item);
        Ok(())
    }

    pub fn reporting_count(&self) -> usize {
        self.reportings.len()
    }

    pub fn get_reporting(&self, id: &str) -> Option<&Reporting> {
        self.reportings.get(id)
    }

    pub fn add_event(&mut self, event: Event) -> Result<(), ApplicationCrashReportingError> {
        if event.subject.trim().is_empty() {
            return Err(ApplicationCrashReportingError::EmptyEventSubject);
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

    fn sample_reporting(id: &str, name: &str) -> Reporting {
        Reporting {
            id: id.into(),
            name: name.into(),
            kind: "default".into(),
            detail: String::new(),
        }
    }

    #[test]
    fn test_create() {
        let m = ApplicationCrashReporting::new("ApplicationCrashReporting");
        assert_eq!(m.name, "ApplicationCrashReporting");
        assert_eq!(m.reporting_count(), 0);
        assert_eq!(m.event_count(), 0);
    }

    #[test]
    fn test_add_reporting() {
        let mut m = ApplicationCrashReporting::new("ApplicationCrashReporting");
        m.add_reporting(sample_reporting("a1", "Alpha")).unwrap();
        assert_eq!(m.reporting_count(), 1);
        assert!(m.get_reporting("a1").is_some());
    }

    #[test]
    fn test_empty_name_rejected() {
        let mut m = ApplicationCrashReporting::new("ApplicationCrashReporting");
        assert!(m.add_reporting(sample_reporting("x", "   ")).is_err());
    }

    #[test]
    fn test_duplicate_rejected() {
        let mut m = ApplicationCrashReporting::new("ApplicationCrashReporting");
        m.add_reporting(sample_reporting("x", "One")).unwrap();
        assert!(m.add_reporting(sample_reporting("x", "Two")).is_err());
    }

    #[test]
    fn test_event_validation() {
        let mut m = ApplicationCrashReporting::new("ApplicationCrashReporting");
        let bad = Event { id: "e1".into(), subject: "".into(), detail: String::new(), timestamp: 0 };
        assert!(m.add_event(bad).is_err());
        let good = Event { id: "e1".into(), subject: "boot".into(), detail: String::new(), timestamp: 1 };
        m.add_event(good).unwrap();
        assert_eq!(m.event_count(), 1);
    }
}
