use super::{Edit, RevisionId, TextRange};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChangeSet {
    revision_before: RevisionId,
    revision_after: RevisionId,
    range_before: TextRange,
    range_after: TextRange,
    inserted_text: String,
    removed_text: String,
    inverse_edit: Edit,
}

impl ChangeSet {
    pub fn new(
        revision_before: RevisionId,
        revision_after: RevisionId,
        range_before: TextRange,
        range_after: TextRange,
        inserted_text: String,
        removed_text: String,
        inverse_edit: Edit,
    ) -> Self {
        Self {
            revision_before,
            revision_after,
            range_before,
            range_after,
            inserted_text,
            removed_text,
            inverse_edit,
        }
    }

    pub const fn revision_before(&self) -> RevisionId {
        self.revision_before
    }

    pub const fn revision_after(&self) -> RevisionId {
        self.revision_after
    }

    pub const fn range_before(&self) -> TextRange {
        self.range_before
    }

    pub const fn range_after(&self) -> TextRange {
        self.range_after
    }

    pub fn inserted_text(&self) -> &str {
        &self.inserted_text
    }

    pub fn removed_text(&self) -> &str {
        &self.removed_text
    }

    pub const fn inverse_edit(&self) -> &Edit {
        &self.inverse_edit
    }

    pub fn forward_edit(&self) -> Edit {
        if self.removed_text.is_empty() {
            return Edit::Insert {
                offset: self.range_before.start(),
                text: self.inserted_text.clone(),
            };
        }

        if self.inserted_text.is_empty() {
            return Edit::Delete {
                range: self.range_before,
            };
        }

        Edit::Replace {
            range: self.range_before,
            text: self.inserted_text.clone(),
        }
    }
}
