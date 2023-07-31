<script>
	import { Breadcrumb, BreadcrumbItem, Button, Modal } from 'flowbite-svelte';
	import Login from '$lib/Login.svelte';

	let isLoggedIn = false; // TODO use store
	let isOpen = false;
	let sync = false;

	const login = () => {
		isOpen = true;
	};
	const onSubmit = () => {
		console.log('settings saved');
	};
</script>

<div>
	<Breadcrumb aria-label="Breadcrumbs" class="mb-10">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
		<BreadcrumbItem>Profile</BreadcrumbItem>
	</Breadcrumb>
	<h1 class="my-2 text-2xl font-bold dark:text-gray-200">Profile</h1>

	<p class="my-2 dark:text-gray-200">
		You need to log in to sync your Notes.
		<Button on:click={login} size="sm" class="ml-2">Log in</Button>
	</p>
	<form on:submit={onSubmit}>
		<section class="my-6 border-b border-slate-400 py-6">
			<label
				class="text-sm font-medium {isLoggedIn
					? 'cursor-pointer text-gray-900 dark:text-gray-300'
					: 'text-gray-400 dark:text-gray-500'} flex items-center"
			>
				<input
					type="checkbox"
					disabled={isLoggedIn ? false : true}
					bind:checked={sync}
					class="peer sr-only mr-2 h-4 w-4 rounded border-gray-300 bg-gray-100
                        text-primary-600 focus:ring-2 focus:ring-primary-500 dark:border-gray-600
                        dark:bg-gray-700 dark:ring-offset-gray-800 dark:focus:ring-primary-600"
				/>
				<span
					class="relative mr-3 h-6 w-11 shrink-0 rounded-full bg-gray-400 after:absolute after:left-[2px] after:top-0.5 after:h-5 after:w-5 after:rounded-full after:border after:border-gray-300 after:bg-white after:transition-all after:content-[''] peer-checked:bg-primary-600 peer-checked:after:translate-x-full peer-checked:after:border-white peer-focus:ring-4 peer-focus:ring-primary-300 dark:border-gray-600 dark:bg-gray-700 dark:peer-focus:ring-primary-800"
				/>
				Sync your notes
			</label>
		</section>
		<Button type="submit">Save Settings</Button>
	</form>
	{#if isOpen}
		<Modal bind:open={isOpen} outsideclose>
			<Login />
		</Modal>
	{/if}
</div>
