<script lang="ts">
  import Layout from './Layout.svelte';
  import Modal from '../../lib/Modal.svelte';
  import Form from './Form.svelte';
  import type { UserShowProps } from '../../shared_definitions';

  const { users }: UserShowProps = $props();
  let selected: number | undefined = $state();
  let selectedUser = $derived(users.find((user) => user.id === selected));
  let addingUser = $state(false);
</script>

<Layout>
  {#each users as user}
    <div class="flex">
      <p class="flex-auto text-3xl font-bold underline">
        Hello, there {user.name}
        ({user.titles.join(' ')})
      </p>
      <button
        class="flex-auto rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700"
        onclick={() => (selected = user.id)}>Edit</button
      >
    </div>
  {/each}
  {#key selected}
    <Modal showModal={selected !== undefined} onclose={() => (selected = undefined)}>
      {#snippet header()}
        <h1>Edit Contact</h1>
      {/snippet}
      <Form user={selectedUser} onsubmit={() => (selected = undefined)} /></Modal
    >
  {/key}
  <button
    class="flex-auto rounded bg-blue-500 px-4 py-2 font-bold text-white hover:bg-blue-700"
    onclick={() => (addingUser = true)}>Add User</button
  >
  <Modal bind:showModal={addingUser}>
    {#snippet header()}
      <h1>Add Contact</h1>
    {/snippet}
    <Form onsubmit={() => (addingUser = false)} />
  </Modal>
</Layout>
