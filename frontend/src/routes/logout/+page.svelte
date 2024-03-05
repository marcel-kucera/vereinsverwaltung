<script lang="ts">
  import { goto } from "$app/navigation";
  import MdWrapper from "$lib/components/mdWrapper.svelte";
  import { authStore } from "$lib/stores/authStore.svelte";
  import { untrack } from "svelte";

  $effect(() => {
    untrack(() => {
      authStore.logout();
      authStore.logoutPromise?.then(() => {
        goto("/");
      });
    });
  });
</script>

<MdWrapper>
  <div class="flex h-screen">
    <div class="m-auto">
      {#await authStore.logoutPromise}
        Logging Out
      {:then}
        Logged Out
      {:catch err}
        Error: {err}
      {/await}
    </div>
  </div>
</MdWrapper>
