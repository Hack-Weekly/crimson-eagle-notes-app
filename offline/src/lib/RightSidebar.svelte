<script lang="ts">
	import { Tooltip } from 'flowbite-svelte';
</script>

<!-- right items -->
<div
	class="hidden md:flex justify-evenly items-start fixed top-0 bottom-0 right-0 w-64 py-12 border-l border-slate-700"
>
	<button
		class="m-3 rounded bg-transparent text-slate-700 dark:text-gray-400 hover:bg-slate-700 dark:hover:bg-gray-400 hover:text-slate-100 dark:hover:text-gray-700"
		on:click={console.log}
	>
	<a
		href="/favorites"
	>
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-8 h-8">
			<path
				fill-rule="evenodd"
				d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z"
				clip-rule="evenodd"
			/>
		</svg>
		<Tooltip arrow={false}>Star Note</Tooltip>
	</a>
	</button>
	<a
		href="/profile"
		class="m-3 rounded bg-transparent text-slate-700 dark:text-gray-400 hover:bg-slate-700 dark:hover:bg-gray-400 hover:text-slate-100 dark:hover:text-gray-700"
	>
		<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-8 h-8">
			<path
				fill-rule="evenodd"
				d="M18.685 19.097A9.723 9.723 0 0021.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 003.065 7.097A9.716 9.716 0 0012 21.75a9.716 9.716 0 006.685-2.653zm-12.54-1.285A7.486 7.486 0 0112 15a7.486 7.486 0 015.855 2.812A8.224 8.224 0 0112 20.25a8.224 8.224 0 01-5.855-2.438zM15.75 9a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z"
				clip-rule="evenodd"
			/>
		</svg>
		<Tooltip arrow={false}>Profile</Tooltip>
	</a>
</div>
