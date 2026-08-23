use std::collections::HashSet;
use std::fmt;

/// Opaque workplace IDs. Counters are process-global on this instance
/// (same law as `src/workplace/ids.js`), not per workspace.
#[derive(Default)]
pub struct Ids {
    workspaces: u64,
    tabs: u64,
    panes: u64,
    sessions: u64,
    retired: HashSet<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct IdError {
    pub id: String,
}

impl fmt::Display for IdError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "dory: refused to reuse retired id {}", self.id)
    }
}

impl std::error::Error for IdError {}

impl Ids {
    pub fn new() -> Self {
        Self::default()
    }

    fn mint(&self, id: String) -> Result<String, IdError> {
        if self.retired.contains(&id) {
            return Err(IdError { id });
        }
        Ok(id)
    }

    /// `w1`, `w2`, …  (JS `window()`; public Rust name is `workspace`.)
    pub fn workspace(&mut self) -> Result<String, IdError> {
        self.workspaces += 1;
        self.mint(format!("w{}", self.workspaces))
    }

    /// `{workspace_id}:t{n}` with a global tab counter.
    pub fn tab(&mut self, workspace_id: &str) -> Result<String, IdError> {
        self.tabs += 1;
        self.mint(format!("{workspace_id}:t{}", self.tabs))
    }

    /// `{workspace_id}:p{n}` with a global pane counter.
    pub fn pane(&mut self, workspace_id: &str) -> Result<String, IdError> {
        self.panes += 1;
        self.mint(format!("{workspace_id}:p{}", self.panes))
    }

    /// Journal session `s1`, `s2`, … — not a pane id.
    pub fn session(&mut self) -> Result<String, IdError> {
        self.sessions += 1;
        self.mint(format!("s{}", self.sessions))
    }

    /// Closed IDs are never reused. A later mint of the same string errors.
    pub fn retire(&mut self, id: impl Into<String>) {
        self.retired.insert(id.into());
    }
}

#[cfg(test)]
mod tests {
    use super::Ids;

    #[test]
    fn mint_sequence_w1_t1_p1() {
        let mut ids = Ids::new();
        let w = ids.workspace().unwrap();
        let t = ids.tab(&w).unwrap();
        let p = ids.pane(&w).unwrap();
        assert_eq!(w, "w1");
        assert_eq!(t, "w1:t1");
        assert_eq!(p, "w1:p1");
    }

    #[test]
    fn retire_w1_next_workspace_is_w2() {
        let mut ids = Ids::new();
        let w1 = ids.workspace().unwrap();
        assert_eq!(w1, "w1");
        ids.retire(&w1);
        let w2 = ids.workspace().unwrap();
        assert_eq!(w2, "w2");
        assert_ne!(w2, w1);
    }

    #[test]
    fn refuse_reuse_of_retired_id() {
        let mut ids = Ids::new();
        ids.retire("w1");
        let err = ids.workspace().unwrap_err();
        assert_eq!(err.id, "w1");
        assert_eq!(err.to_string(), "dory: refused to reuse retired id w1");
    }

    #[test]
    fn session_is_not_a_pane_id() {
        let mut ids = Ids::new();
        let w = ids.workspace().unwrap();
        let pane = ids.pane(&w).unwrap();
        let session = ids.session().unwrap();
        assert_eq!(session, "s1");
        assert_eq!(pane, "w1:p1");
        assert_ne!(session, pane);
        assert!(
            !session.contains(":p"),
            "session {session} must not look like a pane id"
        );
    }
}
