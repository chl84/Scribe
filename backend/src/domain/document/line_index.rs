use super::{DocumentError, Position, TextOffset, TextRange, TextSnapshot};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LineIndex {
    line_starts: Vec<TextOffset>,
}

impl LineIndex {
    pub fn from_snapshot(snapshot: &TextSnapshot) -> Self {
        let mut line_starts = vec![TextOffset::new(0)];

        for (offset, ch) in snapshot.as_str().char_indices() {
            if ch == '\n' {
                line_starts.push(TextOffset::new(offset + ch.len_utf8()));
            }
        }

        Self { line_starts }
    }

    pub fn line_count(&self) -> usize {
        self.line_starts.len()
    }

    pub fn line_start(&self, line: usize) -> Option<TextOffset> {
        self.line_starts.get(line).copied()
    }

    pub fn offset_to_position(
        &self,
        snapshot: &TextSnapshot,
        offset: TextOffset,
    ) -> Result<Position, DocumentError> {
        if offset.value() > snapshot.len_bytes() {
            return Err(DocumentError::RangeOutOfBounds {
                len: snapshot.len_bytes(),
                start: offset,
                end: offset,
            });
        }

        if !snapshot.as_str().is_char_boundary(offset.value()) {
            return Err(DocumentError::InvalidUtf8Boundary { offset });
        }

        let line_index = match self.line_starts.binary_search(&offset) {
            Ok(index) => index,
            Err(next_index) => next_index.saturating_sub(1),
        };
        let line_start = self.line_starts[line_index];
        let column = snapshot.char_column(line_start, offset)?;

        Ok(Position::new(line_index, column))
    }

    pub fn position_to_offset(
        &self,
        snapshot: &TextSnapshot,
        position: Position,
    ) -> Result<TextOffset, DocumentError> {
        let line_start =
            self.line_start(position.line())
                .ok_or(DocumentError::PositionOutOfBounds {
                    line: position.line(),
                    column: position.column(),
                })?;
        let line_end = self
            .line_start(position.line() + 1)
            .unwrap_or_else(|| TextOffset::new(snapshot.len_bytes()));
        let range = TextRange::new(line_start, line_end)?;
        let line_text = snapshot.slice(range)?;

        let mut byte_offset = 0usize;
        let mut columns = 0usize;

        while columns < position.column() {
            let Some(character) = line_text[byte_offset..].chars().next() else {
                return Err(DocumentError::PositionOutOfBounds {
                    line: position.line(),
                    column: position.column(),
                });
            };

            if character == '\n' {
                return Err(DocumentError::PositionOutOfBounds {
                    line: position.line(),
                    column: position.column(),
                });
            }

            byte_offset += character.len_utf8();
            columns += 1;
        }

        Ok(TextOffset::new(line_start.value() + byte_offset))
    }

    pub fn apply_change(
        &mut self,
        change_start: TextOffset,
        removed_len: usize,
        inserted_text: &str,
    ) {
        let change_end = change_start.checked_add(removed_len);
        let start_index = self
            .line_starts
            .partition_point(|offset| *offset <= change_start);
        let end_index = self
            .line_starts
            .partition_point(|offset| *offset <= change_end);

        let delta = inserted_text.len() as isize - removed_len as isize;
        let mut inserted_starts = Vec::new();

        for (offset, ch) in inserted_text.char_indices() {
            if ch == '\n' {
                inserted_starts.push(TextOffset::new(
                    change_start.value() + offset + ch.len_utf8(),
                ));
            }
        }

        let inserted_count = inserted_starts.len();
        self.line_starts
            .splice(start_index..end_index, inserted_starts);

        for line_start in &mut self.line_starts[start_index + inserted_count..] {
            if line_start.value() > change_end.value() {
                *line_start = TextOffset::new(((line_start.value() as isize) + delta) as usize);
            }
        }
    }
}
