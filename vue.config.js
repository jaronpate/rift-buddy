let webpack = require("webpack");

module.exports = {
  chainWebpack: config => {
    if (process.platform !== "darwin") {
        config.plugin('IgnorePlugin').use(webpack.IgnorePlugin, [{ resourceRegExp: /^fsevents$/ }])
    }
  }
}