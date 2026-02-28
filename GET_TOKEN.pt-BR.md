*[English Version](GET_TOKEN.en-US.md)*

# Como obter o Token do Skoob no Android (Chrome)

No seu caso, o token está armazenado em um cookie chamado `tokenClient`.

## Script para Copiar o Token

Copie o código abaixo:

```javascript
javascript:prompt("Copie seu Token Skoob:", document.cookie.match(/tokenClient=([^;]+)/)[1])
```

### Passo a Passo

1.  Acesse [skoob.com.br](https://www.skoob.com.br) e faça login.
2.  Toque na **barra de endereços** do Chrome.
3.  **Digite** `javascript:` manualmente.
4.  **Cole** o restante do código: `prompt("Copie seu Token Skoob:", document.cookie.match(/tokenClient=([^;]+)/)[1])`
    *   O resultado final na barra deve ser exatamente: `javascript:prompt("Copie seu Token Skoob:", document.cookie.match(/tokenClient=([^;]+)/)[1])`
5.  Não aperte **Enter**, aperte na recomendação abaixo da barra de navegação.
6.  Uma caixa aparecerá com o token (começando com `eyJ...`). Copie-o.

## Usando no CLI

Com o token copiado, rode:

```bash
uv run main.py --token "COLE_AQUI"
```
*(As aspas são importantes)*