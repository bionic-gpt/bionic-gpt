curl -OL https://github.com/dobicinaitis/tailwind-cli-extra/releases/latest/download/tailwindcss-extra-linux-x64
chmod +x tailwindcss-extra-linux-x64
./tailwindcss-extra-linux-x64 -i ./input.css -o ./dist/tailwind.css

# Generate timestamp and rename tailwind.css file
TIMESTAMP=$(date +%s)
mv ./dist/tailwind.css ./dist/tailwind-${TIMESTAMP}.css

# Update the reference in the source code
sed -i "s|/tailwind\.css|/tailwind-${TIMESTAMP}.css|g" src/layouts/layout.rs

curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
. $HOME/.cargo/env
cargo run