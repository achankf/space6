import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import path from "path";

const __dirname = path.resolve();

export default {
  entry: "./src/index.tsx",
  devtool: "source-map",
  plugins: [
    new HtmlWebpackPlugin(),
    new WasmPackPlugin({
      crateDirectory: __dirname,
    }),
  ],
  resolve: {
    extensions: [".tsx", ".ts", ".js"],
  },
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: "ts-loader",
        exclude: /node_modules/,
      },
    ],
  },
  experiments: {
    syncWebAssembly: true,
  },
  ignoreWarnings: [
    {
      module: /.\/pkg\/index_bg.js/,
    },
  ],
  performance: { hints: false },
  mode: "development",
};
