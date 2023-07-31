<script lang="ts">
	import { onMount } from 'svelte';
	import { slide } from 'svelte/transition';
	import { Tooltip } from 'flowbite-svelte';

	let isDropdownOpen = false;
	let dropdown: HTMLDivElement;

	function toggleDropdown() {
		isDropdownOpen = !isDropdownOpen;
	}

	const onNewNote = (color: string) => {
		console.log(`New note will be added in color ${color}`);
		localStorage.setItem('noteColor', color);
	};

	const onDelete = () => {
		console.log('Deleting notes');
	};

	onMount(() => {
		// handle esc and outside clicks for the dropdown
		const handleKeyDown = (event: KeyboardEvent) => {
			if (event.key === 'Escape') {
				isDropdownOpen = false;
			}
		};
		const handleClick = (event: MouseEvent) => {
			if (dropdown && !dropdown.contains(event.target as Node)) {
				isDropdownOpen = false;
			} else if (dropdown && dropdown.contains(event.target as Node)) {
				isDropdownOpen = true;
			}
		};

		document.addEventListener('keydown', handleKeyDown, false);
		document.addEventListener('mousedown', handleClick, false);

		return () => {
			document.removeEventListener('keydown', handleKeyDown, false);
			document.removeEventListener('mousedown', handleClick, false);
		};
	});
</script>

<!-- left items -->
<div
	class="fixed bottom-0 left-0 top-0 hidden w-32 flex-col items-center justify-between border-r border-slate-700 py-12 md:flex"
