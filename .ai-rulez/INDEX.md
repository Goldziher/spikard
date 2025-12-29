# Spikard AI-Rulez Configuration Index

**Location**: `/Users/naamanhirschfeld/workspace/spikard/.ai-rulez/`
**Schema Version**: ai-rules-v3
**Last Updated**: 2025-12-29

## Quick Navigation

### Start Here
1. **[QUICK_REFERENCE.md](QUICK_REFERENCE.md)** - Fast agent/rule lookup and common workflows
2. **[MIGRATION_SUMMARY.md](MIGRATION_SUMMARY.md)** - Detailed migration documentation and analysis

### Configuration Files (Canonical)
3. **[custom-agents.yaml](custom-agents.yaml)** - 14 spikard-specific agents in YAML format
4. **[custom-rules.yaml](custom-rules.yaml)** - 30 spikard-specific rules in YAML format
5. **[custom-profiles.yaml](custom-profiles.yaml)** - spikard-web-framework profile definition

### Reference Files
6. **[config.yaml](config.yaml)** - Base spikard configuration (unchanged)
7. **[mcp.yaml](mcp.yaml)** - MCP server configuration (unchanged)

### Historical Reference
- **[agents/](agents/)** - Original 14 agent definitions (.md files)
- **[rules/](rules/)** - Original 30 rule definitions (.md files)

---

## File Descriptions

### Configuration Files

#### custom-agents.yaml (341 lines)
**Contains**: 14 spikard-specific agents categorized by function
- **Middleware & HTTP** (3 Sonnet agents): middleware-architect, workspace-architect, rust-polyglot-architect
- **Language Bindings** (5 Haiku agents): python-engineer, typescript-engineer, ruby-engineer, php-engineer, wasm-engineer
- **Quality & Testing** (2 Haiku agents): fixture-tester, integration-qa
- **Documentation** (2 Haiku agents): docs-strategist, docs-scribe
- **Build & CI** (2 Haiku agents): build-and-ci-ops, interop-build-engineer

**Use When**: Need to understand agent roles, responsibilities, and coordination patterns

**Format**: YAML following ai-rules-v3 schema

#### custom-rules.yaml (824 lines)
**Contains**: 30 spikard-specific rules grouped by domain
- **Fixture-Driven Testing** (4 CRITICAL): fixture-driven-testing, fixture-first-testing, fixture-backed-testing, fixture-aligned-error-handling
- **HTTP Contracts** (4 HIGH/CRITICAL): handler-trait-abstraction, http-input-validation, http-error-contracts, header-cookie-security
- **FFI & Bindings** (2 CRITICAL/HIGH): cross-language-error-boundaries, thin-binding-pattern-architecture
- **Performance** (6 CRITICAL/MEDIUM): zero-copy-json-to-python-conversion, pyo3-async-performance, pyo3-extension-module-management, optimized-serialization-path, async-friendly-performance, cross-target-performance
- **Middleware & Hooks** (2 HIGH): tower-http-middleware-stack, lifecycle-hooks-implementation
- **Code Organization** (3 HIGH/MEDIUM): layered-code-organization, workspace-separation, workspace-organization
- **Security & Standards** (3 HIGH/CRITICAL): request-surface-security, ext-php-rs-binding-configuration, php-psr-compliance-standards-enforcement
- **Code Quality** (2 MEDIUM): consistent-tooling, lint-formatting-discipline

**Use When**: Need detailed implementation guidance, deciding on priorities, enforcing standards

**Format**: YAML with detailed descriptions and implementation sections, following ai-rules-v3 schema

#### custom-profiles.yaml (560 lines)
**Contains**: spikard-web-framework profile extending web-framework.yaml
- **Composition**: Inherits from `/Users/naamanhirschfeld/workspace/kreuzberg-dev/ai-rulez/profiles/web-framework.yaml`
- **Agents**: 25 total (11 inherited + 14 custom)
- **Rules**: 39 priority rules (9 inherited + 30 custom)
- **Model Routing**: Sonnet for architecture, Haiku for implementation/testing/docs
- **Quality Gates**: 95% Rust coverage, 80% language coverage, fixture parity
- **Toolchain**: Rust 2024, Python 3.10+, TypeScript 5.0+, Ruby 3.2+, PHP 8.2+, WASM
- **Tasks**: Complete test/lint/build task definitions

