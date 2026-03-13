use super::{
    ChangeSet, DocumentError, DocumentId, Edit, NewlinePolicy, RevisionId, TextOffset, TextRange,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    id: DocumentId,
    revision: RevisionId,
    text: String,
    newline_policy: NewlinePolicy,
}

impl Document {
    pub fn open(id: DocumentId, text: impl Into<String>) -> Self {
        let text = text.into();
        let newline_policy = NewlinePolicy::detect(&text);

        Self {
            id,
            revision: RevisionId::initial(),
            text,
            newline_policy,
        }
    }

    pub const fn id(&self) -> DocumentId {
        self.id
    }

    pub const fn revision(&self) -> RevisionId {
        self.revision
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub const fn newline_policy(&self) -> NewlinePolicy {
        self.newline_policy
    }

    pub fn len_bytes(&self) -> usize {
        self.text.len()
    }

    pub fn is_empty(&self) -> bool {
        self.text.is_empty()
    }

    pub fn apply_edit(&mut self, edit: Edit) -> Result<ChangeSet, DocumentError> {
        match edit {
            Edit::Insert { offset, text } => self.insert(offset, text),
            Edit::Delete { range } => self.delete(range),
            Edit::Replace { range, text } => self.replace(range, text),
        }
    }

    fn insert(
        &mut self,
        offset: TextOffset,
        text: String,
    ) -> Result<ChangeSet, DocumentError> {
        self.validate_offset(offset)?;

        let revision_before = self.revision;
        let range_before = TextRange::empty_at(offset);
        self.text.insert_str(offset.value(), &text);
        self.revision = self.revision.next();

        let range_after = TextRange::new(offset, offset.checked_add(text.len()))?;
        let inverse_edit = Edit::Delete { range: range_after };

        Ok(ChangeSet::new(
            revision_before,
            self.revision,
            range_before,
            range_after,
            text,
            String::new(),
            inverse_edit,
        ))
    }

    fn delete(&mut self, range: TextRange) -> Result<ChangeSet, DocumentError> {
        self.validate_range(range)?;

        let revision_before = self.revision;
        let removed_text = self.text[range.start().value()..range.end().value()].to_string();
        self.text.replace_range(range.start().value()..range.end().value(), "");
        self.revision = self.revision.next();

        let range_after = TextRange::empty_at(range.start());
        let inverse_edit = Edit::Insert {
            offset: range.start(),
            text: removed_text.clone(),
        };

        Ok(ChangeSet::new(
            revision_before,
            self.revision,
            range,
            range_after,
            String::new(),
            removed_text,
            inverse_edit,
        ))
    }

    fn replace(&mut self, range: TextRange, text: String) -> Result<ChangeSet, DocumentError> {
        self.validate_range(range)?;

        let revision_before = self.revision;
        let removed_text = self.text[range.start().value()..range.end().value()].to_string();
        self.text
            .replace_range(range.start().value()..range.end().value(), &text);
        self.revision = self.revision.next();

        let range_after = TextRange::new(range.start(), range.start().checked_add(text.len()))?;
        let inverse_edit = Edit::Replace {
            range: range_after,
            text: removed_text.clone(),
        };

        Ok(ChangeSet::new(
            revision_before,
            self.revision,
            range,
            range_after,
            text,
            removed_text,
            inverse_edit,
        ))
    }

    fn validate_range(&self, range: TextRange) -> Result<(), DocumentError> {
        let len = self.text.len();

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
        if offset.value() > self.text.len() {
            return Err(DocumentError::RangeOutOfBounds {
                len: self.text.len(),
                start: offset,
                end: offset,
            });
        }

        if !self.text.is_char_boundary(offset.value()) {
            return Err(DocumentError::InvalidUtf8Boundary { offset });
        }

        Ok(())
    }
}
