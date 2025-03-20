## CMD + I

Desktop App

### Project structure

```
cmdi/
├── build.rs // maps environment variables to rust runtime
├── src/
│   ├── assets.rs // static assets
│   ├── services/ // modules with side effects (e.g. API calls, file system operations, OS interactions)
│   ├── ui/ // entities that can be rendered
│   ├── utils/ // side-effect free utility functions
│   ├── events.rs // App & UI events
│   ├── errors.rs // custom error types
│   ├── main.rs // entry point, initializes the services and returs the root UI view
│   ├── root.rs // root view
│   ├── state.rs // global state
│   ├── theme.rs // theme
```