**Use When**: Setting up the development environment, understanding the complete system design, validating compliance

**Format**: YAML profile with full composition model, following ai-rules-v3 schema

### Documentation Files

#### MIGRATION_SUMMARY.md (466 lines)
**Purpose**: Comprehensive documentation of the migration from individual .md files to YAML configuration

**Sections**:
1. Overview - Executive summary
2. Agent migration - 14 agents with categorization
3. Rule migration - 30 rules with domain grouping
4. Profile migration - spikard-web-framework composition
5. Domain analysis - Shared vs. unique breakdown
6. Migration impact - How to use the configs
7. Schema compliance - Verification
8. Next steps - Immediate and ongoing actions
9. Migration checklist - What was completed
10. References - File locations and links
11. Glossary - Key terms defined

**Use When**: Understanding the full scope of the migration, onboarding new team members, decision-making about shared vs. custom content

#### QUICK_REFERENCE.md (336 lines)
**Purpose**: Fast lookup guide for agents, rules, workflows, and concepts

**Sections**:
1. File locations - Directory structure
2. Agent quick lookup - All 14 custom agents + inherited agents
3. Rule quick lookup - All 30 custom rules with categories
4. Decision matrices - When to use which agent/rule
5. Common workflows - Add handler, add binding, fix bug, optimize
6. Profiles - How to use spikard-web-framework
7. Key concepts - Handler trait, thin binding, fixture-first, etc.
8. Links & references - Where to find things
9. Maintenance checklist - Ongoing tasks

**Use When**: Need quick answers, choosing the right agent/rule, understanding common patterns, onboarding

#### INDEX.md (this file)
**Purpose**: Navigation and file index for the entire configuration

**Use When**: First arriving at the directory, need to understand the overall structure

---

## Recommended Reading Order

### For New Team Members
1. INDEX.md (this file) - Get oriented
2. QUICK_REFERENCE.md - Understand agents and rules
3. custom-profiles.yaml - See the complete system design
4. MIGRATION_SUMMARY.md - Deep dive into migration details

### For Daily Development
1. QUICK_REFERENCE.md - Fast lookups
2. custom-agents.yaml - Find the right agent for your task
3. custom-rules.yaml - Reference specific rules you're implementing
4. Original .md files (agents/, rules/) - For detailed historical context

### For Decision-Making
1. MIGRATION_SUMMARY.md - Shared vs. unique analysis
2. custom-profiles.yaml - Quality gates and constraints
3. custom-agents.yaml - Agent coordination matrix
4. custom-rules.yaml - Rule relationships and enforcement

### For System Design
1. custom-profiles.yaml - Complete system overview
2. MIGRATION_SUMMARY.md - Integration with shared ai-rulez
3. custom-agents.yaml - Agent coordination patterns
4. custom-rules.yaml - Rule enforcement order

---

## Key Features

### Schema Compliance
- All YAML files follow ai-rules-v3 schema
- Compatible with ai-rulez tooling (`ai-rulez validate`, `ai-rulez generate`, etc.)
- Backward compatible with existing config.yaml

### Comprehensive Coverage
- 14 agents covering all aspects of spikard development
- 30 rules with detailed implementation guidance
- 1 profile composing agents and rules into a complete system

### Fixture-First Development
- 4 dedicated fixture-driven testing rules
- Emphasis on testing_data/ as source of truth
- Cross-language fixture parity enforcement

### Multi-Language Support
- HTTP framework support for Rust, Python, Node.js, Ruby, PHP, WASM
- Language-specific binding engineers for each platform
- Cross-language error handling and serialization contracts

### Quality Assurance
- Quality gates specified in profile (95% Rust, 80% language coverage)
- fixture-parity testing across all languages
- Tool enforcement (biome, clippy, phpstan, etc.)

---

## Common Use Cases

### "I'm adding a new HTTP handler"
→ See **custom-rules.yaml**: fixture-driven-testing, handler-trait-abstraction, http-input-validation

### "I'm implementing Python bindings"
→ See **custom-agents.yaml**: python-engineer, rust-polyglot-architect
→ See **custom-rules.yaml**: zero-copy-json-to-python-conversion, pyo3-async-performance

