<script lang="ts">
  import {
    makeDeletedMemberRepo,
    makeMemberRepo,
    type Member,
  } from "$lib/api/member";
  import { makePaymentRepo } from "$lib/api/payment";
  import { modalStore } from "$lib/stores/modalStore.svelte";
  import Button from "./button.svelte";
  import Heading from "./heading.svelte";

  let { member, updateMemberlist } = $props<{
    member: Member;
    updateMemberlist: () => void;
  }>();

  let paymentRepo = makePaymentRepo(member.id);

  function load() {
    paymentRepo.get();
  }

  function deleteMember() {
    let memberRepo = member.deleted
      ? makeDeletedMemberRepo()
      : makeMemberRepo();

    modalStore.ask(
      `Mitglied ${member.deleted ? "wiederherstellen" : "löschen"}?`,
      `Bist du dir sicher, dass du das Mitglied "${member.firstname} ${member.lastname}" ${member.deleted ? "wiederherstellen" : "löschen"} möchtest?`,
      () => {
        memberRepo.delete(member.id);
        memberRepo.deletePromise?.then(updateMemberlist);
      },
    );
  }
</script>

<details class="bg-gray-800 open:bg-gray-700 hover:bg-gray-700 rounded-md">
  <summary on:click={load} class="cursor-pointer p-2 text-l my-2"
    >{member.firstname + " " + member.lastname}
    {member.paid ? "✅" : "❌"}</summary
  >
  <main class="m-2">
    <Heading>Info:</Heading>
    <ul>
      <li>Id: {member.id}</li>
      <li>Email: {member.email}</li>
      <li>Postleitzahl: {member.plz}</li>
      <li>Stadt: {member.city}</li>
      <li>Straße: {member.street}</li>
      <li>Hausnummer: {member.housenumber}</li>
      <li>IBAN: {member.iban}</li>
      <li>BIC: {member.bic}</li>
      <li>SEPA: {member.sepa ? "erlaubt" : "nicht erlaubt"}</li>
      <li>
        Beitrittsdatum: {new Date(member.joindate).toLocaleDateString("de-DE")}
      </li>
      <li>
        Geburtstag: {new Date(member.birthday).toLocaleDateString("de-DE")}
      </li>
      <li>Mandatsnummber: {member.mandate}</li>
      <li>Beitrag: {member.fee}€</li>
      {#if member.note}
        <li>
          <span>Notiz:</span>
          <pre class="bg-gray-800 p-2 rounded-lg m-2">{member.note}</pre>
        </li>
      {/if}
    </ul>
    <div class="m-4"></div>
    <Heading>Zahlungen:</Heading>
    <div>Dieses Jahr: {member.paid ? "Bezahlt ✅" : "Nicht Bezahlt ❌"}</div>
    <div>Gezahlte Jahre:</div>
    {#await paymentRepo.data}
      Laden...
    {:then payments}
      <ul class="list-disc list-inside">
        {#each payments as payment}
          <li>{payment.year}</li>
        {/each}
      </ul>
    {:catch err}
      <p>Fehler: {err}</p>
    {/await}
    <div class="inline-block">
      <div class="m-4 flex gap-4">
        <Button link href="/editmember?id={member.id}">
          <span>Bearbeiten</span>
        </Button>
        <Button danger on:click={deleteMember}
          >{member.deleted ? "Wiederherstellen" : "Löschen"}</Button
        >
      </div>
    </div>
  </main>
</details>
