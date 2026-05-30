# AGENTS.md

本文件用于指导后续 agent 维护此 fork 的简体中文汉化分支。默认使用简体中文输出；命令、路径、代码标识符、API 字段和第三方名称保持原文。

## 汉化分支维护目标

- 长期跟踪 `warpdotdev/warp` 官方 `master`。
- 保留并持续更新 `zh-cn-i18n-pr11739-master` 上的中文界面改动。
- 每次合并官方更新后，必须检查官方新增功能是否已经接入 localization，并补齐 `zh-CN` 翻译。
- 不把“合并成功”当成“汉化完整”；合并后必须跑汉化验证。

## 合并官方更新流程

在当前汉化分支维护时，优先使用以下流程：

```bash
git fetch upstream
git fetch origin

git switch master
git merge --ff-only upstream/master
git push origin master

git switch zh-cn-i18n-pr11739-master
git merge --no-ff upstream/master
```

如果本地 `master` 或汉化分支有未提交修改，先停止并说明工作区状态，不要覆盖用户改动。

建议开启 Git 冲突记忆，减少重复处理同类冲突：

```bash
git config rerere.enabled true
git config rerere.autoupdate true
git config merge.conflictStyle zdiff3
```

## 合并后的汉化检查

每次合并 `upstream/master` 后，必须至少检查以下内容：

1. `app/assets/bundled/locales/en-US.json` 与 `app/assets/bundled/locales/zh-CN.json` 的 key 是否完全一致。
2. 新增英文 UI 文案是否通过 `localization::text_for_app(...)`、`text_for_app_with_args(...)` 或同类 localization API 获取。
3. 新增带参数文案的 `{placeholder}` 是否在英文和中文中完全一致。
4. 菜单、按钮、toast、placeholder、tooltip、settings、agent/onboarding/code review 等用户可见界面是否仍有硬编码英文。
5. `crates/warp_search_core` 等 app 外 crate 如暂时无法注入 app localization，必须记录 fallback 限制，不能声称完整汉化。

优先运行这些验证：

```bash
cargo test -p warp_localization --test localization_tests
cargo test -p warp localization_tests
```

如果涉及完整打包，还需要至少运行当前 CI 对应的 macOS OSS DMG workflow，或说明未运行原因。

## 新增官方功能的汉化处理

官方新增功能通常有两种情况：

- 已经新增到 `en-US.json`：必须在 `zh-CN.json` 添加同名 key，并翻译为自然的简体中文。
- 代码里直接写了英文 UI 文案：必须先把英文提取成 localization key，再同步加入 `en-US.json` 和 `zh-CN.json`。

翻译时保留占位符、快捷键、产品名、命令、路径、模型名和品牌名。例如 `{count}`、`Warp Drive`、`OpenAI`、`git status` 不应被错误翻译或删除。

## 推荐提交拆分

合并和翻译补齐分开提交，方便回滚和定位：

```text
feat(i18n): 合并 upstream master 到汉化分支
i18n: 补齐 upstream 新增文案翻译
ci: 修复汉化分支打包验证
```

不要把无关重构、格式化、打包脚本调整和翻译补齐混在同一个提交里，除非它们是同一个失败验证的必要修复。

## 推送与危险操作

- `git push`、`git reset`、`git rebase`、`git clean`、force push、删除分支或删除文件前必须先获得用户确认。
- 发现冲突时只解决与当前合并/汉化相关的文件，不要顺手改无关代码。
- 无法验证时必须明确说明原因，并给出可执行的验证命令。
