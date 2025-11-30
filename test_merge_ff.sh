alias minigit=/home/vinic/UFRN/2025.2/Rust/minirust/target/release/minigit
#!/bin/bash

rm -rf test
mkdir test
cd test

echo "--- Iniciando Teste Fast-Forward ---"

# Setup Inicial
minigit init
echo "Arquivo Base" > base.txt
minigit add base.txt
minigit commit "C1: Commit Inicial"

# Criar e mudar para a branch
minigit branch feature-rapida
minigit checkout feature-rapida

# Adicionar commits na branch (master fica parada no tempo)
echo "Nova funcionalidade" > feature.txt
minigit add feature.txt
minigit commit "C2: Adiciona feature"

echo "Melhoria na funcionalidade" >> feature.txt
minigit add feature.txt
minigit commit "C3: Melhora feature"

# Voltar para master
minigit checkout master

# Merge
# Não deve criar um novo commit hash, apenas atualizar o HEAD para o hash de C3.
echo "--- Executando Merge na Master ---"
minigit merge feature-rapida

echo "--- Conteúdo Final da Pasta (Deve conter feature.txt) ---"
ls -l