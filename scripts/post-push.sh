#!/bin/bash
# Post-push hook to check CI status on main/develop branches
# Install this hook by running: scripts/setup-dev.sh

PROTECTED_BRANCHES="main|develop"

echo ""
echo "🔍 Checking CI status for protected branches..."

check_branch_status() {
    local branch=$1
    
    # Get latest workflow runs for the branch
    local status=$(gh api \
        "repos/:owner/:repo/actions/runs?branch=$branch&per_page=5" \
        --jq '.workflow_runs[0] | "\(.status)|\(.conclusion)|\(.name)"' 2>/dev/null)
    
    if [ -z "$status" ]; then
        return
    fi
    
    IFS='|' read -r run_status conclusion workflow_name <<< "$status"
    
    if [ "$run_status" = "completed" ]; then
        if [ "$conclusion" = "failure" ]; then
            echo "❌ WARNING: Branch '$branch' has failing CI checks!"
            echo "   Latest workflow: $workflow_name (failed)"
            echo "   View: https://github.com/$(gh repo view --json nameWithOwner -q .nameWithOwner)/actions"
        elif [ "$conclusion" = "success" ]; then
            echo "✅ Branch '$branch': All CI checks passing"
        fi
    elif [ "$run_status" = "in_progress" ]; then
        echo "⏳ Branch '$branch': CI checks in progress"
    fi
}

# Check status of main/develop
for branch in main develop; do
    if git rev-parse --verify origin/$branch &>/dev/null; then
        check_branch_status "$branch"
    fi
done

echo ""
