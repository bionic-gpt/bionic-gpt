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

top_pane=$(tmux display-message -p -t "$session:app" '#{pane_id}')
middle_pane=$(tmux split-window -v -p 66 -t "$top_pane" -c "$workspace" -P -F '#{pane_id}')
bottom_pane=$(tmux split-window -v -p 50 -t "$middle_pane" -c "$workspace" -P -F '#{pane_id}')

tmux send-keys -t "$top_pane" "just wa" Enter
tmux send-keys -t "$middle_pane" "just wp" Enter
tmux send-keys -t "$bottom_pane" "just wt" Enter

tmux select-pane -t "$top_pane"

attach
