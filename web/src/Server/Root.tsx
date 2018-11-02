import * as React from "react"
import { renderToString } from "react-dom/server"
import { Request, Response } from "express"
import { App } from "../App"

export class Root {
  static route(_: Request, response: Response) {
    response.render("index", {
      content: renderToString(<App />)
    })
  }
}
