# reinhardt-web 15-minute Full-stack (WASM frontend) Demo Storyboard

## Overview

- **Total runtime**: 15:00
- **Narration**: English, dubbed in afterward via text-to-speech
- **Concept diagrams**: Manim (Scenes 1, 2, 9a, 10)
- **Live coding**: VSCode + macOS Terminal (Scenes 3–9b)
- **Target audience**: Django/DRF users, Axum/Actix users looking for a batteries-included framework, Rust devs evaluating full-stack (WASM frontend) options
- **Goal**: Show viewers that a Rust full-stack app — REST API + WASM SPA — can be delivered in fifteen minutes, Django-style

## Scene list

| # | Part | Time | Medium |
|---|------|------|--------|
| 1 | Opening / problem framing | 0:00-1:30 | Manim |
| 2 | Reinhardt intro | 1:30-2:30 | Manim |
| 3 | Project scaffolding (+ frontend) | 2:30-4:00 | VSCode + Terminal |
| 4 | Defining the model | 4:00-5:30 | VSCode |
| 5 | Migrations | 5:30-6:30 | Terminal |
| 6 | Serializer + ViewSet | 6:30-8:00 | VSCode |
| 7 | WASM frontend component | 8:00-10:30 | VSCode + Browser |
| 8 | Admin UI demo | 10:30-12:00 | Browser |
| 9 | DI / auth (concept + impl) | 12:00-13:30 | Manim + VSCode |
| 10 | Comparison and wrap-up | 13:30-15:00 | Manim |

---

## Scene details

### Scene 1: Opening / problem framing (0:00-1:30)

**Medium**: Manim

**Screen composition**:

- (0:00-0:15) A black screen fades in to the line `Rust is fast.` Speed bars for Rust / Go / Python / Ruby extend from left to right; Rust runs away from the pack
- (0:15-0:40) `But building a web app takes...` appears. Crate-name tiles — `axum`, `tower`, `sea-orm`, `utoipa`, `jwt`, `tracing`, `sqlx-cli` — rain down and get connected by arrows into a dense dependency graph (the "wiring explosion")
- (0:40-1:10) The Django logo appears on the right. As `django-admin startproject` is typed, ORM / Admin / Auth / Migration / Serializer blocks pop in all at once. Contrast it with the Rust side on the left
- (1:10-1:30) `Django: 1 command.` and `Rust: 10+ crates.` line up with a giant `?` between them. Wipe into Scene 2

**Narration (English)**:

> (0:00) Rust is fast.
> (0:05) Memory-safe, type-checked at compile time, fearless about concurrency.
> (0:15) But the moment you try to build a real web application in Rust, things get rough.
> (0:30) Router, ORM, auth, OpenAPI, migrations—
> (0:45) you end up assembling them yourself, crate by crate.
> (1:00) Meanwhile in Django, one command gives you ORM, admin, auth, and more.
> (1:15) What if we could keep Rust's performance — and get back Django's productivity?

**Captions**:

- Bottom-right: `Rust is fast, but slow to ship.`
- End: `What if Django was written in Rust?`

**Word count**: ~105 words

---

### Scene 2: Reinhardt intro (1:30-2:30)

**Medium**: Manim

**Screen composition**:

- (1:30-1:45) Logo animation. `Reinhardt` emerges at center; the subtitle `🦀 Django's productivity, Rust's performance` fades in
- (1:45-2:10) Polylithic architecture diagram. A central `reinhardt` hub is surrounded by nodes `reinhardt-core`, `reinhardt-orm`, `reinhardt-di`, `reinhardt-auth`, `reinhardt-admin`, `reinhardt-api`, `reinhardt-graphql`, `reinhardt-ws`. Each node pulses independently to convey that partial adoption is possible
- (2:10-2:30) The episode goal appears at the bottom: `Build a full-stack app in 15 minutes`, followed by the 5-step strip `Model → Migration → API → WASM UI → Auth` scrolling horizontally

**Narration (English)**:

