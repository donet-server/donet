#!/bin/bash
# Intended to be called from the project root directory.
# Synchronizes the Gitlab Git repository to the Github mirror repo.

GITLAB_USER=donet-server
GITLAB_NAME=donet
MIRROR_USER=donet-server
MIRROR_NAME=donet

git clone --bare https://gitlab.com/$GITLAB_USER/$GITLAB_NAME.git/
cd $GITLAB_NAME.git
git push --mirror git@github.com:$MIRROR_USER/$MIRROR_NAME
rm -rf $GITLAB_NAME.git

