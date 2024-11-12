<script lang="ts">
  import Show from './User/Show.svelte';
  import type { UserShowProps } from '../shared_definitions';

  let props: UserShowProps = $props();
</script>

<Show {...props} />
