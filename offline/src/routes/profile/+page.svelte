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
	<h1 class="text-2xl font-bold my-2">Profile</h1>

	<p class="my-2">
		You need to log in to sync your Notes.
		<Button on:click={login} size="sm" class="ml-2">Log in</Button>
	</p>
	<form on:submit={onSubmit}>
		<section class="my-6 py-6 border-b border-slate-400">
			<label
				class="text-sm font-medium {isLoggedIn
					? 'text-gray-900 dark:text-gray-300 cursor-pointer'
					: 'text-gray-400 dark:text-gray-500'} flex items-center"
			>
				<input
					type="checkbox"
					disabled={isLoggedIn ? false : true}
					bind:checked={sync}
					class="w-4 h-4 mr-2 sr-only peer rounded bg-gray-100 border-gray-300
                        dark:ring-offset-gray-800 dark:bg-gray-700 dark:border-gray-600 dark:focus:ring-primary-600
                        text-primary-600 focus:ring-2 focus:ring-primary-500"
				/>
				<span
					class="mr-3 shrink-0 bg-gray-400 rounded-full peer-focus:ring-4 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:bg-white after:border-gray-300 after:border after:rounded-full after:transition-all dark:bg-gray-700 dark:border-gray-600 peer-focus:ring-primary-300 dark:peer-focus:ring-primary-800 peer-checked:bg-primary-600 w-11 h-6 after:top-0.5 after:left-[2px] after:h-5 after:w-5 relative"
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
