<script>
	import AppBar from '../lib/AppBar.svelte';
	import Header from '../lib/Header.svelte';
	import LeftSidebar from '../lib/LeftSidebar.svelte';
	import RightSidebar from '../lib/RightSidebar.svelte';
	import '../app.css';
</script>

<LeftSidebar />
<Header />
<RightSidebar />
<main class="m-0 px-12 pt-4 pb-12 md:ml-32 md:mr-64 md:px-12 md:pt-4 md:pb-12 xl:px-24">
	<slot />
</main>
<AppBar />
