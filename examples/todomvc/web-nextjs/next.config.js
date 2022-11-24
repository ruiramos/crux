const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const path = require("path");

/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: false,
  swcMinify: true,
};

// see https://github.com/wasm-tool/wasm-pack-plugin/issues/112
let loaded = false;

module.exports = {
  webpack(nextConfig) {
    if (!loaded) {
      nextConfig.plugins.push(
        new WasmPackPlugin({
          crateDirectory: path.resolve(__dirname, "..", "shared"),
          extraArgs: "--target web",
          outDir: path.resolve(__dirname, "shared", "core"),
        })
      );
      loaded = true;
    }

    return nextConfig;
  },
};
