<script lang="ts">
  import { router } from '@inertiajs/svelte';
  import type { NewUser, User } from '../../shared_definitions';
  import { userIdRoute, usersRoute } from '../../routes';

  let { user: originalUser, onsubmit }: { user?: User; onsubmit?: () => void } = $props();

  let user: User | NewUser = $state(originalUser ?? { name: '', titles: [] });

  function submit(e) {
    e.preventDefault();
    if ('id' in user) {
      router.put(userIdRoute({ id: user.id }), user);
    } else {
      router.post(usersRoute({}), user);
    }
    onsubmit?.();
  }
</script>

<form onsubmit={submit}>
  <input type="text" bind:value={user.name} />
  <!--
   {#if form.errors.email}
  <div class="form-error">{form.errors.email}</div>
  {/if}
    -->
  {#each user?.titles as _, i}
    <input type="text" bind:value={user.titles[i]} />
  {/each}
  <button
    class="rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700"
    onclick={(e) => {
      e.preventDefault();
      user.titles.push('');
    }}>Add Title</button
  >
  <button type="submit">Submit</button>
</form>
