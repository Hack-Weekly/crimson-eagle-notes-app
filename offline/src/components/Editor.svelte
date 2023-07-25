<script lang="ts">
	import {browser} from '$app/environment'
	import { Editor, rootCtx, defaultValueCtx } from '@milkdown/core'
	import { commonmark } from '@milkdown/preset-commonmark'
	import { nord } from '@milkdown/theme-nord'
    import {gfm} from '@milkdown/preset-gfm'
	import { prism, prismConfig } from '@milkdown/plugin-prism'
	import { history } from '@milkdown/plugin-history';
	import { clipboard } from '@milkdown/plugin-clipboard';
  // Prism Imports
import css from 'refractor/lang/css'
import javascript from 'refractor/lang/javascript'
import typescript from 'refractor/lang/typescript'
import jsx from 'refractor/lang/jsx'
import tsx from 'refractor/lang/tsx'
import 'prism-themes/themes/prism-nord.css'


	const markdown =
	`**Start Writing Markdown**`
	
	function editor(dom: any) {
		if (browser) {
	  Editor.make()
		.config((ctx) => {
		  ctx.set(rootCtx, dom)
		  ctx.set(defaultValueCtx, markdown)
      ctx.set(prismConfig.key, {
      configureRefractor: (refractor) => {
        refractor.register(css)
        refractor.register(javascript)
        refractor.register(typescript)
        refractor.register(jsx)
        refractor.register(tsx)
      },
    })
    })
		.config(nord)
		.use(commonmark)
    .use(gfm)
    .use(prism)
	.use(history)
	.use(clipboard)
		.create();
	
	}
	}
	</script>
	
	<main>
	  <div use:editor />
	</main>