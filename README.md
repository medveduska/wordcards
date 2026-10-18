# Language Flashcards

Language Flashcards is a Rust and Yew web application for studying vocabulary from CSV and XLSX datasets.

## Features

- **Multi-wordset management**: Create and organize unlimited named wordsets (e.g., "HSK 1", "Week 3 vocabulary") all stored locally in your browser. A collection list shows card counts and the selected wordset. New users are guided from naming a wordset to importing a file or adding their first card. Wordset options, exports, and full backups are tucked into separate disclosures.
- **CSV/XLSX import and export**: Import UTF-8 CSV files or the first worksheet of an XLSX workbook into an empty wordset. Invalid files show actionable errors without importing partial data. Choose Export CSV or Export XLSX to download the active wordset with example sentences and known/unknown status preserved.
- **Full progress backup**: Download your complete app state (all wordsets and study position) as a JSON backup file, or restore from a previous backup to switch devices or recover from data loss.
- **Flexible study modes**: Switch between normal (character → pinyin → translation) and reverse (translation → pinyin → character) directions. Shuffle card order and randomize study sessions.
- **Three-stage reveal system**: Click a flashcard to progress through three stages (e.g., character, pinyin, meaning) at your own pace.
- **Example sentences**: Import an optional sentence in column 4 or enter it in Add New Flashcard. Click or tap Example sentence below a card to reveal or hide it; each card starts collapsed. Examples are saved locally and preserved in CSV/XLSX exports and full backups.
- **Word Review tracking**: Mark cards as known to move them to the Word Review table. Restore or delete mastered words, and view all known and unknown words in one place.
- **Automatic persistence**: All progress is saved automatically to browser local storage after every action. Works entirely offline after the page loads.

## Project Structure

The Rust workspace lives under `flashcards/`.

```text
flashcards/
|- Cargo.toml
|- Cargo.lock
|- frontend/
|  |- Cargo.toml
|  |- Trunk.toml
|  |- index.html
|  '- src/
|     |- app.rs
|     |- csv_io.rs
|     |- export_io.rs
|     |- import_io.rs
|     |- xlsx_export.rs
|     |- xlsx_io.rs
|     |- main.rs
|     |- model.rs
|     '- storage.rs
```

## Development

Format the workspace:

```powershell
cargo fmt --manifest-path .\flashcards\Cargo.toml
```

Check the workspace:

```powershell
cargo check --manifest-path .\flashcards\Cargo.toml
```

Run the frontend locally with Trunk:

```powershell
cd .\flashcards\frontend
trunk serve
```

For GitHub Pages deployments from this repository, Trunk is configured with `public_url = "/"` because the site is served from the custom domain `wordcards.chinesewithbaiba.eu`.

## Deployment

GitHub Pages publishing is automated by [.github/workflows/deploy-pages.yml](.github/workflows/deploy-pages.yml). Pushing changes to `master` rebuilds `flashcards/frontend` with Trunk and publishes `flashcards/frontend/dist` to the `gh-pages` branch.

You can also run the workflow manually with `workflow_dispatch` if you need to republish without a new commit.

Because Trunk 0.21.14 still injects a live-reload websocket client into the generated HTML for this app, the deploy workflow runs [scripts/strip-trunk-autoreload.ps1](scripts/strip-trunk-autoreload.ps1) after `trunk build --release` to remove that dev-only snippet before publishing.

## Notes

- Generated output such as `target/` and `frontend/dist/` is intentionally ignored.
- The frontend crate is organized by responsibility: UI in `app.rs`, domain types in `model.rs`, persistence in `storage.rs`, shared import validation in `import_io.rs`, export/download handling in `export_io.rs`, and format handling in `csv_io.rs`, `xlsx_io.rs`, and `xlsx_export.rs`.
- Import is available only when a wordset contains no cards, preventing silent overwrites; Export is always available for the active wordset.
- Import columns are `word,pinyin,translation[,example_sentence[,known]]` (A–E in XLSX). Word and translation are required; leave the pinyin cell blank when unused. Example sentence is optional in column 4. Known is optional in column 5 and may be blank, `true`, or `false` (Excel boolean cells are supported). Empty rows are ignored. XLSX vocabulary cells must contain text; numbers, dates, and spreadsheet errors are rejected.
- Matching headers are optional (`example_sentence`, `example sentence`, or `example` are accepted for column 4). Older four-column files using `known` as their header or headerless `true`/`false` in column 4 remain supported. Use an `example_sentence` header to import literal `true` or `false` as example text. Both export formats include headers and all five columns. XLSX exports use a Flashcards worksheet with frozen headers, filters, wrapped text, and a boolean known column. Existing saved wordsets and JSON backups without example sentences continue to load.
- CSV must use UTF-8 (with or without a BOM), commas, and valid double-quote escaping. Both formats reject replacement characters, unexpected controls, private-use/noncharacters, and common mojibake sequences while allowing multilingual text and combining pinyin tone marks. Some already-garbled text is still valid Unicode, so automatic detection cannot identify every encoding mistake.

