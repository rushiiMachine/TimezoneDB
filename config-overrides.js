const path = require('path');

module.exports = {
    paths: function (paths, env) {
        paths.proxySetup = path.resolve(__dirname, 'setupDevProxy.js');
        paths.appIndexJs = path.resolve(__dirname, 'src-ui/index.tsx');
        paths.appSrc = path.resolve(__dirname, 'src-ui');
        return paths;
    },
}
