#!/bin/bash
MINIGIT="$(pwd)/target/release/minigit"

rm -rf test
mkdir test
cd test

echo "--- Iniciando Teste Fast-Forward ---"

# Setup Inicial
"$MINIGIT" init
echo "Arquivo Base" > base.txt
"$MINIGIT" add base.txt
"$MINIGIT" commit "C1: Commit Inicial"

# Criar e mudar para a branch
"$MINIGIT" branch feature-rapida
"$MINIGIT" checkout feature-rapida

# Adicionar commits na branch (master fica parada no tempo)
echo "Nova funcionalidade" > feature.txt
"$MINIGIT" add feature.txt
"$MINIGIT" commit "C2: Adiciona feature"

echo "Melhoria na funcionalidade" >> feature.txt
"$MINIGIT" add feature.txt
"$MINIGIT" commit "C3: Melhora feature"

# Voltar para master
"$MINIGIT" checkout master

# Merge
# Não deve criar um novo commit hash, apenas atualizar o HEAD para o hash de C3.
echo "--- Executando Merge na Master ---"
"$MINIGIT" merge feature-rapida

echo "--- Conteúdo Final da Pasta (Deve conter feature.txt) ---"
ls -l

cd ..