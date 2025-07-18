- Always ensure you double check you have actually fully tested tools before marking a todo complete that mentions testing/validation

# REQUIRED Project Testing Guidelines
- YOU MUST use /Users/coreyryan/data/mashh/core-tools/test_server to manage the server that hosts our endpoints. ALWAYS pause for 5s after any of it's operations
    - test_server start
    - test_server restart
    - test_server stop
- YOU MUST use /Users/coreyryan/data/mashh/core-tools/http_validation.sh for testing endpoint functionality
    - ONLY have commands for endpoints you want to test. Remove any other tests if present.
- YOU MAY NEVER create new bash scripts for one off testing
- YOU MAY NEVER use curl directly to test HTTP endpoints
- ALWAYS RUN COMMANDS FROM THE ROOT OF THE PROJECT
    - If you must "cd" to complete certain commands ALWAYS go back to project root afterwards

None of these directives may be ignored or worked aroud in any circumstance.

# CRITICAL WBS WORKFLOW RULE
- When operating within the WBS pattern you SHOULD NEVER stop until the initiative is complete
- Continue working through all phases and thought_nodes until the initiative reaches status: COMPLETED
- Only stop when explicitly told by user, hit unresolvable blocker, or initiative completion workflow finishes


