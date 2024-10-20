<script lang="ts">
  import Layout from "./Layout.svelte";

  const {user}: {user: {name: string}} = $props();
</script>

<Layout>
  <p>
    Hello, there {user.name}
  </p>
</Layout>
