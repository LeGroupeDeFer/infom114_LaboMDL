const path = require('path');
const MiniCssExtractPlugin = require('mini-css-extract-plugin');

module.exports = {
  entry: ['./front/js/index.js', './front/scss/main.scss'],
  output: {
    path: path.resolve(__dirname, 'static'),
    publicPath: '/',
    filename: 'js/bundle.js',
  },
  resolve: {
    extensions: ['.js', '.jsx', '.css', '.sass', '.scss'],
    alias: {
      unanimity: path.resolve(__dirname, 'front/js/'),
    },
  },
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        use: [{ loader: 'babel-loader' }],
      },
      {
        test: /\.s[ac]ss$/i,
        exclude: /node_modules/,
        use: [MiniCssExtractPlugin.loader, 'css-loader', 'sass-loader'],
      },
      {
        test: /\.svg$/,
        use: ['@svgr/webpack'],
      },
    ],
  },
  plugins: [
    new MiniCssExtractPlugin({
      filename: 'css/[name].css',
      chunkFilename: '[id].css',
    }),
  ],
};
