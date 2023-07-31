<script>
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
	import Note from '$lib/Note.svelte';
	import { notes } from '$lib/stores/note';

	// Only get notes that are starred
	$: starredNotes = $notes.filter((note) => note.starred);
</script>

<div>
	<Breadcrumb aria-label="Breadcrumbs" class="mb-10">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
		<BreadcrumbItem>Favorites</BreadcrumbItem>
	</Breadcrumb>
	<h1 class="my-2 text-2xl font-bold dark:text-gray-200">Favorites</h1>
	<div
		class="grid grid-cols-1 gap-12 sm:grid-cols-2 md:grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4"
	>
		{#each starredNotes as note (note.id)}
			<Note {note} />
		{/each}
	</div>
</div>
