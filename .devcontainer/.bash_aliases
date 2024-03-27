# Git aliases.
alias gst='git status'
alias gcm='git checkout main'
alias c=clear
alias gp='git push'
alias gcam='git commit -a -m'
alias gpsup="git push --set-upstream origin $(git symbolic-ref -q HEAD | sed -e 's|^refs/heads/||')"
alias gcb='git checkout -b'
alias gcr='f() { git checkout -b $1 origin/$1; }; f'
alias gitsetup='git config --global user.name \$NAME && git config --global user.email \$EMAIL'
alias gsu='git submodule update --recursive --remote'
alias gdb='git branch | grep -v "main" | xargs git branch -D'

# Database
alias dburl='export DATABASE_URL=$(kubectl get secret database-urls -n bionic-gpt -o jsonpath="{.data.migrations-url}" | base64 --decode | sed "s/bionic-db-cluster-rw:5432/localhost:5432/")'
# dbport doesn't stay open for postgres https://github.com/kubernetes/kubernetes/issues/111825
alias dbport='kubectl port-forward -n bionic-gpt bionic-db-cluster-1 5432:5432'
alias dbmate='dbmate --no-dump-schema --migrations-dir /workspace/crates/db/migrations'
alias dbdown='while dbmate down; do :; done'
alias db='psql $DATABASE_URL'
alias dbapp='psql $APP_DATABASE_URL'

alias p='sudo chmod 777 /var/run/docker.sock'

# Watch
alias watch-zola='cd /workspace/website && zola serve --drafts --interface 0.0.0.0 --port 7704 --base-url localhost'
alias wz=watch-zola
alias watch-app='mold -run cargo watch --workdir /workspace/ -w crates/daisy-rsx -w crates/ui-pages -w crates/axum-server -w crates/db -w crates/asset-pipeline/dist -w crates/asset-pipeline/images --no-gitignore -x "run --bin axum-server"'
alias wa=watch-app
alias watch-pipeline='npm install --prefix /workspace/crates/asset-pipeline && npm run start --prefix /workspace/crates/asset-pipeline'
alias wp=watch-pipeline
alias watch-embeddings='mold -run cargo watch --workdir /workspace/ -w crates/embeddings-api -w crates/pipeline-job --no-gitignore -x "run --bin pipeline-job"'
alias we=watch-embeddings
alias watch-tailwind='cd /workspace/crates/asset-pipeline && npx tailwindcss -i ./input.css -o ./dist/output.css --watch'
alias wt=watch-tailwind

# Kind cluster
alias kc='mkdir -p ~/.kube && mv k3s.yaml ~/.kube/config'
alias kdburl='export DATABASE_URL=$(kubectl get secret database-urls -n bionic-gpt -o jsonpath="{.data.migrations-url}" | base64 --decode | sed "s/bionic-db-cluster-rw:5432/host.docker.internal:5432/")'

# mirrord
alias ma='mirrord exec -n bionic-gpt --steal -t deployment/bionic-gpt cargo watch -- --workdir /workspace/ -w crates/daisy-rsx -w crates/ui-pages -w crates/axum-server -w crates/db -w crates/asset-pipeline/dist -w crates/asset-pipeline/images --no-gitignore -x "run --bin axum-server"' 

# Spell check
alias spell='docker run --rm -ti -v $HOST_PROJECT_PATH/website/content:/workdir tmaier/markdown-spellcheck:latest "**/*.md"'

# Ollama
alias ollama='docker exec -it bionic-gpt_devcontainer_llm-api_1 ollama'