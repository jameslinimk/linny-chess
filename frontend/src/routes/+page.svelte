<script lang="ts">
	import { onMount } from "svelte"
	import init, { get_attribute_infos } from "../../../rust/pkg/chessv2_rust.js"

	onMount(async () => {
		await init()
		const infos = get_attribute_infos()
		console.log(infos)
	})
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
