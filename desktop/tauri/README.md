# HyCanvas Desktop Preview

Status: **experimental scaffold**, not a standalone/offline desktop release.
This directory is additive: it does not change the original frontend, server,
database, or the separate Capacitor Android work.

## What works in this stage

- Native Tauri 2 window on supported operating systems.
- Only probes the local Go server at `127.0.0.1:8005/readyz`.
- Opens the existing local HyCanvas UI when ready. Otherwise displays a static
  status page; no hosted endpoint is used.
- No plugin, IPC command or native filesystem privilege is exposed to webpages.

## Build the preview

Install Rust, Tauri system prerequisites and a current WebView runtime,
then from `desktop/tauri` run:

```sh
cargo tauri build --no-bundle
```

The executable appears under `desktop/tauri/src-tauri/target/release` when
the build succeeds. For development, run `cargo tauri dev` from this folder.

To display the actual editor, first start an existing *locally configured*
HyCanvas Go server on port 8005, with a reachable PostgreSQL database. The
production `npm run build:dist` already builds Go with its frontend embedded.
This wrapper does not yet launch the Go process or configure PostgreSQL.

## Target architecture (not yet implemented)

1. **Portable data model and migrations:** preserve existing PostgreSQL-backed
   server mode. Prototype a separate single-user local storage adapter or a
   packaged, managed PostgreSQL runtime, with reversible upgrades and backups.
   The current backend uses `pgxpool.Pool` pervasively, so changing all queries
   to SQLite is **not** a small driver swap.
2. **Process lifecycle:** launch a known/verified bundled Go sidecar from Rust,
   bind only to loopback on a reserved port, validate readiness, handle crash and
   shutdown, isolate per-user data, and never kill unrelated services.
3. **Offline assets:** vendor fonts, templates and built-in image resources that
   are currently fetched at runtime. Audit external requests and disable remote
   functions without a network connection.
4. **Portable bundles:** Windows folder + portable EXE, Linux AppImage or tarball,
   macOS .app archive (with signing/notarization decisions). Build and test each
   OS in native CI, not merely by cross-compiling from Windows.
5. **Local-first UX:** create/open local project files, import/export and
   recover safely. Use existing `.hyc` document format where possible and
   preserve full-fidelity project round trips.

## Upstream strategy

HyCanvas's current `docs/roadmap/requirements.md` explicitly lists native
desktop/mobile apps as out of scope. **Do not open an unsolicited PR adding
this entire wrapper.** First discuss the scope in an upstream issue/discussion.

Keep optional desktop work under `desktop/tauri`. Submit separately:

- Platform-neutral fixes (offline asset handling, path portability, storage).
- Tested, backwards-compatible hooks in the Go backend, if needed.
- Desktop shell itself only if upstream maintainers agree to support it.

Build against the upstream `development` branch. Keep Android on its own
branch and avoid changing `packages/*`, `frontend/*`, or `backend/*` in
this initial scaffold.

No data migration, schema change, upstream pull request or merge is performed.
