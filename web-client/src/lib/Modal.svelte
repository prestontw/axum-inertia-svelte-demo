<script lang="ts">
  import type { Snippet } from 'svelte';
  let {
    showModal = $bindable(),
    header,
    children,
    onclose,
  }: { showModal: boolean; header?: Snippet; children?: Snippet; onclose?: () => void } = $props();

  let dialog: HTMLDialogElement = $state();

  $effect(() => {
    if (showModal) {
      dialog.showModal();
    } else {
      dialog.close();
    }
  });
</script>

<!-- svelte-ignore a11y_click_events_have_key_events, a11y_no_noninteractive_element_interactions -->
<dialog
  bind:this={dialog}
  onclose={() => {
    showModal = false;
    onclose?.();
  }}
  onclick={(e) => {
    if (e.target === dialog) dialog.close();
  }}
>
  <div>
    {@render header?.()}
    <hr />
    {@render children?.()}
    <hr />
    <!-- svelte-ignore a11y_autofocus -->
    <button autofocus onclick={() => dialog.close()}>close modal</button>
  </div>
</dialog>
