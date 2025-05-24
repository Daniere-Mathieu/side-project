module.exports = {
  root: true,
  env: {
    browser: true,
    es6: true,
    node: true,
  },
  parserOptions: {
    sourceType: "module",
    ecmaVersion: 2020, // Optional: Depending on the ECMAScript version you want to use
    // Specify the TypeScript parser
    parser: "@typescript-eslint/parser",
    // Project setting is required for rules that need type information
    project: ["./tsconfig.json"], // Adjust the path according to your project structure
  },
  plugins: ["svelte3", "@typescript-eslint"], // Include TypeScript plugin
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended", // Use recommended rules from @typescript-eslint/eslint-plugin
    "plugin:@typescript-eslint/recommended-requiring-type-checking", // Optional: For rules that require type information
  ],
  overrides: [
    {
      files: ["**/*.svelte"],
      processor: "svelte3/svelte3",
    },
    {
      // Add a new override for TypeScript files
      files: ["**/*.ts", "**/*.tsx"], // Adjust the glob patterns according to your project structure
      parser: "@typescript-eslint/parser",
      plugins: ["@typescript-eslint"],
      extends: [
        "plugin:@typescript-eslint/recommended",
        "plugin:@typescript-eslint/recommended-requiring-type-checking",
      ],
    },
  ],
  rules: {
    // Your custom rules
  },
  settings: {
    "svelte3/ignore-styles": () => true,
  },
};