### "I need to fix a cross-language bug"
→ See **custom-agents.yaml**: integration-qa, rust-polyglot-architect
→ See **custom-rules.yaml**: fixture-backed-testing, cross-language-error-boundaries

### "I'm designing new middleware"
→ See **custom-agents.yaml**: middleware-architect, workspace-architect
→ See **custom-rules.yaml**: tower-http-middleware-stack, lifecycle-hooks-implementation

### "I need to understand the system architecture"
→ See **custom-profiles.yaml**: Complete system overview
→ See **MIGRATION_SUMMARY.md**: Integration model, domain analysis

### "I'm onboarding to the project"
→ Start with **QUICK_REFERENCE.md**
→ Then read **custom-profiles.yaml** for the big picture
→ Reference **custom-agents.yaml** and **custom-rules.yaml** as needed

---

## File Statistics

| File | Lines | Size | Type | Schema |
|------|-------|------|------|--------|
| custom-agents.yaml | 341 | 15 KB | Configuration | ai-rules-v3 |
| custom-rules.yaml | 824 | 35 KB | Configuration | ai-rules-v3 |
| custom-profiles.yaml | 560 | 21 KB | Configuration | ai-rules-v3 |
| MIGRATION_SUMMARY.md | 466 | 19 KB | Documentation | Markdown |
| QUICK_REFERENCE.md | 336 | 12 KB | Documentation | Markdown |
| INDEX.md | This file | ~ | Navigation | Markdown |
| **Total** | **2,527** | **~90 KB** | - | - |

---

## Integration with Shared AI-Rulez

Spikard's custom configuration extends, not replaces, the shared ai-rulez:

```
Shared AI-Rulez (Foundation)
├─ 24 generic agents
├─ 20+ generic rules
├─ 7 reusable profiles (including web-framework.yaml)
└─ Foundation for polyglot Rust projects

    +

Spikard Custom Config (Specifics)
├─ 14 HTTP-specific agents
├─ 30 fixture-driven rules
├─ 1 spikard-web-framework profile extending web-framework.yaml
└─ HTTP framework + fixture-first development patterns

    =

Complete Spikard System
├─ 25 total agents (11 inherited + 14 custom)
├─ 39 total priority rules (9 inherited + 30 custom)
├─ 1 spikard-web-framework profile
└─ Ready for fixture-driven HTTP development
```

**Base Path**: `/Users/naamanhirschfeld/workspace/kreuzberg-dev/ai-rulez/`

---

## Next Steps

1. **Verify Setup**:
   ```bash
   ls -la /Users/naamanhirschfeld/workspace/spikard/.ai-rulez/custom-*.yaml
   ```

2. **Validate Configuration**:
   ```bash
   ai-rulez validate
   ```

3. **Generate Documentation**:
   ```bash
   ai-rulez generate   # Creates/updates CLAUDE.md
   ```

4. **Team Onboarding**:
   - Share QUICK_REFERENCE.md for fast learning
   - Share MIGRATION_SUMMARY.md for complete understanding
   - Start with QUICK_REFERENCE.md workflows

5. **Continuous Use**:
   - Reference custom-agents.yaml when assigning tasks
   - Reference custom-rules.yaml when making decisions
   - Update documentation as needed (docs-strategist role)

---

## Support & Maintenance

### Who to Contact for Changes

| Area | Agent | Model |
|------|-------|-------|
| Architecture decisions | rust-polyglot-architect, workspace-architect | Sonnet |
| Rule clarifications | docs-scribe, docs-strategist | Haiku |
| Profile updates | build-and-ci-ops | Haiku |
| Agent coordination | Any agent + context from MIGRATION_SUMMARY.md | - |

### Maintenance Schedule

- **Monthly**: Regenerate CLAUDE.md via `ai-rulez generate`
- **Per PR**: Verify fixture coverage and cross-language parity
- **Quarterly**: Review and update quality gates
- **As Needed**: Update ADRs (docs-strategist) and CLAUDE.md

---

## Version Information

- **Schema Version**: ai-rules-v3
- **Migration Date**: 2025-12-29
- **Configuration Version**: spikard-web-framework v1.0
- **Status**: Production Ready

---

**Questions?** Start with [QUICK_REFERENCE.md](QUICK_REFERENCE.md) then refer to [MIGRATION_SUMMARY.md](MIGRATION_SUMMARY.md) for comprehensive details.
