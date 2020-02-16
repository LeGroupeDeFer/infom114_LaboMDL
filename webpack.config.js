module.exports = {
  entry: './front/js/index.js',
  module: {
    rules: [
      {
        test: /\.(js|jsx)$/,
        exclude: /node_modules/,
        use: [
          { loader: "babel-loader" }
        ]
      },
    ]
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