path = require('path');

module.exports = {
  entry: './front/js/index.js',
  module: {
    rules: [
      {
        test: /\.jsx?$/,
        exclude: /node_modules/,
        use: [
          { loader: "babel-loader" }
        ]
      },
      {
        test: /\.s[ac]ss$/i,
        exclude: /node_modules/,
        use: [
          'style-loader',
          'css-loader',
          'sass-loader'
        ]
      }
    ]
  },
  devtool: 'eval-source-map',
  resolve: {
    extensions: ['.js', '.jsx', '.css', '.sass'],
    alias: {
      'unanimity': path.resolve(__dirname, 'front/js/')
    }
  },
  output: {
    path: __dirname + '/static/js',
    publicPath: '/js/',
    filename: 'bundle.js',
  },
  devServer: {
    contentBase: './static/js'
  }
};
