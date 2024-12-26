<script lang="ts">
  import { useForm } from '@inertiajs/svelte';
  import type { User } from '../../shared_definitions';
  import { userIdRoute, usersRoute } from '../../routes';

  let {
    name: originalName,
    titles: originalTitles,
    id,
    onsubmit,
  }: { name?: string; titles?: string[]; id?: number; onsubmit?: () => void } = $props();

  const form = useForm({ name: originalName ?? '', titles: originalTitles ?? [], id });

  function submit(e) {
    e.preventDefault();

    if (id) {
      $form.put(userIdRoute({ id }));
    } else {
      $form.post(usersRoute({}));
    }
    onsubmit?.();
  }
  $inspect($form);
</script>

<form onsubmit={submit}>
  <input type="text" bind:value={$form.name} />
  <!--
   {#if form.errors.email}
  <div class="form-error">{form.errors.email}</div>
  {/if}
    -->
  {#each $form.titles as _, i}
    <input type="text" bind:value={$form.titles[i]} />
  {/each}
  <button
    class="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700"
    onclick={(e) => {
      e.preventDefault();
      $form.titles.push('');
    }}>Add Title</button
  >
  <button type="submit">Submit</button>
</form>
