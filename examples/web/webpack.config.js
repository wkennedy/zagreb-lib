const path = require('path');
const CopyWebpackPlugin = require('copy-webpack-plugin');
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

module.exports = {
    entry: './index.js',
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: 'index.js',
    },
    mode: 'development',
    plugins: [
        new CopyWebpackPlugin({
            patterns: [
                { from: './index.html', to: './' },
            ],
        }),
        new WasmPackPlugin({
            crateDirectory: path.resolve(__dirname, '../../'),
            outDir: path.resolve(__dirname, 'dist'),
        }),
    ],
    experiments: {
        asyncWebAssembly: true,
        syncWebAssembly: true,
    },
    module: {
        rules: [
            {
                test: /\.wasm$/,
                type: "webassembly/async",
            }
        ],
    },
    devServer: {
        contentBase: path.resolve(__dirname, 'dist'),
        compress: true,
        port: 8080,
    },
};