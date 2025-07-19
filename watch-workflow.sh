#!/bin/bash

# Workflow Monitor Script
# Watches GitHub Actions workflow runs and exits when complete

set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Default values
REPO="${GITHUB_REPOSITORY:-$(git config --get remote.origin.url | sed 's/.*github.com[:/]\(.*\)\.git/\1/')}"
WORKFLOW_NAME="${1:-CI}"
POLL_INTERVAL="${POLL_INTERVAL:-10}"

# Function to print usage
usage() {
    echo "Usage: $0 [workflow_name]"
    echo "  workflow_name: Name of the workflow to monitor (default: CI)"
    echo ""
    echo "Environment variables:"
    echo "  POLL_INTERVAL: Seconds between checks (default: 10)"
    echo "  GITHUB_REPOSITORY: Override repo detection"
    exit 1
}

# Parse arguments
if [[ "$1" == "-h" ]] || [[ "$1" == "--help" ]]; then
    usage
fi

echo -e "${BLUE}🔍 Monitoring workflow: ${WORKFLOW_NAME}${NC}"
echo -e "${BLUE}📦 Repository: ${REPO}${NC}"
echo -e "${BLUE}⏱️  Poll interval: ${POLL_INTERVAL}s${NC}"
echo ""

# Get the latest workflow run
get_latest_run() {
    gh run list \
        --workflow="$WORKFLOW_NAME" \
        --repo="$REPO" \
        --limit=1 \
        --json databaseId,status,conclusion,headBranch,event,createdAt \
        --jq '.[0]'
}

# Get workflow jobs
get_workflow_jobs() {
    local run_id=$1
    gh run view "$run_id" \
        --repo="$REPO" \
        --json jobs \
        --jq '.jobs[] | {name, status, conclusion, startedAt}'
}

# Get the latest run first
echo -e "${YELLOW}⏳ Checking for workflow runs...${NC}"
INITIAL_RUN=$(get_latest_run)
INITIAL_ID=$(echo "$INITIAL_RUN" | jq -r '.databaseId // empty')
INITIAL_STATUS=$(echo "$INITIAL_RUN" | jq -r '.status // empty')
INITIAL_TIME=$(echo "$INITIAL_RUN" | jq -r '.createdAt // empty')

if [[ -z "$INITIAL_ID" ]]; then
    echo -e "${RED}❌ No workflow runs found${NC}"
    exit 1
fi

# Check if the latest run is recent (within last 10 minutes)
if [[ -n "$INITIAL_TIME" ]]; then
    # Remove trailing Z and handle both GNU and BSD date
    CLEAN_TIME=$(echo "$INITIAL_TIME" | sed 's/Z$//')
    if command -v gdate >/dev/null 2>&1; then
        # Use GNU date if available (macOS with coreutils)
        RUN_TIME=$(gdate -d "${CLEAN_TIME}+00:00" +%s)
        CURRENT_TIME=$(gdate +%s)
    elif date --version >/dev/null 2>&1; then
        # GNU date
        RUN_TIME=$(date -d "${CLEAN_TIME}+00:00" +%s)
        CURRENT_TIME=$(date +%s)
    else
        # BSD date (macOS)
        RUN_TIME=$(date -j -u -f "%Y-%m-%dT%H:%M:%S" "$CLEAN_TIME" +%s)
        CURRENT_TIME=$(date +%s)
    fi
    TIME_DIFF=$((CURRENT_TIME - RUN_TIME))
    
    # If run is within last 10 minutes and completed, monitor it
    if [[ $TIME_DIFF -lt 600 ]] && [[ "$INITIAL_STATUS" == "completed" ]]; then
        echo -e "${BLUE}📊 Found recent completed workflow (${TIME_DIFF}s ago)${NC}"
        RUN_ID="$INITIAL_ID"
    elif [[ "$INITIAL_STATUS" == "in_progress" ]] || [[ "$INITIAL_STATUS" == "queued" ]]; then
        echo -e "${GREEN}✅ Found active workflow${NC}"
        RUN_ID="$INITIAL_ID"
    else
        # Wait for a new run
        echo -e "${YELLOW}⏳ Waiting for new workflow to start...${NC}"
        while true; do
            CURRENT_RUN=$(get_latest_run)
            CURRENT_ID=$(echo "$CURRENT_RUN" | jq -r '.databaseId // empty')
            CURRENT_STATUS=$(echo "$CURRENT_RUN" | jq -r '.status // empty')
            
            # Check if this is a new run or an in-progress run
            if [[ "$CURRENT_ID" != "$INITIAL_ID" ]] || [[ "$CURRENT_STATUS" == "in_progress" ]] || [[ "$CURRENT_STATUS" == "queued" ]]; then
                RUN_ID="$CURRENT_ID"
                break
            fi
            
            echo -n "."
            sleep "$POLL_INTERVAL"
        done
    fi
