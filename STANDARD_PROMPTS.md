# Standardized Session Prompts - Core Tools Project

## Session Start Prompt

**Use this prompt to start any new session:**

```
Continue the core-tools migration project. Start by running the memory primer queries from SESSION_HANDOFF.md to establish context, then proceed with the assigned work.
```

### Expected Flow:
1. Session runs memory primer queries from SESSION_HANDOFF.md
2. Session establishes constitutional rules and operational orientation
3. Session reviews current project status and assigned tasks
4. Session begins work with full context

## Session Validation Prompt

**Use this prompt at the end of sessions or when discovering context gaps:**

```
Run post-session validation using POST_SESSION_VALIDATION.md to assess context gaps and identify memory system improvements. This is a learning exercise to evolve the memory framework.
```

### Expected Flow:
1. Session works through POST_SESSION_VALIDATION.md categories
2. Session identifies what context is missing
3. Session analyzes why information wasn't discoverable
4. Session makes specific memory system improvements
5. Session updates SESSION_HANDOFF.md if needed

## Prompt Design Principles

### Session Start Prompt:
- **Simple and direct**: Clear instructions for immediate action
- **References SESSION_HANDOFF.md**: Uses established breadcrumb trail
- **Establishes context first**: Memory primer before work
- **Flexible**: Works for any session regardless of specific tasks

### Session Validation Prompt:
- **Learning focused**: Emphasizes improvement over checklist completion
- **References POST_SESSION_VALIDATION.md**: Uses dedicated validation file
- **Improvement oriented**: Focuses on evolving memory system
- **Separate from breadcrumb trail**: Independent assessment

## Usage Guidelines

### When to Use Session Start Prompt:
- Beginning of every new session
- When context has been lost during a session
- When restarting after interruption
- When unclear about current priorities

### When to Use Session Validation Prompt:
- End of sessions (recommended)
- When discovering significant context gaps
- When memory system seems incomplete
- When preparing major memory system updates

## Integration Points

### SESSION_HANDOFF.md Integration:
- Session start prompt references SESSION_HANDOFF.md
- Creates dependency on established breadcrumb trail
- Ensures constitutional rules are loaded first

### POST_SESSION_VALIDATION.md Integration:
- Session validation prompt references POST_SESSION_VALIDATION.md
- Creates learning-focused validation experience
- Drives memory system evolution

## Examples

### Starting a New Session:
```
User: [Session Start Prompt]
Assistant: I'll start by running the memory primer queries from SESSION_HANDOFF.md to establish context.

[Runs memory primer queries]
[Establishes constitutional rules and operational orientation]
[Reviews current project status]
[Begins assigned work with full context]
```

### Running Validation:
```
User: [Session Validation Prompt]
Assistant: I'll work through POST_SESSION_VALIDATION.md to assess context gaps and identify memory system improvements.

[Validates context across categories]
[Identifies gaps and root causes]
[Recommends specific memory system improvements]
[Updates memory entities and relations]
[Evolves the memory framework]
```

## Standardization Benefits

### Consistency:
- Same prompts work across all sessions
- Predictable flow for context establishment
- Standardized validation experience

### Evolution:
- Regular validation drives memory system improvements
- Each session leaves system better than found
- Continuous learning and adaptation

### Efficiency:
- No need to customize prompts for different sessions
- Established patterns reduce cognitive load
- Focus on work rather than process

---
*These prompts should be used consistently across all core-tools sessions to maintain systematic memory management and continuous improvement.*