module.exports = function (api) {
  api.cache(false);
  const env = process.env.NODE_ENV;

  return {
    sourceType: "unambiguous",
    presets: [
      env === "test"
        ? [
            "@babel/preset-env",
            {
              bugfixes: true,
              corejs: { version: 3, proposals: true },
              shippedProposals: true,
              targets: {
                node: "current",
              },
              useBuiltIns: "usage",
            },
          ]
        : [
            "@babel/preset-env",
            {
              bugfixes: true,
              corejs: { version: 3, proposals: true },
              modules: false,
              shippedProposals: true,
              useBuiltIns: "usage",
            },
          ],
      "@babel/preset-typescript",
    ],
    plugins: [
      [
        "@babel/plugin-transform-runtime",
        {
          regenerator: false,
          useESModules: env !== "test",
        },
      ],
    ],
  };
};
