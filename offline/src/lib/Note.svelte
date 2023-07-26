<script lang="ts">
	import { Tooltip } from 'flowbite-svelte';
	import { notes } from '$lib/stores/note';
	import type { NoteType } from '../types/note.type';

	export let note: NoteType;

	const onStar = () => {
		console.log('Star clicked');
		notes.updateNote(note.id, { starred: !note.starred });
	};
</script>

<div
	class="flex flex-col relative w-full p-8 aspect-[5/6] rounded-2xl shadow-lg shadow-slate-400 bg-note-{note.color}"
>
	<button
		class="absolute z-10 top-3 right-3 p-2 text-slate-700 dark:text-gray-400 hover:text-primary-600 dark:hover:text-primary-500 group"
		on:click={onStar}
	>
		{#if note.starred}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				class="block group-hover:hidden w-8 h-8 fill-current"
			>
				<path
					fill-rule="evenodd"
					d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z"
					clip-rule="evenodd"
				/>
			</svg>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="hidden group-hover:block w-8 h-8 fill-none stroke-current"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"
				/>
			</svg>
		{:else}
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				stroke-width="1.5"
				stroke="currentColor"
				class="block group-hover:hidden w-8 h-8 fill-none stroke-current"
			>
				<path
					stroke-linecap="round"
					stroke-linejoin="round"
					d="M11.48 3.499a.562.562 0 011.04 0l2.125 5.111a.563.563 0 00.475.345l5.518.442c.499.04.701.663.321.988l-4.204 3.602a.563.563 0 00-.182.557l1.285 5.385a.562.562 0 01-.84.61l-4.725-2.885a.563.563 0 00-.586 0L6.982 20.54a.562.562 0 01-.84-.61l1.285-5.386a.562.562 0 00-.182-.557l-4.204-3.602a.563.563 0 01.321-.988l5.518-.442a.563.563 0 00.475-.345L11.48 3.5z"
				/>
			</svg>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				class="hidden group-hover:block w-8 h-8 fill-current"
			>
				<path
					fill-rule="evenodd"
					d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z"
					clip-rule="evenodd"
				/>
			</svg>
		{/if}
		<Tooltip arrow={false}>{note.starred ? 'Remove star' : 'Star Note'}</Tooltip>
	</button>
	<a
		href="/edit/{note.id}"
		class="absolute z-10 bottom-3 right-3 p-2 text-slate-700 dark:text-gray-400 hover:text-primary-600 dark:hover:text-primary-500"
	>
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="w-8 h-8 fill-current">
			<path
				d="M21.731 2.269a2.625 2.625 0 00-3.712 0l-1.157 1.157 3.712 3.712 1.157-1.157a2.625 2.625 0 000-3.712zM19.513 8.199l-3.712-3.712-12.15 12.15a5.25 5.25 0 00-1.32 2.214l-.8 2.685a.75.75 0 00.933.933l2.685-.8a5.25 5.25 0 002.214-1.32L19.513 8.2z"
			/>
		</svg>
		<Tooltip arrow={false}>Edit Note</Tooltip>
	</a>
	<h2 class="w-full mr-6 text-xl font-medium">{note.title}</h2>
	<div class="w-full flex-1 overflow-hidden fade-text">
		<p class="my-2">{note.excerpt}</p>
	</div>
</div>
