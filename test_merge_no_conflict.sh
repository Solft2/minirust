#!/bin/bash
MINIGIT="$(pwd)/target/release/minigit"

rm -rf test
mkdir test
cd test

echo "--- Iniciando Teste Three-Way (Sem Conflito) ---"

# Setup Inicial (Ancestral Comum)
"$MINIGIT" init
echo "Conteúdo Comum" > comum.txt
"$MINIGIT" add comum.txt
"$MINIGIT" commit "C1: Ancestral Comum"

# Caminho da Branch (Mexe no arquivo A)
"$MINIGIT" branch feature-complexa
"$MINIGIT" checkout feature-complexa

echo "Coisa da Feature" > arquivo_feature.txt
"$MINIGIT" add arquivo_feature.txt
"$MINIGIT" commit "C2: Commit na Feature"

# Caminho da Master (Mexe no arquivo B - diferente do A)
"$MINIGIT" checkout master

echo "Coisa da Master" > arquivo_master.txt
"$MINIGIT" add arquivo_master.txt
"$MINIGIT" commit "C3: Commit na Master (Divergencia)"

# 4. Merge
# EXPECTATIVA: 
# - Deve identificar divergência.
# - Deve achar o ancestral comum (C1).
# - Deve mesclar automaticamente (master ganha arquivo_feature, feature já tinha arquivo_master).
# - Deve criar um novo Commit de Merge automaticamente.
echo "--- Executando Merge Recursivo ---"
"$MINIGIT" merge feature-complexa

echo "--- Conteúdo Final da Pasta (Deve ter 3 arquivos) ---"
ls -l

cd ..