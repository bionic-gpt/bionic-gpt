curl -OL https://github.com/dobicinaitis/tailwind-cli-extra/releases/latest/download/tailwindcss-extra-linux-x64
chmod +x tailwindcss-extra-linux-x64
./tailwindcss-extra-linux-x64 -i ./input.css -o ./dist/tailwind.css

# Generate timestamp and rename tailwind.css file
TIMESTAMP=$(date +%s)
mv ./dist/tailwind.css ./dist/tailwind-${TIMESTAMP}.css

export TAILWIND_STYLESHEET="/tailwind-${TIMESTAMP}.css"

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. $HOME/.cargo/env

curl -fsSL https://d2lang.com/install.sh | sh -s --
export PATH="$HOME/.local/bin:$PATH"

cargo run
