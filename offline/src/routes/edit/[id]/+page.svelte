<script lang='ts'>
    import { Button, Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
    import { notes } from '$lib/stores/note';
    import { onMount } from 'svelte';
    import { page } from '$app/stores';
    import type { NoteType } from '../../../types/note.type';
    import { Tabs, TabItem, Textarea } from 'flowbite-svelte';
    import MarkdownIt from 'markdown-it';
	import sanitizeHtml from 'sanitize-html';
    import { afterUpdate } from 'svelte';
    import InfoDrawer from '$lib/InfoDrawer.svelte';

    let note: NoteType | undefined;
    let updatedNoteTitle = '';
    let updatedNoteContent = '';
    let updatedNoteColor = '';
    let exists: boolean = true;
    let theme = localStorage.getItem('noteColor');

    const md = new MarkdownIt({
		breaks: true,
		linkify: true,
		typographer: true
	});

	let result = md.render(updatedNoteContent);

	afterUpdate(() => {
		result = sanitizeHtml(md.render(updatedNoteContent), {
			allowedTags: sanitizeHtml.defaults.allowedTags.concat(['h1', 'h2', 'img'])
		});
	});

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


<div class="w-full">
    <Breadcrumb aria-label="Breadcrumbs" class="mb-10 justify-start">
        <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
        <BreadcrumbItem>Edit</BreadcrumbItem>
    </Breadcrumb>
    <div class="flex flex-col justify-start">
        {#if !exists} <!-- Display error message if note does not exist -->
            <p>Note not found. Please ensure you have the correct ID.</p>
        {:else}
        <Tabs>
            <TabItem open title="Editor">
                <div>
                    <Textarea
                        bind:value={updatedNoteContent}
                        id="content"
                        rows="20"
                        placeholder="Content"
                        class="flex gap-2 rounded-lg focus:outline-none neumorph border-black focus:border-note-{theme}"
                    />
                </div>
            </TabItem>
            <TabItem title="Preview" class="text-{theme}">
                <div
                    id="preview"
                    class="flex gap-2 p-2 px-3 rounded-lg focus:outline-none neumorph border-black"
                >
                    <div class="prose prose-slate prose-sm">
                        {@html result}
                    </div>
                </div>
            </TabItem>
        </Tabs>
    
        <div class="flex items-center bottom-0 p-4 gap-8">
            <InfoDrawer/>
            <Button on:click={updateCurrentNote} class="btn bg-note-{theme}" >Save</Button>
            <Button class="btn bg-note-{theme}"><a href="/">Cancel</a></Button>
        </div>
        {/if}
    </div>
</div>