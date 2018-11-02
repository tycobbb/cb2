import webpack from "webpack"

const config: webpack.Configuration = {
  mode: "development",
  context: __dirname,
  entry: [
    "webpack-hot-middleware/client?path=/__webpack_hmr&timeout=20000",
    "./src/Client/index.ts"
  ],
  output: {
    path: __dirname,
    publicPath: "/",
    filename: "bundle.js"
  },
  devtool: "source-map",
  resolve: {
    extensions: [
      ".ts", ".tsx", ".js"
    ]
  },
  module: {
    rules: [{
      test: /\.tsx?$/,
      exclude: /node_modules/,
      use: {
        loader: "babel-loader",
        options: {
          cacheDirectory: true
        }
      }
    }]
  },
  plugins: [
    new webpack.HotModuleReplacementPlugin(),
    new webpack.NoEmitOnErrorsPlugin()
  ]
}

export default config
