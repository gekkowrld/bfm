local metadata = {
    name = "Dark Mode Theme",
    authors = { "Gordon Onayo" },
    description = "A sleek and dark theme",
    version = "1.0.0",
    license = "MIT",
}

local theme = {
    theme_type = ThemeType.dark,
    window_background = Color:new(13, 17, 23, 0.8),
    text_color = Color:new(240, 246, 252, 1),
}

GeneratedTheme = Theme:New(metadata, theme)

return GeneratedTheme
