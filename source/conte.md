# reinhardt-web 13.5-minute Full-stack (WASM Frontend) Demo Storyboard

## Overview

- **Total runtime**: ~13:30 (13:30 target, flexible within 13:00–14:00)
- **Narration**: English, dubbed in afterward via ElevenLabs TTS (`eleven_v3`)
- **Subject matter**: the polls application from the **Reinhardt Basics Tutorial** — `https://reinhardt-web.dev/quickstart/tutorials/basis/`. The tutorial URL is the viewer's reference implementation; every line of code shown on screen maps to a chapter
- **Medium ratio**: Manim 60–70% / VSCode (static code shots) 30–40% / Browser real footage 5%. **No live typing animations.** Code appears as pre-written static shots or is painted in by Manim
- **Target audience**: Django/DRF users, Axum/Actix users looking for a batteries-included framework, Rust devs evaluating full-stack (WASM frontend) options
- **Goal**: Show that a Rust full-stack app — reactive WASM UI, shared types, ORM, admin, auth — can be described declaratively, with compile-time guarantees, in the same amount of prose as a Django polls tutorial

## Scene list

| # | Part | Time | Medium | Tutorial |
|---|------|------|--------|---|
| 1 | Opening / problem framing | 0:00–1:30 | Manim | — |
| 2 | Reinhardt intro (polylithic architecture) | 1:30–2:45 | Manim | — |
| 3 | Project setup + directory structure | 2:45–4:00 | Manim + Terminal | Part 1 |
| 4 | Models + Migrations (Question, Choice) | 4:00–5:45 | Manim + VSCode (static) | Part 2 |
| 5 | Server functions + reactive client page | 5:45–8:15 | Manim + VSCode (static) | Part 3 |
| 6 | Forms (`form!` + `watch`) | 8:15–9:45 | Manim + VSCode (static) | Part 4 |
| 7 | Admin | 9:45–10:45 | Manim + Browser | Part 7 |
| 8 | DI + Auth (beyond the tutorial) | 10:45–12:15 | Manim | — |
| 9 | Comparison + wrap-up | 12:15–13:30 | Manim | — |

---

## Scene details

### Scene 1: Opening / problem framing (0:00–1:30)

**Medium**: Manim

**Screen composition**:

- (0:00–0:15) A black screen fades in to the line `Rust is fast.` Speed bars for Rust / Go / Python / Ruby extend from left to right; Rust runs away from the pack
- (0:15–0:40) `But building a web app takes...` appears. Crate-name tiles — `axum`, `tower`, `sea-orm`, `utoipa`, `jwt`, `tracing`, `sqlx-cli` — rain down and get connected by arrows into a dense dependency graph (the "wiring explosion")
- (0:40–1:10) The Django logo appears on the right. As `django-admin startproject` is typed, ORM / Admin / Auth / Migration / Serializer blocks pop in all at once. Contrast it with the Rust side on the left
- (1:10–1:30) `Django: 1 command.` and `Rust: 10+ crates.` line up with a giant `?` between them. Wipe into Scene 2

**Narration (English)**:

> (0:00) Rust is fast.
> (0:05) Memory-safe, type-checked at compile time, fearless about concurrency.
> (0:15) But the moment you try to build a real web application in Rust, things get rough.
> (0:30) Router, ORM, auth, OpenAPI, migrations —
> (0:45) you end up assembling them yourself, crate by crate.
> (1:00) Meanwhile in Django, one command gives you ORM, admin, auth, and more.
> (1:15) What if we could keep Rust's performance — and get back Django's productivity?

**Captions**:

- Bottom-right: `Rust is fast, but slow to ship.`
- End: `What if Django was written in Rust?`

**Word count**: ~105 words

---

### Scene 2: Reinhardt intro — polylithic architecture (1:30–2:45)

**Medium**: Manim

**Screen composition**:

- (1:30–1:45) Logo animation. `Reinhardt` emerges at center; the subtitle `🦀 Django's productivity, Rust's performance` fades in
- (1:45–2:20) Polylithic architecture diagram. A central `reinhardt` hub is surrounded by nodes `reinhardt-core`, `reinhardt-orm`, `reinhardt-di`, `reinhardt-auth`, `reinhardt-admin`, `reinhardt-api`, `reinhardt-pages`, `reinhardt-graphql`. Each node pulses independently to convey partial adoption
- (2:20–2:45) The episode goal appears at the bottom: `Build a full-stack polls app in ~13 minutes`, followed by the 5-step strip `Model → Migration → Server fn → Reactive UI → Form → Admin` scrolling horizontally. A small ribbon under the strip reads: `Follows the official Basics Tutorial → reinhardt-web.dev/quickstart/tutorials/basis/`

