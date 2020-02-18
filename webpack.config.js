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
  resolve: {
    extensions: ['.js', '.jsx', '.css', '.sass'],
  },
  output: {
    path: __dirname + '/static/js',
    publicPath: '/',
    filename: 'bundle.js',
  },
  devServer: {
    contentBase: './static/js'
  }
};
