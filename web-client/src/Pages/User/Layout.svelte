<script>
  import { inertia } from "@inertiajs/svelte";
</script>

<main>
  <header>
    <a use:inertia href="/">Home</a>
    <a use:inertia href="/about">About</a>
    <a use:inertia href="/contact">Contact</a>
  </header>
  <article>
    <slot />
  </article>
</main>
