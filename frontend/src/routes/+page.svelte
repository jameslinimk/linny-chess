<script lang="ts">
	import { get_attribute_infos } from "linnychess"
	import { onMount } from "svelte"
	onMount(() => {
		console.log(get_attribute_infos())
	})
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
