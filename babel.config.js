module.exports = function (api) {
  api.cache(false);
  const env = process.env.NODE_ENV;

  return {
    sourceType: "unambiguous",
    ...(env === "test" ? { targets: { node: "current" } } : {}),
    presets: [
      ["@babel/preset-env", { bugfixes: true, shippedProposals: true }],
      "@babel/preset-typescript",
    ],
    plugins: [
      [
        "@babel/plugin-transform-runtime",
        {
          corejs: false,
          helpers: true,
          regenerator: false,
          useESModules: env !== "test",
        },
      ],
      [
        "babel-plugin-polyfill-corejs3",
        {
          method: "usage-global",
          proposals: true,
          shippedProposals: true,
        },
      ],
      ["babel-plugin-polyfill-regenerator", { method: "usage-global" }],
    ],
  };
};
