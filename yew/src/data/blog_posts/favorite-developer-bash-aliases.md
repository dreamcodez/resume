---
title: "Essential Developer Bash Aliases for Productivity"
date: "2024-01-10"
slug: "favorite-developer-bash-aliases"
tags: ["bash", "productivity", "development", "tools", "workflow"]
summary: "A comprehensive collection of bash aliases and functions that have significantly improved my development workflow over 20+ years of software development."
draft: false
author: "Matthew Elder <matt@jupitersoft.net>"
reading_time: 12
---

# Essential Developer Bash Aliases for Productivity

## Introduction

After 20+ years of software development, I've accumulated a collection of bash aliases that significantly improve my daily productivity. These aliases streamline common development tasks, reduce typing, and help maintain consistency across different projects.

## Git Aliases

### Basic Git Operations

```bash
# Quick status check
alias gs='git status'

# Add all files and commit with message
alias gac='git add . && git commit -m'

# Quick push to current branch
alias gp='git push'

# Pull latest changes
alias gl='git pull'

# Show commit history in a nice format
alias glog='git log --oneline --graph --decorate --all'

# Show recent commits with file changes
alias gshow='git log --stat --oneline -10'
```

### Advanced Git Operations

```bash
# Create and switch to new branch
alias gcb='git checkout -b'

# Switch to main/master branch
alias gcm='git checkout main'

# Show current branch
alias gb='git branch --show-current'

# Reset to last commit (keep changes)
alias grh='git reset HEAD'

# Reset to last commit (discard changes)
alias grhh='git reset HEAD --hard'

# Show diff of staged changes
alias gds='git diff --staged'
```

## Development Workflow

### Project Navigation

```bash
# Go to project directory
alias dev='cd ~/dev'

# List projects
alias projects='ls ~/dev'

# Quick edit of common config files
alias vimrc='vim ~/.vimrc'
alias bashrc='vim ~/.bashrc'
alias zshrc='vim ~/.zshrc'
```

### Package Management

```bash
# Node.js
alias ni='npm install'
alias nid='npm install --save-dev'
alias nrb='npm run build'
alias nrd='npm run dev'
alias nrt='npm run test'

# Yarn
alias yi='yarn install'
alias yb='yarn build'
alias yd='yarn dev'
alias yt='yarn test'

# Cargo (Rust)
alias cb='cargo build'
alias cr='cargo run'
alias ct='cargo test'
alias cc='cargo check'
```

## System Operations

### File Operations

```bash
# List files with details
alias ll='ls -la'

# List files by size
alias lsize='ls -laSh'

# List files by modification time
alias ltime='ls -laht'

# Find files by name
alias findf='find . -name'

# Find files by content
alias findc='grep -r'

# Copy with progress
alias cpv='rsync -ah --progress'
```

### System Information

```bash
# Show disk usage
alias dfh='df -h'

# Show memory usage
alias mem='free -h'

# Show process tree
alias pst='pstree'

# Show top processes by CPU
alias topcpu='ps aux | sort -nr -k 3 | head -10'

# Show top processes by memory
alias topmem='ps aux | sort -nr -k 4 | head -10'
```

## Network and Connectivity

### Network Operations

```bash
# Quick ping
alias pingg='ping google.com'

# Show network interfaces
alias neti='ip addr show'

# Show routing table
alias netr='ip route show'

# Test DNS resolution
alias dns='nslookup'

# Check if port is open
alias port='netstat -tulpn | grep'
```

### SSH Operations

```bash
# Quick SSH with key
alias sshk='ssh -i ~/.ssh/id_rsa'

# SSH with verbose output
alias sshv='ssh -v'

# Copy file via SSH
alias scpk='scp -i ~/.ssh/id_rsa'
```

## Docker Operations

### Container Management

```bash
# List running containers
alias dps='docker ps'

# List all containers
alias dpa='docker ps -a'

# List images
alias dim='docker images'

# Remove stopped containers
alias drm='docker rm $(docker ps -a -q)'

# Remove unused images
alias drmi='docker rmi $(docker images -q -f dangling=true)'

# Docker compose shortcuts
alias dc='docker-compose'
alias dcu='docker-compose up'
alias dcd='docker-compose down'
alias dcb='docker-compose build'
```
