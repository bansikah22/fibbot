#!/bin/bash

# Delete local branches containing 'test-fibbot'
git branch | grep 'test-fibbot' | xargs git branch -D

# Delete remote branches containing 'test-fibbot'
git branch -r | grep 'test-fibbot' | sed 's/origin\///' | xargs -I {} git push origin --delete {}
