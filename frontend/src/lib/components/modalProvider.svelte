<script lang="ts">
  import { modalStore } from "$lib/stores/modalStore.svelte";
  import Button from "./button.svelte";
  import Heading from "./heading.svelte";

</script>

{#if modalStore.open}
  <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions a11y-no-static-element-interactions -->
  <div
    class="w-full h-full flex text-gray-100 bg-black/95 fixed z-50"
    on:click={() => modalStore.close()}
  >
    <!-- svelte-ignore a11y-click-events-have-key-events a11y-no-noninteractive-element-interactions a11y-no-static-element-interactions -->
    <div
      class="m-auto p-4 bg-gray-900 shadow rounded-lg"
      on:click|stopPropagation={() => {}}
    >
      <Heading>{modalStore.title}</Heading>
      <p class="text-lg">{modalStore.content}</p>
      <div class="flex gap-3 m-3">
        <Button
          danger
          on:click={() => {
            modalStore.callback();
            modalStore.close();
          }}>Ja</Button
        >
        <Button on:click={modalStore.close} >Nein</Button>
      </div>
    </div>
  </div>
{/if}

<slot />
