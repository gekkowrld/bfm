local metadata = {
    name = "Dark Mode Theme",
    authors = { "Gordon Onayo" },
    description = "A sleek and dark theme",
    version = "0.0.1",
    license = "MIT",
}

local border = {
    color = Color:from_hex("#3d444d"),
    width = 1,
    radius = Radius:new_equal(4),
}

local theme = {
    theme_type = ThemeType.dark,
    background = Color:from_hex("#282A36"),
    color = Color:from_hex("#F8F8F2"),
    primary_color = Color:from_hex("#BD93F9"),
    success_color = Color:from_hex("#50FA7B"),
    warning_color = Color:from_hex("#FFB86C"),
    error_color = Color:from_hex("#FF5555"),
    border = border
}

GeneratedTheme = Theme:New(metadata, theme)

return GeneratedTheme
