const path = require('path');
const CleanWebpackPlugin = require('clean-webpack-plugin')

module.exports = {
  entry: {
    lib: './src/index.js',
    // test: './src/test.js',
  },
    // ['./src/index.js', './src/test.js'],
    // "./src/index.js",
  output: {
    path: path.resolve(__dirname, 'dist'),
    // filename: 'webpack-numbers.js'
    filename: '[name].bundle.js',
    libraryTarget: 'umd',
    // libraryExport: 'default',
    library: 'webpackNumbers'
  },
  devtool: 'inline-source-map',
  plugins: [
    new CleanWebpackPlugin(['dist'], {
      exclude: 'index.html'
    })
  ],
  externals: {
    lodash: {
      commonjs: 'lodash',
      commonjs2: 'lodash',
      amd: 'lodash',
      root: '_'
    }
  },
  mode: 'development'
};