else
    # Fallback if time parsing fails
    if [[ "$INITIAL_STATUS" == "in_progress" ]] || [[ "$INITIAL_STATUS" == "queued" ]]; then
        RUN_ID="$INITIAL_ID"
    else
        echo -e "${YELLOW}⏳ Waiting for new workflow to start...${NC}"
        while true; do
            CURRENT_RUN=$(get_latest_run)
            CURRENT_ID=$(echo "$CURRENT_RUN" | jq -r '.databaseId // empty')
            CURRENT_STATUS=$(echo "$CURRENT_RUN" | jq -r '.status // empty')
            
            if [[ "$CURRENT_ID" != "$INITIAL_ID" ]] || [[ "$CURRENT_STATUS" == "in_progress" ]] || [[ "$CURRENT_STATUS" == "queued" ]]; then
                RUN_ID="$CURRENT_ID"
                break
            fi
            
            echo -n "."
            sleep "$POLL_INTERVAL"
        done
    fi
fi

echo ""
echo -e "${GREEN}✅ Monitoring workflow run: ${RUN_ID}${NC}"

# Display run info
RUN_INFO=$(gh run view "$RUN_ID" --repo="$REPO" --json headBranch,event,createdAt)
echo -e "${BLUE}📍 Branch: $(echo "$RUN_INFO" | jq -r '.headBranch')${NC}"
echo -e "${BLUE}🎯 Event: $(echo "$RUN_INFO" | jq -r '.event')${NC}"
echo -e "${BLUE}🕐 Started: $(echo "$RUN_INFO" | jq -r '.createdAt')${NC}"
echo ""

# Monitor the workflow
LAST_JOB_COUNT=0
SHOW_SUMMARY=true
while true; do
    # Get current run status
    RUN_DATA=$(gh run view "$RUN_ID" --repo="$REPO" --json status,conclusion,jobs)
    STATUS=$(echo "$RUN_DATA" | jq -r '.status')
    CONCLUSION=$(echo "$RUN_DATA" | jq -r '.conclusion // empty')
    
    # Get job statuses
    JOBS=$(echo "$RUN_DATA" | jq -r '.jobs[] | "\(.name)|\(.status)|\(.conclusion // "pending")"')
    JOB_COUNT=$(echo "$RUN_DATA" | jq '.jobs | length')
    
    # Clear screen for update (optional - comment out if you prefer scrolling)
    # clear
    
    # Only show summary if this is the first iteration or status changed
    if [[ "$SHOW_SUMMARY" == "true" ]] || [[ "$STATUS" != "$LAST_STATUS" ]]; then
        echo -e "${BLUE}=== Workflow Status: ${STATUS} ===${NC}"
        echo -e "Time: $(date '+%H:%M:%S')"
        echo ""
        SHOW_SUMMARY=false
        LAST_STATUS="$STATUS"
    fi
    
    # Display job statuses
    echo "Jobs:"
    while IFS='|' read -r name status conclusion; do
        case "$status" in
            "completed")
                if [[ "$conclusion" == "success" ]]; then
                    echo -e "  ${GREEN}✅ ${name}${NC}"
                elif [[ "$conclusion" == "skipped" ]]; then
                    echo -e "  ${YELLOW}⏭️  ${name}${NC}"
                else
                    echo -e "  ${RED}❌ ${name} (${conclusion})${NC}"
                fi
                ;;
            "in_progress")
                echo -e "  ${YELLOW}🔄 ${name}${NC}"
                ;;
            "queued")
                echo -e "  ${BLUE}⏳ ${name}${NC}"
                ;;
            *)
                echo -e "  ❓ ${name} (${status})"
                ;;
        esac
    done <<< "$JOBS"
    
    # Check if workflow is complete
    if [[ "$STATUS" == "completed" ]]; then
        echo ""
        if [[ "$CONCLUSION" == "success" ]]; then
            echo -e "${GREEN}🎉 Workflow completed successfully!${NC}"
            
            # Show workflow URL
            echo ""
            echo -e "${BLUE}View run: https://github.com/${REPO}/actions/runs/${RUN_ID}${NC}"
            exit 0
        else
            echo -e "${RED}💥 Workflow failed with conclusion: ${CONCLUSION}${NC}"
            
            # Show failed jobs
            echo ""
            echo "Failed jobs:"
            echo "$RUN_DATA" | jq -r '.jobs[] | select(.conclusion != "success" and .conclusion != null) | "  - \(.name): \(.conclusion)"'
            
            # Show workflow URL
            echo ""
            echo -e "${BLUE}View run: https://github.com/${REPO}/actions/runs/${RUN_ID}${NC}"
            exit 1
        fi
    fi
    
    # Show progress indicator
    echo ""
    echo -ne "${YELLOW}Refreshing in ${POLL_INTERVAL}s...${NC}"
    sleep "$POLL_INTERVAL"
    echo -ne "\r\033[K" # Clear the line
done