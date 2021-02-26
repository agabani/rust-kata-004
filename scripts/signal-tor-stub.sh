SIGNAL=$1

kill "$SIGNAL" "$(pgrep tor-stub)"
