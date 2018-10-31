import * as express from "express"
import { Root } from "./Root"

export class Server {
  private app = express()

  private start() {
    this.config()
    this.routes()
    this.app.listen(3000)
  }

  private config() {
    this.app.set('views', __dirname)
    this.app.set("view engine", "ejs")
  }

  private routes() {
    this.app.get("/", Root.route)
  }

  // bootstrap
  static start() {
    const server = new Server()
    server.start()
  }
}
