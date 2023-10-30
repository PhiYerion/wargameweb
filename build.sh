#!/bin/bash
# unused-features analyze
export LEPTOS_SITE_ROOT="target/site"
export LEPTOS_SITE_PKG_DIR="pkg"
export LEPTOS_SITE_ADDR="127.0.0.1:3000"
export LEPTOS_RELOAD_PORT="3001"

cargo leptos build --release

mkdir temp
for file in "$LEPTOS_SITE_ROOT/$LEPTOS_SITE_PKG_DIR"/*.wasm; do
	wasm-snip "$file" --snip-rust-fmt-code --snip-rust-panicking-code -o temp/out.snip.wasm
	rm "$file"
	wasm-opt temp/out.snip.wasm -Oz \
	--strip-debug --simplify-globals --vacuum -c --inlining-optimizing --memory-packing --metrics --minify-imports --optimize-casts --optimize-stack-ir --remove-unused-brs --remove-unused-module-elements --simplify-locals-notee --simplify-locals-notee-nostructure --strip-debug --strip-dwarf --strip-eh --strip-producers --strip-target-features -all \
	-o "$file"
done
rm -rf temp

brotli --best "$LEPTOS_SITE_ROOT/$LEPTOS_SITE_PKG_DIR/"*
