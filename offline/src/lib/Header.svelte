<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Input } from 'flowbite-svelte';
	import { searchQuery } from './stores/search';
	import { DarkMode } from 'flowbite-svelte';
</script>

<!-- main header div -->
<header
	id="header"
	class="m-0 flex items-center justify-between p-12 md:ml-32 md:p-12 xl:px-24 xl:py-12"
>
	<div class="flex flex-grow">
		<a href="/"
			><h1 class="mr-32 font-serif text-4xl dark:text-gray-200 md:mr-16 lg:mr-32">Nota</h1></a
		>
		<!-- SearchBar -->
		<form class="max-w-xl flex-grow">
			<Input
				bind:value={$searchQuery}
				size="md"
				placeholder="Search..."
				class="neumorph justify-left flex gap-2 rounded-full border-none
					bg-main text-sm focus:outline-none"
			>
				<button type="submit" class="outline-none" slot="right">
					<Icon icon="simple-line-icons:magnifier" class="h-4 w-4" />
				</button>
			</Input>
		</form>
	</div>
	<div>
		<DarkMode />
	</div>
</header>
