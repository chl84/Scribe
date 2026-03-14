use super::{Document, DocumentError, Position, Selection, TextOffset};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CursorMove {
    Left,
    Right,
    Up,
    Down,
    LineStart,
    LineEnd,
}

pub struct CursorMoveRules;

impl CursorMoveRules {
    pub fn move_selection(
        document: &Document,
        selection: Selection,
        movement: CursorMove,
    ) -> Result<Selection, DocumentError> {
        match movement {
            CursorMove::Left => Self::move_left(document, selection),
            CursorMove::Right => Self::move_right(document, selection),
            CursorMove::Up => Self::move_vertical(document, selection, -1),
            CursorMove::Down => Self::move_vertical(document, selection, 1),
            CursorMove::LineStart => Self::move_to_line_start(document, selection),
            CursorMove::LineEnd => Self::move_to_line_end(document, selection),
        }
    }

    fn move_left(document: &Document, selection: Selection) -> Result<Selection, DocumentError> {
        if !selection.is_caret() {
            return selection.collapse_to_start();
        }

        let current = selection.active();
        if current.value() == 0 {
            return Ok(selection);
        }

        let snapshot = document.snapshot();
        let previous = snapshot
            .as_str()[..current.value()]
            .char_indices()
            .last()
            .map(|(offset, _)| TextOffset::new(offset))
            .unwrap_or(TextOffset::new(0));

        Ok(Selection::caret(previous))
    }

    fn move_right(document: &Document, selection: Selection) -> Result<Selection, DocumentError> {
        if !selection.is_caret() {
            return selection.collapse_to_end();
        }

        let current = selection.active();
        if current.value() == document.len_bytes() {
            return Ok(selection);
        }

        let snapshot = document.snapshot();
        let next_char = snapshot.as_str()[current.value()..]
            .chars()
            .next()
            .ok_or(DocumentError::PositionOutOfBounds { line: 0, column: 0 })?;

        Ok(Selection::caret(TextOffset::new(
            current.value() + next_char.len_utf8(),
        )))
    }

    fn move_vertical(
        document: &Document,
        selection: Selection,
        line_delta: isize,
    ) -> Result<Selection, DocumentError> {
        let position = document.offset_to_position(selection.active())?;
        let target_line = position.line() as isize + line_delta;

        if target_line < 0 {
            return Ok(Selection::caret(TextOffset::new(0)));
        }

        let target_position = Position::new(target_line as usize, position.column());
        let offset = match document.position_to_offset(target_position) {
            Ok(offset) => offset,
            Err(DocumentError::PositionOutOfBounds { .. }) => {
                let fallback = if line_delta < 0 {
                    document.line_start_offset(target_line as usize)?
                } else {
                    document.line_end_offset(target_line as usize)?
                };
                fallback
            }
            Err(error) => return Err(error),
        };

        Ok(Selection::caret(offset))
    }

    fn move_to_line_start(
        document: &Document,
        selection: Selection,
    ) -> Result<Selection, DocumentError> {
        let position = document.offset_to_position(selection.active())?;
        Ok(Selection::caret(document.line_start_offset(position.line())?))
    }

    fn move_to_line_end(
        document: &Document,
        selection: Selection,
    ) -> Result<Selection, DocumentError> {
        let position = document.offset_to_position(selection.active())?;
        Ok(Selection::caret(document.line_end_offset(position.line())?))
    }
}
