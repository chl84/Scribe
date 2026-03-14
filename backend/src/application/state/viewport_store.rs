use std::collections::HashMap;

use crate::domain::document::{DocumentSessionId, ViewportSessionId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ViewportSession {
    document_session_id: DocumentSessionId,
    top_line: usize,
    visible_line_count: usize,
}

impl ViewportSession {
    pub fn new(
        document_session_id: DocumentSessionId,
        top_line: usize,
        visible_line_count: usize,
    ) -> Self {
        Self {
            document_session_id,
            top_line,
            visible_line_count,
        }
    }

    pub fn document_session_id(&self) -> DocumentSessionId {
        self.document_session_id
    }

    pub fn top_line(&self) -> usize {
        self.top_line
    }

    pub fn visible_line_count(&self) -> usize {
        self.visible_line_count
    }

    pub fn set_top_line(&mut self, top_line: usize) {
        self.top_line = top_line;
    }
}

#[derive(Debug, Default)]
pub struct ViewportStore {
    next_viewport_id: u64,
    viewports: HashMap<ViewportSessionId, ViewportSession>,
}

impl ViewportStore {
    pub fn create_viewport(
        &mut self,
        document_session_id: DocumentSessionId,
        top_line: usize,
        visible_line_count: usize,
    ) -> ViewportSessionId {
        self.next_viewport_id += 1;
        let viewport_session_id = ViewportSessionId::new(self.next_viewport_id);
        self.viewports.insert(
            viewport_session_id,
            ViewportSession::new(document_session_id, top_line, visible_line_count.max(1)),
        );

        viewport_session_id
    }

    pub fn get(&self, viewport_session_id: ViewportSessionId) -> Option<&ViewportSession> {
        self.viewports.get(&viewport_session_id)
    }

    pub fn get_mut(
        &mut self,
        viewport_session_id: ViewportSessionId,
    ) -> Option<&mut ViewportSession> {
        self.viewports.get_mut(&viewport_session_id)
    }
}
