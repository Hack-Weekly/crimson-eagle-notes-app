<script lang="ts">
	import { Breadcrumb, BreadcrumbItem, Toast } from 'flowbite-svelte';
	import { notes, isLoading, fetchError } from '$lib/store';
	import { loader } from '$lib/loader.js';
	import Note from '$lib/Note.svelte';

	let error: string | null;
	fetchError.subscribe((value) => {
		error = value;
	});
</script>

<!-- Sticky note zone -->
<div>
	<Breadcrumb aria-label="Breadcrumbs" class="mb-10">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
		<BreadcrumbItem href="/">Folder 1</BreadcrumbItem>
		<BreadcrumbItem href="/">Folder 2</BreadcrumbItem>
		<BreadcrumbItem>Folder 3</BreadcrumbItem>
	</Breadcrumb>

	<div use:loader={isLoading}>
		{#if error}
			<Toast color="red">
				<svelte:fragment slot="icon">
					<svg
						aria-hidden="true"
						class="w-5 h-5"
						fill="currentColor"
						viewBox="0 0 20 20"
						xmlns="http://www.w3.org/2000/svg"
						><path
							fill-rule="evenodd"
							d="M4.293 4.293a1 1 0 011.414 0L10 8.586l4.293-4.293a1 1 0 111.414 1.414L11.414 10l4.293 4.293a1 1 0 01-1.414 1.414L10 11.414l-4.293 4.293a1 1 0 01-1.414-1.414L8.586 10 4.293 5.707a1 1 0 010-1.414z"
							clip-rule="evenodd"
						/></svg
					>
					<span class="sr-only">Error icon</span>
				</svelte:fragment>
				{error}
			</Toast>
		{:else}
			<div class="grid grid-cols-1 sm:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-12">
				{#each $notes as note (note.id)}
					<Note {note} />
				{/each}
			</div>
		{/if}
	</div>
</div>
