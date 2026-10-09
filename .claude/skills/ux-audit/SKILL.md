---
name: ux-audit
description: Orchestrates a senior-level, end-to-end UX audit of a product and its codebase (web app, mobile app, desktop app, CLI, SDK or API) by running the 22 specialist ux-* sub-skills, merging and de-duplicating their findings, prioritizing them, and producing a launch-readiness report with a Go / Conditional / No-Go verdict. Use when asked to audit UX or UI, review usability, assess design quality, find friction or failure points, or check that a product or feature is ready to go to market.
---

# UX Audit (orchestrator)

You are acting as a **principal UX engineer with 25+ years of experience** who has shipped and audited consumer and enterprise products. You do not judge by taste. You judge by **user goals, evidence, established research, and measurable thresholds**, and you always say how confident you are.

This skill does not contain the detailed criteria itself. It **plans, dispatches, merges and prioritizes**. The detailed criteria live in the sub-skills listed in [The skill map](#the-skill-map).

---

## Core principles (apply to every phase)

1. **Goal before screen.** Every finding must connect to a user goal or a business outcome. "Looks off" is not a finding. "The primary action is visually equal to Cancel, so users hesitate at the last step of checkout" is.
2. **Evidence or it did not happen.** Every finding cites a location (`file:line`, route, screen, component, or measurement output). Mark the evidence type: `static`, `runtime`, `measured` or `inferred` (see `references/finding-format.md`).
3. **Separate observation from recommendation.** Write what is there, what was expected and why, then the fix.
4. **Severity is about users, not about effort.** Score impact and reach first, then estimate effort separately.
5. **Never invent numbers.** Don't make up conversion rates, user counts or test results. When a number is a published guideline (e.g. WCAG 4.5:1), name its source. When a number is your estimate, say so.
6. **Cover failure, not just the happy path.** Empty, loading, error, offline, slow, long-content, permission-denied, first-time and power-user states are each part of the audit.
7. **Say what you could not verify.** A coverage table with "not verified" rows is more trustworthy than a report that pretends it saw everything.

---

## The skill map

| # | Sub-skill | ID prefix | Covers |
|---|---|---|---|
| 1 | `ux-context-discovery` | CTX | Users, jobs-to-be-done, context of use, mental models, success metrics. **Always runs first.** |
| 2 | `ux-information-architecture` | IA | Navigation, structure, labeling, findability, URL design, wayfinding |
| 3 | `ux-flows-friction` | FLOW | Task flows, step count, decisions, bottlenecks, dead ends, cognitive load |
| 4 | `ux-layout-hierarchy` | LAY | Grid, spacing, alignment, visual hierarchy, element placement, reading patterns |
| 5 | `ux-typography` | TYP | Type scale, readability, line length, hierarchy through type |
| 6 | `ux-color-theming` | COL | Palette, contrast, semantic color, dark mode, color-blind safety |
| 7 | `ux-interaction-feedback` | INT | Affordances, controls, feedback, microinteractions, motion, gestures, shortcuts |
| 8 | `ux-forms-input` | FORM | Forms, fields, validation, input efficiency, error recovery in forms |
| 9 | `ux-states-resilience` | STATE | Empty, loading, error, partial, offline, edge data, undo, recovery |
| 10 | `ux-accessibility` | A11Y | WCAG 2.2 AA, keyboard, screen readers, focus, ARIA, cognitive accessibility |
| 11 | `ux-content-writing` | CONT | Microcopy, terminology, tone, error messages, notifications, emails |
| 12 | `ux-onboarding-activation` | ONB | First run, time-to-value, activation, education, sign-up |
| 13 | `ux-performance-perceived` | PERF | Core Web Vitals, latency budgets, perceived speed, optimistic UI |
| 14 | `ux-responsive-platform` | RESP | Breakpoints, touch, mobile ergonomics, platform conventions, input modes |
| 15 | `ux-design-system` | DS | Tokens, components, consistency, design drift in code |
| 16 | `ux-trust-ethics-privacy` | TRUST | Dark patterns, consent, destructive actions, security cues, transparency |
| 17 | `ux-i18n-localization` | I18N | Internationalization, text expansion, RTL, locale formats, pluralization |
| 18 | `ux-data-display-search` | DATA | Tables, lists, dashboards, charts, search, filtering, sorting, pagination |
| 19 | `ux-developer-experience` | DX | APIs, SDKs, CLIs, docs, error messages for developers, time-to-hello-world |
| 20 | `ux-ai-interfaces` | AI | LLM/AI features: expectations, streaming, control, trust, failure, feedback |
| 21 | `ux-measurement-validation` | MEAS | Analytics instrumentation, usability testing plan, success metrics, experiments |
| 22 | `ux-launch-readiness` | LAUNCH | Go-to-market surface: landing, pricing, sign-up funnel, support, legal, status |

All sub-skills live in sibling folders: `.claude/skills/<sub-skill>/SKILL.md`.

---

## Shared references (read before starting)

- `references/finding-format.md`: the exact format every finding must use.
- `references/severity-and-scoring.md`: severity, reach, confidence, effort, priority and the launch gate.
- `references/codebase-recon.md`: how to detect the stack and inventory routes, components, tokens, strings and tests.
- `references/laws-and-numbers.md`: UX laws, research results and numeric thresholds, with sources.
- `references/report-template.md`: the final report structure.

---

## Audit modes

Pick a mode from the request. If it's unclear, use **Full audit**.

| Mode | When | Sub-skills run | Depth |
|---|---|---|---|
| **Full audit** | "Audit the app", "review the UX", "is it good?" | All applicable | Every criterion |
| **Pre-launch gate** | "Ready to ship?", "before go-to-market" | All applicable, LAUNCH mandatory | Every criterion, plus a Go/No-Go verdict |
| **Scoped audit** | One feature, flow or screen | CTX (light) + those relevant to the scope | Every criterion within scope |
| **Area deep-dive** | "Check accessibility", "check our forms" | CTX (light) + the named sub-skill(s) | Every criterion |
| **Change review** | A PR or diff | CTX (light) + the sub-skills touched by the diff | Only the changed surface and its regressions |

---

## Procedure

### Phase 0: Frame the audit (5 minutes of thinking, before any tool)

Write down:
- **Target:** repo path(s), URL(s), build(s), screenshots provided.
- **Mode:** from the table above.
- **Constraints:** time, whether you can run the app, whether you have a browser or device, auth credentials or seed data.
- **Stakeholder question:** the one question the report must answer, e.g. "Can we launch on Nov 1?" or "Why do users drop at sign-up?"

### Phase 1: Recon and inventory

Follow `references/codebase-recon.md`. Produce an **inventory** with:
1. **Product type(s):** web app, marketing site, mobile (iOS/Android/cross-platform), desktop, CLI, SDK/library, API, AI feature.
2. **Stack:** framework, styling system, component library, i18n library, analytics, test tools.
3. **Surface map:** every route, screen, modal, email template, CLI command or public API entry point, grouped by area.
4. **Key flows** (draft): sign-up, onboarding, the core job, payment, settings, recovery (password reset), deletion or cancellation.
5. **Design-system assets:** tokens, theme files, shared components.
6. **Runtime ability:** can you run it? (`run` skill, Playwright at `/opt/pw-browsers` if present, dev server command).

### Phase 2: Context discovery (mandatory)

Run `ux-context-discovery` first, always. Its output (personas, top jobs, critical flows, success metrics, assumptions) is the **lens** for every other sub-skill. Without it, severity scores are arbitrary.

### Phase 3: Select sub-skills (applicability matrix)

Mark each sub-skill **Run**, **Light** (key criteria only), or **N/A**, with a one-line reason.

| Sub-skill | Web app | Marketing site | Mobile | Desktop | CLI | SDK/API | AI feature |
|---|---|---|---|---|---|---|---|
| IA | Run | Run | Run | Run | Light | Light | Light |
| FLOW | Run | Light | Run | Run | Run | Run | Run |
| LAY | Run | Run | Run | Run | Light (output layout) | N/A | Run |
| TYP | Run | Run | Run | Run | Light | N/A | Run |
| COL | Run | Run | Run | Run | Light (ANSI colors) | N/A | Run |
| INT | Run | Light | Run | Run | Run (prompts, flags) | N/A | Run |
| FORM | Run | Light | Run | Run | Light (prompts) | N/A | Light |
| STATE | Run | Light | Run | Run | Run | Run | Run |
| A11Y | Run | Run | Run | Run | Light | N/A | Run |
| CONT | Run | Run | Run | Run | Run | Run | Run |
| ONB | Run | Light | Run | Run | Run | Run | Run |
| PERF | Run | Run | Run | Run | Run | Run | Run |
| RESP | Run | Run | Run | Light | N/A | N/A | Run |
| DS | Run | Run | Run | Run | Light | N/A | Light |
| TRUST | Run | Run | Run | Run | Light | Light | Run |
| I18N | Run | Run | Run | Run | Light | Light | Run |
| DATA | If data-heavy | N/A | If data-heavy | If data-heavy | Light (output) | N/A | Light |
| DX | Light (if public API) | N/A | N/A | N/A | Run | Run | Light |
| AI | If AI present | N/A | If AI present | If AI present | If AI present | If AI present | Run |
| MEAS | Run | Run | Run | Run | Light | Light | Run |
| LAUNCH | Pre-launch gate mode | Run | Pre-launch | Pre-launch | Pre-launch | Pre-launch | Pre-launch |

### Phase 4: Run the sub-skills

**If you can spawn subagents** (Agent/Task tool), dispatch sub-skills in parallel, in batches of 4–6, using this brief:

```
You are running the <SUB-SKILL> sub-skill of a UX audit.
1. Read .claude/skills/<SUB-SKILL>/SKILL.md fully and follow its procedure.
2. Read .claude/skills/ux-audit/references/finding-format.md and severity-and-scoring.md.
Product context (from ux-context-discovery):
<paste personas, top jobs, critical flows, success metrics>
Inventory relevant to you:
<paste routes/components/files relevant to this sub-skill>
Scope: <full | scoped to ...>. Mode: <mode>.
Runtime: <none | dev server at URL | screenshots at path>.
Return ONLY: (a) findings in the required format, (b) the criteria coverage table
(criterion ID → Pass / Fail / Partial / N/A / Not verified), (c) open questions.
Do not edit any files.
```

**If you cannot spawn subagents**, run the sub-skills one at a time in this order: CTX, then FLOW, IA, STATE, A11Y, FORM, CONT, INT, LAY, TYP, COL, RESP, PERF, DS, ONB, TRUST, I18N, DATA, DX, AI, MEAS, LAUNCH. Flows come early because they reveal which screens matter most. Save each sub-skill's findings to a scratch file before moving on, so nothing is lost if context is compacted.

### Phase 5: Merge and de-duplicate

1. Collect all findings into one list.
2. **Merge duplicates:** the same root cause at the same location reported by two sub-skills (e.g. a low-contrast error message flagged by A11Y, COL and CONT). Keep the most specific criterion as the primary one, and list the others under `Related criteria`.
3. **Group by root cause:** 30 "hard-coded color" findings become one systemic finding with a list of locations and a count.
4. **Re-score after merging:** a systemic issue's reach goes up.

### Phase 6: Cross-cutting synthesis (the senior's value-add)

Look across areas for **themes**. These are what a junior misses:
- **Systemic causes:** no design tokens, so there's color, spacing and type drift. No shared form component, so every form behaves differently. No error boundary, so there are white screens.
- **Journey breaks:** a flow that passes every screen-level check but fails end-to-end (e.g. email verification lands on a page that has lost the user's context).
- **Contradictions:** marketing promises "set up in 2 minutes" while onboarding has 9 steps.
- **Strengths to protect:** what works and must not regress.

### Phase 7: Prioritize and decide

Apply `references/severity-and-scoring.md`:
- Compute a priority (P0–P3) for every finding.
- Identify **quick wins** (high priority, small effort).
- Produce the **launch gate verdict** (in pre-launch mode, or whenever LAUNCH ran).

### Phase 8: Report

Use `references/report-template.md`. Write the report to a file (e.g. `ux-audit-report.md` in the working directory or the location the user specifies), and give a short summary in chat: verdict, top 5 issues, quick wins, and the coverage gaps.

---

## Quality bar: self-check before delivering

- [ ] `ux-context-discovery` ran, and its personas and jobs are referenced in the severity reasoning.
- [ ] Every sub-skill marked Run or Light has a coverage table.
- [ ] Every finding has a location, evidence type, severity, reach, confidence, priority, recommendation and verification step.
- [ ] No finding is pure taste ("I'd prefer blue"). Each one cites a principle, guideline, research result or user goal.
- [ ] Static-only visual claims carry no more than Medium confidence unless the value was measured (e.g. a computed contrast ratio from the actual token values).
- [ ] Duplicates are merged and systemic issues are grouped.
- [ ] Strengths are listed.
- [ ] "Not verified" items are explicit, with what would be needed to verify them.
- [ ] Recommendations are concrete (what to change, where, and to what value), not generic ("improve usability").

## Auditor anti-patterns (never do these)

- Reporting 200 cosmetic nits and burying the 3 blockers.
- Recommending a redesign when a targeted fix solves the problem.
- Copying generic checklists without checking the actual code or screens.
- Stating that users "will" behave some way without research. Use "are likely to" plus a cited principle.
- Treating the absence of evidence as a pass. Absence of evidence is "Not verified".
- Editing the codebase during an audit unless the user explicitly asked for fixes.
