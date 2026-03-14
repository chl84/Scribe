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
    newline_count: usize,
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Node {
    piece: Piece,
    priority: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
    subtree_bytes: usize,
    subtree_newlines: usize,
}

impl Node {
    fn new(piece: Piece, priority: u64) -> Self {
        Self {
            piece,
            priority,
            left: None,
            right: None,
            subtree_bytes: 0,
            subtree_newlines: 0,
        }
    }

    fn subtree_bytes(node: &Option<Box<Node>>) -> usize {
        node.as_ref().map(|node| node.subtree_bytes).unwrap_or(0)
    }

    fn subtree_newlines(node: &Option<Box<Node>>) -> usize {
        node.as_ref().map(|node| node.subtree_newlines).unwrap_or(0)
    }

    fn recalc(&mut self) {
        self.subtree_bytes =
            Self::subtree_bytes(&self.left) + self.piece.len + Self::subtree_bytes(&self.right);
        self.subtree_newlines = Self::subtree_newlines(&self.left)
            + self.piece.newline_count
            + Self::subtree_newlines(&self.right);
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PieceTree {
    original: String,
    add: String,
    root: Option<Box<Node>>,
    priority_seed: u64,
}

impl PieceTree {
    pub fn new(text: impl Into<String>) -> Self {
        let original = text.into();
        let mut tree = Self {
            original,
            add: String::new(),
            root: None,
            priority_seed: 0x9e37_79b9_7f4a_7c15,
        };

        if !tree.original.is_empty() {
            let piece = tree.make_piece(BufferSource::Original, 0, tree.original.len());
            tree.root = Some(tree.new_node(piece));
        }

        tree
    }

    fn next_priority(&mut self) -> u64 {
        self.priority_seed = self.priority_seed.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut value = self.priority_seed;
        value = (value ^ (value >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        value = (value ^ (value >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        value ^ (value >> 31)
    }

    fn new_node(&mut self, piece: Piece) -> Box<Node> {
        let mut node = Box::new(Node::new(piece, self.next_priority()));
        node.recalc();
        node
    }

    fn make_piece(&self, source: BufferSource, start: usize, len: usize) -> Piece {
        let newline_count = self.buffer_for(source)[start..start + len]
            .bytes()
            .filter(|byte| *byte == b'\n')
            .count();

        Piece {
            source,
            start,
            len,
            newline_count,
        }
    }

    fn append_to_add(&mut self, text: &str) -> Piece {
        let start = self.add.len();
        self.add.push_str(text);
        self.make_piece(BufferSource::Add, start, text.len())
    }

    fn buffer_for(&self, source: BufferSource) -> &str {
        match source {
            BufferSource::Original => &self.original,
            BufferSource::Add => &self.add,
        }
    }

    fn merge(&mut self, left: Option<Box<Node>>, right: Option<Box<Node>>) -> Option<Box<Node>> {
        match (left, right) {
            (None, right) => right,
            (left, None) => left,
            (Some(mut left), Some(mut right)) => {
                if left.priority > right.priority {
                    left.right = self.merge(left.right.take(), Some(right));
                    left.recalc();
                    Some(left)
                } else {
                    right.left = self.merge(Some(left), right.left.take());
                    right.recalc();
                    Some(right)
                }
            }
        }
    }

    fn split(
        &mut self,
        node: Option<Box<Node>>,
        offset: usize,
    ) -> Result<(Option<Box<Node>>, Option<Box<Node>>), DocumentError> {
        let Some(mut node) = node else {
            return Ok((None, None));
        };

        let left_bytes = Node::subtree_bytes(&node.left);

        if offset < left_bytes {
            let (left, right) = self.split(node.left.take(), offset)?;
            node.left = right;
            node.recalc();
            return Ok((left, Some(node)));
        }

        let piece_start = left_bytes;
        let piece_end = piece_start + node.piece.len;

        if offset > piece_end {
            let (left, right) = self.split(node.right.take(), offset - piece_end)?;
            node.right = left;
            node.recalc();
            return Ok((Some(node), right));
        }

        let within_piece = offset - left_bytes;

        if within_piece == 0 {
            let left = node.left.take();
            node.recalc();
            return Ok((left, Some(node)));
        }

        if within_piece == node.piece.len {
            let right = node.right.take();
            node.recalc();
            return Ok((Some(node), right));
        }

        let split_point = node.piece.start + within_piece;
        let buffer = self.buffer_for(node.piece.source);

        if !buffer.is_char_boundary(split_point) {
            return Err(DocumentError::InvalidUtf8Boundary {
                offset: TextOffset::new(offset),
            });
        }

        let left_piece = self.make_piece(node.piece.source, node.piece.start, within_piece);
        let right_piece = self.make_piece(
            node.piece.source,
            split_point,
            node.piece.len - within_piece,
        );

        let mut left_node = self.new_node(left_piece);
        left_node.left = node.left.take();
        left_node.recalc();

        let mut right_node = self.new_node(right_piece);
        right_node.right = node.right.take();
        right_node.recalc();

        Ok((Some(left_node), Some(right_node)))
    }

    fn locate_piece(&self, offset: usize) -> Option<(Piece, usize)> {
        let mut current = self.root.as_ref();
        let mut remaining = offset;

        while let Some(node) = current {
            let left_bytes = Node::subtree_bytes(&node.left);

            if remaining < left_bytes {
                current = node.left.as_ref();
                continue;
            }

            if remaining > left_bytes + node.piece.len {
                remaining -= left_bytes + node.piece.len;
                current = node.right.as_ref();
                continue;
            }

            return Some((node.piece, remaining - left_bytes));
        }

        None
    }

    fn push_snapshot_text(&self, node: &Option<Box<Node>>, target: &mut String) {
        let Some(node) = node else {
            return;
        };

        self.push_snapshot_text(&node.left, target);
        let buffer = self.buffer_for(node.piece.source);
        target.push_str(&buffer[node.piece.start..node.piece.start + node.piece.len]);
        self.push_snapshot_text(&node.right, target);
    }

    fn push_range_text(
        &self,
        node: &Option<Box<Node>>,
        base_offset: usize,
        range: TextRange,
        target: &mut String,
    ) {
        let Some(node) = node else {
            return;
        };

        let left_bytes = Node::subtree_bytes(&node.left);
        let piece_offset = base_offset + left_bytes;
        let piece_end = piece_offset + node.piece.len;

        if range.start().value() < piece_offset {
            self.push_range_text(&node.left, base_offset, range, target);
        }

        if range.start().value() < piece_end && range.end().value() > piece_offset {
            let take_start = range.start().value().saturating_sub(piece_offset);
            let take_end = (range.end().value().min(piece_end)) - piece_offset;
            let buffer = self.buffer_for(node.piece.source);
            target.push_str(&buffer[node.piece.start + take_start..node.piece.start + take_end]);
        }

        if range.end().value() > piece_end {
            self.push_range_text(&node.right, piece_offset + node.piece.len, range, target);
        }
    }

    #[cfg(test)]
    pub fn subtree_newlines(&self) -> usize {
        Node::subtree_newlines(&self.root)
    }
}

impl TextBuffer for PieceTree {
    fn len_bytes(&self) -> usize {
        Node::subtree_bytes(&self.root)
    }

    fn snapshot(&self) -> TextSnapshot {
        let mut text = String::with_capacity(self.len_bytes());
        self.push_snapshot_text(&self.root, &mut text);
        TextSnapshot::new(text)
    }

    fn slice_string(&self, range: TextRange) -> Result<String, DocumentError> {
        if range.end().value() > self.len_bytes() {
            return Err(DocumentError::RangeOutOfBounds {
                len: self.len_bytes(),
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

        let mut text = String::with_capacity(range.len());
        self.push_range_text(&self.root, 0, range, &mut text);
        Ok(text)
    }

    fn is_char_boundary(&self, offset: TextOffset) -> bool {
        if offset.value() > self.len_bytes() {
            return false;
        }

        if offset.value() == self.len_bytes() {
            return true;
        }

        let Some((piece, inner_offset)) = self.locate_piece(offset.value()) else {
            return false;
        };

        if inner_offset == 0 || inner_offset == piece.len {
            return true;
        }

        self.buffer_for(piece.source)
            .is_char_boundary(piece.start + inner_offset)
    }

    fn insert(&mut self, offset: TextOffset, text: &str) -> Result<(), DocumentError> {
        if !self.is_char_boundary(offset) {
            return Err(DocumentError::InvalidUtf8Boundary { offset });
        }

        let piece = self.append_to_add(text);
        let root = self.root.take();
        let (left, right) = self.split(root, offset.value())?;
        let middle = if piece.len == 0 {
            None
        } else {
            Some(self.new_node(piece))
        };
        let merged = self.merge(left, middle);
        self.root = self.merge(merged, right);
        Ok(())
    }

    fn delete(&mut self, range: TextRange) -> Result<(), DocumentError> {
        if range.end().value() > self.len_bytes() {
            return Err(DocumentError::RangeOutOfBounds {
                len: self.len_bytes(),
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

        let root = self.root.take();
        let (left, tail) = self.split(root, range.start().value())?;
        let (_, right) = self.split(tail, range.len())?;
        self.root = self.merge(left, right);
        Ok(())
    }

    fn replace(&mut self, range: TextRange, text: &str) -> Result<(), DocumentError> {
        self.delete(range)?;
        self.insert(range.start(), text)
    }
}
