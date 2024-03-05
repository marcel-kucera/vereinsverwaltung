<script lang="ts">
  let { children } = $props<{ children: any }>();
</script>

<div class="flex">
  <div class="max-w-screen-md m-auto flex-1">
    {@render children()}
  </div>
</div>
