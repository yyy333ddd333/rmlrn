const path = require('path');
//const webpack = require('webpack')
const CleanWebpackPlugin = require('clean-webpack-plugin')

module.exports = {
  entry: {
    lib: './src/index.ts',
  },
  output: {
    path: path.resolve(__dirname, 'dist'),
    filename: 'bundle.js',
  },
  devtool: 'inline-source-map',
  plugins: [
    new CleanWebpackPlugin(['dist'], {
      exclude: 'index.html'
    }),
//    new webpack.ProvidePlugin({
//      _: 'lodash'
//    })
  ],
  module: {
    rules: [
      {
        test: /\.tsx?$/,
        use: 'ts-loader',
        exclude: /node_modules/
      }
//      {
//        test: require.resolve('./src/globals.js'),
//        use: 'exports-loader?file,parse=helpers.parse'
//      },
//      {
//        test: require.resolve('./src/index.js'),
//        use: 'imports-loader?this=>window'
//      },
    ]
  },
  resolve: {
    extensions: [ '.tsx', '.ts', '.js' ]
  },
//  externals: {
//    lodash: {
//      commonjs: 'lodash',
//      commonjs2: 'lodash',
//      amd: 'lodash',
//      root: '_'
//    }
//  },
  mode: 'development'
};
