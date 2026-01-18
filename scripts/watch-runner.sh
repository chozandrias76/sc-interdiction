#!/bin/bash
# Watch GitHub Actions self-hosted runner logs in tmux
# Usage: ./scripts/watch-runner.sh

RUNNER_DIR="${GITHUB_RUNNER_DIR:-$HOME/actions-runner}"
SESSION_NAME="gh-runner"

# Check if runner directory exists
if [[ ! -d "$RUNNER_DIR/_diag" ]]; then
    echo "Error: Runner directory not found at $RUNNER_DIR"
    echo "Set GITHUB_RUNNER_DIR environment variable if runner is elsewhere"
    exit 1
fi

# Kill existing session if it exists
tmux kill-session -t "$SESSION_NAME" 2>/dev/null

# Create new tmux session
tmux new-session -d -s "$SESSION_NAME" -n "runner"

# Pane 0 (top-left): Job status summary - live updates
tmux send-keys -t "$SESSION_NAME:0.0" "watch -n 2 'grep -E \"Running job:|completed with result\" $RUNNER_DIR/_diag/Runner_*.log 2>/dev/null | tail -15'" C-m

# Split horizontally for pane 1 (top-right): Current worker log
tmux split-window -h -t "$SESSION_NAME:0.0"
tmux send-keys -t "$SESSION_NAME:0.1" "tail -f $RUNNER_DIR/_diag/Worker_*.log 2>/dev/null | grep --line-buffered -E 'Running job|completed|STDOUT|Starting process|Finished process|result:|ERROR|WARN'" C-m

# Split pane 0 vertically for pane 2 (bottom-left): Runner connection status
tmux split-window -v -t "$SESSION_NAME:0.0"
tmux send-keys -t "$SESSION_NAME:0.2" "tail -f $RUNNER_DIR/_diag/Runner_*.log 2>/dev/null | grep --line-buffered -E 'Listening|Job.*received|Running job|completed'" C-m

# Split pane 1 vertically for pane 3 (bottom-right): Interactive shell
tmux split-window -v -t "$SESSION_NAME:0.1"
tmux send-keys -t "$SESSION_NAME:0.3" "echo '=== Runner Commands ==='; echo 'gh run list         - List workflow runs'; echo 'gh run view <id>    - View run details'; echo 'gh run watch <id>   - Watch run live'; echo ''; cd $RUNNER_DIR" C-m

# Set pane titles (if terminal supports it)
tmux select-pane -t "$SESSION_NAME:0.0" -T "Job Summary"
tmux select-pane -t "$SESSION_NAME:0.1" -T "Worker Log"
tmux select-pane -t "$SESSION_NAME:0.2" -T "Runner Status"
tmux select-pane -t "$SESSION_NAME:0.3" -T "Commands"

# Select the worker log pane by default
tmux select-pane -t "$SESSION_NAME:0.1"

# Attach to session
tmux attach-session -t "$SESSION_NAME"
