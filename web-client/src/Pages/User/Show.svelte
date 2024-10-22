<script lang="ts">
  import Layout from "./Layout.svelte";
  import type {UserShowProps} from "../../shared_definitions"

  const {user}: UserShowProps = $props();
</script>

<Layout>
  <p class="underline text-3xl font-bold">
    Hello, there {user.name}
  </p>
</Layout>