**Narration (English)**:

> (1:30) That's why we built Reinhardt.
> (1:38) A framework that fuses Django's philosophy with Rust's type safety.
> (1:50) Its defining trait is a polylithic architecture.
> (1:58) Use the full stack, or pull in just the ORM, or just the DI container — your call.
> (2:12) Today we'll build the polls app from the official basics tutorial —
> (2:25) a full-stack WASM application: model, migrations, server functions, a reactive form, and admin.
> (2:38) All the code on screen is in the tutorial. Follow along at your own pace.

**Captions**:

- Center: `Polylithic = pick what you need`
- Bottom (2:38–): `reinhardt-web.dev/quickstart/tutorials/basis/`

**Word count**: ~105 words

---

### Scene 3: Project setup + directory structure (2:45–4:00)

**Medium**: Manim (primary) + Terminal (brief)

**Tutorial mapping**: Part 1 — Project Setup

**Screen composition**:

- (2:45–3:00) **Terminal** (real recording, not typed live — played back at 1.5×). Two commands executed back-to-back:
  ```
  $ reinhardt-admin startproject polls_project --template pages
  $ cd polls_project && reinhardt-admin startapp --with-pages polls
  ```
  Success lines print in accent orange
- (3:00–3:30) **Manim M3a**: the generated tree paints itself in stages. Root first (`Cargo.toml`, `Makefile.toml`, `build.rs`, `settings/`), then the three pillars — `src/apps/`, `src/server_fn/`, `src/shared/`, `src/client/` — fly in from left/right. Each pillar gets a one-line role caption:
  - `src/apps/polls/` — models, views, admin
  - `src/server_fn/` — typed RPC bridge
  - `src/shared/types.rs` — DTOs (server + WASM)
  - `src/client/` — WASM UI (router, pages, components)
- (3:30–3:50) **Manim M3b**: a dashboard of `cargo make` tasks lights up one by one — `dev`, `makemigrations`, `migrate`, `test`, `collectstatic`, `wasm-build-dev`, `showurls`, `check`. Each task card is a rounded badge that pulses on reveal
- (3:50–4:00) Transition: the `src/apps/polls/` pillar zooms in and wipes into Scene 4

**Narration (English)**:

> (2:45) Two commands scaffold the whole thing.
> (2:55) `startproject` with the pages template gives you a Rust crate plus a WASM client in one layout.
> (3:08) `startapp` adds the polls app, wired into `config/apps.rs` automatically.
> (3:22) Three pillars to notice: `apps/` holds the Django-style model-view layout, `shared/types.rs` holds the DTOs used by both sides, and `client/` is your WASM frontend.
> (3:42) `cargo make` is the one-stop task runner — migrate, test, build WASM, collect static files, all here.

**Captions**:

- Top-left throughout: `Part 1 / 7 — Project Setup`
- On pillars: orange highlight when mentioned

**Word count**: ~100 words

---

### Scene 4: Models + Migrations (4:00–5:45)

**Medium**: Manim + VSCode static code shot

**Tutorial mapping**: Part 2 — Models and Database

**Screen composition**:

- (4:00–4:25) **VSCode static shot** of `src/apps/polls/models.rs` (full file fits on screen). The `#[model]` attribute line and the `#[field]` attribute lines pulse in accent orange as narration hits them:
  ```rust
  use serde::{Deserialize, Serialize};
  use chrono::{DateTime, Utc};
  use reinhardt::prelude::*;
  use reinhardt::db::associations::ForeignKeyField;

  #[model(app_label = "polls", table_name = "questions")]
  #[derive(Serialize, Deserialize)]
  pub struct Question {
      #[field(primary_key = true)]
      id: i64,

      #[field(max_length = 200)]
      question_text: String,

      #[field(auto_now_add = true)]
      pub_date: DateTime<Utc>,
  }

  #[model(app_label = "polls", table_name = "choices")]
  #[derive(Serialize, Deserialize)]
  pub struct Choice {
      #[field(primary_key = true)]
      id: i64,

      #[rel(foreign_key, related_name = "choices")]
      question: ForeignKeyField<Question>,

      #[field(max_length = 200)]
      choice_text: String,

      #[field(default = 0)]
      votes: i32,
  }
  ```
