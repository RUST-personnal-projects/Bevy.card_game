# Watch app in release mode
watch_release:
    cargo watch -x  "run --profile release-native --no-default-features"

# Watch app in debug mode
watch_debug:
    cargo watch -x "run --features bevy/dynamic_linking"

# Build wasm executable in the chosen mode: debug or release
build_wasm mode="debug":
    @if [ {{mode}} = "release" ]; then \
        cargo build --target wasm32-unknown-unknown --release; \
    else \
        cargo build --target wasm32-unknown-unknown; \
    fi
    @wasm-bindgen --no-typescript --target web \
    --out-dir ./out/ \
    --out-name "card_game" \
    ./target/wasm32-unknown-unknown/{{mode}}/card_game.wasm

# Copy wasm executable to frontend directory, path deduced from FRONT_PATH env variable
copy_to_front:
    @if [[ -v FRONT_PATH ]]; then \
        mkdir -p $FRONT_PATH/public; \
        cp web/* $FRONT_PATH/public; \
        cp out/* $FRONT_PATH/public; \
        cp -r assets $FRONT_PATH/public; \
        echo "Copied wasm files to front folder"; \
    else \
        echo "Error: FRONT_PATH var is not set"; \
    fi

# Build and then move the game to frontend directory in the chosen mode: debug or release
wasm_setup mode="debug":
    @just build_wasm {{mode}}
    @just copy_to_front

# Build app in distribution mode
distribution:
    cargo build --profile distribution \
#     -F tracing/release_max_level_off -F log/release_max_level_off
# uncomment after adding log crates

# Run tests, capture is optional
test capture="false":
    if [ {{capture}} = true ]; then \
        cargo watch -x "test --features bevy/dynamic_linking"; \
    else \
        cargo watch -x "test --features bevy/dynamic_linking -- --nocapture"; \
    fi
