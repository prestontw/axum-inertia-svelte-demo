<script>
  import Layout from './Layout.svelte'

  export let user
</script>

<Layout>
<p>
Hello, there {user.name}
</p>
</Layout>
