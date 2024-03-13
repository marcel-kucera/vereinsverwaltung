<script lang="ts">
  import { unixToDatestring } from "$lib/util";
  import Input from "$lib/components/input.svelte";
  import Button from "$lib/components/button.svelte";
  import MdWrapper from "$lib/components/mdWrapper.svelte";
  import { goto } from "$app/navigation";
  import { makeMemberRepo, type MemberNew } from "$lib/api/member";

  let m: MemberNew = $state({}) as MemberNew;
  let joindate = $state(unixToDatestring(new Date().getTime()));
  let birthday = $state("");

  let repo = makeMemberRepo();
  function submit() {
    m.joindate = new Date(joindate).getTime();
    m.birthday = new Date(birthday).getTime();
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
    <Input
      name="lastname"
      bind:text={m.lastname}
      placeholder="Nachname"
      required
    />
    <Input name="email" bind:text={m.email} placeholder="E-Mail" required />
    <Input name="plz" bind:text={m.plz} placeholder="PLZ" required />
    <Input name="city" bind:text={m.city} placeholder="Stadt" required />
    <Input name="street" bind:text={m.street} placeholder="Straße" required />
    <Input
      name="housenumber"
      bind:text={m.housenumber}
      placeholder="Hausnummer"
      required
    />
    <Input name="iban" bind:text={m.iban} placeholder="IBAN" required />
    <Input name="bic" bind:text={m.bic} placeholder="BIC" required />
    <div class="m-1">
      <label for="sepa">SEPA:</label>
      <input
        type="checkbox"
        name="sepa"
        bind:checked={m.sepa}
        class="bg-gray-600 text-xl scale-[2] m-2"
      />
    </div>
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

    <div>
      <label for="joindate">Geburtsdatum:</label>
      <input
        class="bg-gray-700 rounded-lg p-3"
        type="date"
        name="birthday"
        bind:value={birthday}
        required
      />
    </div>
    <Input
      type="text"
      name="mandate"
      bind:text={m.mandate}
      placeholder="Mandatsnummber"
    />
    <Input
      type="number"
      name="fee"
      bind:number={m.fee}
      placeholder="Beitrag"
      step="0.01"
    />
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
