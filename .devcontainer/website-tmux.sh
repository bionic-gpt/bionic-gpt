#!/usr/bin/env bash
set -euo pipefail

session="bionic-website"
workspace="/workspace"

if ! command -v tmux >/dev/null 2>&1; then
    sudo apt-get update -qq
    sudo apt-get install -y -qq tmux
fi

tmux set-option -g mouse on

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

tmux new-session -d -s "$session" -n website -c "$workspace"

top_pane=$(tmux display-message -p -t "$session:website" '#{pane_id}')
bottom_pane=$(tmux split-window -v -p 50 -t "$top_pane" -c "$workspace" -P -F '#{pane_id}')

tmux send-keys -t "$top_pane" "just ws" Enter
tmux send-keys -t "$bottom_pane" "just wts" Enter

tmux select-pane -t "$top_pane"

attach
