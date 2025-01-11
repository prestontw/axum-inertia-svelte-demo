<script lang="ts">
  import { useForm } from '@inertiajs/svelte';
  import type { User } from '../../shared_definitions';
  import { userIdRoute, usersRoute } from '../../routes';

  let { user: originalUser, onsubmit }: { user?: User; onsubmit?: () => void } = $props();

  let form = useForm(originalUser ? { ...originalUser } : { name: '', titles: [] });

  function submit(e: SubmitEvent) {
    e.preventDefault();
    if ('id' in $form) {
      $form.put(userIdRoute({ id: $form.id }));
    } else {
      $form.post(usersRoute({}));
    }
    onsubmit?.();
  }
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
      $form.titles = [...$form.titles, ''];
    }}>Add Title</button
  >
  <button type="submit">Submit</button>
</form>
