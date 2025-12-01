#!/bin/bash
MINIGIT="$(pwd)/target/release/minigit"

echo "--- Limpando ambiente de teste ---"
rm -rf test
mkdir test
cd test

echo "--- 1. Configuração Inicial (Ancestral Comum) ---"
"$MINIGIT" init

# Criamos um arquivo que servirá de base para o conflito
echo "Linha 1: Original" > compartilhado.txt
"$MINIGIT" add compartilhado.txt
"$MINIGIT" commit "C1: Adiciona arquivo compartilhado original"

echo "--- 2. Criando Branch Develop (Theirs) ---"
# Simulamos o trabalho de outra branch
"$MINIGIT" branch develop
"$MINIGIT" checkout develop

# Alteramos a mesma linha que a master vai alterar depois
echo "Linha 1: Alterada pela Develop" > compartilhado.txt
"$MINIGIT" add compartilhado.txt
"$MINIGIT" commit "C2: Muda linha 1 na develop"

echo "--- 3. Voltando para Master (Ours) ---"
# Simulamos nosso trabalho paralelo
"$MINIGIT" checkout master

# Alteramos a mesma linha com conteúdo diferente
echo "Linha 1: Alterada pela Master" > compartilhado.txt
"$MINIGIT" add compartilhado.txt
"$MINIGIT" commit "C3: Muda linha 1 na master"

echo "--- 4. Executando Merge (ESPERADO: CONFLITO) ---"
# Aqui o sistema deve parar e avisar do conflito
"$MINIGIT" merge develop

echo ""
echo "--- 5. Verificação do Resultado ---"

# Mostra o conteúdo do arquivo para garantir que os marcadores foram inseridos
echo "Conteúdo atual de compartilhado.txt:"
echo "----------------------------------------"
cat compartilhado.txt
echo -e "\n----------------------------------------"

# Validação crítica para o seu fix do MERGE_HEAD
echo ""
echo "Verificando estado interno do .minigit:"
if [ -f .minigit/MERGE_HEAD ]; then
    echo "[SUCESSO] Arquivo .minigit/MERGE_HEAD existe."
    echo "Hash contido em MERGE_HEAD: $(cat .minigit/MERGE_HEAD)"
else
    echo "[ERRO CRÍTICO] O arquivo .minigit/MERGE_HEAD não foi criado! O commit final ficará errado."
fi

echo ""
echo "--- 6. Como prosseguir agora? ---"
echo "Para finalizar o teste manualmente, execute:"
echo "1. Edite compartilhado.txt removendo os marcadores (<<<<, ====, >>>>)"
echo "2. minigit add compartilhado.txt"
echo "3. minigit commit 'Merge com resolução de conflito' ou minigit merge --continue"

cd ..