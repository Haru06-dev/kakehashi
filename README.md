# 架け橋 Kakehashi

A native Windows desktop app for learning Japanese — built with [Tauri](https://tauri.app/), running entirely offline (including AI features) with no cloud API required.

Kakehashi (架け橋, "bridge") aims to be a single, cohesive toolkit covering grammar structure, spaced-repetition vocabulary review, shadowing practice, JLPT-level content, and conversational practice with a local AI partner — all backed by real open dictionary data, not just flashcards.

## Features

- **Grammar Visualizer** — paste any Japanese sentence and see it broken into particle-highlighted chunks, with AI-generated furigana and romaji
- **SRS Flashcards** — spaced-repetition review across multiple decks, with mastery tracking
- **Shadowing** — practice pronunciation against native TTS audio, with speech-recognition feedback comparing your attempt
- **Sentence Builder** — drag-and-order sentence chunks, with AI-generated grammar insight (or a rule-based fallback if AI is off)
- **Vocabulary & Kanji** — tag-based organization, real pitch-accent data via [Kanjium](https://github.com/mifunetoshiro/kanjium), bulk import of JLPT-tagged word/kanji lists, search, and a "learned" tag for progress tracking
- **JLPT Prep** — kanji grids, reading passages (including AI-generated passages grounded in your own imported vocab/kanji for a given level), and per-level progress bars
- **AI Partner** — a local-AI Japanese conversation partner with multi-chat support (star/rename/delete), gentle mistake correction, and voice mode
- **Home Dashboard** — streak tracking, due cards, recent chats, and JLPT progress at a glance
- **Data Manager** — full CRUD across every data type, bulk delete with select-all, and JSON backup/restore

All AI features run **fully on-device** via [WebLLM](https://github.com/mlc-ai/web-llm) over WebGPU — no server, no API key, no data leaving your machine. The rest of the app works with zero AI dependency at all.

## Requirements

- **Windows 10/11** with WebView2 (ships with Windows by default on most systems)
- **A GPU with WebGPU support** (via DirectX 12) for AI features — most GPUs from the last several years qualify. Non-AI features work regardless.
- Internet connection for: first-time AI model download (a few GB, one-time), and any JLPT/Kanjium dataset imports

## Tech Stack

- **Frontend**: vanilla HTML/CSS/JS (no framework, no build step) + Tailwind CDN
- **Shell**: [Tauri v2](https://tauri.app/) (Rust + WebView2)
- **Local AI**: [WebLLM](https://github.com/mlc-ai/web-llm) (Qwen3 family models, quantized)
- **Storage**: IndexedDB for app data; WebLLM manages its own separate model cache
- **Data sources**: [Kanjium](https://github.com/mifunetoshiro/kanjium) (pitch accent, CC BY-SA), [davidluzgouveia/kanji-data](https://github.com/davidluzgouveia/kanji-data) (kanji + JLPT levels, MIT), [jamsinclair/open-anki-jlpt-decks](https://github.com/jamsinclair/open-anki-jlpt-decks) (JLPT vocab, MIT)

## Building from Source

```powershell
# Install dependencies
npm install

# Dev mode (fast iteration, opens a live window)
npx tauri dev

# Production build (creates an installer)
npx tauri build
```

The built installer lands in `src-tauri/target/release/bundle/nsis/`.

**First build note**: Tauri needs Rust (via [rustup](https://rustup.rs/)) and the MSVC C++ Build Tools on Windows. The very first `tauri dev`/`tauri build` compiles ~450 crates from scratch and can take 10-15+ minutes; subsequent builds are much faster since it's cached.

## Project Structure

```
KakehashiApp/
├── frontend/
│   └── index.html       # The entire app — UI, logic, everything
├── src-tauri/
│   ├── src/
│   │   ├── main.rs       # Entry point (also handles WebView2 data dir redirect)
│   │   └── lib.rs
│   ├── tauri.conf.json   # Tauri app configuration
│   └── icons/
└── package.json
```

## License

No license file has been added yet — all rights reserved by default until one is chosen. If you're the repo owner and want this open for others to use/modify, consider adding an [MIT](https://choosealicense.com/licenses/mit/) or [Apache 2.0](https://choosealicense.com/licenses/apache-2.0/) license.

Third-party datasets bundled/fetched by the app retain their own original licenses (see Data Sources above).
