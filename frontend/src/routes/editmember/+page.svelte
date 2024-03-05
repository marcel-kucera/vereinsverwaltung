<script lang="ts">
  import { page } from "$app/stores";
  import Button from "$lib/components/button.svelte";
  import MdWrapper from "$lib/components/mdWrapper.svelte";
  import Files from "./files.svelte";
  import Infoform from "./infoform.svelte";
  import Paymentform from "./paymentform.svelte";

  let id: number | undefined = $state();

  // this has to be an effect, because searchparams are not updated instantly
  $effect(() => {
    let idString = $page.url.searchParams.get("id");
    id = idString == null ? NaN : parseInt(idString);
  });
</script>

<MdWrapper>
  <div class="my-4">
    <Button link href="/">Zurück</Button>
  </div>
  {#if id == undefined}
    loading
  {:else if isNaN(id)}
    pageerror
  {:else}
    <div class="flex flex-col gap-12">
      <Infoform {id} />
      <Paymentform {id} />
      <Files {id} />
    </div>
  {/if}
</MdWrapper>
