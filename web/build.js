require("esbuild").buildSync({
  entryPoints: ["src/index.tsx"],
  bundle: true,
  minify: true,
  sourcemap: true,
  target: ["chrome88", "firefox85", "safari14", "edge88"],
  outfile: "dist/index.js",
  define: {
    "process.env.NODE_ENV": '"production"',
    "process.env": '{"NODE_ENV": "production"}',
  },
});
