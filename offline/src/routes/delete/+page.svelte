<script lang='ts'>
    import { notes } from '$lib/stores/note';
    import type { NoteType } from '../../types/note.type';

    let currentNotes: NoteType[];
    notes.subscribe(value => {
        currentNotes = value;
    });

    let showNotes = false;
    // let areYouSure = false; // on note click pop are you sure, if yes is click run deleteNoteById, if cancel areYouSure = false;

    const seeCurrentNotes = () => {
        console.log(currentNotes);
        showNotes = true;
    }

    const deleteNoteById = (idToDelete: number) => {
        notes.deleteNote(idToDelete);
        console.log($notes);
        // window.location.href = "/"; // redirects to home page after deletion
    }

    /*const areYouSure = () => {areYouSure = true;}
    {#if areYouSure}
    Show pop up*/




</script>

<div>
    <div class="flex pb-4">
        <h1 class="text-3xl pr-3"> Which note would you like to delete? </h1>
        <button class="bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded-full" on:click={seeCurrentNotes}> See Notes </button>
    </div>
    {#if showNotes}
        <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-12">
            {#each currentNotes as note}
                <button on:click={() => deleteNoteById(note.id)}>
                    <div class="flex flex-col relative w-full p-8 aspect-[5/6] rounded-2xl gap-8 shadow-lg shadow-slate-400 bg-note-{note.color}">
                        <h2 class="w-full mr-6 text-xl font-medium">{note.title}</h2>
                        <div class="w-full flex-1 overflow-hidden fade-text">
                            <p class="my-2">{note.excerpt}</p>
                        </div>
                    </div>
                </button>
            {/each}
        </div>
    {/if}
</div>