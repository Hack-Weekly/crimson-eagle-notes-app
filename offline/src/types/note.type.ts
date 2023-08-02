export type NoteType = {
    id: number,
    title: string,
    excerpt: string,
    content: string,
    color: string,
    starred: boolean,
    created_on: Date,
    updated_on: Date,
}
export type PaginatedNotes = {
    records: NoteType[],
    total: number,
    current_page: number,
    per_page: 12,
}

export interface User {
    id: number;
    name: string;
    email: string;
    password: string;
}
export interface Database {
    users: User[];
    notes: NoteType[];
    paginatednotes:  PaginatedNotes[];
  }