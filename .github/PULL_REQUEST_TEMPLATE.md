# Pull Request

## 📄 Summary

> Clearly describe what this PR changes and why it is necessary.  
> Link to any relevant issue(s) below.

---

## 🧩 Affected Module(s)

Mark the modules impacted by this PR:

- [ ] HexaStore
- [ ] HexaCast
- [ ] HexaRun
- [ ] HexaTrigger
- [ ] HexaWatch
- [ ] HexaBridge
- [ ] Core Logic
- [ ] Documentation
- [ ] CI / Infra

---

## ✏️ PR Title Format

> ⚠️ PR title must follow [Conventional Commits](https://www.conventionalcommits.org/) format.  
> **Only these types are allowed:**

```text
feat, fix, chore, refactor, test, docs, ci, perf, build, release, hotfix, style
```

✅ Examples:

```text
feat: add lifecycle validator
fix: resolve webhook retry bug
chore: clean up unused dependencies
refactor: simplify event dispatcher
test: add integration test for HexaCast
docs: update contributing guide
ci: improve workflow caching
perf: reduce latency in trigger matching
build: update Cargo.lock and bump versions
release: prepare v0.3.0 release
hotfix: patch panic in runtime engine
style: apply rustfmt to all modules
```

---

## ✅ Checklist

Before submitting, make sure you've completed the following:

- [ ] My branch name follows format: `<type>/<short-description>` (e.g., `feat/auth-handler`)
- [ ] My PR title starts with one of the approved types listed above
- [ ] My code passes formatting (`cargo fmt`)
- [ ] I ran Clippy linter (`cargo clippy`) and resolved warnings
- [ ] I ran tests successfully (`cargo test`)
- [ ] I linked related issues using keywords like `Closes #42`
- [ ] I ensured this PR has no unrelated changes
- [ ] This PR is ready for review and does not include unfinished work

---

## 🔗 Related Issues

> If this PR addresses one or more issues, link them here:

```text
Closes #
```

---

## 💬 Additional Notes (Optional)

> Include any test instructions, screenshots, diagrams, or context useful for reviewers.
