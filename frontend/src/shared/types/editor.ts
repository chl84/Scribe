export type NewlineMode = "lf" | "crlf";

export interface RangeDto {
  start: number;
  end: number;
}

export interface ChangeSetDto {
  revision_before: number;
  revision_after: number;
  range_before: RangeDto;
  range_after: RangeDto;
  inserted_text: string;
  removed_text: string;
}

export interface DocumentSnapshotDto {
  document_id: number;
  revision: number;
  text: string;
  line_count: number;
  newline_mode: NewlineMode;
  path: string | null;
}

export interface EditResultDto {
  document_id: number;
  changes: ChangeSetDto[];
}

export type EditCommandDto =
  | {
      kind: "insert";
      offset: number;
      text: string;
    }
  | {
      kind: "delete";
      start: number;
      end: number;
    }
  | {
      kind: "replace";
      start: number;
      end: number;
      text: string;
    };