> (1:30) That's why we built Reinhardt.
> (1:38) A framework that fuses Django's philosophy with Rust's type safety.
> (1:50) Its defining trait is a polylithic architecture.
> (1:58) Use the full stack, or pull in just the ORM, or just the DI container — your call.
> (2:10) Today, we'll build a full-stack app in fifteen minutes — REST API plus a WASM single-page frontend.
> (2:22) Model, migrations, API, a reactive UI, admin, authentication — all of it.

**Captions**:

- Under logo: `reinhardt-web v0.1`
- Bottom-right icons: 🦀 + 🐍 (Rust + Django)

**Word count**: ~100 words

---

### Scene 3: Project scaffolding (+ frontend) (2:30-4:00)

**Medium**: VSCode + macOS Terminal

**Screen composition**:

- (2:30-2:45) Full-screen Terminal (clean zsh prompt). Sanity-check the install with `reinhardt-admin --version` (`reinhardt-admin 0.1.0`)
- (2:45-3:10) Type and run `reinhardt-admin startproject fullstack_demo --with-pages`. The generated file tree prints out; highlight `manage.rs`, `Cargo.toml`, `src/`, `src/frontend/`, `config/`
- (3:10-3:30) `cd fullstack_demo && reinhardt-admin startapp posts`. Under `src/apps/posts/`, `models.rs`, `serializers.rs`, `views.rs`, `urls.rs` appear
- (3:30-3:50) Switch to VSCode. Show the project tree in the left pane; open `src/lib.rs` and highlight the generated structure, including `#[app_config(name = "posts")]` and the `src/frontend/` module
- (3:50-4:00) Open `Cargo.toml` and emphasize `reinhardt = { version = "0.1", features = ["full", "frontend"] }`. Then create `local.toml` for local environment overrides (never committed):

```toml
[core]
secret_key = "local-dev-secret-change-in-production"
debug = true

[core.database]
url = "sqlite://./db.sqlite3"

[auth.token]
signing_key = "local-dev-token-secret"
```

**Narration (English)**:

> (2:30) Let's start by scaffolding the project.
> (2:38) If you know Django, `startproject` will feel right at home — but this one takes a `--with-pages` flag.
> (2:52) One command gives you a manager script, a config module, an app directory, and a frontend module that compiles to WebAssembly.
> (3:10) Next, add a `posts` app. `startapp` separates models, views, and serializers into their own files from day one.
> (3:30) Open it in the editor. The backend tree looks like Django; the `frontend/` folder holds pages, components, and the reactive router.
> (3:45) A single `features = ["full", "frontend"]` in `Cargo.toml` pulls in ORM, admin, auth, and the WASM runtime — every battery, pre-charged.
> (3:55) Drop a `local.toml` for the database URL and token signing key — environment-specific values that never hit version control.

**Captions**:

- Command subtitle: `$ reinhardt-admin startproject fullstack_demo --with-pages`
- Bottom-right: `Django-style scaffolding + WASM frontend, in Rust`

**Word count**: ~155 words

---

### Scene 4: Defining the model (4:00-5:30)

**Medium**: VSCode (dark theme, JetBrains Mono 14pt)

**Screen composition**:

- (4:00-4:10) Open `src/apps/posts/models.rs` and add `use reinhardt::prelude::*;` with a typing animation
- (4:10-4:50) Type out the `Post` struct:

```rust
use chrono::{DateTime, Utc};
use reinhardt::prelude::*;
use serde::{Deserialize, Serialize};

#[model(app_label = "posts", table_name = "posts")]
#[derive(Serialize, Deserialize)]
pub struct Post {
    #[field(primary_key = true)]
    pub id: i64,

    #[field(max_length = 200)]
    pub title: String,

    #[field(max_length = 10000)]
    pub body: String,

    #[field(default = false)]
    pub published: bool,

    #[field(auto_now_add = true)]
    pub created_at: DateTime<Utc>,
}
```

- (4:50-5:10) Hover over `#[model(...)]`. Show rust-analyzer's popup listing the generated API: `Post::new`, `Post::objects()`, `Post::table()`, and so on
- (5:10-5:20) Append `impl Post { pub fn summary(&self) -> String { ... } }`. Emphasize that domain methods live in an ordinary `impl` block
- (5:20-5:30) Open `src/users.rs` and define the `User` model using `#[user]` — the macro generates `BaseUser`, `AuthIdentity`, and password-hashing boilerplate automatically:

