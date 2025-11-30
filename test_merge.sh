alias minigit=/home/vinic/UFRN/2025.2/Rust/minirust/target/release/minigit

rm -rf test
mkdir test

cd test

minigit init
echo "nothing" >> .gitignore
minigit add .gitignore
minigit commit "Initial commit"

echo "Linha 1: Original" > compartilhado.txt
minigit add compartilhado.txt
minigit commit "Adiciona arquivo compartilhado"

minigit branch develop
minigit checkout develop

echo "Linha 1: Alterada pela Develop" > compartilhado.txt
minigit add compartilhado.txt
minigit commit "Muda linha 1 na develop"

minigit checkout master
echo "Linha 1: Alterada pela Master" > compartilhado.txt
minigit add compartilhado.txt
minigit commit "Muda linha 1 na master"

minigit merge develop