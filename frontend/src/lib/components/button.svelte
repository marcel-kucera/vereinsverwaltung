<script lang="ts">
  let {
    href,
    link = false,
    confirm = false,
    danger = false,
    highlight = false,
    children,
    ...rest
  } = $props<{
    link?: boolean;
    href?: string;
    danger?: boolean;
    confirm?: boolean;
    highlight?: boolean;
    children: any;
    [key: string]: any;
  }>();

  function getColors() {
    if (danger) {
      return "bg-red-800 hover:bg-red-600";
    } else if (confirm) {
      return "bg-green-800 hover:bg-green-600";
    } else if (highlight){
      return "bg-gray-700 hover:bg-gray-500"
    } else {
      return "bg-gray-800 hover:bg-gray-600";
    }
  }
</script>

<svelte:element this={link ? "a" : "div"} {href}>
  <button
    class={"cursor-pointer inline-block p-2 rounded-lg shadow-lg min-w-16" +
      " " +
      getColors()}
    on:click
    {...rest}
  >
    {@render children()}
  </button>
</svelte:element>
