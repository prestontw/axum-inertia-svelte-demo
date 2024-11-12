<script lang="ts">
  import { router } from '@inertiajs/svelte';
  import type { User } from '../../shared_definitions';

  let {
    name: originalName,
    titles: originalTitles,
    id,
  }: { name?: string; titles?: string[]; id?: number } = $props();

  let name = $state(originalName ?? '');
  let titles = $state(originalTitles ?? []);

  function submit(e) {
    e.preventDefault();
    let form: Partial<User> = { name, titles, id };
    if (id) {
      router.put(`/users/${id}`, form);
    } else {
      router.post('/users', form);
    }
  }
  $inspect(titles);
</script>

<form onsubmit={submit}>
  <input type="text" bind:value={name} />
  <!--
   {#if form.errors.email}
  <div class="form-error">{form.errors.email}</div>
  {/if}
    -->
  {#each titles as title, i}
    <input type="text" bind:value={titles[i]} />
  {/each}
  <button
    class="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700"
    onclick={(e) => {
      e.preventDefault();
      titles.push('');
    }}>Add Title</button
  >
  <button type="submit">Submit</button>
</form>
