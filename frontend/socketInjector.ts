import type { PluginOption } from "vite"
import { injectSIO } from "./src/lib/server/socket"

export const viteInjector: PluginOption = {
	name: "socket.io injector",
	configureServer(server) {
		if (server.httpServer) injectSIO(server.httpServer)
	},
}
