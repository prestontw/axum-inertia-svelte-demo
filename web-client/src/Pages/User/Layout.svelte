<script lang="ts">
  import { inertia } from '@inertiajs/svelte';
  import type { Snippet } from 'svelte';
  let { children }: { children: Snippet } = $props();
</script>

<main>
  <header>
    <a use:inertia href="/">Home</a>
  </header>
  <article>
    {@render children()}
  </article>
</main>
