import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri';
import type { PaginatedNotes } from "../types/note.type";

export interface Note {
    id: number;
    color: string;
    title: string;
    excerpt: string;
    starred: boolean;
}

const defaultNotes: Note[] = [
    {
        id: 1,
        color: "orange",
        title: "First note's title",
        excerpt: "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed dignissim consequat elementum.",
        starred: false,
    },
    {
        id: 2,
        color:"green",
        title:"Look at this!",
        excerpt:"Nam id diam eu arcu fringilla hendrerit at id lectus. Nullam vel posuere eros, at aliquet elit.",
        starred: true,
    },
    {
        id: 3,
        color: "blue",
        title: "This is very interesting",
        excerpt: "Praesent ante lectus, congue a ex vel, auctor condimentum tellus.",
        starred: false,
    },
    {
        id: 4,
        color: "pink",
        title: "OMG No WAY",
        excerpt: "Fusce erat lorem, molestie sed suscipit id, rhoncus vel arcu. Praesent nec neque turpis.",
        starred: false,
    }
];

export const isLoading = writable(false);
export const fetchError = writable<string | null>(null);

const loadNotes = async (): Promise<Note[]> => {
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
        return fetchNotes ? fetchNotes : defaultNotes;
    }
    return JSON.parse(storedNotes);
};

const saveNotes = (notes: Note[]) => {
    localStorage.setItem('notes', JSON.stringify(notes));
};

const noteStore = writable<Note[]>(await loadNotes());


const createNotesStore = () => {
    const { subscribe, update, set } = noteStore;
  
    return {
      subscribe,
      setNotes: (notes: Note[]) => set(notes),
      addNote: (note: Note) => update(notes => {
        const newNotes = [...notes, note];
        saveNotes(newNotes);
        return newNotes;
      }),
      deleteNote: (id: number) => update(notes => {
        const newNotes = notes.filter(note => note.id !== id);
        saveNotes(newNotes);
        return newNotes;
      }),
      updateNote: (id: number, updatedFields: Partial<Note>) => update(notes => {
        const newNotes = notes.map(note => note.id === id ? { ...note, ...updatedFields } : note);
        saveNotes(newNotes);
        return newNotes;
      }),
    };
};
  
  
export const notes = createNotesStore();