# Security Policy

Windjammer UI is a UI component library and reactive runtime for Windjammer. Security reports are welcome, and we aim to answer them quickly and kindly.

## Supported versions

The default branch is the supported line. Fixes land there first; tagged releases pick them up as patch releases.

## Reporting a vulnerability

Please report vulnerabilities privately rather than in a public issue:

1. Open a [GitHub Security Advisory](https://github.com/jeffreyfriedman/windjammer-ui/security/advisories/new) for this repository (preferred).
2. If advisories are unavailable to you, open an issue titled "Security contact request" with no technical details, and a maintainer will start a private conversation with you.

Helpful details: the affected path or package, the commit or version, steps to reproduce, a proof of concept if you have one, and the impact you expect.

## What to expect

- Acknowledgement within a few days.
- A status update once the report is triaged, with a severity assessment.
- A fix on the default branch, then a published advisory crediting you (if you wish).

## Scope notes

- Components render HTML strings. Escape untrusted text before it reaches a component that renders raw markup, and review any use of raw-HTML helpers in your own app.
- The WASM and SSR paths process input from the browser or the network; reports of injection, sandbox or denial-of-service issues there are in scope.
- Issues in the Windjammer compiler itself belong in the [windjammer](https://github.com/jeffreyfriedman/windjammer) repository.
