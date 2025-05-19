<!--
SPDX-FileCopyrightText: 2025 Hüsamettin Arabacı
SPDX-License-Identifier: MIT
-->

# PROJECT_BOARD.md

This document describes the structure and usage of the hexaFn GitHub Project Board used for tracking issues, workflows, and development progress.

---

## 🧭 Board Overview

The hexaFn project board is organized using GitHub Projects (v2) with the following columns:

1. **Inbox** – New issues or ideas needing triage  
2. **Backlog** – Approved but not yet scheduled items  
3. **In Progress** – Actively being worked on  
4. **In Review** – Awaiting PR review or discussion  
5. **Blocked** – Work paused due to dependencies  
6. **Done** – Completed tasks  
7. **Closed** – Rejected or archived items  

---

## 🏷️ Label Integration

Cards are automatically categorized using GitHub Labels, such as:

- `type: bug`, `type: feature`, `type: question`, etc.
- `status: in progress`, `status: review`, `status: blocked`
- `priority: high`, `priority: low`

These labels help sort and filter cards efficiently.

---

## 🔄 Workflow Rules

- Issues moved to **In Progress** must be assigned to a contributor.
- Pull Requests are linked to issues and automatically update board state.
- The **In Review** column is for PRs with open reviews.
- Cards in **Done** are periodically cleaned up to **Closed**.

---

## 🔧 Automation

GitHub Actions and project workflows automate the board:

- Auto-assign labels on issue creation
- Auto-move cards based on PR status
- Auto-close completed issues when PRs are merged

---

## 🔗 Access the Board

Visit the live project board here:  
➡️ [`BOARD`](https://github.com/orgs/hTuneSys/projects/15/views/1)