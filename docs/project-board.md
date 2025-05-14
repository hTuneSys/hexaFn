# 🗂️ hexaFn Project Board Design

This document defines the layout, purpose, and automation rules of the official **hexaFn Kanban Board** under GitHub Projects.

---

## 📌 Columns (Board Workflow)

| Order | Column Title         | Description                                                  |
|-------|----------------------|--------------------------------------------------------------|
| 1️⃣   | 📥 Inbox / Ideas     | New ideas, rough suggestions, or exploratory thoughts       |
| 2️⃣   | 📝 To Do             | Accepted, ready-to-start issues with clear scope            |
| 3️⃣   | 🚧 In Progress       | Tasks actively being developed                              |
| 4️⃣   | 🔎 In Review         | Pull requests under code review                             |
| 5️⃣   | ✅ Done              | Completed & merged tasks                                    |
| 6️⃣   | 🧊 Backlog           | Low-priority or postponed tasks                             |
| 7️⃣   | ♻️ Needs Discussion  | Items awaiting community or team feedback                   |

---

## 🏷️ Recommended Labels

| Label              | Purpose                         |
|--------------------|----------------------------------|
| `type:feature`     | New features                    |
| `type:bug`         | Bug reports                     |
| `type:refactor`    | Code cleanup / structural change|
| `type:docs`        | Documentation tasks             |
| `type:ci`          | CI/CD and automation changes    |
| `module:store`     | Related to HexaStore module     |
| `module:cast`      | Related to HexaCast module      |
| `priority:high`    | High-priority items             |
| `help wanted`      | Welcoming external contributions|

---

## 🔄 Suggested Automation Rules (Manual or Action-based)

| Event                          | Column              |
|--------------------------------|---------------------|
| Issue created (needs triage)  | 📥 Inbox            |
| Issue accepted                | 📝 To Do            |
| Developer starts work         | 🚧 In Progress      |
| Pull request opened           | 🔎 In Review        |
| Pull request merged           | ✅ Done             |
| Deferred or low priority      | 🧊 Backlog          |
| Community input required      | ♻️ Needs Discussion |

---

## 🧪 Example Cards

| Card Title                            | Suggested Column      |
|--------------------------------------|------------------------|
| Define plugin system for HexaRun     | 📥 Inbox               |
| Add `init` command to CLI            | 📝 To Do               |
| Implement WASM execution engine      | 🚧 In Progress         |
| Refactor HexaCast message model      | 🔎 In Review           |
| Basic project scaffold (done)        | ✅ Done                |
| Add Redis integration to HexaBridge  | 🧊 Backlog             |
| Should HexaTrigger support cron?     | ♻️ Needs Discussion    |

---

## 🛠 How to Create This Board

1. Go to: https://github.com/orgs/hTuneSys/projects
2. Click **New Project** → Choose **Board** view
3. Name it: `hexaFn Board`
4. Create the 7 columns above manually
5. Assign issues/PRs to the board manually or via automation rules

For automation, you can also use GitHub Actions or third-party bots.

---

This board keeps the hexaFn lifecycle modular, visible, and traceable.

_Questions? Contact: info@hexafn.com_

Happy building 🚀
