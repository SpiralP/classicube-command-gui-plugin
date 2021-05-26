const path = require("path");

module.exports = {
  root: true,
  ignorePatterns: ["*.js"],
  parser: "@typescript-eslint/parser",
  parserOptions: {
    project: path.join(__dirname, "tsconfig.json"),
  },
  plugins: ["@typescript-eslint"],
  extends: [
    "eslint:recommended",
    "plugin:@typescript-eslint/recommended",

    "airbnb-typescript",

    "@blueprintjs/eslint-config",
    "plugin:@blueprintjs/recommended",

    "prettier",

    "plugin:react/recommended",
    "plugin:react-hooks/recommended",
  ],
  rules: {
    "header/header": 0,
    "import/order": 0,
    "import/prefer-default-export": 0,
    "no-console": 1,

    "@typescript-eslint/explicit-module-boundary-types": 0,
    "react/require-default-props": 0,
    "react/jsx-boolean-value": [2, "never"],

    "@typescript-eslint/tslint/config": 0,
    "react/jsx-no-bind": 0,
  },
};
