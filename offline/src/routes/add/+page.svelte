<script lang="ts">
	import { Button } from 'flowbite-svelte';
	import MarkdownIt from 'markdown-it';
	import sanitizeHtml from 'sanitize-html';
	import { Tabs, TabItem, Textarea } from 'flowbite-svelte';
	import { afterUpdate } from 'svelte';
	import { notes } from '$lib/stores/note';
	import type { NoteType } from '../../types/note.type';
	import InfoDrawer from '$lib/InfoDrawer.svelte';
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';

	let newNoteTitle = '';
	let newNoteContent = '';
	let theme = localStorage.getItem('noteColor');

	const md = new MarkdownIt({
		breaks: true,
		linkify: true,
		typographer: true
	});

	let result = md.render(newNoteContent);

	afterUpdate(() => {
		result = sanitizeHtml(md.render(newNoteContent), {
			allowedTags: sanitizeHtml.defaults.allowedTags.concat(['h1', 'h2', 'img'])
		});
	});

	const addNewNote = () => {
		console.log('Adding new note');

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
			updated_on: new Date()
		};

		console.log(newNote);
		notes.addNote(newNote);
		window.location.href = '/';
	};

	console.log(localStorage.getItem('notes'));
</script>

<div class="flex flex-col justify-start">
	<Breadcrumb aria-label="Breadcrumbs" class="mb-10 justify-start">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
		<BreadcrumbItem>Add</BreadcrumbItem>
	</Breadcrumb>
	<Tabs>
		<TabItem open title="Editor">
			<div>
				<Textarea
					bind:value={newNoteContent}
					id="content"
					rows="20"
					placeholder="Content"
					class="neumorph flex gap-2 rounded-lg border-black focus:outline-none focus:border-note-{theme}"
				/>
			</div>
		</TabItem>
		<TabItem title="Preview" class="text-{theme}">
			<div
				id="preview"
				class="neumorph flex gap-2 rounded-lg border-black p-2 px-3 focus:outline-none"
			>
				<div class="prose-slat prose prose-sm dark:prose-invert">
					{@html result}
				</div>
			</div>
		</TabItem>
	</Tabs>

	<div class="bottom-0 flex items-center gap-8 p-4">
		<InfoDrawer />
		<Button on:click={addNewNote} class="btn bg-note-{theme}">Save</Button>
		<Button class="btn bg-note-{theme}"><a href="/">Cancel</a></Button>
	</div>
</div>
