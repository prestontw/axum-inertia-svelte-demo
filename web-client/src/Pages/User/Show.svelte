<script lang="ts">
  import Layout from './Layout.svelte';
  import type { UserShowProps } from '../../shared_definitions';

  const { user }: UserShowProps = $props();
</script>

<Layout>
  <p class="text-3xl font-bold underline">
    Hello, there {user.name}
  </p>
</Layout>
