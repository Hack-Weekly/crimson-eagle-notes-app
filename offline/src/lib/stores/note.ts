import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { NoteType, PaginatedNotes } from "../../types/note.type";

export const isLoading = writable(false);
export const fetchError = writable<string | null>(null);

const loadNotes = async (): Promise<NoteType[]> => {
    fetchError.set(null);
    const storedNotes = localStorage.getItem('notes');

    if (!storedNotes) {
        isLoading.set(true);
        const fetchNotes = await invoke<PaginatedNotes>('fetch_notes')
            .then(res => res.records)
            .catch(err => {
                console.log(err)
                fetchError.set(err)
            })
            .finally(() => isLoading.set(false));
        return fetchNotes ? fetchNotes : [];
    }
    return JSON.parse(storedNotes);
};

const saveNotes = (notes: NoteType[]) => {
    localStorage.setItem('notes', JSON.stringify(notes));
};

const noteStore = writable<NoteType[]>([]);

(async () => {
    const loadedNotes = await loadNotes();
    noteStore.set(loadedNotes);
})();

const createNotesStore = () => {
    const { subscribe, update, set } = noteStore;
  
    return {
      subscribe,
      setNotes: (notes: NoteType[]) => set(notes),
      addNote: (note: NoteType) => update(notes => {
        const newNotes = [...notes, note];
        saveNotes(newNotes);
        return newNotes;
      }),
      deleteNote: (id: number) => update(notes => {
        const newNotes = notes.filter(note => note.id !== id);
        saveNotes(newNotes);
        return newNotes;
      }),
      updateNote: (id: number, updatedFields: Partial<NoteType>) => update(notes => {
        const newNotes = notes.map(note => note.id === id ? { ...note, ...updatedFields } : note);
        saveNotes(newNotes);
        return newNotes;
      }),
    };
};
  
  
export const notes = createNotesStore();