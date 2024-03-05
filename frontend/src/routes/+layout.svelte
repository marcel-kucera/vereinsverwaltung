<script lang="ts">
  import Center from "$lib/components/center.svelte";
  import ModalProvider from "$lib/components/modalProvider.svelte";
  import { authStore } from "$lib/stores/authStore.svelte";
  import "../app.css";
  import Login from "./login.svelte";

  $effect(() => {
    authStore.checkStatus();
  });
</script>

<ModalProvider>
  {#await authStore.loginStatus}
    <Center>Loading...</Center>
  {:then status}
    {#if status}
      <slot />
    {:else}
      <Login />
    {/if}
  {:catch err}
    <Center>
      Error getting Login Status: {err}
    </Center>
  {/await}
</ModalProvider>
