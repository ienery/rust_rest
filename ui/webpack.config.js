const path = require('path');
const webpack = require('webpack');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const CleanWebpackPlugin = require('clean-webpack-plugin');

module.exports = {
  entry: {
    vendor: ["react", "react-dom"],
    //main: './src/pages/main.tsx',
    index: './src/pages/index.tsx'
  },
  plugins: [
    new webpack.optimize.CommonsChunkPlugin({
        names: ["vendor"],
        minChunks: Infinity
      }),
    //   new webpack.optimize.CommonsChunkPlugin({
    //     name: "common-pages",
    //     chunks: ["main", "index"]
    //   }),
  ],
  module: {
    rules: [
      {
        test: /\.jsx$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['env', 'react']
          }
        }
      },
      {
        test: /\.tsx$/,
        loader: 'awesome-typescript-loader'
      }
    ],
    
  },
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx']
  },
  output: {
    filename: '[name].bundle.js',
    chunkFilename: '[name].bundle.js',
    path: path.resolve(__dirname, 'out/assets/js/'),
    publicPath: "/assets/js/"
  }
};