#!/bin/bash
MINIGIT="$(pwd)/target/release/minigit"

rm -rf test
mkdir test
cd test

echo "=== Teste do Comando Status ==="
echo ""

echo "--- 1. Inicializando repositório ---"
"$MINIGIT" init
echo ""

echo "--- 2. Status em repositório vazio (nada para commitar) ---"
"$MINIGIT" status
echo ""

echo "--- 3. Criando arquivo e adicionando ao staging area ---"
echo "Conteúdo do primeiro arquivo" > arquivo1.txt
"$MINIGIT" add arquivo1.txt
echo "Status após adicionar arquivo1.txt:"
"$MINIGIT" status
echo ""

echo "--- 4. Criando segundo arquivo (não adicionado) ---"
echo "Conteúdo do segundo arquivo" > arquivo2.txt
echo "Status com arquivo não rastreado:"
"$MINIGIT" status
echo ""

echo "--- 5. Commitando o primeiro arquivo ---"
"$MINIGIT" commit "C1: Primeiro commit com arquivo1.txt"
echo "Status após commit:"
"$MINIGIT" status
echo ""

echo "--- 6. Modificando arquivo1.txt no worktree ---"
echo "Conteúdo modificado do arquivo1" > arquivo1.txt
echo "Status com arquivo modificado (não staged):"
"$MINIGIT" status
echo ""

echo "--- 7. Adicionando arquivo2.txt ao staging ---"
"$MINIGIT" add arquivo2.txt
echo "Status com novo arquivo staged:"
"$MINIGIT" status
echo ""

echo "--- 8. Modificando e adicionando arquivo1.txt (staged) ---"
echo "Conteúdo mais modificado do arquivo1" > arquivo1.txt
"$MINIGIT" add arquivo1.txt
echo "Status com modificações staged:"
"$MINIGIT" status
echo ""

echo "--- 9. Commitando as mudanças ---"
"$MINIGIT" commit "C2: Segundo commit com modificações"
echo "Status após segundo commit:"
"$MINIGIT" status
echo ""

echo "--- 10. Criando múltiplos arquivos não rastreados ---"
echo "Arquivo novo 1" > novo1.txt
echo "Arquivo novo 2" > novo2.txt
echo "Arquivo novo 3" > novo3.txt
echo "Status com múltiplos arquivos não rastreados:"
"$MINIGIT" status
echo ""

echo "--- 11. Adicionando alguns e deixando outros sem rastrear ---"
"$MINIGIT" add novo1.txt novo2.txt
echo "Status com alguns arquivos novos staged e outros não rastreados:"
"$MINIGIT" status
echo ""

echo "--- 12. Removendo um arquivo do worktree (mas está no staging) ---"
rm arquivo1.txt
echo "Status com arquivo deletado do worktree:"
"$MINIGIT" status
echo ""

echo "=== Teste Concluído ==="
cd ..
