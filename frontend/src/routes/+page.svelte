<script lang="ts">
  import { makeDeletedMemberRepo, makeMemberRepo } from "$lib/api/member";
  import Accordion from "$lib/components/accordion.svelte";
  import Button from "$lib/components/button.svelte";
  import MdWrapper from "$lib/components/mdWrapper.svelte";

  let show_deleted = $state(false);
  let repo = $state(makeMemberRepo());

  $effect(() => {
    repo = show_deleted ? makeDeletedMemberRepo() : makeMemberRepo();
  });
  $effect(() => {
    repo.get();
  });
</script>

<MdWrapper>
  <div class="my-4 flex gap-4">
    <Button href="/addmember" link>Neues Mitglied</Button>
    <Button danger href="/logout" link>Logout</Button>
  </div>

  <div>
    <Button on:click={() => (show_deleted = !show_deleted)}>
      <span>Gelöschte Mitglieder anzeigen</span>
      <input
        type="checkbox"
        class="cursor-pointer"
        bind:checked={show_deleted}
      />
    </Button>
  </div>

  <div>
    {#await repo.data}
      <div>loading...</div>
    {:then members}
      {#each members as m}
        <Accordion member={m} updateMemberlist={() => repo.get()} />
      {/each}
    {:catch error}
      <div>{error}</div>
    {/await}
  </div>
</MdWrapper>

<style>
</style>
