
module.exports = {
    mode: "production",
    devtool: "source-map",
    resolve: {
        extensions: [".ts", ".tsx", ".js", ".jsx"],
    },
    module: {
        rules: [
            {
                test: /\.ts(x?)$/,
                exclude: /node_modules/,
                use: [
                    {
                        loader: "ts-loader",
                    }
                ]
            },
        ]
    },
    output: {
        path: __dirname + "/public/dist/",
        filename: "bundle.js",
    }
};
