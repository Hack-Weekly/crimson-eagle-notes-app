<script lang='ts'>
    import { Button, Modal } from 'flowbite-svelte'
    let popupModal = false;

    let currentNotes = localStorage.getItem('notes');
    currentNotes = currentNotes ? JSON.parse(currentNotes) : [];


    let showNotes = false;
    // let areYouSure = false; on note click pop are you sure, if yes is click run deleteNoteById, if cancel areYouSure = false;

    const seeCurrentNotes = () => {
        console.log(currentNotes);
        showNotes = true;
    }

    const deleteNoteById = (idToDelete) => {
        currentNotes = currentNotes.filter(note => note.id !== idToDelete);
        localStorage.setItem('notes', JSON.stringify(currentNotes));
        console.log(currentNotes);
        window.location.href = "/";
    }

    /*const areYouSure = () => {areYouSure = true;}
    {#if areYouSure}
    () => popupModal = true
    () => deleteNoteById(note.id)
    Show pop up*/




</script>

<div>
    <div class="flex pb-4">
        <h1 class="text-3xl font-serif pr-3"> Which note would you like to delete? </h1>
        <button class="bg-orange-500 hover:bg-orange-600 text-white font-bold py-2 px-4 rounded-full font-serif" on:click={seeCurrentNotes}> See Notes </button>
    </div>
    {#if showNotes}
        <div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-12">
            {#each currentNotes as note}
                <button on:click={() => popupModal = true}>
                    <div class="flex flex-col relative w-full p-8 aspect-[5/6] rounded-2xl gap-8 shadow-lg shadow-slate-400 bg-note-{note.color}">
                        <h2 class="w-full mr-6 text-xl font-medium">{note.title}</h2>
                        <div class="w-full flex-1 overflow-hidden fade-text">
                            <p class="my-2">{note.excerpt}</p>
                        </div>
                    </div>
                </button>
                <Modal bind:open={popupModal} size="xs" class="bg-main" backdropClass="bg-gray-500 bg-opacity-30 dark:bg-opacity-80" autoclose>
                    <div class="text-center bg-main">
                      <svg aria-hidden="true" class="mx-auto mb-4 w-14 h-14 text-gray-400 dark:text-gray-200" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg"><path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path></svg>
                      <h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">Are you sure you want to delete this note?</h3>
                      <Button color="red" on:click={() => deleteNoteById(note.id)} class="mr-2">Yes, I'm sure</Button>
                      <Button color='alternative'>No, cancel</Button>
                    </div>
                </Modal>
            {/each}
        </div>
    {/if}
</div>