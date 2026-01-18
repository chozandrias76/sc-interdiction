#!/bin/bash
# Branch protection pre-push hook
# Prevents direct pushes to protected branches (main, develop)
# Enforces that changes go through proper PR workflow

# Colors for output
RED='\033[0;31m'
YELLOW='\033[1;33m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Protected branches that require PRs
PROTECTED_BRANCHES="^(main|develop)$"

# Branch naming conventions
RELEASE_BRANCH_PATTERN="^release/[0-9]+\.[0-9]+\.[0-9]+(-rc\.[0-9]+|-beta\.[0-9]+|-alpha\.[0-9]+)?$"
FEATURE_BRANCH_PATTERN="^(feature|feat|bugfix|fix|hotfix|chore|docs|refactor|test|ci)/.*$"

# Get current branch
current_branch=$(git symbolic-ref --short HEAD 2>/dev/null)

# Read push info from stdin (format: local_ref local_sha remote_ref remote_sha)
while read local_ref local_sha remote_ref remote_sha; do
    # Extract remote branch name from ref
    remote_branch=$(echo "$remote_ref" | sed 's|refs/heads/||')

    # Check if pushing to a protected branch
    if [[ "$remote_branch" =~ $PROTECTED_BRANCHES ]]; then

        # Check if we're on a valid source branch for this target
        if [[ "$remote_branch" == "main" ]]; then
            # main only accepts pushes from release/* branches
            if [[ ! "$current_branch" =~ $RELEASE_BRANCH_PATTERN ]]; then
                echo -e "${RED}ERROR: Direct push to 'main' blocked${NC}"
                echo ""
                echo "Pushes to 'main' must come from a release branch."
                echo "  Expected branch format: release/X.Y.Z"
                echo "  Current branch: $current_branch"
                echo ""
                echo -e "${YELLOW}Correct workflow:${NC}"
                echo "  1. Create release branch: git checkout -b release/X.Y.Z"
                echo "  2. Push release branch: git push -u origin release/X.Y.Z"
                echo "  3. Create PR from release/X.Y.Z to main"
                echo "  4. Merge PR after review and CI passes"
                echo ""
                exit 1
            fi

        elif [[ "$remote_branch" == "develop" ]]; then
            # develop accepts pushes from feature/*, bugfix/*, etc. or release/* (back-merge)
            if [[ ! "$current_branch" =~ $FEATURE_BRANCH_PATTERN ]] && \
               [[ ! "$current_branch" =~ $RELEASE_BRANCH_PATTERN ]] && \
               [[ "$current_branch" != "develop" ]]; then
                echo -e "${RED}ERROR: Direct push to 'develop' blocked${NC}"
                echo ""
                echo "Pushes to 'develop' must come from a feature/bugfix branch."
                echo "  Expected formats: feature/*, feat/*, bugfix/*, fix/*, hotfix/*, chore/*, docs/*, refactor/*, test/*, ci/*"
                echo "  Current branch: $current_branch"
                echo ""
                echo -e "${YELLOW}Correct workflow:${NC}"
                echo "  1. Create feature branch: git checkout -b feature/my-feature"
                echo "  2. Push feature branch: git push -u origin feature/my-feature"
                echo "  3. Create PR from feature/my-feature to develop"
                echo "  4. Merge PR after review and CI passes"
                echo ""
                exit 1
            fi

            # If pushing from develop to develop, warn about direct push
            if [[ "$current_branch" == "develop" ]]; then
                echo -e "${YELLOW}WARNING: Pushing directly to 'develop'${NC}"
                echo "Consider using a feature branch and PR for better traceability."
                echo ""
                # Allow but warn (remove this block to hard-block)
            fi
        fi
    fi
done

echo -e "${GREEN}Branch protection check passed${NC}"
exit 0
