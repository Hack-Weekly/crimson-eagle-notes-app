<script lang='ts'>
    import { Label } from 'flowbite-svelte';
    import { Button } from 'flowbite-svelte';
    import { notes } from '$lib/stores/note';
	import type { NoteType } from '../../types/note.type';

    let newNoteTitle = '';
    let newNoteContent = '';

    const addNewNote = () => {
        console.log("Adding new note");

        let currentNotes = JSON.parse(localStorage.getItem('notes') || '[]');
        console.log(currentNotes.length);

        const newNote: NoteType = {
            id: Date.now(),
            color: localStorage.getItem('noteColor')!, 
            title: newNoteTitle,
            excerpt: newNoteContent.substr(0, 50), 
            content: newNoteContent,
            starred: false, 
            created_on: new Date(),
            updated_on: new Date(),
        };
        console.log(newNote);

        notes.addNote(newNote);
        
        window.location.href = "/";
    }

    console.log(localStorage.getItem('notes'));

</script>


<div class="flex flex-col justify-start items-center gap-4">
    <Label for='large-input' class='block mb-2'>Title</Label>
    <textarea bind:value={newNoteTitle} id='title' class="flex gap-2 w-96 h-10 items-center rounded-lg text-sm focus:outline-none neumorph border-black" placeholder="Title..."/>
    <Label for='large-input' class='block mb-2'>Content</Label>
    <textarea bind:value={newNoteContent} id='content' class="flex gap-2 w-96 h-96 rounded-lg text-sm focus:outline-none neumorph border-black break-words" placeholder="Content"/>
    <div class="flex items-center bottom-0 p-4 gap-8">
        <Button on:click={addNewNote} class="btn"> Save </Button>
        <Button class="btn"><a href="/">Cancel</a></Button>
    </div>
</div>