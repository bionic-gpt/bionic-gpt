---
name: website-screenshot
description: Capture and inspect rendered local websites from the devcontainer using Playwright and Chromium.
---

Run from the project root, outside the agent sandbox:

```sh
bash .agents/skills/website-screenshot/scripts/capture.sh http://localhost:8080
```

The script reuses cached tooling and saves full-page PNGs to
`./tmp/website-<UTC timestamp>.png`. An optional second argument sets the output
directory. Set `DEVCONTAINER_ID` to override automatic container identification.

Give brief progress updates and report timeouts promptly. Inspect the saved
image and provide a clickable file link so the user can see the result.
