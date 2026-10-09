# UX Audit skill group

A set of **23 Agent Skills** that let an AI agent audit a product's UX the way a principal UX engineer with 25+ years of experience would: deeply, with evidence, with consistent severity scoring, and with a launch-readiness verdict.

- **1 orchestrator:** `ux-audit` plans the audit, runs the specialists, merges the findings and writes the report.
- **22 specialist sub-skills:** each one is the deep checklist and procedure for one area of UX.
- **Shared references:** one finding format, one scoring model, one recon procedure, one sheet of laws and thresholds, one report template, so every specialist produces output that can be merged.

## Structure

```
.claude/skills/
├── ux-audit/                      ← orchestrator (start here)
│   ├── SKILL.md
│   ├── README.md                  ← this file
│   └── references/
│       ├── finding-format.md      ← mandatory finding + coverage-table format
│       ├── severity-and-scoring.md← S0–S4, R1–R3, confidence, P0–P3, launch gate
│       ├── codebase-recon.md      ← how to inventory routes, components, tokens, strings
│       ├── laws-and-numbers.md    ← UX laws, research, WCAG/CWV/typography thresholds
│       └── report-template.md     ← final report structure
├── ux-context-discovery/          CTX    users, jobs, context, success metrics (always first)
├── ux-information-architecture/   IA     navigation, labels, structure, URLs
├── ux-flows-friction/             FLOW   step-by-step flows, bottlenecks, seams
├── ux-layout-hierarchy/           LAY    grid, spacing, hierarchy, placement, Fitts
├── ux-typography/                 TYP    type scale, readability
├── ux-color-theming/              COL    contrast, semantic color, dark mode
├── ux-interaction-feedback/       INT    states, feedback, motion, overlays, gestures
├── ux-forms-input/                FORM   fields, validation, input efficiency
├── ux-states-resilience/          STATE  empty/loading/error/offline/edge cases, undo
├── ux-accessibility/              A11Y   WCAG 2.2 AA, keyboard, screen readers
├── ux-content-writing/            CONT   microcopy, terminology, errors, emails
├── ux-onboarding-activation/      ONB    first run, time-to-value, activation
├── ux-performance-perceived/      PERF   Core Web Vitals, latency, perceived speed
├── ux-responsive-platform/        RESP   breakpoints, mobile ergonomics, iOS/Android/desktop
├── ux-design-system/              DS     tokens, components, drift (root causes)
├── ux-trust-ethics-privacy/       TRUST  dark patterns, consent, pricing, security UX
├── ux-i18n-localization/          I18N   i18n readiness, RTL, locale formats
├── ux-data-display-search/        DATA   tables, dashboards, charts, search, filters
├── ux-developer-experience/       DX     SDKs, APIs, CLIs, docs, developer errors
├── ux-ai-interfaces/              AI     LLM/AI feature UX, agents, trust calibration
├── ux-measurement-validation/     MEAS   analytics, usability tests, experiments
└── ux-launch-readiness/           LAUNCH go-to-market surface + Go/No-Go gate
```

Each `SKILL.md` has the same anatomy, so agents (and people) always know where to look:

1. **Frontmatter:** `name` and `description` (the description is what makes an agent pick the skill).
2. **Senior mindset:** how an expert thinks about this area, and why.
3. **Scope:** what's in, what's out, and which sibling skill owns the rest.
4. **Procedure:** numbered steps to follow.
5. **Criteria:** an ID'd table (e.g. `FORM-09`) with the check, the fail signal and the default severity.
6. **Code probes:** search patterns to find evidence in a codebase.
7. **Output:** the artifacts to return.
8. **Done when:** the completion criteria, so the agent knows when it has finished.

**Total: 465 criteria** across 22 areas, each traceable by ID from finding to report.

## How to use it

### In Claude Code (this repo)
The skills are discovered automatically from `.claude/skills/`. Ask, for example:
- "Run a full UX audit of this app": triggers `ux-audit` (full mode).
- "Are we ready to launch?": `ux-audit` in pre-launch gate mode.
- "Audit our checkout flow": a scoped audit.
- "Check accessibility only": an area deep-dive (`ux-accessibility`, with light context discovery first).
- "Review the DX of our SDK": `ux-developer-experience`.

For a large audit, the orchestrator dispatches sub-skills to subagents in parallel when the agent has a subagent tool; otherwise it runs them in sequence.

### Use across all your projects
Copy the folders to your personal skills directory:
```bash
cp -r .claude/skills/ux-* ~/.claude/skills/
```

### Share with other AI agents (non-Claude-Code)
The skills are plain Markdown, with no tooling dependencies. To use them with another agent:
1. Give the agent `ux-audit/SKILL.md` plus the `references/` folder as its system or task instructions.
2. Make the sub-skill files available (in context, through retrieval, or as files the agent can read), and tell it: "When the orchestrator says to run `<sub-skill>`, read `<sub-skill>/SKILL.md` and follow it."
3. Require the output format in `references/finding-format.md`, so the results from different agents or runs can be merged.

For multi-agent systems, give each specialist agent one sub-skill file plus the references. The orchestrator agent gets `ux-audit/SKILL.md` and uses the dispatch brief in Phase 4.

## Typical outputs

- `ux-audit-report.md`: executive summary, verdict, scorecard per area, cross-cutting themes, critical-flow walkthroughs, prioritized findings, strengths, roadmap, and coverage/limitations.
- Per-area artifacts: sitemap tree, flow step tables, contrast tables, state matrices, token adoption counts, event catalog, usability test plan, launch gate decision.

## Extending the group

To add a new area (e.g. `ux-voice-interfaces`, `ux-gamification`, `ux-enterprise-admin`):
1. Create `.claude/skills/ux-<area>/SKILL.md` with the same eight sections.
2. Choose a unique criterion ID prefix.
3. Add the area to the skill map and applicability matrix in `ux-audit/SKILL.md`.
4. Keep findings in the shared format, so the orchestrator can merge them.

## Principles the whole group enforces

- Goal before screen; the user's job is the yardstick.
- Evidence for every finding (`file:line`, route, measurement), plus an honest confidence level.
- Severity reflects users, not effort.
- Never invent numbers; cite the source of each threshold.
- Failure states are part of the product.
- What could not be verified is reported, never assumed to pass.
