const { CopyRspackPlugin } = require("@rspack/core");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const MonacoWebpackPlugin = require("monaco-editor-webpack-plugin");

const path = require("path");
const isDev = process.env.NODE_ENV === "development";
const dist = path.resolve(__dirname, "../docs");

const featureGPU = process.env.FEATURE_GPU === "1";

/**
 * @type {import('@rspack/core').Configuration}
 */
const rspackConfig = {
  mode: isDev ? "development" : "production",
  entry: {
    index: "./src/index.ts",
    editor: "./src/editor.ts",
  },
  devtool: isDev ? "inline-source-map" : false,
  output: {
    path: dist,
    filename: "[name].js",
  },
  resolve: {
    extensions: [".ts", ".js"],
  },
  experiments: {
    asyncWebAssembly: true,
    syncWebAssembly: true,
  },
  module: {
    rules: [
      {
        test: /\.ts$/,
        loader: "builtin:swc-loader",
        options: {
          jsc: {
            parser: {
              syntax: "typescript",
            },
          },
        },
        type: "javascript/auto",
      },
      {
        test: /\.css$/,
        use: ["style-loader", "css-loader"],
        type: "javascript/auto",
      },
      {
        test: /\.ttf$/,
        type: "asset/resource",
      },
      {
        test: /\.ino$/,
        type: "asset/source",
      },
      {
        test: /\.h$/,
        type: "asset/source",
      },
      {
        test: /\.md$/,
        type: "asset/source",
      },
      {
        test: /\.hex$/,
        type: "asset/source",
      },
      {
        test: /\.bin$/,
        use: ["arraybuffer-loader"],
      },
      {
        test: /\.txt$/,
        type: "asset/source",
      },
    ],
  },
  plugins: [
    new CopyRspackPlugin({
      patterns: [{ from: "static", to: dist }],
    }),

    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "../"),
      extraArgs: featureGPU ? "--features gpu" : "",
      watchDirectories: [
        path.resolve(__dirname, "../../gorilla-physics/src"),
        path.resolve(__dirname, "../../esp32rs/src"),
        path.resolve(__dirname, "static"),
        path.resolve(__dirname, "src"),
      ],
    }),

    new MonacoWebpackPlugin(),
  ],
  stats: {
    warnings: false,
  },
  performance: {
    hints: false,
  },
  cache: isDev ? {
    type: "persistent",
    buildDependencies: [__filename],
  } : false,
};

module.exports = rspackConfig;
