alias dbg:= debug
alias rel:= release
alias dist:= distribution
alias wasm:= wasm-release

debug:
    cargo watch -x "run --features bevy/dynamic_linking"

release:
    cargo run --profile release

wasm-release:
    cargo build --release --target wasm32-unknown-unknown
    wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "card_game" \
    ./target/wasm32-unknown-unknown/release/card_game.wasm

distribution:
    cargo run --profile distribution \
#     -F tracing/release_max_level_off -F log/release_max_level_off
# uncomment after adding log crates

test target="-- --nocapture":
    cargo watch -x "test {{target}}"