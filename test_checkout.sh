rm -rf test
mkdir test

cd test

minigit init
echo "nothing" >> .gitignore
minigit add .gitignore
minigit commit "Initial commit"

echo "Creating branch 'develop' and switching to it..."
minigit branch develop
minigit checkout develop

echo "Add uncommited changes to working directory..."
echo "Some uncommited changes" >> uncommited.txt

echo "Switching back to master branch..."
minigit checkout master

cd ..

