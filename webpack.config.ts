import WasmPackPlugin from "@wasm-tool/wasm-pack-plugin";
import { BundleAnalyzerPlugin } from "webpack-bundle-analyzer";
import { CleanWebpackPlugin } from "clean-webpack-plugin";
import CopyWebpackPlugin from "copy-webpack-plugin";
import ForkTsCheckerWebpackPlugin from "fork-ts-checker-webpack-plugin";
import HtmlWebpackPlugin from "html-webpack-plugin";
import MiniCssExtractPlugin from "mini-css-extract-plugin";
import path from "path";
import OptimizeCSSAssetsPlugin from "optimize-css-assets-webpack-plugin";
import postCssPresetEnv from "postcss-preset-env";
import PreloadWebpackPlugin from "preload-webpack-plugin";
import TerserPlugin from "terser-webpack-plugin";
import webpack from "webpack";
import WorkboxWebpackPlugin from "workbox-webpack-plugin";

const dist = path.resolve(__dirname, "dist");

const env = process.env.NODE_ENV;
const isEnvProd = env === "production";

const getStyleLoaders = (cssOptions: Object, preProcessor?: string) => {
  const loaders = [
    isEnvProd ? { loader: MiniCssExtractPlugin.loader } : "style-loader",
    {
      loader: "css-loader",
      options: { sourceMap: true, ...cssOptions }
    },
    {
      loader: "postcss-loader",
      options: {
        ident: "postcss",
        plugins: () => [postCssPresetEnv()],
        sourceMap: true
      }
    }
  ];
  if (preProcessor) {
    loaders.push({
      loader: preProcessor,
      options: { sourceMap: true }
    });
  }
  return loaders;
};

const config: webpack.Configuration = {
  entry: "./js/index.ts",
  output: {
    path: dist,
    filename: `[name].${isEnvProd ? "[contenthash]." : ""}js`,
    chunkFilename: `[name].${isEnvProd ? "[contenthash]." : ""}js`
  },
  devServer: {
    compress: true,
    contentBase: dist,
    hot: true,
    overlay: true,
    publicPath: "/",
    watchContentBase: true
  },
  devtool: isEnvProd ? "source-map" : "cheap-module-source-map",
  resolve: {
    extensions: [".wasm", ".ts", ".mjs", ".js", ".json"]
  },
  module: {
    rules: [
      {
        oneOf: [
          {
            test: /\.css$/,
            use: getStyleLoaders({ importLoaders: 1 })
          },
          {
            test: /\.(scss|sass)$/,
            use: getStyleLoaders({ importLoaders: 2 }, "sass-loader")
          },
          {
            test: /\.(js|mjs|ts)$/,
            exclude: /@babel(?:\/|\\{1,2})runtime|core-js|wasm/,
            loader: "babel-loader",
            options: {
              cacheDirectory: true,
              cacheCompression: isEnvProd,
              compact: isEnvProd
            }
          }
        ]
      }
    ]
  },
  optimization: {
    minimizer: [
      new TerserPlugin({
        cache: true,
        parallel: true,
        sourceMap: true
      }),
      new OptimizeCSSAssetsPlugin({
        cssProcessorOptions: {
          map: {
            inline: false,
            annotation: true
          }
        }
      })
    ],
    runtimeChunk: true
  },
  plugins: [
    isEnvProd &&
      new BundleAnalyzerPlugin({ analyzerMode: "static", openAnalyzer: false }),
    new CleanWebpackPlugin(),
    new CopyWebpackPlugin([
      { from: "public", to: ".", ignore: ["index.html"] }
    ]),
    new HtmlWebpackPlugin({ template: "public/index.html" }),
    new ForkTsCheckerWebpackPlugin({
      async: env === "development",
      checkSyntacticErrors: true,
      silent: true,
      useTypescriptIncrementalApi: true
    }),
    isEnvProd &&
      new MiniCssExtractPlugin({
        filename: "[name].[contenthash].css",
        chunkFilename: "[name].[contenthash].css"
      }),
    isEnvProd &&
      new PreloadWebpackPlugin({
        fileBlacklist: [/\.map/, /\.wasm/]
      }),
    new WasmPackPlugin({
      crateDirectory: path.resolve(__dirname, "./")
    }),
    isEnvProd &&
      new WorkboxWebpackPlugin.GenerateSW({
        clientsClaim: true,
        // @ts-ignore
        dontCacheBustURLsMatching: /(\.\w{20}\.|\w{20}\.module\.wasm)/,
        importWorkboxFrom: "local",
        navigateFallback: "/index.html",
        navigateFallbackBlacklist: [
          // Exclude URLs containing a dot, as they're likely a resource in
          // public/ and not a SPA route
          new RegExp("/[^/]+\\.[^/]+$")
        ],
        skipWaiting: true
      })
  ].filter(Boolean)
};

export default config;
