# reinhardt-blog-demo

## Project Overview

A **15-minute demo video** production project for the Reinhardt full-stack web framework (a Django-like Rust framework). The demo shows how to build a REST API + WASM SPA in 15 minutes via live coding.

- **Script**: `source/conte.md` — 10 scenes, 15 minutes total; defines narration, screen layout, and captions
- **Target audience**: Django/DRF users, Axum/Actix users, Rust developers evaluating full-stack options
- **Language**: English (narration, code comments, commit messages, all written content)

## Directory Structure

```
source/
  conte.md          # Master script (changes here affect all notebooks)
  narrator.ipynb    # Generate per-scene narration audio via ElevenLabs TTS
  manim.ipynb       # Render concept-diagram animations (Scenes 1, 2, 9a, 10)
  moviepy.ipynb     # Composite all footage (in progress)
  audio/            # Generated .mp3 files
  media/            # Manim render output
  pyproject.toml    # Python deps managed by uv
  .venv/            # Virtual environment (source/.venv)
.demo/demo.yaml     # VSCode Demo Time extension script for live-coding scenes (3–9)
```

## Environment Setup

```bash
cd source
uv sync
```

Required env var:
- `ELEVENLABS_API_KEY` — needed to run narrator.ipynb

Python: `>=3.12.12`

Dependencies: `manim>=0.20.1`, `moviepy>=2.2.1`, `elevenlabs>=1.0.0`, `pyyaml>=6.0`, `ipykernel>=6.0`

## Notebook Roles and Execution Order

1. **`narrator.ipynb`** — Generates narration audio with ElevenLabs v3. Has a cache layer (text + model hash); unchanged scenes are skipped. Use `SCENES_TO_GENERATE` to target specific scenes.
2. **`manim.ipynb`** — Renders concept-diagram animations for Scenes 1, 2, 9a, 10.
3. **`moviepy.ipynb`** — Final composite of live-coding footage and Manim output (in progress).

## Design Conventions

Color scheme (Tokyo Night base):

| Usage | Hex |
|---|---|
| Background | `#1a1b26` |
| Foreground | `#c0caf5` |
| Accent (Reinhardt) | `#f74c00` |
| Rust | `#ce422b` |
| Django | `#0c4b33` |

## ElevenLabs Notes

- Each API call consumes credits — avoid unnecessary regeneration.
- narrator.ipynb caches by text + model hash; only changed scenes are re-generated.
- Set `SCENES_TO_GENERATE = [1, 2, 3]` to limit which scenes are processed.
- Model: `eleven_v3` (high quality, higher cost).

## Scene List

| # | Content | Time | Medium |
|---|---------|------|--------|
| 1 | Opening / problem framing | 0:00–1:30 | Manim |
| 2 | Reinhardt introduction | 1:30–2:30 | Manim |
| 3 | Project scaffolding | 2:30–4:00 | VSCode + Terminal |
| 4 | Model definition | 4:00–5:30 | VSCode |
| 5 | Migrations | 5:30–6:30 | Terminal |
| 6 | Serializer + ViewSet | 6:30–8:00 | VSCode |
| 7 | WASM frontend component | 8:00–10:30 | VSCode + Browser |
| 8 | Admin UI demo | 10:30–12:00 | Browser |
| 9 | DI / auth (concept + impl) | 12:00–13:30 | Manim + VSCode |
| 10 | Comparison and wrap-up | 13:30–15:00 | Manim |