```rust
use reinhardt::prelude::*;
use reinhardt_auth::hasher::Argon2Hasher;
use serde::{Deserialize, Serialize};

#[user(hasher = Argon2Hasher, username_field = "email")]
#[model(app_label = "users", table_name = "auth_user")]
#[derive(Serialize, Deserialize)]
pub struct User {
    #[field(primary_key = true)]
    pub id: i64,

    #[field(max_length = 150, unique = true)]
    pub username: String,

    #[field(max_length = 255, unique = true)]
    pub email: String,

    pub password_hash: Option<String>,

    #[field(default = true)]
    pub is_active: bool,
}
```

**Narration (English)**:

> (4:00) Next, the model.
> (4:08) Tag a struct with `#[model(...)]` and Reinhardt treats it as an ORM entity.
> (4:22) Per-field constraints go in `#[field(...)]` — max length, primary key, auto timestamps. The knobs mirror Django's field options.
> (4:45) The important part: every one of those constraints is checked at compile time.
> (5:00) The macro synthesizes `Post::new`, `Post::objects`, and friends. All type-safe, all static.
> (5:15) Domain methods live in ordinary `impl` blocks. No magic — just code you can read.
> (5:22) The User model uses `#[user]` — one attribute replaces the entire `impl BaseUser` block. Hasher and username field, declared once.

**Captions**:

- Macro annotation popup: `expanded by reinhardt-macros`
- Bottom-right: `Type-safe ORM, Django-style fields.`

**Word count**: ~140 words

---

### Scene 5: Migrations (5:30-6:30)

**Medium**: Terminal

**Screen composition**:

- (5:30-5:45) Switch to Terminal. Run `cargo run --bin manage makemigrations posts` and display `Created migration 0001_initial`
- (5:45-6:00) Cut briefly to VSCode to show the auto-generated `migrations/0001_initial.rs`; highlight the `CreateTable { table: "posts", columns: [...] }` structure (just surface its existence, don't ask viewers to read it)
- (6:00-6:20) Back to Terminal, run `cargo run --bin manage migrate`. Scroll the `Applying posts.0001_initial... OK` log
- (6:20-6:30) Confirm the real table exists (e.g. `sqlite3 db.sqlite3 ".schema posts"`)

**Narration (English)**:

> (5:30) With the model in place, migrations are next.
> (5:38) `makemigrations` compares the model against the migration history and emits the delta as a Rust file.
> (5:55) Because the output is Rust, you can review it with a regular `git diff`. No raw SQL strings to eyeball.
> (6:10) `migrate` applies the schema to the database for real.
> (6:22) The Django experience — but now statically typed end to end.

**Captions**:

- Command subtitle: `$ cargo run --bin manage makemigrations posts`
- Bottom-right: `Migrations are Rust code, not SQL strings.`

**Word count**: ~85 words

---

### Scene 6: Serializer + ViewSet (6:30-8:00)

**Medium**: VSCode

**Screen composition**:

- (6:30-7:00) Open `src/apps/posts/serializers.rs` and declare `PostSerializer`. It derives `Validate` and carries per-field length constraints — the framework enforces them automatically on incoming requests via `pre_validate` on `#[server_fn]`:

```rust
use reinhardt::prelude::*;
use reinhardt::Validate;
use super::models::Post;

#[derive(Clone, serde::Serialize, serde::Deserialize, Validate)]
pub struct PostSerializer {
    #[serde(skip_deserializing)]
    pub id: i64,

    #[validate(length(min = 1, max = 200, message = "title must be 1–200 chars"))]
    pub title: String,

    #[validate(length(min = 1, max = 10000, message = "body must be 1–10000 chars"))]
    pub body: String,

    pub published: bool,

    #[serde(skip_deserializing)]
    pub created_at: chrono::DateTime<chrono::Utc>,
}

// From<Post> covers the read path (DB → response).
// Input validation is delegated to pre_validate on #[server_fn].
impl From<Post> for PostSerializer {
    fn from(p: Post) -> Self {
        Self { id: p.id, title: p.title, body: p.body,
               published: p.published, created_at: p.created_at }
    }
}
```

- (7:00-7:20) Side-by-side comparison: `PostSerializer` on the left, DRF's `ModelSerializer` on the right. Animate the 1:1 correspondence — same intent, different surface
- (7:20-7:50) Open `src/apps/posts/views.rs` and declare the ViewSet using the real `#[viewset]` macro + `ModelViewSet<M, S>` builder (pagination / filter / ordering included):

```rust
use reinhardt::prelude::*;
use reinhardt::views::viewsets::{FilterConfig, OrderingConfig, PaginationConfig};
use super::{models::Post, serializers::PostSerializer};

#[reinhardt::viewset]
pub fn viewset() -> ModelViewSet<Post, PostSerializer> {
    ModelViewSet::new("posts")
        .with_pagination(PaginationConfig::page_number(10, Some(100)))
        .with_filters(FilterConfig::new()
            .with_filterable_fields(vec!["published".into(), "title".into()]))
        .with_ordering(OrderingConfig::new()
            .with_ordering_fields(vec!["created_at".into(), "title".into()]))
}
```

- (7:50-8:00) Show `#[viewset]`'s expansion popup, listing the five auto-generated handlers `list / retrieve / create / update / destroy`

**Narration (English)**:

> (6:30) Now the serializer — the shape of the API.
> (6:38) A plain Rust struct plus serde is the contract — `PostSerializer` doubles as the request body and the response body.
> (6:55) Read-only fields? `#[serde(skip_deserializing)]`. Conceptually it's DRF's `ModelSerializer`.
> (7:10) The difference is that this mapping is checked at compile time.
> (7:22) Then the ViewSet. `#[reinhardt::viewset]` wraps a builder for `ModelViewSet<Post, PostSerializer>`, with pagination, filtering, and ordering as one-liners.
> (7:45) That's it — list, retrieve, create, update, and destroy are all generated. About twenty lines, a full CRUD API.

**Captions**:

- Progress bar: `7 min left`
- Bottom of screen: `#[viewset] + ModelViewSet<M, S> = DRF ModelViewSet, statically verified.`

**Word count**: ~130 words

---

### Scene 7: WASM frontend component (8:00-10:30)

**Medium**: VSCode + Browser

**Screen composition**:

- (8:00-8:15) First write a `#[server_fn]` RPC that the page will call, in `src/apps/posts/server_fn.rs`. The same Rust type (`PostSerializer`) crosses the compile boundary:

```rust
use reinhardt::prelude::*;
use reinhardt::pages::ServerFnError;
use crate::apps::posts::{models::Post, serializers::PostSerializer};

#[server_fn]
pub async fn list_posts(
    #[inject] db: DatabaseConnection,
) -> std::result::Result<Vec<PostSerializer>, ServerFnError> {
    let rows = Post::objects().all_with_db(&db).await?;
    Ok(rows.into_iter().map(PostSerializer::from).collect())
}
```

- (8:15-9:25) Open `src/pages/posts_page.rs` and write the page with `page!` (the real DSL — no `#[component]`, no `view!`):

```rust
use reinhardt::pages::prelude::*;
use crate::apps::posts::{serializers::PostSerializer, server_fn::list_posts};

pub fn posts_page() -> Page {
    let load_posts = use_action(|_: ()| async move {
        list_posts().await.map_err(|e| e.to_string())
    });
    load_posts.dispatch(());
    let ls = load_posts.clone();

    page!(|ls: Action<Vec<PostSerializer>, String>| {
        section class: "posts" {
            h1 { "Posts" }
            {
                Page::Fragment(ls.result().unwrap_or_default().iter()
                    .map(|p| page!(|title: String, body: String| {
                        li { h2 { { title } } p { { body } } }
                    })(p.title.clone(), p.body.clone()))
                    .collect::<Vec<_>>())
            }
        }
    })(ls)
}
```

- (9:25-9:45) Hover over `list_posts` — rust-analyzer shows it's a `#[server_fn]`: same signature on both sides, `PostSerializer` is the single wire contract, no schema drift
- (9:45-10:05) Switch to `src/pages/mod.rs` and register the page in the router with a single line: `.route("/", posts_page)`
- (10:05-10:20) Run `cargo run --bin manage runserver --with-pages` in Terminal. The backend and WASM bundler boot together
- (10:20-10:30) Open the browser at `http://127.0.0.1:8000/`. The page dispatches `list_posts`, receives the JSON, and renders live

**Narration (English)**:

> (8:00) Ten minutes in, the JSON API is live. Now the UI.
> (8:08) Reinhardt's `reinhardt::pages` module compiles to WebAssembly — same crate, same types.
> (8:20) Server data is exposed through `#[server_fn]` — an async function that becomes a typed RPC the frontend can call directly.
> (8:40) On the client, `use_action` turns that RPC into a reactive resource and re-renders when the data arrives.
> (9:00) The `page!` macro is the view DSL — braces, child blocks, and plain Rust expressions. No JSX.
> (9:20) `PostSerializer` is the contract. Backend and frontend share the same Rust type — no schema drift, ever.
> (9:50) `runserver --with-pages` boots the API and the WASM bundler in one step.
> (10:15) Open the browser and the list is rendering — end-to-end, fully typed.

**Captions**:

- Top-right: `same Rust type, backend ↔ frontend`
- Bottom: `WASM SPA in 2.5 minutes`

**Word count**: ~180 words

---

### Scene 8: Admin UI demo (10:30-12:00)

**Medium**: Browser (Safari / Arc)

**Screen composition**:

- (10:30-10:45) Cut back to VSCode and add `src/apps/posts/admin.rs`:

```rust
use reinhardt::prelude::*;
use super::models::Post;

#[admin(model = Post, list_display = [id, title, published, created_at])]
pub struct PostAdmin;
```

- (10:45-11:00) In Terminal, run `cargo run --bin manage createsuperuser` (fast-forward the prompt input 2× and mask the password)
- (11:00-11:20) In the browser, hit `http://127.0.0.1:8000/admin/`. After login, the dashboard appears
- (11:20-11:50) The posts list view. Point-overlay the columns named in `list_display`. Click a record to open the edit page, toggle `published`, and save. Returning to the list, the `✓` flips
- (11:50-12:00) Close the browser; punch caption "All from one attribute."

**Narration (English)**:

> (10:30) And there's still the crown jewel: admin.
> (10:36) Attach `#[admin(...)]` to a model and the admin page exists.
> (10:50) `createsuperuser` — same command, same spirit as Django.
> (11:05) Open `/admin/` in the browser, and there it is.
> (11:20) List, search, edit, delete — everything works.
> (11:35) The `list_display` columns appear exactly as declared.
> (11:50) Total hand-written code: a few lines.

**Captions**:

- Top of browser: `Reinhardt Admin`
- Bottom-right: `Batteries included. Really.`

**Word count**: ~85 words

---

### Scene 9: DI / auth (12:00-13:30)

**Medium**: Manim (12:00-12:20 concept diagram) + VSCode (12:20-13:30 implementation)

#### Scene 9a: DI concept diagram (Manim, 12:00-12:20)

- (12:00-12:06) Display the `publish_post(...)` `#[server_fn]` signature at center. Highlight the arguments `#[inject] AuthUser(user): AuthUser<User>` and `#[inject] posts: PostRepository`
- (12:06-12:14) Arrows extend from the signature; a **DI Container** box enters from the right. Inside the container, two slots (`AuthUser<User>`, `PostRepository`) line up. Arrows return from each slot back into the signature's arguments. The caption "resolved at compile time" appears
- (12:14-12:20) Behind the DI Container, a "Provider Registry" layer is revealed. Visualize the resolution chains `TokenAuthConfig → AuthUser<User>` and `#[injectable_factory] post_repository → PostRepository` with arrows. Fade out into the VSCode segment

**Narration (English, 12:00-12:18)**:

> (12:00) Before the code, one picture.
> (12:04) `#[inject]` marks each argument as a dependency.
> (12:09) At compile time, Reinhardt walks a provider registry and wires them in.
> (12:15) No runtime reflection. No hidden globals.

**Captions**:

- Top-right: `#[inject] → DI Container → Provider`
- Bottom-center: `Resolved at compile time`

**Manim implementation note**: a `Scene9DI` cell in `source/manim.ipynb` uses the same 2-row grid + elbow-arrow layout as Scenes 1 and 2 — function signature on the left column, DI Container in the middle, Provider Registry on the right.

#### Scene 9b: Implementation (VSCode, 12:20-13:30)

**Screen composition**:

- (12:20-12:35) Add a `TokenAuthConfig`-backed `AuthSettings` fragment and compose it into `ProjectSettings` in `src/config/settings.rs`:

```rust
use reinhardt::prelude::*;
use reinhardt_auth::rest_authentication::TokenAuthConfig;

#[settings(fragment = true)]
pub struct AuthSettings {
    pub token: TokenAuthConfig,
}

#[settings(core: CoreSettings, auth: AuthSettings)]
pub struct ProjectSettings;
```

- (12:35-12:50) Declare `PostRepository` as a DI-resolved dependency in `src/apps/posts/repositories.rs` using `#[injectable_factory]`. The factory itself is injected into — no global state. `save` takes `&mut Post` and returns `Result<()>` (reinhardt's prelude alias):

```rust
use reinhardt::prelude::*;
use super::models::Post;

pub struct PostRepository { db: DatabaseConnection }

impl PostRepository {
    pub async fn get(&self, id: i64) -> Result<Post> {
        Post::objects().get(id).first_with_db(&self.db).await
    }
    pub async fn save(&self, post: &mut Post) -> Result<()> {
        post.save().await
    }
}

#[injectable_factory]
pub async fn post_repository(
    #[inject] db: DatabaseConnection,
) -> PostRepository {
    PostRepository { db }
}
```

- (12:50-13:00) Add `PublishInput` to `src/apps/posts/serializers.rs` — a validated request body for the publish RPC:

```rust
#[derive(serde::Deserialize, Validate)]
pub struct PublishInput {
    #[validate(range(min = 1, message = "id must be a positive integer"))]
    pub id: i64,
}
```

- (13:00-13:20) In `src/apps/posts/server_fn.rs`, add the publish RPC. `#[server_fn(pre_validate = true)]` auto-calls `.validate()` on `input` before the handler runs — invalid input returns 400 without reaching application code. `Guard<IsActiveUser>` enforces authorization — inactive users get 403:

```rust
use reinhardt::prelude::*;
use reinhardt::pages::ServerFnError;
use reinhardt_auth::{AuthUser, guard::{Guard, IsActiveUser}};
use crate::users::User;
use super::{repositories::PostRepository, serializers::{PostSerializer, PublishInput}};

#[server_fn(pre_validate = true)]
pub async fn publish_post(
    input: PublishInput,
    #[inject] AuthUser(user): AuthUser<User>,
    #[inject] _: Guard<IsActiveUser>,
    #[inject] posts: PostRepository,
) -> std::result::Result<PostSerializer, ServerFnError> {
    let mut post = posts.get(input.id).await?;
    post.publish_by(&user)?;
    posts.save(&mut post).await?;
    Ok(PostSerializer::from(post))
}
```

- (13:20-13:25) Mirror the same pattern on the frontend. In `src/pages/posts_page.rs`, wire `publish_post` into a button via `use_action`, passing `PublishInput { id }`:

```rust
let publish = use_action(|id: i64| async move {
    publish_post(PublishInput { id }).await.map_err(|e| e.to_string())
});
```

- (13:25-13:30) In Terminal, side by side: `curl` without a token → `401`, active user token → `200`, inactive user → `403`

**Narration (English)**:

> (12:20) Now the code.
> (12:25) `AuthSettings` is a `#[settings]` fragment composed into `ProjectSettings` — token auth turned on, no globals.
> (12:38) `PostRepository` is registered with `#[injectable_factory]`. One async function describes how to build it; the container does the rest.
> (12:50) `PublishInput` carries a validated id — `#[validate(range(min = 1))]` keeps invalid calls from ever reaching the database.
> (13:00) `pre_validate = true` on the `#[server_fn]` attribute calls `.validate()` automatically. No boilerplate, no forgotten checks.
> (13:08) `Guard<IsActiveUser>` is injected alongside the user — inactive accounts get 403 before the body runs.
> (13:18) No token: 401. Inactive user: 403. Valid active token: 200.
> (13:28) Declarative to write, statically enforced. That's the Reinhardt way.

**Captions**:

- Center: `401 → 200`
- Bottom-right: `DI + Auth, declared, not wired.`

**Word count**: ~120 words

---

### Scene 10: Comparison and wrap-up (13:30-15:00)

**Medium**: Manim

**Screen composition**:

- (13:30-14:10) On the left, a vertical achievement checklist:
  - ✅ Model
  - ✅ Migration
  - ✅ CRUD API
  - ✅ WASM Frontend
  - ✅ Admin
  - ✅ Auth + DI
  On the right, an `≈ 80 lines of Rust` counter
- (14:10-14:50) Call to action: the command `cargo install reinhardt-admin-cli` and links to `github.com/kent8192/reinhardt-web` / `reinhardt-web.dev`
- (14:50-15:00) Logo fades out; close with `Django's productivity. Rust's performance.` centered

**Narration (English)**:

> (13:30) Fifteen minutes. That's what we just did.
> (13:40) Model, migrations, CRUD, WASM UI, admin, auth, DI.
> (13:55) About eighty lines of Rust — and the frontend and backend share types.
> (14:12) Wiring the same stack out of Axum, sea-orm, utoipa, and a separate frontend framework would take hundreds of lines — and a lot of design decisions.
> (14:30) Django's productivity, with Rust's performance. Full stack. That's what Reinhardt is aiming at.
> (14:45) `cargo install reinhardt-admin-cli` — you can start today.
> (14:55) See you next time.

**Captions**:

- Final center: `Django's productivity. Rust's performance.`
- Bottom: `github.com/kent8192/reinhardt-web`

**Word count**: ~110 words

---

## Narration budget check

| Scene | Runtime | Words | Words/min |
|-------|---------|-------|-----------|
| 1 | 1:30 | ~105 | 70 |
| 2 | 1:00 | ~100 | 100 |
| 3 | 1:30 | ~140 | 93 |
| 4 | 1:30 | ~120 | 80 |
| 5 | 1:00 | ~85 | 85 |
| 6 | 1:30 | ~130 | 87 |
| 7 | 2:30 | ~180 | 72 |
| 8 | 1:30 | ~85 | 57 |
| 9 | 1:30 | ~120 | 80 |
| 10 | 1:30 | ~110 | 73 |
| **Total** | **15:00** | **~1,175** | avg 78 |

Prioritize the live-coding footage: intentionally keep the narration sparse so there is breathing room (beats). Leave the keystroke sounds in place and keep voiceover to the minimum.

## Production notes

- **VSCode setup**: `Cascadia Code` or `JetBrains Mono` 14pt, a Tokyo Night-style dark theme. Enable rust-analyzer inlay hints
- **Terminal**: zsh + Starship. Fully opaque black background; simplify the prompt to just `$`
- **Typing animation**: 60–80 wpm keystroke animation. The actual code is pasted; keystroke sounds are recorded separately and layered on top
- **Fast-forwarded sections**: `cargo run` compile waits, WASM bundle builds (`trunk build`), `createsuperuser` interactive prompts, and `curl` latency all run at 2×–4×
- **Color cue**: pulse macro attributes (`#[model]`, `#[viewset]`, `#[server_fn]`, `#[inject]`, `#[settings]`, `#[injectable_factory]` family) in yellow the first time they appear
- **Manim**: background color matches the VSCode theme at `#1a1b26`; accent is Reinhardt's brand color (`#f74c00`); use the elbow-arrow layout established in `source/manim.ipynb` for Scenes 1, 2, 9a
- **BGM**: slow synth for Scenes 1–2 / 9a / 10; lo-fi hip-hop for Scenes 3–8 / 9b
- **SFX**: short "pop" on command execution, "ding" on test success, a short "buzz" when 401 appears