>
	<div id="add" class="relative mx-auto">
		<button
			class="m-3 rounded bg-transparent text-slate-700 hover:bg-slate-700 hover:text-slate-100 dark:text-gray-400 dark:hover:bg-gray-400 dark:hover:text-gray-700"
			on:click={toggleDropdown}
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				class="h-8 w-8 fill-current"
			>
				<path
					fill-rule="evenodd"
					d="M12 3.75a.75.75 0 01.75.75v6.75h6.75a.75.75 0 010 1.5h-6.75v6.75a.75.75 0 01-1.5 0v-6.75H4.5a.75.75 0 010-1.5h6.75V4.5a.75.75 0 01.75-.75z"
					clip-rule="evenodd"
				/>
			</svg>
			<Tooltip arrow={false}>Add new Note</Tooltip>
		</button>
		{#if isDropdownOpen}
			<div
				bind:this={dropdown}
				class="mt-8 flex flex-col items-center"
				in:slide={{ duration: 600 }}
				out:slide={{ duration: 600 }}
			>
				{#each ['orange', 'green', 'blue', 'pink'] as color}
					<a
						href="/add"
						on:click={() => onNewNote(color)}
						class="m-3 h-6 w-6 bg-note-{color} rounded-full hover:blur-sm"
						title="Add {color} note">&nbsp;</a
					>
				{/each}
			</div>
		{/if}
	</div>
	<div class="flex flex-col items-center">
		<button
			class="m-3 rounded bg-transparent text-slate-700 hover:bg-slate-700 hover:text-slate-100 dark:text-gray-400 dark:hover:bg-gray-400 dark:hover:text-gray-700"
			on:click={console.log}
		>
			<a href="/favorites">
				<svg
					xmlns="http://www.w3.org/2000/svg"
					viewBox="0 0 24 24"
					fill="currentColor"
					class="h-8 w-8"
				>
					<path
						fill-rule="evenodd"
						d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z"
						clip-rule="evenodd"
					/>
				</svg>
				<Tooltip arrow={false}>Star Note</Tooltip>
			</a>
		</button>
		<a href="/delete">
			<button
				class="m-3 bg-transparent text-slate-700 hover:text-primary-600 dark:text-gray-400 dark:hover:text-primary-500"
				on:click={onDelete}
			>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="h-8 w-8 fill-current">
					<path
						fill-rule="evenodd"
						d="M16.5 4.478v.227a48.816 48.816 0 013.878.512.75.75 0 11-.256 1.478l-.209-.035-1.005 13.07a3 3 0 01-2.991 2.77H8.084a3 3 0 01-2.991-2.77L4.087 6.66l-.209.035a.75.75 0 01-.256-1.478A48.567 48.567 0 017.5 4.705v-.227c0-1.564 1.213-2.9 2.816-2.951a52.662 52.662 0 013.369 0c1.603.051 2.815 1.387 2.815 2.951zm-6.136-1.452a51.196 51.196 0 013.273 0C14.39 3.05 15 3.684 15 4.478v.113a49.488 49.488 0 00-6 0v-.113c0-.794.609-1.428 1.364-1.452zm-.355 5.945a.75.75 0 10-1.5.058l.347 9a.75.75 0 101.499-.058l-.346-9zm5.48.058a.75.75 0 10-1.498-.058l-.347 9a.75.75 0 001.5.058l.345-9z"
						clip-rule="evenodd"
					/>
				</svg>
				<Tooltip arrow={false}>Delete Notes</Tooltip>
			</button>
		</a>
		<a
			href="/settings"
			class="m-3 rounded bg-transparent text-slate-700 hover:bg-slate-700 hover:text-slate-100 dark:text-gray-400 dark:hover:bg-gray-400 dark:hover:text-gray-700"
		>
			<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" class="h-8 w-8 fill-current">
				<path
					fill-rule="evenodd"
					d="M11.078 2.25c-.917 0-1.699.663-1.85 1.567L9.05 4.889c-.02.12-.115.26-.297.348a7.493 7.493 0 00-.986.57c-.166.115-.334.126-.45.083L6.3 5.508a1.875 1.875 0 00-2.282.819l-.922 1.597a1.875 1.875 0 00.432 2.385l.84.692c.095.078.17.229.154.43a7.598 7.598 0 000 1.139c.015.2-.059.352-.153.43l-.841.692a1.875 1.875 0 00-.432 2.385l.922 1.597a1.875 1.875 0 002.282.818l1.019-.382c.115-.043.283-.031.45.082.312.214.641.405.985.57.182.088.277.228.297.35l.178 1.071c.151.904.933 1.567 1.85 1.567h1.844c.916 0 1.699-.663 1.85-1.567l.178-1.072c.02-.12.114-.26.297-.349.344-.165.673-.356.985-.57.167-.114.335-.125.45-.082l1.02.382a1.875 1.875 0 002.28-.819l.923-1.597a1.875 1.875 0 00-.432-2.385l-.84-.692c-.095-.078-.17-.229-.154-.43a7.614 7.614 0 000-1.139c-.016-.2.059-.352.153-.43l.84-.692c.708-.582.891-1.59.433-2.385l-.922-1.597a1.875 1.875 0 00-2.282-.818l-1.02.382c-.114.043-.282.031-.449-.083a7.49 7.49 0 00-.985-.57c-.183-.087-.277-.227-.297-.348l-.179-1.072a1.875 1.875 0 00-1.85-1.567h-1.843zM12 15.75a3.75 3.75 0 100-7.5 3.75 3.75 0 000 7.5z"
					clip-rule="evenodd"
				/>
			</svg>
			<Tooltip arrow={false}>Settings</Tooltip>
		</a>
		<a
			href="/profile"
			class="m-3 rounded bg-transparent text-slate-700 hover:bg-slate-700 hover:text-slate-100 dark:text-gray-400 dark:hover:bg-gray-400 dark:hover:text-gray-700"
		>
			<svg
				xmlns="http://www.w3.org/2000/svg"
				viewBox="0 0 24 24"
				fill="currentColor"
				class="h-8 w-8"
			>
				<path
					fill-rule="evenodd"
					d="M18.685 19.097A9.723 9.723 0 0021.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 003.065 7.097A9.716 9.716 0 0012 21.75a9.716 9.716 0 006.685-2.653zm-12.54-1.285A7.486 7.486 0 0112 15a7.486 7.486 0 015.855 2.812A8.224 8.224 0 0112 20.25a8.224 8.224 0 01-5.855-2.438zM15.75 9a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"
					clip-rule="evenodd"
				/>
			</svg>
			<Tooltip arrow={false}>Profile</Tooltip>
		</a>
	</div>
</div>
