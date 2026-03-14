use super::ChangeSet;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EditTransaction {
    changes: Vec<ChangeSet>,
}

impl EditTransaction {
    pub fn new(changes: Vec<ChangeSet>) -> Self {
        Self { changes }
    }

    pub fn changes(&self) -> &[ChangeSet] {
        &self.changes
    }

    pub fn is_empty(&self) -> bool {
        self.changes.is_empty()
    }

    pub fn push(&mut self, change: ChangeSet) {
        self.changes.push(change);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct UndoManager {
    undo_stack: Vec<EditTransaction>,
    redo_stack: Vec<EditTransaction>,
    in_progress: Option<EditTransaction>,
}

impl UndoManager {
    pub fn begin_transaction(&mut self) {
        if self.in_progress.is_none() {
            self.in_progress = Some(EditTransaction::new(Vec::new()));
        }
    }

    pub fn commit_transaction(&mut self) {
        if let Some(transaction) = self.in_progress.take() {
            if !transaction.is_empty() {
                self.undo_stack.push(transaction);
                self.redo_stack.clear();
            }
        }
    }

    pub fn record(&mut self, change: ChangeSet) {
        if let Some(transaction) = self.in_progress.as_mut() {
            transaction.push(change);
            self.redo_stack.clear();
            return;
        }

        self.undo_stack.push(EditTransaction::new(vec![change]));
        self.redo_stack.clear();
    }

    pub fn pop_undo(&mut self) -> Option<EditTransaction> {
        self.undo_stack.pop()
    }

    pub fn pop_redo(&mut self) -> Option<EditTransaction> {
        self.redo_stack.pop()
    }

    pub fn push_undo(&mut self, transaction: EditTransaction) {
        self.undo_stack.push(transaction);
    }

    pub fn push_redo(&mut self, transaction: EditTransaction) {
        self.redo_stack.push(transaction);
    }
}
