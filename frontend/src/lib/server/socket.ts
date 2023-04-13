import type { Server as HttpServer } from "http"
import { Server } from "socket.io"

export interface ServerToClientEvents {
	noArg: () => void
	basicEmit: (a: number, b: string, c: Buffer) => void
	withAck: (d: string, callback: (e: number) => void) => void
}

export interface ClientToServerEvents {
	hello: () => void
}

interface InterServerEvents {
	ping: () => void
}

interface SocketData {
	name: string
	age: number
}

export const injectSIO = (server: HttpServer) => {
	const io = new Server<ClientToServerEvents, ServerToClientEvents, InterServerEvents, SocketData>(server)

	io.on("connection", (socket) => {
		console.log("SocketIO connected")

		socket.on("disconnect", () => {
			console.log("SocketIO disconnected")
		})
	})

	console.log("SocketIO injected")
}
