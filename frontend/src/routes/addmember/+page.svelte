<script lang="ts">
  import { unixToDatestring } from "$lib/util";
  import Input from "$lib/components/input.svelte";
  import Button from "$lib/components/button.svelte";
  import MdWrapper from "$lib/components/mdWrapper.svelte";
  import { goto } from "$app/navigation";
  import { makeMemberRepo, type MemberNew } from "$lib/api/member";

  let m: MemberNew = $state({}) as MemberNew;
  let joindate = $state(unixToDatestring(new Date().getTime()));

  let repo = makeMemberRepo();
  function submit() {
    m.joindate = new Date(joindate).getTime();
    repo.add(m);
    repo.addPromise?.then(() => {
      goto("/");
    });
  }
</script>

<MdWrapper>
  <div class="m-3">
    <Button href="/" link>Zurück</Button>
  </div>

  <form on:submit|preventDefault={submit} class="flex flex-col gap-3 mb-20">
    <Input
      name="firstname"
      bind:text={m.firstname}
      placeholder="Vorname"
      required
    />
    <br />
    <Input
      name="lastname"
      bind:text={m.lastname}
      placeholder="Nachname"
      required
    />
    <br />
    <Input name="email" bind:text={m.email} placeholder="E-Mail" required />
    <br />
    <Input name="plz" bind:text={m.plz} placeholder="PLZ" required />
    <br />
    <Input name="city" bind:text={m.city} placeholder="Stadt" required />
    <br />
    <Input name="street" bind:text={m.street} placeholder="Straße" required />
    <br />
    <Input
      name="housenumber"
      bind:text={m.housenumber}
      placeholder="Hausnummer"
      required
    />
    <br />
    <Input name="iban" bind:text={m.iban} placeholder="IBAN" required />
    <br />
    <Input name="bic" bind:text={m.bic} placeholder="BIC" required />
    <br />
    <div class="m-1">
      <label for="sepa">SEPA:</label>
      <input
        type="checkbox"
        name="sepa"
        bind:checked={m.sepa}
        class="bg-gray-600 text-xl scale-[2] m-2"
      />
    </div>
    <br />
    <div class="">
      <label for="joindate">Beitrittsdatum:</label>
      <input
        class="bg-gray-700 rounded-lg p-3 m-2"
        type="date"
        name="joindate"
        bind:value={joindate}
        required
      />
    </div>
    <br />
    <Button><span class="text-xl">Hinzufügen</span></Button>
    {#if repo.addPromise}
      <p class="m-3">
        {#await repo.addPromise}
          hinzufügen...
        {:then}
          Mitglied hinzugefügt
        {:catch err}
          some error: {err}
        {/await}
      </p>
    {/if}
  </form>
</MdWrapper>
