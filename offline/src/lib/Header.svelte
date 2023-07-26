<script lang="ts">
	import Icon from '@iconify/svelte';
	import { Input } from 'flowbite-svelte';

	let query = '';

	const onSearch = () => {
		console.log(query);
	};
</script>

<!-- main header div -->
<header id="header" class="m-0 p-12 md:ml-32 md:p-12 xl:px-24 xl:py-12 flex items-center">
	<a href="/"><h1 class="mr-32 md:mr-16 lg:mr-32 font-serif text-4xl">Nota</h1></a>
	<!-- SearchBar -->
	<form on:submit|preventDefault={onSearch} class="grow max-w-md">
		<Input
			bind:value={query}
			size="md"
			placeholder="Search..."
			class="flex gap-2 items-center rounded-full text-sm focus:outline-none
                bg-main neumorph border-none"
		>
			<button type="submit" class="outline-none" slot="right">
				<Icon icon="simple-line-icons:magnifier" class="w-4 h-4" />
			</button>
		</Input>
	</form>
</header>
