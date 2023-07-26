<script lang='ts'>
    import { Label, Input } from 'flowbite-svelte';
    import { Button } from 'flowbite-svelte';



    const addNewNote = () => {
        console.log("Adding new note");

        console.log(localStorage.getItem('noteColor'));

        const newNoteTitle = document.getElementById('title').value;
        const newNoteContent = document.getElementById('content').value;

        let currentNotes = localStorage.getItem('notes');
        currentNotes = currentNotes ? JSON.parse(currentNotes) : [];

        const newNote = {
            id: currentNotes.length + 1,
            color: localStorage.getItem('noteColor'), 
            title: newNoteTitle,
            excerpt: newNoteContent.substr(0, 50), 
            starred: false, 
        };

        currentNotes.push(newNote);

        localStorage.setItem('notes', JSON.stringify(currentNotes));
        
        window.location.href = "/";
    }

    console.log(localStorage.getItem('notes'));

</script>


<div class="flex flex-col justify-start items-center gap-4">
    <Label for='large-input' class='block mb-2'>Title</Label>
    <textarea id='title' class="flex gap-2 w-96 h-10 items-center rounded-lg text-sm focus:outline-none neumorph border-black" placeholder="Title..."/>
    <Label for='large-input' class='block mb-2'>Content</Label>
    <textarea id='content' class="flex gap-2 w-96 h-96 rounded-lg text-sm focus:outline-none neumorph border-black break-words" placeholder="Content"/>
    <div class="flex items-center bottom-0 p-4 gap-8">
        <Button on:click={addNewNote} class="btn"> Save </Button>
        <Button class="btn"><a href="/">Cancel</a></Button>
    </div>
</div>