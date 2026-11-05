#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ContextAction {
    pub id: String,
    pub label: String,
    pub command: String,
}

#[derive(Debug, Default)]
pub struct ActionsManager {
    actions: Vec<ContextAction>,
}

impl ActionsManager {
    pub fn new() -> Self { Self { actions: Vec::new() } }
    pub fn register(&mut self, id: &str, label: &str, cmd: &str) {
        self.actions.push(ContextAction { id: id.into(), label: label.into(), command: cmd.into() });
    }
    pub fn count(&self) -> usize { self.actions.len() }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_actions_manager_aaa() {
        // Arrange
        let mut mgr = ActionsManager::new();
        // Act
        mgr.register("open-terminal", "Open In Terminal", "sigmaos-term --dir %F");
        // Assert
        assert_eq!(mgr.count(), 1);
    }
}
