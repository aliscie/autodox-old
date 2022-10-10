const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

module.exports = {
    mode: "development",
    entry: "./index.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "[name].bundle.js",
        clean : true
    },
    plugins: [
        new HtmlWebpackPlugin({ template: "./index.html" }),

        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, "."),
        }),
        new CopyPlugin({
            patterns: [
                {  from : "./public", to : "public"} ,
                {  from : "./icons", to : "icons"} 
            ],
        }),
    ],
    experiments: {
        syncWebAssembly: true,
    },
    devServer:{
        port : 8080
    }
};
