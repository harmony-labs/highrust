# Highrust: Agent Working Prompt

**Instructions for the Agent (Roo):**

1. **Read This Prompt First**
   - Every time you start working, read this prompt in full to remind yourself of your responsibilities and workflow.

2. **Project Documentation Check**
   - Before taking any action, check for the existence of the following documents in the `context` directory:
     - a) A document describing the intention of this project.
     - b) A document describing the technical details, goals, and design of the project.
     - c) A document containing a step-wise set of tasks and subtasks required to implement the project completely.
     - d) A document or section tracking our progress against those tasks (e.g., using `[ ]` and `[x]` checklists).

3. **Decompose the Plan**
   - If PLAN.md exists, decompose it into a complete set of tasks and subtasks, and record these in the c) document (e.g., `context/TASKS.md`).
   - Ensure each task is actionable, specific, and can be checked off as progress is made.

4. **Startup Workflow**
   - At the start of each new interaction:
     1. Read this prompt and all relevant docs (a, b, c, d).
     2. Determine the current place in the project (what is done, what is next).
     3. Identify the next sprint of work (the next logical set of tasks/subtasks).
     4. Perform the next sprint of work.
     5. After completing the sprint, commit and push your work to the repository.

5. **Continuous Progress**
   - After each sprint, update the progress tracking document (d) to reflect completed tasks.
   - Always ensure your actions are aligned with the project intention, technical goals, and design.

6. **Self-Improvement**
   - If you notice missing or unclear documentation, create or update the relevant docs in `context` to improve future workflows.

---

**Summary:**  
Always orient yourself by reading this prompt and the latest project docs, decompose the plan into actionable tasks, track progress, and work iteratively—committing and pushing after each sprint.

---

**Files to Maintain in `context`:**
- `README_PROMPT.md` (this file)
- `INTENTION.md` (project intention)
- `DESIGN.md` (technical details/goals/design)
- `TASKS.md` (stepwise tasks/subtasks)
- `PROGRESS.md` (progress tracking, or as a section in TASKS.md)
- `PLAN.md` (the main project plan, for reference)