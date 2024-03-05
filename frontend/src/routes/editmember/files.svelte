<script lang="ts">
  import { fileLink, makeFileRepo } from "$lib/api/file";
  import Button from "$lib/components/button.svelte";
  import Heading from "$lib/components/heading.svelte";
  import { modalStore } from "$lib/stores/modalStore.svelte";

  let { id } = $props<{ id: number }>();
  let fileForm: FileList | undefined = $state();

  let repo = makeFileRepo(id);
  $effect(() => {
    repo.get();
  });

  async function submit() {
    if (fileForm && fileForm[0]) {
      repo.add(fileForm[0]);
    }
  }

  function delFile(id: number, name: string) {
    modalStore.ask(
      "Datei löschen",
      `Willst du wirklich die Datei "${name}" löschen?`,
      () => {
        repo.delete(id);
      },
    );
  }
</script>

<div class="mb-10">
  <Heading>Dateien:</Heading>

  <div class="border-gray-800 border p-3 m-3 rounded-lg shadow">
    <form on:submit|preventDefault={submit} class="flex flex-col gap-2">
      <input
        type="file"
        bind:files={fileForm}
        class="bg-gray-700 p-2 rounded-lg"
      />
      <Button>Hochladen</Button>
    </form>
  </div>

  {#if repo.addPromise}
    <p class="m-3">
      {#await repo.addPromise}
        Läd hoch...
      {:then}
        Hochgeladen
      {:catch err}
        error: {err}
      {/await}
    </p>
  {/if}

  {#await repo.data}
    loading
  {:then list}
    {#if list.length > 0}
      <ul class="flex flex-col gap-2">
        {#each list as entry}
          <li class="flex">
            <Button
              on:click={() => delFile(entry.id, entry.name)}
              disabled={repo.deleteInflight}
              >{entry.id == repo.deleteId ? "..." : "X"}
            </Button>
            <a href={fileLink(entry.id)} class="my-auto ml-3">
              {entry.name}
            </a>
          </li>
        {/each}
      </ul>
    {:else}
      <p>Keine Dateien</p>
    {/if}
  {:catch err}
    <p>some error: {err}</p>
  {/await}
</div>
