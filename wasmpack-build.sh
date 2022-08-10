wasm-pack build --release --out-dir nodejs_pkg --out-name accrete-node --target nodejs &&
wasm-pack build --release --out-dir web_pkg --out-name accrete --target web;

mkdir pkg;
cp nodejs_pkg/* pkg/;
cp web_pkg/* pkg;
rm -rf nodejs_pkg;
rm -rf web_pkg;