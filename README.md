## CMD + I

Desktop App

### Project structure

```
cmdi/
├── build.rs // maps environment variables to rust runtime
├── src/
│   ├── assets.rs // static assets
│   ├── services/ // modules with side effects (e.g. API calls, file system operations)
│   ├── ui/ // modules with UI components
│   ├── events.rs // App & UI events
│   ├── errors.rs // custom error types
│   ├── hotkeys.rs // global hotkeys
│   ├── main.rs // entry point, initializes the services and returs the root UI view
│   ├── root.rs // root UI view
│   ├── state.rs // global state
│   ├── storage.rs // persistent storage
│   ├── theme.rs // theme
│   ├── window.rs // main window
```
