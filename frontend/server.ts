import express from "express"
import { createServer } from "http"
import { handler } from "./build/handler.js"
import { injectSIO } from "./src/lib/server/socket.js"

const app = express()
const server = createServer(app)

injectSIO(server)
app.use(handler)

server.listen(3000, () => {
	console.log("Running on http://localhost:3000")
})
