<script>
  import { Drawer, Button, CloseButton } from 'flowbite-svelte';
  import { sineIn } from 'svelte/easing';
  
  let theme = localStorage.getItem('noteColor');
  let hidden1 = true; 
  let transitionParams = {
    x: -320,
    duration: 200,
    easing: sineIn
  };
</script>

<div class="text-center">
  <Button class="bg-note-{theme}" on:click={() => (hidden1 = false)}>Info</Button>
</div>

<Drawer class="w-800" transitionType="fly" {transitionParams} bind:hidden={hidden1} id='sidebar1'>
<div class='flex items-center'>
  <h5
    id="drawer-label"
    class="inline-flex items-center mb-4 text-base font-semibold text-gray-500 dark:text-gray-400">
    <svg class="w-5 h-5 mr-2" aria-hidden="true" fill="currentColor" viewBox="0 0 20 20" xmlns="http://www.w3.org/2000/svg"><path fill-rule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clip-rule="evenodd"></path></svg>Markdown Tips
  </h5>
  <CloseButton on:click={() => (hidden1 = true)} class='mb-4 dark:text-white'/>
    </div>
  <p class="mb-6 text-lg text-gray-500 dark:text-gray-400">
<b>Nota uses Markdown to format your notes</b><br><br>
Add bold text by surrounding it with two **asterisks**<br>
Add italic text by surrounding it with one *asterisk*<br>
Add a code block by surrounding it with ```backticks```<br>
Add a [link](www.link.com) by surrounding it with square brackets <br>
Add a quote by adding a greater than > sign<br>
Add a heading by adding a hash # sign<br>
Add a list by adding a - dash<br>
  </p>

</Drawer>