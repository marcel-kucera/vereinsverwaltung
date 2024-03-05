<script lang="ts">
  import { makeMemberRepo } from "$lib/api/member";
  import Accordion from "$lib/components/accordion.svelte";
  import Button from "$lib/components/button.svelte";
  import MdWrapper from "$lib/components/mdWrapper.svelte";

  let repo = makeMemberRepo();

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
