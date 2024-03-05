<script lang="ts">
  let { children } = $props<{ children: any }>();
</script>

<div class="flex h-screen">
  <div class="m-auto">
    {@render children()}
  </div>
</div>