- (4:25–4:55) **Manim M4a**: the `#[model]` macro is shown on the left; arrows fan out to synthesized symbols on the right — `Question::objects()`, `Question::field_question_text()`, `Question::new()`, `Question::pk()`. A compile-time checkmark stamp lands on each. Caption: `macro-synthesized, checked at compile time`
- (4:55–5:20) **Manim M4b**: Question–Choice ERD. Two tables with the foreign-key arrow labeled `question: ForeignKeyField<Question>`. The `related_name = "choices"` badge peels off the arrow onto the Question side. A tooltip bubble shows `question.choices()` resolving to `Vec<Choice>`
- (5:20–5:45) **Terminal** playback:
  ```
  $ cargo make makemigrations
  Created migrations/polls/0001_initial.rs
  $ cargo make migrate
  Applied polls.0001_initial
  ```
  Camera tilts to show `migrations/polls/0001_initial.rs` — a pure Rust file (no raw SQL) — then returns

**Narration (English)**:

> (4:00) First, the models.
> (4:05) The `#[model]` attribute turns a plain struct into an ORM entity. Per-field constraints go in the `#[field]` attribute — primary key, max length, auto-add timestamps.
> (4:25) Every constraint is checked at compile time. The macro synthesizes typed accessors — `Question::objects()`, `Question::field_question_text()`, and friends — so there are no stringly-typed column names.
> (4:55) `Choice` adds a foreign key back to `Question`. The `related_name` lets you traverse the relation by name, and the type system knows the shape on both sides.
> (5:20) Migrations: `makemigrations` emits a Rust file you can git-diff. `migrate` applies it. No raw SQL in your review queue.

**Captions**:

- Top-left: `Part 2 / 7 — Models & Database`
- On macro: `#[model]` → orange halo

**Word count**: ~125 words

---

### Scene 5: Server functions + reactive client page (5:45–8:15)

**Medium**: Manim (primary) + VSCode static code shots

**Tutorial mapping**: Part 3 — Views and URLs

**Screen composition**:

- (5:45–6:10) **Manim M5a**: architecture diagram. A horizontal timeline — `WASM client` on the left, `server_fn` in the middle, `Database` on the right. A request flows left-to-right; the response flows back. The `#[server_fn]` attribute is highlighted as the *bridge*. A box labeled `src/shared/types.rs` sits above the timeline; arrows from both client and server touch it. Caption: `one type definition, two sides, zero schema drift`
- (6:10–6:50) **VSCode static shot** of `src/server_fn/polls.rs`. Focus on `get_questions` and `get_question_detail`:
  ```rust
  #[server_fn]
  pub async fn get_questions(
      #[inject] _db: reinhardt::DatabaseConnection,
  ) -> Result<Vec<QuestionInfo>, ServerFnError> {
      let manager = Question::objects();
      let questions = manager.all().all().await
          .map_err(|e| ServerFnError::application(e.to_string()))?;
      Ok(questions.into_iter().take(5).map(QuestionInfo::from).collect())
  }

  #[server_fn]
  pub async fn get_question_detail(
      question_id: i64,
      #[inject] _db: reinhardt::DatabaseConnection,
  ) -> Result<(QuestionInfo, Vec<ChoiceInfo>), ServerFnError> {
      /* … as in tutorial Part 3 … */
  }
  ```
  `#[server_fn]` pulses orange. `#[inject]` pulses amber. `QuestionInfo` and `ChoiceInfo` pulse cyan to signal they come from `shared/types.rs`
- (6:50–7:20) **Manim M5b**: reactive cycle animation. A circle with four arcs: `dispatch` → `await RPC` → `signal update` → `re-render`. A small `use_action` badge orbits inside. A timeline below shows: `load_questions.dispatch(())` → `await` → `Action<Vec<QuestionInfo>, String>` state changes to `Ok(..)` → `watch { … }` repaints the DOM. Caption: `declarative reactivity, no VDOM diffing`
- (7:20–7:50) **VSCode static shot** of `src/client/components/polls.rs`:
  ```rust
  pub fn polls_index() -> Page {
      let load_questions = use_action(|_: ()| async move {
          get_questions().await.map_err(|e| e.to_string())
      });
      load_questions.dispatch(());

      let error_signal = load_questions.clone();
      let data_signal = load_questions.clone();

      page!(|error_signal: Action<Vec<QuestionInfo>, String>,
              data_signal: Action<Vec<QuestionInfo>, String>| {
          div {
              class: "max-w-4xl mx-auto px-4 mt-12",
              h1 { class: "mb-4", "Polls" }
              watch {
                  if let Some(err) = error_signal.error() {
                      div { class: "alert-danger", { err } }
                  }
              }
              watch {
                  if let Some(list) = data_signal.ok() {
                      ul {
                          { list.iter().map(|q| rsx! {
                              li { a { href: format!("/polls/{}/", q.id), "{q.question_text}" } }
                          }) }
                      }
                  }
              }
          }
      })(error_signal, data_signal)
  }
  ```
  `page!`, `use_action`, and `watch` pulse orange as they are named in narration
