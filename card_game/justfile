alias dbg:= debug
alias rel:= release
alias dist:= distribution

debug:
    cargo watch -x "run --features bevy/dynamic_linking"

release:
    cargo run --profile release

distribution:
    cargo run --profile distribution \
#     -F tracing/release_max_level_off -F log/release_max_level_off
# uncomment after adding log crates

test target="-- --nocapture":
    cargo watch -x "test {{target}}"