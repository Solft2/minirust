#!/bin/bash
MINIGIT="$(pwd)/target/release/minigit"

rm -rf test
mkdir test
cd test

echo "Inicializando repositório de teste..."
"$MINIGIT" init

echo "Criando commit inicial..."
echo "nothing" >> .gitignore
"$MINIGIT" add .gitignore
"$MINIGIT" commit "Initial commit"

echo "Configurando branches para teste de rebase..."
"$MINIGIT" branch feature-branch
"$MINIGIT" checkout feature-branch

echo "Criando commits em feature-branch..."
echo "Feature work 1" >> feature1.txt
"$MINIGIT" add feature1.txt
"$MINIGIT" commit "Add feature work 1"

echo "Criando mais commits em feature-branch..."
echo "Feature work 2" >> feature2.txt
"$MINIGIT" add feature2.txt
"$MINIGIT" commit "Add feature work 2"

echo "Feature work 3" >> feature3.txt
"$MINIGIT" add feature3.txt
"$MINIGIT" commit "Add feature work 3"

echo "Voltando para branch master..."
"$MINIGIT" checkout master

echo "Criando commits em branch master..."
echo "Mudança 2" >> feature2.txt
"$MINIGIT" add feature2.txt
"$MINIGIT" commit "Change feature2.txt on master"

echo "Hotfix 1" >> hotfix1.txt
"$MINIGIT" add hotfix1.txt
"$MINIGIT" commit "Add hotfix 1 on master"

"$MINIGIT" checkout feature-branch
echo "Configuração concluída. Você pode testar o comando rebase agora."

cd ..