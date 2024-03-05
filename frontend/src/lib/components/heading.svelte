<script lang="ts">
  let { children } = $props<{ children: any }>();
</script>

<h2 class="text-3xl font-bold my-1">
  {@render children()}
</h2>