- (7:50–8:05) **VSCode static shot** of `src/client/router.rs` routes:
  ```rust
  Router::new()
      .route("/", || index_page())
      .route("/polls/{question_id}/", || { /* polls_detail_page */ })
      .route("/polls/{question_id}/results/", || { /* polls_results_page */ })
      .not_found(|| error_page("Page not found"))
  ```
- (8:05–8:15) Transition: a small browser window pops up in the corner showing the polls list rendering. No sound, no interaction — just proof it works. Wipe to Scene 6

**Narration (English)**:

> (5:45) The bridge between WASM and server is the `#[server_fn]` attribute.
> (5:58) A plain async function, annotated once, becomes a typed RPC the frontend can call directly. No JSON hand-rolling, no OpenAPI layer.
> (6:15) Arguments marked `#[inject]` are resolved by the DI container — the database connection just appears.
> (6:35) Return types come from `shared/types.rs`. Backend and frontend share the same Rust type. Schema drift, gone.
> (6:55) On the client, `use_action` turns the RPC into a reactive resource. Dispatch it, and a `watch` block in the `page!` macro re-renders the moment the response arrives.
> (7:25) No JSX, no virtual DOM diff — just Rust expressions inside a macro.
> (7:50) Register the routes, hit the URL, and the list renders.

**Captions**:

- Top-left: `Part 3 / 7 — Views and URLs`
- Bottom (7:50–): `http://localhost:8000/polls/`

**Word count**: ~160 words

---

### Scene 6: Forms — `form!` macro (8:15–9:45)

**Medium**: Manim + VSCode static code shot

**Tutorial mapping**: Part 4 — Forms and Generic Views

**Screen composition**:

- (8:15–8:40) **Manim M6a**: the `form!` macro is shown as a declarative blueprint on the left. Arrows fan out to generated artifacts on the right: `HTML <form>`, `field validators`, `submit handler`, `reactive state`. A caption reads: `one declaration, everything downstream generated`
- (8:40–9:15) **VSCode static shot** of the `VotingForm` declaration in `src/client/components/polls.rs`:
  ```rust
  let voting_form = form! {
      name: VotingForm,
      server_fn: submit_vote,
      method: Post,
      state: { loading, error },

      fields: {
          question_id: HiddenField {
              initial: qid.to_string(),
          },
          choice_id: ChoiceField {
              widget: RadioSelect,
              required,
              label: "Select your choice",
              class: "form-check",
              choices_from: "choices",
              choice_value: "id",
              choice_label: "choice_text",
          },
      },

      watch: {
          submit_button: |form| { /* disabled while loading */ },
          error_display: |form| { /* render form.error */ },
          success_navigation: |form| { /* go to /results/ on ok */ },
      },
  };
  ```
  Keywords `form!`, `fields:`, `watch:` pulse orange in sequence
- (9:15–9:35) **Manim**: the three `watch` blocks animate as three parallel tracks. Each track shows how form state (`loading`, `error`, `ok`) flows into DOM updates. Caption: `watch blocks = localized reactive bindings`
- (9:35–9:45) Brief browser recording of the voting form rendered, a radio selected, click → navigates to the results page

**Narration (English)**:

> (8:15) Forms. The `form!` macro is the single recommended path — there is no separate generic-view layer.
> (8:35) You declare the server function to submit to, the fields with their widgets and validation, and the reactive watchers for the submit button, error display, and success navigation.
> (9:00) The macro generates the HTML, wires validation, and binds every `watch` block to form state — loading, error, or ok.
> (9:25) No templating engine. No controller class. Just a declaration that compiles to a WASM component.

**Captions**:

- Top-left: `Part 4 / 7 — Forms`

**Word count**: ~90 words

---

### Scene 7: Admin (9:45–10:45)

