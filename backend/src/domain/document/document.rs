use super::{
    ChangeSet, CursorMove, CursorMoveRules, DocumentError, DocumentId, Edit, LineIndex,
    NewlinePolicy, PieceTable, Position, RevisionId, Selection, TextBuffer, TextOffset, TextRange,
    TextSnapshot, UndoManager,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    id: DocumentId,
    revision: RevisionId,
    buffer: PieceTable,
    line_index: LineIndex,
    history: UndoManager,
    newline_policy: NewlinePolicy,
}

impl Document {
    pub fn open(id: DocumentId, text: impl Into<String>) -> Self {
        let text = text.into();
        let newline_policy = NewlinePolicy::detect(&text);
        let buffer = PieceTable::new(text);
        let line_index = LineIndex::from_snapshot(&buffer.snapshot());

        Self {
            id,
            revision: RevisionId::initial(),
            buffer,
            line_index,
            history: UndoManager::default(),
            newline_policy,
        }
    }

    pub const fn id(&self) -> DocumentId {
        self.id
    }

    pub const fn revision(&self) -> RevisionId {
        self.revision
    }

    pub fn text(&self) -> String {
        self.buffer.snapshot().as_str().to_string()
    }

    pub fn snapshot(&self) -> TextSnapshot {
        self.buffer.snapshot()
    }

    pub const fn newline_policy(&self) -> NewlinePolicy {
        self.newline_policy
    }

    pub fn len_bytes(&self) -> usize {
        self.buffer.len_bytes()
    }

    pub fn is_empty(&self) -> bool {
        self.buffer.len_bytes() == 0
    }

    pub fn line_count(&self) -> usize {
        self.line_index.line_count()
    }

    pub fn offset_to_position(&self, offset: TextOffset) -> Result<Position, DocumentError> {
        let snapshot = self.buffer.snapshot();
        self.line_index.offset_to_position(&snapshot, offset)
    }

    pub fn position_to_offset(&self, position: Position) -> Result<TextOffset, DocumentError> {
        let snapshot = self.buffer.snapshot();
        self.line_index.position_to_offset(&snapshot, position)
    }

    pub fn line_start_offset(&self, line: usize) -> Result<TextOffset, DocumentError> {
        self.line_index
            .line_start(line)
            .ok_or(DocumentError::PositionOutOfBounds { line, column: 0 })
    }

    pub fn line_end_offset(&self, line: usize) -> Result<TextOffset, DocumentError> {
        let start = self.line_start_offset(line)?;
        let next_start = self
            .line_index
            .line_start(line + 1)
            .unwrap_or_else(|| TextOffset::new(self.len_bytes()));
        let snapshot = self.buffer.snapshot();
        let line_text = snapshot.slice(TextRange::new(start, next_start)?)?;
        let line_end = line_text
            .strip_suffix('\n')
            .map(|trimmed| trimmed.len())
            .unwrap_or_else(|| line_text.len());

        Ok(TextOffset::new(start.value() + line_end))
    }

    pub fn begin_transaction(&mut self) {
        self.history.begin_transaction();
    }

    pub fn commit_transaction(&mut self) {
        self.history.commit_transaction();
    }

    pub fn move_selection(
        &self,
        selection: Selection,
        movement: CursorMove,
    ) -> Result<Selection, DocumentError> {
        CursorMoveRules::move_selection(self, selection, movement)
    }

    pub fn apply_edit(&mut self, edit: Edit) -> Result<ChangeSet, DocumentError> {
        self.apply_edit_internal(edit, true)
    }

    pub fn undo(&mut self) -> Result<Option<Vec<ChangeSet>>, DocumentError> {
        let Some(transaction) = self.history.pop_undo() else {
            return Ok(None);
        };

        let mut applied = Vec::with_capacity(transaction.changes().len());

        for change in transaction.changes().iter().rev() {
            applied.push(self.apply_edit_internal(change.inverse_edit().clone(), false)?);
        }

        self.history.push_redo(transaction);

        Ok(Some(applied))
    }

