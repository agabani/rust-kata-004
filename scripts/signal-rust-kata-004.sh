SIGNAL=$1

kill "$SIGNAL" "$(pgrep rust-kata-004)"
