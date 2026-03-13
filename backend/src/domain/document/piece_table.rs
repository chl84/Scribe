use super::{DocumentError, TextBuffer, TextOffset, TextRange, TextSnapshot};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BufferSource {
    Original,
    Add,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Piece {
    source: BufferSource,
    start: usize,
    len: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceTable {
    original: String,
    add: String,
    pieces: Vec<Piece>,
    len_bytes: usize,
}

impl PieceTable {
    pub fn new(text: impl Into<String>) -> Self {
        let original = text.into();
        let len_bytes = original.len();
        let pieces = if len_bytes == 0 {
            Vec::new()
        } else {
            vec![Piece {
                source: BufferSource::Original,
                start: 0,
                len: len_bytes,
            }]
        };

        Self {
            original,
            add: String::new(),
            pieces,
            len_bytes,
        }
    }

    fn append_to_add(&mut self, text: &str) -> Piece {
        let start = self.add.len();
        self.add.push_str(text);

        Piece {
            source: BufferSource::Add,
            start,
            len: text.len(),
        }
    }

    fn buffer_for(&self, source: BufferSource) -> &str {
        match source {
            BufferSource::Original => &self.original,
            BufferSource::Add => &self.add,
        }
    }

    fn split_at_offset(&mut self, offset: usize) -> Result<usize, DocumentError> {
        if offset > self.len_bytes {
            return Err(DocumentError::RangeOutOfBounds {
                len: self.len_bytes,
                start: TextOffset::new(offset),
                end: TextOffset::new(offset),
            });
        }

        if offset == 0 {
            return Ok(0);
        }

        if offset == self.len_bytes {
            return Ok(self.pieces.len());
        }

        let mut consumed = 0usize;

        for index in 0..self.pieces.len() {
            let piece = self.pieces[index];
            let next_consumed = consumed + piece.len;

            if offset == consumed {
                return Ok(index);
            }

            if offset == next_consumed {
                return Ok(index + 1);
            }

            if offset < next_consumed {
                let inner_offset = offset - consumed;
                let buffer = self.buffer_for(piece.source);
                let split_point = piece.start + inner_offset;

                if !buffer.is_char_boundary(split_point) {
                    return Err(DocumentError::InvalidUtf8Boundary {
                        offset: TextOffset::new(offset),
                    });
                }

                let left = Piece {
                    source: piece.source,
                    start: piece.start,
                    len: inner_offset,
                };
                let right = Piece {
                    source: piece.source,
                    start: split_point,
                    len: piece.len - inner_offset,
                };

                self.pieces[index] = left;
                self.pieces.insert(index + 1, right);

                return Ok(index + 1);
            }

            consumed = next_consumed;
        }

        Ok(self.pieces.len())
    }

    fn merge_adjacent_pieces(&mut self) {
        if self.pieces.is_empty() {
            return;
        }

        let mut merged: Vec<Piece> = Vec::with_capacity(self.pieces.len());

        for piece in self.pieces.drain(..) {
            if piece.len == 0 {
                continue;
            }

            if let Some(last) = merged.last_mut() {
                if last.source == piece.source && last.start + last.len == piece.start {
                    last.len += piece.len;
                    continue;
                }
            }

            merged.push(piece);
        }

        self.pieces = merged;
    }

    fn remove_range(&mut self, range: TextRange) -> Result<(), DocumentError> {
        let start_index = self.split_at_offset(range.start().value())?;
        let end_index = self.split_at_offset(range.end().value())?;
        self.pieces.drain(start_index..end_index);
        self.len_bytes -= range.len();
        self.merge_adjacent_pieces();
        Ok(())
    }

    fn insert_piece(&mut self, offset: TextOffset, piece: Piece) -> Result<(), DocumentError> {
        let index = self.split_at_offset(offset.value())?;
        if piece.len > 0 {
            self.pieces.insert(index, piece);
            self.len_bytes += piece.len;
            self.merge_adjacent_pieces();
        }
        Ok(())
    }
}

impl TextBuffer for PieceTable {
    fn len_bytes(&self) -> usize {
        self.len_bytes
    }

    fn snapshot(&self) -> TextSnapshot {
        let mut text = String::with_capacity(self.len_bytes);

        for piece in &self.pieces {
            let buffer = self.buffer_for(piece.source);
            text.push_str(&buffer[piece.start..piece.start + piece.len]);
        }

        TextSnapshot::new(text)
    }

    fn is_char_boundary(&self, offset: TextOffset) -> bool {
        if offset.value() > self.len_bytes {
            return false;
        }

        if offset.value() == self.len_bytes {
            return true;
        }

        let mut consumed = 0usize;

        for piece in &self.pieces {
            let next_consumed = consumed + piece.len;

            if offset.value() == consumed || offset.value() == next_consumed {
                return true;
            }

            if offset.value() < next_consumed {
                let inner_offset = offset.value() - consumed;
                let buffer = self.buffer_for(piece.source);
                return buffer.is_char_boundary(piece.start + inner_offset);
            }

            consumed = next_consumed;
        }

        false
    }

    fn insert(&mut self, offset: TextOffset, text: &str) -> Result<(), DocumentError> {
        if !self.is_char_boundary(offset) {
            return Err(DocumentError::InvalidUtf8Boundary { offset });
        }

        let piece = self.append_to_add(text);
        self.insert_piece(offset, piece)
    }

    fn delete(&mut self, range: TextRange) -> Result<(), DocumentError> {
        if range.end().value() > self.len_bytes {
            return Err(DocumentError::RangeOutOfBounds {
                len: self.len_bytes,
                start: range.start(),
                end: range.end(),
            });
        }

        if !self.is_char_boundary(range.start()) {
            return Err(DocumentError::InvalidUtf8Boundary {
                offset: range.start(),
            });
        }

        if !self.is_char_boundary(range.end()) {
            return Err(DocumentError::InvalidUtf8Boundary {
                offset: range.end(),
            });
        }

        self.remove_range(range)
    }

    fn replace(&mut self, range: TextRange, text: &str) -> Result<(), DocumentError> {
        self.delete(range)?;
        self.insert(range.start(), text)
    }
}