    pub fn redo(&mut self) -> Result<Option<Vec<ChangeSet>>, DocumentError> {
        let Some(transaction) = self.history.pop_redo() else {
            return Ok(None);
        };

        let mut applied = Vec::with_capacity(transaction.changes().len());

        for change in transaction.changes() {
            applied.push(self.apply_edit_internal(change.forward_edit(), false)?);
        }

        self.history.push_undo(transaction);

        Ok(Some(applied))
    }

    fn apply_edit_internal(
        &mut self,
        edit: Edit,
        record_history: bool,
    ) -> Result<ChangeSet, DocumentError> {
        match edit {
            Edit::Insert { offset, text } => self.insert(offset, text, record_history),
            Edit::Delete { range } => self.delete(range, record_history),
            Edit::Replace { range, text } => self.replace(range, text, record_history),
        }
    }

    fn insert(
        &mut self,
        offset: TextOffset,
        text: String,
        record_history: bool,
    ) -> Result<ChangeSet, DocumentError> {
        self.validate_offset(offset)?;

        let revision_before = self.revision;
        let range_before = TextRange::empty_at(offset);
        self.buffer.insert(offset, &text)?;
        self.line_index.apply_change(offset, 0, &text);
        self.revision = self.revision.next();

        let range_after = TextRange::new(offset, offset.checked_add(text.len()))?;
        let inverse_edit = Edit::Delete { range: range_after };

        let change_set = ChangeSet::new(
            revision_before,
            self.revision,
            range_before,
            range_after,
            text,
            String::new(),
            inverse_edit,
        );

        if record_history {
            self.history.record(change_set.clone());
        }

        Ok(change_set)
    }

    fn delete(&mut self, range: TextRange, record_history: bool) -> Result<ChangeSet, DocumentError> {
        self.validate_range(range)?;

        let revision_before = self.revision;
        let snapshot = self.buffer.snapshot();
        let removed_text = snapshot.slice(range)?.to_string();
        self.buffer.delete(range)?;
        self.line_index
            .apply_change(range.start(), range.len(), "");
        self.revision = self.revision.next();

        let range_after = TextRange::empty_at(range.start());
        let inverse_edit = Edit::Insert {
            offset: range.start(),
            text: removed_text.clone(),
        };

        let change_set = ChangeSet::new(
            revision_before,
            self.revision,
            range,
            range_after,
            String::new(),
            removed_text,
            inverse_edit,
        );

        if record_history {
            self.history.record(change_set.clone());
        }

        Ok(change_set)
    }

    fn replace(
        &mut self,
        range: TextRange,
        text: String,
        record_history: bool,
    ) -> Result<ChangeSet, DocumentError> {
        self.validate_range(range)?;

        let revision_before = self.revision;
        let snapshot = self.buffer.snapshot();
        let removed_text = snapshot.slice(range)?.to_string();
        self.buffer.replace(range, &text)?;
        self.line_index
            .apply_change(range.start(), range.len(), &text);
        self.revision = self.revision.next();

        let range_after = TextRange::new(range.start(), range.start().checked_add(text.len()))?;
        let inverse_edit = Edit::Replace {
            range: range_after,
            text: removed_text.clone(),
        };

        let change_set = ChangeSet::new(
            revision_before,
            self.revision,
            range,
            range_after,
            text,
            removed_text,
            inverse_edit,
        );

        if record_history {
            self.history.record(change_set.clone());
        }

        Ok(change_set)
    }

    fn validate_range(&self, range: TextRange) -> Result<(), DocumentError> {
        let len = self.buffer.len_bytes();

        if range.end().value() > len {
            return Err(DocumentError::RangeOutOfBounds {
                len,
                start: range.start(),
                end: range.end(),
            });
        }

        self.validate_offset(range.start())?;
        self.validate_offset(range.end())
    }

    fn validate_offset(&self, offset: TextOffset) -> Result<(), DocumentError> {
        if offset.value() > self.buffer.len_bytes() {
            return Err(DocumentError::RangeOutOfBounds {
                len: self.buffer.len_bytes(),
                start: offset,
                end: offset,
            });
        }

        if !self.buffer.is_char_boundary(offset) {
            return Err(DocumentError::InvalidUtf8Boundary { offset });
        }

        Ok(())
    }
}
