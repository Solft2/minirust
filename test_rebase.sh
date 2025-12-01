rm -rf test
mkdir test
cd test

echo "Initializing test repository..."
minigit init

echo "Creating initial commit..."
echo "nothing" >> .gitignore
minigit add .gitignore
minigit commit "Initial commit"

echo "Setting up branches for rebase test..."
minigit branch feature-branch
minigit checkout feature-branch

echo "Creating commits on feature-branch..."
echo "Feature work 1" >> feature1.txt
minigit add feature1.txt
minigit commit "Add feature work 1"

echo "Creating more commits on feature-branch..."
echo "Feature work 2" >> feature2.txt
minigit add feature2.txt
minigit commit "Add feature work 2"

echo "Feature work 3" >> feature3.txt
minigit add feature3.txt
minigit commit "Add feature work 3"

echo "Switching back to master branch..."
minigit checkout master

echo "Creating commits on master branch..."
echo "Mudança 2" >> feature2.txt
minigit add feature2.txt
minigit commit "Change feature2.txt on master"

echo "Hotfix 1" >> hotfix1.txt
minigit add hotfix1.txt
minigit commit "Add hotfix 1 on master"

minigit checkout feature-branch
echo "Setup complete. You can now test the rebase command."

cd ..