# Minigit
Sistema de controle de versão escrito em Rust, baseado no Git.

## Como rodar localmente

Para listar os comandos:
```
cargo run --help
```

Para rodar um comando:
```
cargo run <comando>
```

## Como testar

```
mkdir test
cargo build --release
alias minigit='<caminho_absoluto_do_projeto>/target/release/minigit'
cd test
minigit <commando>
```