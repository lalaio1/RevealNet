cat <<'EOF'
__________                          .__    _______          __    .__                 __         .__  .__
\______   \ _______  __ ____ _____  |  |   \      \   _____/  |_  |__| ____   _______/  |______  |  | |  |
 |       _// __ \  \/ // __ \\__  \ |  |   /   |   \_/ __ \   __\ |  |/    \ /  ___/\   __\__  \ |  | |  |
 |    |   \  ___/\   /\  ___/ / __ \|  |__/    |    \  ___/|  |   |  |   |  \\___ \  |  |  / __ \|  |_|  |__
 |____|_  /\___  >\_/  \___  >____  /____/\____|__  /\___  >__|   |__|___|  /____  > |__| (____  /____/____/
        \/     \/          \/     \/              \/     \/               \/     \/            \/
EOF

set -e

REPO_URL="https://github.com/lalaio1/RevealNet"
INSTALL_DIR="$HOME/.local/bin"
BIN_NAME="RevealNet"
ALIAS_NAME="revealnet"

echo "[*] Verificando dependencias..."

if ! command -v git &>/dev/null; then
    echo "[x] Git nao encontrado. Instale com: sudo apt install git"
    exit 1
fi

if ! command -v curl &>/dev/null && ! command -v wget &>/dev/null; then
    echo "[x] Nem curl nem wget foram encontrados. Instale um deles para continuar."
    exit 1
fi

echo "[*] Clonando repositório RevealNet (se necessario)..."
WORK_DIR="$(mktemp -d)"
cd "$WORK_DIR"

git clone --depth 1 "$REPO_URL" repo || {
    echo "[!] Repositório ja clonado, continuando..."
}

cd repo/build

if [ ! -f "$BIN_NAME" ]; then
    echo "[x] Binario '$BIN_NAME' nao encontrado em ./build/"
    exit 1
fi

echo "[*] Criando diretório de instalaçao: $INSTALL_DIR"
mkdir -p "$INSTALL_DIR"

echo "[*] Copiando binario para $INSTALL_DIR"
cp "$BIN_NAME" "$INSTALL_DIR/$ALIAS_NAME"
chmod +x "$INSTALL_DIR/$ALIAS_NAME"

SHELL_RC=""
if [ -n "$ZSH_VERSION" ]; then
    SHELL_RC="$HOME/.zshrc"
elif [ -n "$BASH_VERSION" ]; then
    SHELL_RC="$HOME/.bashrc"
else
    SHELL_RC="$HOME/.profile"
fi

if ! echo "$PATH" | grep -q "$INSTALL_DIR"; then
    echo "[*] Adicionando $INSTALL_DIR ao PATH em $SHELL_RC"
    echo -e "\n# RevealNet CLI\nexport PATH=\"\$PATH:$INSTALL_DIR\"" >> "$SHELL_RC"
    echo "[!] Reinicie o terminal ou rode: source $SHELL_RC"
fi

echo "[✓] Instalaçao concluida!"
source ~/.bashrc
echo "Agora voce pode executar a ferramenta com: $ALIAS_NAME"


