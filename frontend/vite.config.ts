import { sveltekit } from "@sveltejs/kit/vite"
import { defineConfig } from "vite"
import topLevelAwait from "vite-plugin-top-level-await"
import wasm from "vite-plugin-wasm"
import { viteInjector } from "./socketInjector"

export default defineConfig({
	plugins: [sveltekit(), wasm(), topLevelAwait(), viteInjector],
})
