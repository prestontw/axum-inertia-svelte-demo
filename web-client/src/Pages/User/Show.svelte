<script lang="ts">
  import Layout from "./Layout.svelte";
  import type {UserShowProps} from "../../shared_definitions"

  const {user}: UserShowProps = $props();
</script>

<Layout>
  <p>
    Hello, there {user.name}
  </p>
</Layout>
