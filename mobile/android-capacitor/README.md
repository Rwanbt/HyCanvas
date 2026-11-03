# HyCanvas Android preview (online-only)

> **Experimental, unofficial Android build.** This is an independently maintained
> proof of concept, not an official HyCanvas release and **not ready for app-store
> distribution**.

This is a small [Capacitor](https://capacitorjs.com/) Android shell that loads
the hosted HyCanvas editor at **https://hycanvas.art**. It does **not** bundle
the HyCanvas frontend or backend, store documents offline, or provide local
editing. An internet connection and an available HyCanvas server are required.

The native shell is intentionally isolated from the existing npm workspaces;
you do not need to install/build the full monorepo for this first Android test.
The Android project contains **no native third-party plugins** and requests
only the Android `INTERNET` permission by default.

## Requirements

- Node.js 22+ and npm
- JDK 21
- Android SDK with API 36 and build-tools installed
- A physical Android phone with Android System WebView updated, or an emulator

## Build a debug APK

```sh
cd mobile/android-capacitor
npm ci
npx cap sync android
cd android
./gradlew assembleDebug
```

On Windows PowerShell, run `.\gradlew.bat assembleDebug` instead.

The APK is created at:

```text
mobile/android-capacitor/android/app/build/outputs/apk/debug/app-debug.apk
```

Install it with `adb install -r <path-to-app-debug.apk>` if your phone is
connected with USB debugging enabled. Or copy the APK to the phone and open it.

## Real-device test checklist

- [ ] Install APK; verify the launcher name and first launch.
- [ ] Launch with Wi-Fi and cellular data: hosted home page loads over HTTPS.
- [ ] Sign in / sign out (including redirects); no login is captured by native code.
- [ ] Create a design, edit text and shapes, and save it.
- [ ] Import a local image using the Android picker.
- [ ] Try PNG and PDF export and find resulting files on the phone.
- [ ] Rotate the phone, zoom, pan, drag with touch; test back navigation.
- [ ] Put the app in the background, restore it, and reopen saved work.
- [ ] Test an external link (must not silently receive native plugin access).
- [ ] With airplane mode on, confirm that the app requires the hosted service.
- [ ] Review Android logs for crashes and WebView errors.

**Not yet tested on a physical Android device.** Do not assume login, file
choosers, downloads, keyboard, or complex pointer gestures work correctly
until this checklist is completed.

## Security and shipping limitations

Capacitor's `server.url` setting is designed for *development live reload*,
not production distribution. Capacitor's maintainers advise **against**
loading an external website as a production Capacitor app, for security and
app-store-policy reasons. This proof of concept uses a fixed HTTPS URL solely
for the first build/test cycle. Never add privileged Capacitor plugins while
untrusted remotely served content can reach the bridge. Do not publish this
debug APK as a production release.

For an upstream-ready implementation, bundle the static Next.js frontend
in `webDir`, make the API endpoint configurable, explicitly handle browser
authentication and file downloads, verify native-bridge trust boundaries,
and validate gesture/layout support on mobile.

The parent project and this fork are licensed under AGPL-3.0-only. Follow
the project's `CONTRIBUTING.md`, `CLA.md`, and `TRADEMARKS.md` before
proposing an upstream pull request.
