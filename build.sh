#!/bin/bash
# unused-features analyze
export LEPTOS_SITE_ROOT="target/site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:80"
export LEPTOS_STYLE_FILE="style/main-tailwind.scss"

npx tailwindcss -i ./style/main.scss -o "$LEPTOS_STYLE_FILE"
cargo leptos build --release

mkdir temp
for file in "$LEPTOS_SITE_ROOT/$LEPTOS_SITE_PKG_DIR"/*.wasm; do
	wasm-snip "$file" -o temp/out.snip.wasm
	rm "$file"
	wasm-opt temp/out.snip.wasm -Oz	-o "$file"
done
rm -rf temp

echo "brotli --best $LEPTOS_SITE_ROOT/$LEPTOS_SITE_PKG_DIR/*"
brotli --best "$LEPTOS_SITE_ROOT/$LEPTOS_SITE_PKG_DIR/"*

./target/release/wargameweb
