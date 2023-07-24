<script lang='ts' src="https://unpkg.com/@themesberg/flowbite@latest/dist/flowbite.bundle.js">

	import Icon from '@iconify/svelte';
	import { onDestroy, onMount } from 'svelte';
	import { slide, crossfade } from 'svelte/transition';

	let isDropdownOpen = false;

	function toggleDropdown() {
		isDropdownOpen = !isDropdownOpen;
	}
	
	function logIconColor(color: string) {
    console.log(`Clicked icon color: ${color}`);
	toggleDropdown();
  	}

	function handleClickOutside(event: { target: Node | null; }) {
		const dropdown = document.getElementById('add');
		if (dropdown && !dropdown.contains(event.target)) {
			isDropdownOpen = false;
		}
	}

	onMount(() => {
		document.addEventListener('click', handleClickOutside);
	});

	// Remove the event listener when the component is destroyed to prevent memory leaks
	onDestroy(() => {
		document.removeEventListener('click', handleClickOutside);
	});

</script>





<!-- main header div -->
<div id="header" class="flex mt-10">
	<!-- left items -->
	<div class="flex justify-start items-center w-1/2 ">
		<div id="add" class="flex w-20 justify-center ml-10 mr-10 relative" >
			<button class="add-button z-10" on:click={() => toggleDropdown()}>
				<Icon icon="ph:plus-fill" class="w-9 h-9 asc" />
			</button>
			{#if isDropdownOpen}
				<div class="flex flex-col mt-9 ml-1.4 absolute"  in:slide={{duration: 600}} out:slide={{duration: 600}}>
					<button on:click={() => logIconColor('blue')}><Icon icon="openmoji:blue-circle" class="w-7 h-7"/></button>
					<button on:click={() => logIconColor('red')}><Icon icon="openmoji:red-circle" class="w-7 h-7" /></button>
					<button on:click={() => logIconColor('orange')}><Icon icon="openmoji:orange-circle" class="w-7 h-7" /></button>
					<button on:click={() => logIconColor('purple')}><Icon icon="openmoji:purple-circle" class="w-7 h-7" /></button>
					<button on:click={() => logIconColor('green')}><Icon icon="openmoji:green-circle" class="w-7 h-7" /></button>
					<button on:click={() => logIconColor('yellow')}><Icon icon="openmoji:yellow-circle" class="w-7 h-7" /></button>
				</div>
      		{/if}
		</div>
		<div class="flex w-40 justify-center">
			<h1 class="font-serif text-4xl">Nota</h1>
		</div>
	</div>
	
	<!-- SearchBar -->
	<input class="w-2/4 h-11 px-5 pr-10 rounded-full bg-main text-sm focus:outline-none" type="text" placeholder="Search.."/>


	<!-- right items -->
	<div class="flex justify-end w-1/2">
		<div id="favorite" class="flex w-20 justify-center">
			<button>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-8 h-8">
					<path fill-rule="evenodd" d="M10.788 3.21c.448-1.077 1.976-1.077 2.424 0l2.082 5.007 5.404.433c1.164.093 1.636 1.545.749 2.305l-4.117 3.527 1.257 5.273c.271 1.136-.964 2.033-1.96 1.425L12 18.354 7.373 21.18c-.996.608-2.231-.29-1.96-1.425l1.257-5.273-4.117-3.527c-.887-.76-.415-2.212.749-2.305l5.404-.433 2.082-5.006z" clip-rule="evenodd" />
				</svg>		
			</button>  
		</div>
		<div id="user" class="flex w-20 justify-center mr-10">
			<button>
				<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 24 24" fill="currentColor" class="w-8 h-8">
					<path fill-rule="evenodd" d="M18.685 19.097A9.723 9.723 0 0021.75 12c0-5.385-4.365-9.75-9.75-9.75S2.25 6.615 2.25 12a9.723 9.723 0 003.065 7.097A9.716 9.716 0 0012 21.75a9.716 9.716 0 006.685-2.653zm-12.54-1.285A7.486 7.486 0 0112 15a7.486 7.486 0 015.855 2.812A8.224 8.224 0 0112 20.25a8.224 8.224 0 01-5.855-2.438zM15.75 9a3.75 3.75 0 11-7.5 0 3.75 3.75 0 017.5 0z" clip-rule="evenodd" />
				</svg>	
			</button>	  
		</div>
	</div>
</div>

<!-- turn the add button to a drop down menu that when clicked shows these icons as buttons -->


<!-- Sticky note zone -->
<div>

</div>