**Medium**: Manim concept + Browser real footage

**Tutorial mapping**: Part 7 — Admin Customization

**Screen composition**:

- (9:45–10:05) **Manim M7a**: `admin_site.register::<Question>(ModelAdmin::default())` on the left. An arrow points to a browser frame on the right showing `/admin/polls/question/` materialized as a list view. A second line shows `register::<Choice>(…)` doing the same. Caption: `zero custom HTML, full CRUD UI`. A short VSCode-like snippet is pinned:
  ```rust
  pub fn register_admin(admin_site: &mut AdminSite) {
      admin_site.register::<Question>(ModelAdmin::default());
      admin_site.register::<Choice>(ModelAdmin::default());
  }
  ```
- (10:05–10:30) **Browser recording** (real, pre-recorded):
  - `createsuperuser` command finishes in terminal
  - Navigate to `/admin/` — login
  - Admin dashboard shows Polls section with Question and Choice
  - Click Question → list, click one → edit form, change `question_text`, save
- (10:30–10:45) Back to the Manim frame with a tally: `✓ List ✓ Create ✓ Edit ✓ Delete ✓ Filter — 3 lines of Rust`

**Narration (English)**:

> (9:45) The admin. Register a model, get a CRUD UI.
> (9:55) Three lines of Rust — one `register` call per model — and the admin routes, forms, filters, and pagination all exist.
> (10:10) `createsuperuser` is the same command, same spirit, as Django.
> (10:22) Every column, filter, and edit form you see was generated from the model definition alone.
> (10:38) Custom layouts and search live on `ModelAdmin` — but the default is already production-shaped.

**Captions**:

- Top-left: `Part 7 / 7 — Admin`
- Tally card (10:30–): `CRUD for free`

**Word count**: ~85 words

---

### Scene 8: DI + Auth (beyond the tutorial) (10:45–12:15)

**Medium**: Manim primary

**Tutorial mapping**: *not covered in the Basics tutorial* — this is Reinhardt's differentiator, included so viewers understand the scaling story

**Screen composition**:

- (10:45–11:00) Title card: `Beyond the tutorial: DI + Auth`. Subtitle: `same declarative shape, statically enforced`
- (11:00–11:30) **Manim M8a**: provider registry diagram (adapted from previous Scene 9a).
  - Handler signature on top: `async fn cast_vote(#[inject] repo: ChoiceRepository, #[inject] user: AuthUser<User>, #[inject] _: Guard<IsActiveUser>, …)`
  - Below, the provider registry box shows registered factories: `ChoiceRepository`, `DatabaseConnection`, `AuthBackend`
  - Arrows at **compile time** (labeled with a clock icon) wire each `#[inject]` argument to its factory. Missing registrations would be a *compile error* — shown with a red strike-through on a hypothetical missing entry
  - Caption: `provider resolution at compile time — no runtime reflection`
- (11:30–12:00) **Manim M8b**: guard chain animation.
  - Request packet flows left-to-right through a pipeline: `→ AuthBackend` → `→ Guard<IsActiveUser>` → `→ handler body`
  - A second packet without a token bounces off `AuthBackend` with `401`
  - A third packet with a valid-but-inactive token passes `AuthBackend` but bounces off `IsActiveUser` with `403`
  - Only a valid active token reaches the handler, returning `200`
  - Small VSCode-style inlaid snippet (static):
    ```rust
    #[server_fn]
    pub async fn cast_vote(
        input: VoteInput,
        #[inject] AuthUser(user): AuthUser<User>,
        #[inject] _: Guard<IsActiveUser>,
        #[inject] choices: ChoiceRepository,
    ) -> Result<ChoiceInfo, ServerFnError> { /* … */ }
    ```
  - Caption: `401 → 403 → 200, before your handler body runs`
- (12:00–12:15) A bottom ribbon teases: `See the REST tutorial for full auth code → reinhardt-web.dev/quickstart/tutorials/rest/`

**Narration (English)**:

> (10:45) The basics tutorial stops here — but polls gets more interesting with authentication.
> (10:55) Reinhardt's DI container wires dependencies at compile time.
> (11:05) Each `#[inject]` argument resolves to a registered factory. Missing a factory? It's a compile error, not a runtime panic.
> (11:25) Guards are injectable too. A `Guard<IsActiveUser>` checks the user's state before your handler body runs.
> (11:45) No token? The auth layer returns 401. Inactive user? The guard returns 403. Only active authenticated calls reach your code.
> (12:05) Declarative to write, statically enforced. See the REST tutorial for the full implementation.

