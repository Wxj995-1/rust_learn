#!/usr/bin/env bash
# 用法: ./git_upload.sh ["提交说明"]
set -e

# 仓库目录：默认当前目录，可用 REPO_DIR=... 覆盖
REPO_DIR="${REPO_DIR:-$(pwd)}"
cd "$REPO_DIR"

# 提交说明：优先用参数，否则用时间戳
MSG="${1:-update: $(date '+%Y-%m-%d %H:%M:%S')}"

# 1. 还不是仓库就初始化
if [ ! -d .git ]; then
    git init
    echo "已初始化 git 仓库"
fi

# 2. 没有远程就提示先配置
if ! git remote get-url origin >/dev/null 2>&1; then
    echo "未配置远程 origin，请先执行："
    echo "  git remote add origin <你的仓库地址>"
    exit 1
fi

# 3. 暂存全部改动
git add -A

# 4. 没有改动就直接退出
if git diff --cached --quiet; then
    echo "没有改动，无需提交"
    exit 0
fi

# 5. 提交
git commit -m "$MSG"

# 6. 推到当前分支
BRANCH="$(git rev-parse --abbrev-ref HEAD)"
git push -u origin "$BRANCH"

echo "上传完成: $BRANCH -> origin"
