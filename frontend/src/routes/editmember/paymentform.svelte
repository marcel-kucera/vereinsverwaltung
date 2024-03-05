<script lang="ts">
  import Heading from "$lib/components/heading.svelte";
  import Input from "$lib/components/input.svelte";
  import Button from "$lib/components/button.svelte";
  import { makePaymentRepo } from "$lib/api/payment";

  let { id } = $props<{ id: number }>();
  let year = $state(new Date().getFullYear());

  let repo = makePaymentRepo(id);
  $effect(() => {
    repo.get();
  });

  async function submit() {
    repo.add(year);
  }

  async function delPayment(id: number) {
    repo.delete(id);
  }
</script>

<div>
  <Heading>Zahlungen:</Heading>
  {#await repo.data}
    loading
  {:then p}
    <div class="border-gray-800 border p-3 m-3 rounded-lg shadow">
      <form on:submit|preventDefault={submit} class="flex flex-col gap-2">
        <Input
          type="number"
          placeholder="Jahr"
          name="year"
          bind:number={year}
        />
        <Button>Hinzufügen</Button>
      </form>
    </div>
    {#if repo.addPromise}
      <p class="m-3">
        {#await repo.addPromise}
          Füge Zahlung hinzu...
        {:then}
          Zahlung hinzugefügt
        {:catch err}
          some error: {err}
        {/await}
      </p>
    {/if}

    <ul class="flex flex-col gap-2">
      {#each p as payment}
        <li class="flex">
          <Button
            on:click={() => delPayment(payment.id)}
            disabled={repo.deleteInflight}
          >
            {payment.id == repo.deleteId ? "..." : "X"}
          </Button>
          <span class="my-auto ml-3">
            {payment.year}
          </span>
        </li>
      {/each}
    </ul>
  {/await}
</div>
