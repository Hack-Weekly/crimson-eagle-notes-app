<script>
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
    import Note from '$lib/Note.svelte';
    import { notes } from '$lib/store';
    
    // Only get notes that are starred
    $: starredNotes = $notes.filter(note => note.starred);
</script>

<div>
    <Breadcrumb aria-label="Breadcrumbs" class="mb-10">
        <BreadcrumbItem href="/" home>Home</BreadcrumbItem>
        <BreadcrumbItem>Favorites</BreadcrumbItem>
    </Breadcrumb>
    <h1 class="text-2xl font-bold my-2">Favorites</h1>
    <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-12">
        {#each starredNotes as note (note.id)}
            <Note {note} />
        {/each}
    </div>
</div>