<script lang="ts">
  let { name, placeholder, text=$bindable(), type, number=$bindable(), ...rest } = $props<{
    name: string;
    placeholder: string;
    type?: string;
    text?: string;
    number?: number;
    [key: string]: any;
  }>();

  // some code for validation if that should ever be useful
  // let touched = $state(false);
  // let blurred = $state(false);
  // let showValidation = $derived(touched && blurred);

  // class:invalid:outline-red-500={showValidation}
  // class:invalid:outline={showValidation}
  // on:focus={() => (touched = true)}
  // on:blur={() => (blurred = true)}
  let inputClasses = "peer bg-gray-700 p-4 rounded w-full";
</script>

<div class="relative text-gray-50">
  {#if type == "number"}
    <input
      type="number"
      {name}
      {placeholder}
      {...rest}
      bind:value={number}
      class={inputClasses}
    />
  {:else}
    <input
      type={type}
      {name}
      {placeholder}
      {...rest}
      bind:value={text}
      class={inputClasses}
    />
  {/if}
  <label
    for={name}
    class="text-sm absolute top-0 left-0 px-4 text-gray-500 visible peer-placeholder-shown:invisible"
    >{placeholder}</label
  >
</div>
