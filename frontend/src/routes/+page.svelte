<script lang="ts">
	import { onMount } from "svelte"
	import init, { get_fen, set_game } from "../../pkg"

	onMount(async () => {
		await init()
		set_game()
		console.log(get_fen())
	})
</script>

<h1>Welcome to SvelteKit</h1>
<p>Visit <a href="https://kit.svelte.dev">kit.svelte.dev</a> to read the documentation</p>
