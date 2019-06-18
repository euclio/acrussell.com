const CopyPlugin = require('copy-webpack-plugin');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = {
    entry: {
        github_snippets: './static/js/github_snippets.js',
        styles: './styles.scss',
    },
    module: {
        rules: [
            {
                test: /\.s?css$/,
                use: [
                    MiniCssExtractPlugin.loader,
                    'css-loader',
                    'postcss-loader',
                    'sass-loader',
                ],
            },
        ],
    },
    plugins: [
        new MiniCssExtractPlugin(),
        new CopyPlugin([
            { from: './static/images/favicon.ico', to: '.', },
            { from: './static/images/icons', to: 'images/icons' },
            { from: './static/images/photos', to: 'images/photos' },
            { from: './static/images/slideshow', to: 'images/slideshow' },
        ]),
    ],
};
