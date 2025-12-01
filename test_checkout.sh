#!/bin/bash
MINIGIT="$(pwd)/target/release/minigit"

rm -rf test
mkdir test

cd test

"$MINIGIT" init
echo "nothing" >> .gitignore
"$MINIGIT" add .gitignore
"$MINIGIT" commit "Initial commit"

echo "Creating branch 'develop' and switching to it..."
"$MINIGIT" branch develop
"$MINIGIT" checkout develop

echo "Add uncommited changes to working directory..."
echo "Some uncommited changes" >> uncommited.txt

echo "Switching back to master branch..."
"$MINIGIT" checkout master

cd ..

