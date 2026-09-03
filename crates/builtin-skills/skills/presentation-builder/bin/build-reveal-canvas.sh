#!/bin/sh
set -eu

if [ "$#" -lt 3 ]; then
  echo "usage: build-reveal-canvas.sh <deck-name> <title> <slides-html-path>" >&2
  exit 2
fi

deck_name="$1"
title="$2"
slides_file="$3"
skill_dir="/home/user/skills/presentation-builder"

case "$deck_name" in
  *[!A-Za-z0-9._-]*|'')
    echo "deck-name must use only letters, numbers, dots, underscores, or hyphens" >&2
    exit 2
    ;;
esac

if [ ! -f "$slides_file" ]; then
  echo "slides HTML file not found: $slides_file" >&2
  exit 2
fi

output_dir="/home/user/output/$deck_name"
output_file="$output_dir/CANVAS.md"
mkdir -p "$output_dir"

{
  printf '%s\n' '---'
  printf 'name: %s\n' "$deck_name"
  printf 'title: %s\n' "$title"
  printf '%s\n' 'type: text/html'
  printf '%s\n' '---'
  printf '%s\n' '<!doctype html>'
  printf '%s\n' '<html lang="en">'
  printf '%s\n' '<head>'
  printf '%s\n' '<meta charset="utf-8">'
  printf '%s\n' '<meta name="viewport" content="width=device-width, initial-scale=1.0, maximum-scale=1.0, user-scalable=no">'
  printf '<title>%s</title>\n' "$title"
  printf '%s\n' '<style>'
  cat "$skill_dir/reveal/reset.css"
  cat "$skill_dir/reveal/reveal.css"
  cat "$skill_dir/reveal/theme/serif.css"
  printf '%s\n' '
html, body { margin: 0; width: 100%; height: 100%; overflow: hidden; }
.reveal { font-family: ui-serif, Georgia, Cambria, "Times New Roman", Times, serif; }
.reveal .slides section { box-sizing: border-box; }
.reveal h1, .reveal h2, .reveal h3 { letter-spacing: 0; }
.reveal table { font-size: 0.62em; }
'
  printf '%s\n' '</style>'
  printf '%s\n' '</head>'
  printf '%s\n' '<body>'
  printf '%s\n' '<div class="reveal"><div class="slides">'
  cat "$slides_file"
  printf '%s\n' '</div></div>'
  printf '%s\n' '<script>'
  sed 's#</script#<\\/script#g' "$skill_dir/reveal/reveal.js"
  printf '%s\n' '</script>'
  printf '%s\n' '<script>'
  printf '%s\n' 'Reveal.initialize({ controls: true, progress: true, center: true, hash: false, transition: "slide", embedded: false });'
  printf '%s\n' '</script>'
  printf '%s\n' '</body></html>'
} > "$output_file"

printf '%s\n' "$output_file"
