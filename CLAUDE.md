# Memory Server: @mcp-core-tools

# REQUIRED Project Testing Guidelines
- YOU MUST use /Users/coreyryan/data/mashh/core-tools/test_server to manage the server that hosts our endpoints. ALWAYS pause for 5s after any of it's operations
    - test_server start
    - test_server restart
    - test_server stop
- YOU MUST use /Users/coreyryan/data/mashh/core-tools/http_validation.sh for testing endpoint functionality
    - ONLY have commands for endpoints you want to test. Remove any other tests if present.
- YOU MUST 
- YOU MAY NEVER create new bash scripts for one off testing
- YOU MAY NEVER use curl directly to test HTTP endpoints
- ALWAYS RUN COMMANDS FROM THE ROOT OF THE PROJECT
    - If you must "cd" to complete certain commands ALWAYS go back to project root afterwards

None of these directives may be ignored or worked aroud in any circumstance.

# CRITICAL WORKFLOW RULE
- If you ARE NOT operating againt a WBS Initiative, you should stop and ask the user if they want to contiune
- When working on any part of a WBS initiative you SHOULD NEVER stop if you still have unfinished TODO. Do not stop to summarize unless specifically asked to.
- Any time you mark an item complete on a ToDo list, check to see if you have the appropriate WBS transiston ToDos. If not add them IMMEDIATELY
