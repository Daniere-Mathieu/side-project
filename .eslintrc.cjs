module.exports = {
  root: true,
  env: {
    browser: true,
    es6: true,
    node: true,
  },
  parserOptions: {
    sourceType: "module",
  },
  plugins: ["svelte3"],
  extends: ["eslint:recommended"],
  overrides: [
    {
      files: ["**/*.svelte"],
      processor: "svelte3/svelte3",
    },
  ],
  rules: {},
  settings: {
    "svelte3/ignore-styles": () => true,
  },
};
