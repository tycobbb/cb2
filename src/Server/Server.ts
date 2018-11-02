import express from "express"
import { Root } from "./Root"

// hot reloading
import webpack from "webpack"
import devMiddleware from "webpack-dev-middleware"
import hotMiddleware from "webpack-hot-middleware"

export class Server {
  private app = express()

  private async start() {
    this.setConfig()
    await this.addHotReloading()
    this.addRoutes()
    this.app.listen(3000)
  }

  private setConfig() {
    this.app.set("views", __dirname)
    this.app.set("view engine",  "ejs")
  }

  private async addHotReloading() {
    // const hotReloading = new HotReloading()
    // this.app.use(hotReloading.middleware)

    const { default: config } = await import("../../webpack.config")
    const compiler = webpack(config)

    this.app.use(devMiddleware(compiler, {
      logLevel:   "warn",
      publicPath: config.output!.publicPath!
    }))

    this.app.use(hotMiddleware(compiler, {
      log:       console.log,
      path:      "/__webpack_hmr",
      heartbeat: 10 * 1000
    }))
  }

  private addRoutes() {
    this.app.get("/", Root.route)
  }

  // bootstrap
  static start() {
    const server = new Server()
    server.start()
  }
}
