import type { EditCommandDto } from "../../../shared/types/editor";
import { utf8ByteLength } from "../../../shared/utils/utf8";

export function deriveEditCommand(
  previousText: string,
  nextText: string,
): EditCommandDto | null {
  if (previousText === nextText) {
    return null;
  }

  const previousChars = Array.from(previousText);
  const nextChars = Array.from(nextText);
  const previousByteLengths = previousChars.map(utf8ByteLength);
  const totalPreviousBytes = previousByteLengths.reduce(
    (sum, length) => sum + length,
    0,
  );

  let prefixLength = 0;
  let startOffset = 0;

  while (
    prefixLength < previousChars.length &&
    prefixLength < nextChars.length &&
    previousChars[prefixLength] === nextChars[prefixLength]
  ) {
    startOffset += previousByteLengths[prefixLength];
    prefixLength += 1;
  }

  let previousEnd = previousChars.length;
  let nextEnd = nextChars.length;
  let trailingBytes = 0;

  while (
    previousEnd > prefixLength &&
    nextEnd > prefixLength &&
    previousChars[previousEnd - 1] === nextChars[nextEnd - 1]
  ) {
    previousEnd -= 1;
    nextEnd -= 1;
    trailingBytes += previousByteLengths[previousEnd];
  }

  const endOffset = totalPreviousBytes - trailingBytes;
  const insertedText = nextChars.slice(prefixLength, nextEnd).join("");

  if (startOffset === endOffset) {
    return {
      kind: "insert",
      offset: startOffset,
      text: insertedText,
    };
  }

  if (insertedText.length === 0) {
    return {
      kind: "delete",
      start: startOffset,
      end: endOffset,
    };
  }

  return {
    kind: "replace",
    start: startOffset,
    end: endOffset,
    text: insertedText,
  };
}
