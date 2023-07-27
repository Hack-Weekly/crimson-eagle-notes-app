<script lang='ts'>
    import { Label, Button } from 'flowbite-svelte';
    import { notes } from '$lib/stores/note';
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import type { NoteType } from '../../../types/note.type';

    let note: NoteType | undefined;
    let updatedNoteTitle = '';
    let updatedNoteContent = '';
    let updatedNoteColor = '';
    let exists: boolean = true;

    const updateCurrentNote = () => {
        if (!note) {
            return;
        }

        const updatedNote: NoteType = {
            ...note,
            title: updatedNoteTitle,
            excerpt: updatedNoteContent.substr(0, 50), 
            content: updatedNoteContent,
            color: updatedNoteColor,
            updated_on: new Date(),
        };
        
        notes.updateNote(note.id, updatedNote);
        window.location.href = "/";
    }

    onMount(async () => {
        const noteId = $page.params.id;
        const currentNotes = $notes;
        note = currentNotes.find(n => n.id === parseInt(noteId));
        if (note) {
            updatedNoteTitle = note.title;
            updatedNoteContent = note.content;
            updatedNoteColor = note.color;
        } else {
            exists = false;
        }
    });
</script>

<div class="flex flex-col justify-start items-center gap-4">
    {#if !exists} <!-- Display error message if note does not exist -->
        <p>Note not found. Please ensure you have the correct ID.</p>
    {:else}
        <Label for='large-input' class='block mb-2'>Title</Label>
        <textarea bind:value={updatedNoteTitle} id='title-input' class="flex gap-2 w-96 h-10 items-center rounded-lg text-sm focus:outline-none neumorph border-black" placeholder="Title..."/>
        <Label for='large-input' class='block mb-2'>Content</Label>
        <textarea bind:value={updatedNoteContent} id='content-input' class="flex gap-2 w-96 h-96 rounded-lg text-sm focus:outline-none neumorph border-black break-words" placeholder="Content"/>
        <div class="flex items-center bottom-0 p-4 gap-8">
            <Button on:click={updateCurrentNote} class="btn"> Save </Button>
            <Button class="btn"><a href="/">Cancel</a></Button>
        </div>
    {/if}
</div>
