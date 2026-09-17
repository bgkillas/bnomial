wasm:
    wasm-pack build --no-opt --out-dir www/pkg --target web --profile release
    wasm-opt -O4 -all -o www/pkg/bnomial_lib_bg.wasm www/pkg/bnomial_lib_bg.wasm
run_wasm:
    cd www && python3 -m http.server 8080
