const path = require('path');
const webpack = require('webpack');
//const HtmlWebpackPlugin = require('html-webpack-plugin');
//const CleanWebpackPlugin = require('clean-webpack-plugin');

module.exports = {
  entry: {
    preload: './src/pages/preload.tsx',
    index: './src/pages/index.tsx'
  },
  watch: false,
  plugins: [
    // new config.optimization.splitChunks({
    //     names: ["vendor"],
    //     minChunks: Infinity
    //   }),
    //   new webpack.optimize.CommonsChunkPlugin({
    //     name: "common-pages",
    //     chunks: ["main", "index"]
    //   }),
  ],
  module: {
    rules: [
      {
        test: /\.js(x?)$/,
        exclude: /(node_modules|bower_components)/,
        use: {
          loader: 'babel-loader',
          options: {
            presets: ['env', 'react']
          }
        }
      },
      {
        test: /\.ts(x?)$/,
        loader: 'awesome-typescript-loader'
      },
      {
        test: /\.css$/,
        use: [ 'style-loader', 'css-loader' ]
      }
    ],
    
  },
  resolve: {
    extensions: ['.js', '.jsx', '.ts', '.tsx']
  },
  optimization: {
      splitChunks: {
            chunks: "async",
            minSize: 30000,
            minChunks: 1,
            maxAsyncRequests: 5,
            maxInitialRequests: 3,
            name: true,
            cacheGroups: {
                commons: {
                    test: /[\\/]node_modules[\\/]/,
                    name: "vendors",
                    chunks: "all"
                }
      }
    }
  },
  output: {
    filename: '[name].bundle.js',
    chunkFilename: '[name].bundle.js',
    path: path.resolve(__dirname, 'out/assets/js/bundles/'),
    publicPath: "/assets/js/bundles/"
  }
};