**Captions**:

- Top-left: `Bonus — DI + Auth`
- Bottom ribbon (12:00–): `reinhardt-web.dev/quickstart/tutorials/rest/`

**Word count**: ~115 words

---

### Scene 9: Comparison + wrap-up (12:15–13:30)

**Medium**: Manim

**Screen composition**:

- (12:15–12:55) On the left, a vertical achievement checklist:
  - ✅ Model (`#[model]`)
  - ✅ Migrations (Rust-diffable)
  - ✅ Server fn (`#[server_fn]` + `#[inject]`)
  - ✅ Reactive UI (`page!` + `use_action` + `watch`)
  - ✅ Forms (`form!`)
  - ✅ Admin (`ModelAdmin`)
  - ✅ DI + Auth (`Guard<…>`)
  On the right, a counter tallies `~100 lines of your own Rust` (vs. `600+ lines` of hand-wired Axum + sea-orm + utoipa equivalent on a greyed-out panel)
- (12:55–13:20) Call to action card:
  ```
  $ cargo install reinhardt-admin-cli
  $ reinhardt-admin startproject my_app --template pages
  ```
  With links: `reinhardt-web.dev/quickstart/tutorials/basis/` (top) and `github.com/kent8192/reinhardt-web` (bottom)
- (13:20–13:30) Logo fades out; close with `Django's productivity. Rust's performance.` centered

**Narration (English)**:

> (12:15) Thirteen minutes. That's what we just did.
> (12:25) Model, migrations, server functions, reactive UI, forms, admin, DI, auth —
> (12:40) roughly a hundred lines of Rust you wrote yourself. The same stack assembled by hand out of Axum, sea-orm, utoipa, and a separate frontend framework would take six hundred lines and a lot of design decisions.
> (13:00) Everything on screen lives in the basics tutorial. Clone it, run it, modify it.
> (13:15) `cargo install reinhardt-admin-cli` — start today.
> (13:25) See you next time.

**Captions**:

- Final center: `Django's productivity. Rust's performance.`
- Bottom: `reinhardt-web.dev/quickstart/tutorials/basis/`

**Word count**: ~105 words

---

## Narration budget check

| Scene | Runtime | Words | Words/min |
|-------|---------|-------|-----------|
| 1 | 1:30 | ~105 | 70 |
| 2 | 1:15 | ~105 | 84 |
| 3 | 1:15 | ~100 | 80 |
| 4 | 1:45 | ~125 | 71 |
| 5 | 2:30 | ~160 | 64 |
| 6 | 1:30 | ~90 | 60 |
| 7 | 1:00 | ~85 | 85 |
| 8 | 1:30 | ~115 | 77 |
| 9 | 1:15 | ~105 | 84 |
| **Total** | **~13:30** | **~990** | avg 73 |

Kept sparser than before — the Manim animations need breathing room. Dense technical words are sprinkled with pauses of half a second to a second.

## Production notes

- **VSCode shots**: pre-composed static screenshots (not live typing). `JetBrains Mono` 14pt, Tokyo Night dark theme, rust-analyzer inlay hints on. Pan/zoom transitions handled in moviepy
- **Terminal shots**: zsh + Starship. Fully opaque black background; prompt simplified to `$`. Recordings are sped up 1.5×–2× to keep pacing tight
- **Manim**: background `#1a1b26`, accent `#f74c00`, Rust red `#ce422b`, Django green `#0c4b33`. Font matches VSCode shots for continuity. Elbow-arrow layout as established in `source/manim.ipynb`
- **Macro highlight convention**: every macro (`#[model]`, `#[server_fn]`, `#[inject]`, `form!`, `page!`, `watch`, `use_action`) pulses in accent orange the first time it appears in each scene
- **Shared-type convention**: types from `src/shared/types.rs` (e.g. `QuestionInfo`, `ChoiceInfo`, `VoteInput`) always render in cyan (`#7aa2f7`) to visually signal "this crosses the WASM boundary"
- **BGM**: slow synth for Scenes 1, 2, 9; lo-fi hip-hop for 3–8
- **Reference rigor**: every Rust snippet on screen must also appear, verbatim or near-verbatim, in the corresponding tutorial page. Before recording, diff against `https://reinhardt-web.dev/quickstart/tutorials/basis/<part>/` and the reference repo `examples/examples-tutorial-basis/`
