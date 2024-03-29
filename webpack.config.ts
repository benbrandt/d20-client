import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import { BundleAnalyzerPlugin } from "webpack-bundle-analyzer";
import { CleanWebpackPlugin } from "clean-webpack-plugin";
import CopyWebpackPlugin from "copy-webpack-plugin";
import CssMinimizerPlugin from "css-minimizer-webpack-plugin";
import ForkTsCheckerWebpackPlugin from "fork-ts-checker-webpack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import path from "path";
import postCssPresetEnv from "postcss-preset-env";
import PreloadWebpackPlugin from "preload-webpack-plugin";
import webpack from "webpack";
import WorkboxWebpackPlugin from "workbox-webpack-plugin";
import noopServiceWorkerMiddleware from "./js/noopServiceWorkerMiddleware";

const dist = path.resolve(__dirname, "dist");

const env = process.env.NODE_ENV;
const isEnvProd = env === "production";

const getStyleLoaders = (
  cssOptions: Record<string, unknown>,
  preProcessor?: string
): webpack.RuleSetUse => {
  const loaders = [
    isEnvProd ? { loader: MiniCssExtractPlugin.loader } : "style-loader",
    {
      loader: "css-loader",
      options: { sourceMap: true, ...cssOptions },
    },
    {
      loader: "postcss-loader",
      options: {
        postcssOptions: {
          ident: "postcss",
          plugins: (): unknown[] => [postCssPresetEnv()],
        },
        sourceMap: true,
      },
    },
  ];
  if (preProcessor) {
    loaders.push({
      loader: preProcessor,
      options: { sourceMap: true },
    });
  }
  return loaders;
};

const config: webpack.Configuration = {
  entry: "./js/index.ts",
  experiments: {
    asyncWebAssembly: true,
  },
  output: {
    path: dist,
    filename: `[name].${isEnvProd ? "[contenthash]." : ""}js`,
    chunkFilename: `[name].${isEnvProd ? "[contenthash]." : ""}js`,
  },
  devServer: {
    onBeforeSetupMiddleware(devServer): void {
      devServer.app.use(noopServiceWorkerMiddleware());
    },
    devMiddleware: {
      publicPath: "/",
    },
    static: {
      publicPath: dist,
      watch: true,
    },
    hot: true,
  },
  devtool: isEnvProd ? "source-map" : "cheap-module-source-map",
  resolve: {
    extensions: [".wasm", ".ts", ".mjs", ".js", ".json"],
  },
  module: {
    rules: [
      {
        oneOf: [
          {
            test: /\.css$/,
            use: getStyleLoaders({ importLoaders: 1 }),
          },
          {
            test: /\.(scss|sass)$/,
            use: getStyleLoaders({ importLoaders: 2 }, "sass-loader"),
          },
          {
            test: /\.(js|mjs|ts)$/,
            exclude: /@babel(?:\/|\\{1,2})runtime|core-js|wasm/,
            loader: "babel-loader",
            options: {
              cacheDirectory: true,
              cacheCompression: isEnvProd,
              compact: isEnvProd,
            },
          },
        ],
      },
    ],
  },
  optimization: {
    minimizer: ["...", new CssMinimizerPlugin()],
    runtimeChunk: true,
  },
  plugins: [
    isEnvProd &&
      new BundleAnalyzerPlugin({ analyzerMode: "static", openAnalyzer: false }),
    new CleanWebpackPlugin(),
    new CopyWebpackPlugin({
      patterns: [
        { from: "public", to: ".", globOptions: { ignore: ["**/index.html"] } },
      ],
    }),
    new HtmlWebpackPlugin({ template: "public/index.html" }),
    new ForkTsCheckerWebpackPlugin({ async: env === "development" }),
    isEnvProd &&
      new MiniCssExtractPlugin({
        filename: "[name].[contenthash].css",
        chunkFilename: "[name].[contenthash].css",
      }),
    isEnvProd &&
      new PreloadWebpackPlugin({
        fileBlacklist: [/\.map/, /\.wasm/],
      }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "./"),
      pluginLogLevel: "info",
    }),
    isEnvProd &&
      new WorkboxWebpackPlugin.GenerateSW({
        cleanupOutdatedCaches: true,
        clientsClaim: true,
        navigateFallback: "/index.html",
        navigateFallbackDenylist: [
          // Exclude URLs containing a dot, as they're likely a resource in
          // public/ and not a SPA route
          new RegExp("/[^/]+\\.[^/]+$"),
        ],
        skipWaiting: true,
        sourcemap: true,
      }),
  ].filter(Boolean),
};

export default config;
