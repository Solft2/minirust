#!/bin/bash

rm -rf test
mkdir test
cd test

echo "--- Iniciando Teste Three-Way (Sem Conflito) ---"

# Setup Inicial (Ancestral Comum)
minigit init
echo "Conteúdo Comum" > comum.txt
minigit add comum.txt
minigit commit "C1: Ancestral Comum"

# Caminho da Branch (Mexe no arquivo A)
minigit branch feature-complexa
minigit checkout feature-complexa

echo "Coisa da Feature" > arquivo_feature.txt
minigit add arquivo_feature.txt
minigit commit "C2: Commit na Feature"

# Caminho da Master (Mexe no arquivo B - diferente do A)
minigit checkout master

echo "Coisa da Master" > arquivo_master.txt
minigit add arquivo_master.txt
minigit commit "C3: Commit na Master (Divergencia)"

# 4. Merge
# EXPECTATIVA: 
# - Deve identificar divergência.
# - Deve achar o ancestral comum (C1).
# - Deve mesclar automaticamente (master ganha arquivo_feature, feature já tinha arquivo_master).
# - Deve criar um novo Commit de Merge automaticamente.
echo "--- Executando Merge Recursivo ---"
minigit merge feature-complexa

echo "--- Conteúdo Final da Pasta (Deve ter 3 arquivos) ---"
ls -l