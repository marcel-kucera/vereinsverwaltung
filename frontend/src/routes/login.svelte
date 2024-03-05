<script lang="ts">
  import type { Login } from "$lib/api/login";
  import Button from "$lib/components/button.svelte";
  import Input from "$lib/components/input.svelte";
  import MdWrapper from "$lib/components/mdWrapper.svelte";
  import { authStore } from "$lib/stores/authStore.svelte";

  let auth_data = $state<Login>({} as Login);

  function submit() {
    authStore.login(auth_data);
  }
</script>

<MdWrapper>
  <div class="flex h-screen flex-col gap-4">
    <div class="m-auto flex flex-col">
      <form
        class="flex flex-col gap-4 m-auto"
        on:submit|preventDefault={submit}
      >
        <Input
          name="username"
          placeholder="Benutzername"
          bind:text={auth_data.name}
        />
        <Input
          name="password"
          placeholder="Passwort"
          type="password"
          bind:text={auth_data.password}
        />
        <div class="m-auto">
          <Button>Anmelden</Button>
        </div>
      </form>

      {#if authStore.loginPromise}
        <div>
          {#await authStore.loginPromise}
            Loading...
          {:then}
            Logged in :)
          {:catch err}
            Error: {err}
          {/await}
        </div>
      {/if}
    </div>
  </div>
</MdWrapper>
