# Axum Explainer — quick, friendly intro

A tiny, interactive Axum app that shows how a Rust web server works using server-side templates (Askama) and small HTMX fragments. It's meant to teach by example — short, readable code and a live UI you can poke at.

Why this project is great for beginners
- Small and focused: only a few files to read (`src/` and `templates/`).
- Hands-on: run it locally and explore the code while seeing changes in the browser.
- Uses real tools you'll meet in production: Axum, Askama, HTMX, Tokio.

Quick start — 1 command

Run this from the project root:

```bash
# default: 0.0.0.0:3001
AXUM_EXPLAINER_APP_HOST=0.0.0.0 AXUM_EXPLAINER_APP_PORT=3001 cargo run
```

Open http://127.0.0.1:3001 and click "Start with the Stack".

What you will learn (in 10 minutes)
- How `Router` maps URLs to handlers (`src/router.rs`).
- How to use `State` to share read-only data across handlers (`src/state.rs`).
- How handlers return HTML via Askama templates (`src/handlers/`).
- How small HTMX partials update parts of the page without a full reload (`templates/partials`).

Where to look first (guided path)
1. `src/main.rs` — app entry, logging, and server start.
2. `src/router.rs` — routes and static file serving.
3. `src/handlers/pages.rs` — full page handlers and template structs.
4. `templates/` — open `base.html` and `index.html` to see how data flows from Rust into HTML.

Tips for exploring
- Start the server and open the UI; then search for a template variable in `templates/` and find where it is set in `src/`.
- Try editing a template (small text), save, and refresh to see the change.
- The code is intentionally simple: many values are hard-coded in `src/models/content.rs` so you can read them easily.

Small, safe improvements you can try
- Replace an `unwrap` in `src/config.rs` with a small `match` to practice error handling.
- Add a tiny unit test for `NavCtx` (tests go in `tests/` or as `#[cfg(test)]` in a module).

If you want more
- See full README with development tips, linting, and recommended CI in the original README (kept in repo history).

License
- Add a `LICENSE` file if you plan to reuse this code publicly (MIT or Apache-2.0 are common).

Questions or stuck? Tell me what you want to explore next — I can make a guided walkthrough or small PR with step-by-step edits.
