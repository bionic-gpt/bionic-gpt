#!/usr/bin/env bash
set -euo pipefail

session="bionic-dev"
workspace="/workspace"

if ! command -v tmux >/dev/null 2>&1; then
    sudo apt-get update -qq
    sudo apt-get install -y -qq tmux
fi

attach() {
    if [ -n "${TMUX:-}" ]; then
        tmux switch-client -t "$session"
    else
        exec tmux attach-session -t "$session"
    fi
}

if tmux has-session -t "$session" 2>/dev/null; then
    attach
    exit 0
fi

tmux new-session -d -s "$session" -n app -c "$workspace"

left_pane=$(tmux display-message -p -t "$session:app" '#{pane_id}')
right_top=$(tmux split-window -h -t "$left_pane" -c "$workspace" -P -F '#{pane_id}')
right_middle=$(tmux split-window -v -p 66 -t "$right_top" -c "$workspace" -P -F '#{pane_id}')
right_bottom=$(tmux split-window -v -p 50 -t "$right_middle" -c "$workspace" -P -F '#{pane_id}')

tmux send-keys -t "$right_top" "just wa" Enter
tmux send-keys -t "$right_middle" "just wp" Enter
tmux send-keys -t "$right_bottom" "just wt" Enter

tmux new-window -d -t "$session" -n shells -c "$workspace"
tmux split-window -h -t "$session:shells" -c "$workspace"

tmux select-window -t "$session:app"
tmux select-pane -t "$left_pane"

attach
