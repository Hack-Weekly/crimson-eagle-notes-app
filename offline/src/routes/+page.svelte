<script lang="ts">
	import { Breadcrumb, BreadcrumbItem, TabItem } from 'flowbite-svelte';
	import Note from '$lib/Note.svelte';
	import { notes } from '$lib/store';
</script>

<!-- Sticky note zone -->
<div>
	<Breadcrumb aria-label="Breadcrumbs" class="mb-10">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
		<BreadcrumbItem href="/">Folder 1</BreadcrumbItem>
		<BreadcrumbItem href="/">Folder 2</BreadcrumbItem>
		<BreadcrumbItem>Folder 3</BreadcrumbItem>
	</Breadcrumb>
	<div
		class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-12"
	>

	{#each $notes as note (note.id)}
		<Note {note} />
	{/each}
	</div>
</div>
