<script lang="ts">
	import { Breadcrumb, BreadcrumbItem } from 'flowbite-svelte';
	import Note from '../lib/Note.svelte';
</script>

<!-- Sticky note zone -->
<div>
	<Breadcrumb aria-label="Breadcrumbs" class="mb-10">
		<BreadcrumbItem href="/" home>Home</BreadcrumbItem>
		<BreadcrumbItem href="/">Folder 1</BreadcrumbItem>
		<BreadcrumbItem href="/">Folder 2</BreadcrumbItem>
		<BreadcrumbItem>Folder 3</BreadcrumbItem>
	</Breadcrumb>
	<div
		class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-1 lg:grid-cols-2 xl:grid-cols-3 3xl:grid-cols-4 gap-12"
	>
		<Note
			id={1}
			color="orange"
			title="First note's title"
			excerpt="Lorem ipsum dolor sit amet, consectetur adipiscing elit. Sed dignissim consequat elementum."
		/>
		<Note
			id={2}
			color="green"
			title="Look at this!"
			excerpt="Nam id diam eu arcu fringilla hendrerit at id lectus. Nullam vel posuere eros, at aliquet elit."
			starred={true}
		/>
		<Note
			id={3}
			color="blue"
			title="This is very interesting"
			excerpt="Praesent ante lectus, congue a ex vel, auctor condimentum tellus."
		/>
		<Note
			id={4}
			color="pink"
			title="OMG No WAY"
			excerpt="Fusce erat lorem, molestie sed suscipit id, rhoncus vel arcu. Praesent nec neque turpis."
		/>
	</div>
</div>
