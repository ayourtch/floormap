#!/bin/sh
pkill tmux
tmux new-session -s foo -d
echo launch floormap
tmux new-window "./target/debug/service-floormap-json"
echo launch upload
tmux new-window "./target/debug/service-upload"
echo "RSP10 auth"
# tmux new-window "./scripts/run-rsp10auth"
tmux new-window "(cd rsp10auth; ./target/debug/rsp10auth)"

