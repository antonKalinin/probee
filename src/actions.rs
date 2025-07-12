use gpui::actions;

actions!(
    app,
    [
        SelectNextAssistant,
        SelectPrevAssistant,
        RunAssistant,
        // windows
        CloseApp,
        ToggleApp,
        OpenSettings,
        OpenPromptEditor,
    ]
);
