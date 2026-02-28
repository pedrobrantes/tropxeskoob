*[Versão em Português](GET_TOKEN.pt-BR.md)*

# How to get the Skoob Token on Android (Chrome)

In this case, the token is stored in a cookie named `tokenClient`.

## Script to Copy the Token

Copy the code below:

```javascript
javascript:prompt("Copy your Skoob Token:", document.cookie.match(/tokenClient=([^;]+)/)[1])
```

### Step by Step

1.  Go to [skoob.com.br](https://www.skoob.com.br) and log in.
2.  Tap on the Chrome **address bar**.
3.  **Type** `javascript:` manually.
4.  **Paste** the rest of the code: `prompt("Copy your Skoob Token:", document.cookie.match(/tokenClient=([^;]+)/)[1])`
    *   The final result in the bar should be exactly: `javascript:prompt("Copy your Skoob Token:", document.cookie.match(/tokenClient=([^;]+)/)[1])`
5.  Do not press **Enter**, tap on the recommendation below the navigation bar instead.
6.  A box will appear with the token (starting with `eyJ...`). Copy it.

## Using it in the CLI

With the copied token, run:

```bash
uv run main.py --token "PASTE_HERE"
```
*(The quotes are important)*
