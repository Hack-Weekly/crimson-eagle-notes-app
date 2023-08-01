<script lang="ts">
	import { Button, Modal, Checkbox } from 'flowbite-svelte';
	import { notes } from '$lib/stores/note';
	import type { NoteType } from '../../types/note.type';

	let popupModal = false;
	let group: number[] = [];

	let currentNotes: NoteType[];
	notes.subscribe((value) => {
		currentNotes = value;
	});

	let showNotes = false;
	// let areYouSure = false; // on note click pop are you sure, if yes is click run deleteNoteById, if cancel areYouSure = false;

	const seeCurrentNotes = () => {
		console.log(currentNotes);
		showNotes = true;
	};

	const deleteSelectedNotes = () => {
		console.log(group);
		for (let i = 0; i < group.length; i++) {
			notes.deleteNote(group[i]);
		}
		console.log(currentNotes);
		window.location.href = '/';
	};

	//assign to delete button next to see notes on:click={() => popupModal = true}
</script>

<div>
	<div class="flex pb-4">
		<h1 class="pr-3 font-serif text-3xl dark:text-gray-200">
			Which note would you like to delete?
		</h1>
		<button
			class="mr-3 rounded-full bg-orange-500 px-4 py-2 font-serif font-bold text-white hover:bg-orange-600"
			on:click={seeCurrentNotes}
		>
			See Notes
		</button>
		<button
			class="rounded-full bg-orange-500 px-4 py-2 font-serif font-bold text-white hover:bg-orange-600"
			on:click={() => (popupModal = true)}
		>
			Delete
		</button>
	</div>
	<Modal
		bind:open={popupModal}
		size="xs"
		class="bg-main dark:bg-dark-main"
		backdropClass="bg-gray-500 bg-opacity-30 dark:bg-opacity-80"
		autoclose
	>
		<div class="bg-main text-center dark:bg-dark-main">
			<svg
				aria-hidden="true"
				class="mx-auto mb-4 h-14 w-14 text-gray-400 dark:text-gray-200"
				fill="none"
				stroke="currentColor"
				viewBox="0 0 24 24"
				xmlns="http://www.w3.org/2000/svg"
				><path
					stroke-linecap="round"
					stroke-linejoin="round"
					stroke-width="2"
					d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"
				/></svg
			>
			<h3 class="mb-5 text-lg font-normal text-gray-500 dark:text-gray-400">
				Are you sure you want to delete the note(s)?
			</h3>
			<Button color="red" on:click={deleteSelectedNotes} class="mr-2">Yes, I'm sure</Button>
			<Button color="alternative">No, cancel</Button>
		</div>
	</Modal>
	{#if showNotes}
		<div class="grid grid-cols-1 gap-12 sm:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4">
			{#each currentNotes as note}
				<button>
					<div
						class="relative flex aspect-[5/6] w-full flex-col gap-8 rounded-2xl p-8 shadow-lg shadow-slate-400 bg-note-{note.color} dark:shadow-slate-700"
					>
						<h2 class="mr-6 w-full text-xl font-medium">{note.title}</h2>
						<div class="fade-text w-full flex-1 overflow-hidden">
							<p class="my-2">{note.excerpt}</p>
						</div>
						<Checkbox bind:group value={note.id} />
					</div>
				</button>
			{/each}
		</div>
	{/if}
</div>
