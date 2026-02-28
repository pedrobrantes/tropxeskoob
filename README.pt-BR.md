# tropxeskoob

Exporte sua estante do Skoob para formatos JSON e CSV com facilidade.

O Skoob é uma rede social brasileira para leitores. Esta ferramenta permite que você faça o backup dos dados da sua estante para análise, migração ou registros pessoais.

[English (US)](README.md)

## Funcionalidades

- Login via e-mail/senha ou token JWT.
- Exporte todos os livros de sua estante (Lidos, Quero Ler, Lendo, etc.).
- Formatos de saída: JSON (dados completos) e CSV (achatado para planilhas).
- Tratamento automático de paginação.
- Limitação de taxa de API respeitosa.

## Instalação

### Usando Cargo

```bash
cargo install tropxeskoob
```

### Usando Nix

```bash
nix profile install github:pedrobrantes/tropxeskoob
```

## Uso

### Login Simples

```bash
tropxeskoob --email seu@email.com
```

### Usando um Token (Recomendado para Login Social)

Se você faz login no Skoob usando Google ou Facebook, você deve usar um token JWT.

```bash
tropxeskoob --token SEU_JWT_TOKEN --user-id SEU_USER_ID
```

Consulte [GET_TOKEN.pt-BR.md](GET_TOKEN.pt-BR.md) para instruções sobre como obter esses valores.

### Opções

```text
Uso: tropxeskoob [OPÇÕES]

Opções:
  -e, --email <EMAIL>        Seu e-mail do Skoob
  -p, --password <PASSWORD>  Sua senha do Skoob
  -t, --token <TOKEN>        Fornecer manualmente um token JWT
  -u, --user-id <USER_ID>    Fornecer manualmente um ID de usuário
  -o, --output <OUTPUT>      Caminho/nome base do arquivo de saída [padrão: full_bookshelf]
      --json                 Exportar para o formato JSON
      --csv                  Exportar para o formato CSV
  -h, --help                 Imprimir ajuda
  -V, --version              Imprimir versão
```

## Contribuindo

Consulte [CONTRIBUTING.md](CONTRIBUTING.md) para obter detalhes sobre nosso código de conduta e o processo de envio de pull requests.

## Licença

Este projeto é licenciado sob qualquer um dos seguintes:

- Apache License, Versão 2.0 ([LICENSE-APACHE](LICENSE-APACHE) ou http://www.apache.org/licenses/LICENSE-2.0)
- Licença MIT ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

à sua escolha.
