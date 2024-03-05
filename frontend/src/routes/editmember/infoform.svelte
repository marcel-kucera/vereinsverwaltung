<script lang="ts">
  import Heading from "$lib/components/heading.svelte";
  import { unixToDatestring } from "$lib/util";
  import Input from "$lib/components/input.svelte";
  import Button from "$lib/components/button.svelte";
  import { makeMemberRepo } from "$lib/api/member";

  let { id } = $props<{ id: number }>();
  let joindate: string = $state("");

  let repo = makeMemberRepo();

  $effect(() => {
    repo.selected.then((m) => (joindate = unixToDatestring(m.joindate)));
  });
  $effect(() => {
    repo.select(id);
  });

  async function submit() {
    let mResolved = await repo.selected;
    mResolved.joindate = new Date(joindate).getTime();
    repo.update(id, mResolved);
  }
</script>

<div>
  <Heading>Info:</Heading>
  {#await repo.selected}
    loading
  {:then m}
    <form on:submit|preventDefault={submit} class="flex flex-col gap-3">
      <Input
        type="text"
        name="firstname"
        bind:text={m.firstname}
        placeholder="Vorname"
      />
      <Input
        type="text"
        name="lastname"
        bind:text={m.lastname}
        placeholder="Nachname"
      />
      <Input
        type="text"
        name="email"
        bind:text={m.email}
        placeholder="E-Mail"
      />
      <Input type="text" name="plz" bind:text={m.plz} placeholder="PLZ" />
      <Input type="text" name="city" bind:text={m.city} placeholder="Stadt" />
      <Input
        type="text"
        name="street"
        bind:text={m.street}
        placeholder="Straße"
      />
      <Input
        type="text"
        name="housenumber"
        bind:text={m.housenumber}
        placeholder="Hausnummer"
      />
      <Input type="text" name="iban" bind:text={m.iban} placeholder="IBAN" />
      <Input type="text" name="bic" bind:text={m.bic} placeholder="BIC" />
      <div class="m-1">
        <label for="sepa">SEPA:</label>
        <input
          type="checkbox"
          name="sepa"
          bind:checked={m.sepa}
          class="bg-gray-600 text-xl scale-[2] ml-3"
        />
      </div>
      <div class="">
        <label for="joindate">Beitrittsdatum:</label>
        <input
          class="bg-gray-700 rounded-lg p-3"
          type="date"
          name="joindate"
          bind:value={joindate}
          required
        />
      </div>
      <div class="flex flex-col gap-1">
        <label for="note">Notiz:</label>
        <textarea
          name="note"
          class="p-2 bg-gray-800 rounded-lg"
          cols="30"
          rows="10"
          bind:value={m.note}
        />
      </div>
      <Button><span class="text-xl">Änderungen übernehmen</span></Button>
    </form>

    {#if repo.updatePromise}
      <p class="m-3">
        {#await repo.updatePromise}
          laden...
        {:then}
          Änderungen übernommen
        {:catch err}
          fehler: {err}
        {/await}
      </p>
    {/if}
  {/await}
</div>
