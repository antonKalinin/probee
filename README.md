## Probee

Desktop App that allows you to quickly run your AI prompts against selected text.

### Project structure

```
probee/
├── build.rs // maps environment variables to rust runtime
├── src/
│   ├── assets.rs // static assets
│   ├── services/ // modules with side effects (e.g. API calls, file system operations, OS interactions)
│   ├── state // global state
│   ├── ui/ // entities that can be rendered
│   ├── utils/ // side-effect free utility functions
│   ├── events.rs // App & UI events
│   ├── errors.rs // custom error types
│   ├── main.rs // entry point, initializes the services, creates windows and starts the app
│   ├── app.rs // root app view
│   ├── settings.rs // root settings view
```
