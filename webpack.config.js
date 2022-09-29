const webpack = require("webpack");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const TerserPlugin = require("terser-webpack-plugin");
const CopyPlugin = require("copy-webpack-plugin");

//
const isDevelopment = process.env.NODE_ENV !== "production";

const path = require('path');
const WasmPackPlugin = require('@wasm-tool/wasm-pack-plugin');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const frontendDirectory = "autodox_assets";
const asset_entry = path.join(frontendDirectory, "index.html");

const distPath = path.resolve(__dirname, "dist");

function initCanisterEnv() {
    let localCanisters, prodCanisters;
    try {
        localCanisters = require(path.resolve(
            ".dfx",
            "local",
            "canister_ids.json"
        ));
    } catch (error) {
        console.log("No local canister_ids.json found. Continuing production");
    }
    try {
        prodCanisters = require(path.resolve("canister_ids.json"));
    } catch (error) {
        console.log("No production canister_ids.json found. Continuing with local");
    }

    const network =
        process.env.DFX_NETWORK ||
        (process.env.NODE_ENV === "production" ? "ic" : "local");

    const canisterConfig = network === "local" ? localCanisters : prodCanisters;

    return Object.entries(canisterConfig).reduce((prev, current) => {
        const [canisterName, canisterDetails] = current;
        prev[canisterName.toUpperCase() + "_CANISTER_ID"] =
            canisterDetails[network];
        return prev;
    }, {});
}

const canisterEnvVariables = initCanisterEnv();
module.exports = (env, argv) => {
    return {
        target: "web",
        mode: isDevelopment ? "development" : "production",

        devtool: isDevelopment ? "source-map" : false,
        optimization: {
            minimize: !isDevelopment,
            minimizer: [new TerserPlugin()],
        },
        resolve: {
            extensions: [".js", ".ts", ".jsx", ".tsx"],
            fallback: {
                assert: require.resolve("assert/"),
                buffer: require.resolve("buffer/"),
                events: require.resolve("events/"),
                stream: require.resolve("stream-browserify/"),
                util: require.resolve("util/"),
            },
        },


        devServer: {

            proxy: {
                "/api": {
                    target: "http://localhost:8000",
                    changeOrigin: true,
                    pathRewrite: {
                        "^/api": "/api",
                    },
                },
            },
            hot: true,
            watchFiles: [path.resolve(__dirname,frontendDirectory)],
            liveReload: true,

            contentBase: distPath,
            compress: argv.mode === 'production',
            port: 8080
        },
        entry: {
            // The autodox_assets.entrypoint points to the HTML file for this build, so we need
            // to replace the extension to `.js`.
            index: path.join(__dirname, asset_entry).replace(/\.html$/, ".js"),
        },
        output: {
            webassemblyModuleFilename: "todomvc.wasm",
            filename: "index.js",
            path: path.join(__dirname, "dist", frontendDirectory),
        },

        module: {
            rules: [
                {
                    test: /\.s[ac]ss$/i,
                    use: [
                        'style-loader',
                        'css-loader',
                        'sass-loader',
                    ],
                },
            ],
        },
        plugins: [
            new WasmPackPlugin({
                crateDirectory: ".",
                extraArgs: "--no-typescript",

            }),
            new HtmlWebpackPlugin({
                template: path.join(__dirname, asset_entry),
                cache: false,
            }),

            new webpack.ProvidePlugin({
                Buffer: [require.resolve("buffer/"), "Buffer"],
                process: require.resolve("process/browser"),
            }),
            new CopyWebpackPlugin({
                patterns: [

                    {
                        from: path.join(__dirname, frontendDirectory, "public"),
                        to: path.join(__dirname, frontendDirectory, "dist"),
                    },
                ],
            }),
            new WasmPackPlugin({
                crateDirectory: ".",
                extraArgs: "--no-typescript",
            }),
            new webpack.EnvironmentPlugin({
                NODE_ENV: "development",
                ...canisterEnvVariables,
            }),
        ],
        watch: argv.mode !== 'production'
    };
};


