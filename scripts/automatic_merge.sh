#!/usr/bin/env bash

git pull origin dev

for i in 77 73 7 65; do
  issue="issue_$i"
  echo "$issue\n--------";
  git checkout "$issue"
  git pull origin "$issue"
  git merge dev -m "Merge branch 'dev' into $issue"
  git push origin "$issue"
  git checkout dev
  git merge "$issue" -m "Merge branch '$issue' into dev"
  git push origin dev
  echo "\n"
done
