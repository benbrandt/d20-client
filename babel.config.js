module.exports = function(api) {
  api.cache(false);

  const env = process.env.NODE_ENV;
  const presets = [
    env === "test"
      ? [
          "@babel/preset-env",
          {
            corejs: 3,
            shippedProposals: true,
            targets: {
              node: "current"
            },
            useBuiltIns: "usage"
          }
        ]
      : [
          "@babel/preset-env",
          {
            corejs: 3,
            modules: false,
            shippedProposals: true,
            useBuiltIns: "usage"
          }
        ],
    "@babel/preset-typescript"
  ];
  const plugins = [
    "@babel/plugin-syntax-dynamic-import",
    [
      "@babel/plugin-transform-runtime",
      {
        regenerator: false,
        useESModules: env !== "test"
      }
    ]
  ];

  return {
    sourceType: "unambiguous",
    presets,
    plugins
  };
};
