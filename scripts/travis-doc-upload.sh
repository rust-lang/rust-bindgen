#!/bin/sh

# License: CC0 1.0 Universal
# https://creativecommons.org/publicdomain/zero/1.0/legalcode

set -e

PROJECT_NAME=bindgen
DOCS_REPO=crabtw/rust-bindgen
SSH_KEY_TRAVIS_ID=26066e6f8a7a

[ "$TRAVIS_BRANCH" = master ]

[ "$TRAVIS_PULL_REQUEST" = false ]

eval key=\$encrypted_${SSH_KEY_TRAVIS_ID}_key
eval iv=\$encrypted_${SSH_KEY_TRAVIS_ID}_iv

mkdir -p ~/.ssh
openssl aes-256-cbc -K $key -iv $iv -in scripts/travis-doc.key.enc -out ~/.ssh/id_rsa -d
chmod 600 ~/.ssh/id_rsa

cargo doc
echo "<meta http-equiv=refresh content=0;url=`echo $PROJECT_NAME`/index.html>" > target/doc/index.html

sudo pip install ghp-import
ghp-import -n target/doc

git push git@github.com:$DOCS_REPO gh-pages
