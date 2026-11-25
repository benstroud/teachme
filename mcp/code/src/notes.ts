// A tiny in-memory note store. The lessons use it as the "backend" a server wraps.
export interface Note {
  id: string;
  title: string;
  body: string;
}

export class NoteStore {
  private notes = new Map<string, Note>();

  constructor(seed: Note[] = []) {
    for (const n of seed) this.notes.set(n.id, n);
  }

  get(id: string): Note | undefined {
    return this.notes.get(id);
  }

  search(query: string): Note[] {
    const q = query.toLowerCase();
    return [...this.notes.values()].filter(
      (n) => n.title.toLowerCase().includes(q) || n.body.toLowerCase().includes(q),
    );
  }

  delete(id: string): boolean {
    return this.notes.delete(id);
  }
}

export const seedNotes: Note[] = [
  { id: "n1", title: "Shopping", body: "milk, eggs, bread" },
  { id: "n2", title: "Ideas", body: "write about MCP servers" },
];
