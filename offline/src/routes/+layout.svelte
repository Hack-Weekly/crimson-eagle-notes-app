<script>
	import AppBar from '../lib/AppBar.svelte';
	import Header from '../lib/Header.svelte';
	import LeftSidebar from '../lib/LeftSidebar.svelte';
	import RightSidebar from '../lib/RightSidebar.svelte';
	import '../app.css';
	import { invalidate } from '$app/navigation'
  	import { onMount } from 'svelte'

	export let data

	let { supabase, session } = data
	$: ({ supabase, session } = data)

	onMount(() => {
	const {
		data: { subscription },
	} = supabase.auth.onAuthStateChange((event, _session) => {
		if (_session?.expires_at !== session?.expires_at) {
		invalidate('supabase:auth')
		}
	})

	return () => subscription.unsubscribe()
	});
</script>

<LeftSidebar />
<Header />
<main class="m-0 px-12 pt-4 pb-24 md:ml-32 md:px-12 md:pt-4 md:pb-12 xl:px-24">
	<slot />
</main>
<AppBar />
