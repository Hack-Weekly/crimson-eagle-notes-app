<script>
  import { Button, Label, Input, Checkbox } from 'flowbite-svelte';

  export let data
  let { supabase } = data
  $: ({ supabase } = data)

  let register = false;
  let isLoggedIn;

  const toggleRegister = () => {
    register = !register;
  }

  let email
  let password

  const handleSignUp = async () => {
    await supabase.auth.signUp({
      email,
      password,
      options: {
        emailRedirectTo: `${location.origin}/auth/callback`,
      },
    })
  }

  const handleSignIn = async () => {
    await supabase.auth.signInWithPassword({
      email,
      password,
    })
    isLoggedIn = true;
    localStorage.setItem('isLoggedIn', 'true')
    window.location.href = '/';
  }

  const handleSignOut = async () => {
    await supabase.auth.signOut()
  }
</script>
  
<div class="flex justify-center">
  <form class="flex flex-col space-y-6 w-2/3" on:submit="{handleSignUp}">
    <div class = "flex justify-center">
      <h3 class="text-xl font-medium text-gray-900 dark:text-white">
        Welcome!
      </h3>
    </div>
    <Label class="space-y-2">
      <span>Email</span>
      <Input type="email" name="email" placeholder="name@company.com" bind:value="{email}" required />
    </Label>
    <Label class="space-y-2">
      <span>Your password</span>
      <Input type="password" name="password" placeholder="•••••" bind:value="{password}" required />
    </Label>
    <div class="flex items-start">
      <Checkbox>Remember me</Checkbox>
      <a
        href="/lost-password"
        class="ml-auto text-sm text-primary-700 hover:underline dark:text-primary-500"
        >Lost password?</a
      >
    </div>


    {#if register}
    <Button type="submit" class="w-full">Register your account</Button>
    {:else}
      {#if isLoggedIn}
        <Button class="w-full" on:click="{handleSignOut}">Logout of your account</Button>
      {:else}
        <Button class="w-full" on:click="{handleSignIn}">Login to your account</Button>
      {/if}
    {/if}


    <div class="text-sm font-medium text-gray-500 dark:text-gray-300">
        {#if register}
        Already registered?
        {:else}
        Not registered?
        {/if}
      <button
        on:click={toggleRegister}
        class="text-primary-700 hover:underline dark:text-primary-500">
        {#if register}
        <Button class="w-full">Login to your account</Button>
        {:else}
        <Button class="w-full">Register your account</Button>
        {/if}
      </button>
    </div>
  </form>
</div>