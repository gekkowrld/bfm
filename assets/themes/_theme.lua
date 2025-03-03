--[[
    This is a theme file declaration mod file.
    This file will be included by default and is expected to be overridden by the user.
    If the values are not overridden, then the default values will be used.
-- ]]

-- This is the metadata for the theme.
-- This is the information that will be displayed in the theme manager.
---@class METADATA
METADATA = {
    name = "theme-name",
    authors = {},
    description = "What does this theme do?",
    version = "1.0.0",
    license = "MIT",
    url = "https://theme.website"
}
---@enum ThemeType
ThemeType = {
    light = "light",
    dark = "dark"
}

---@class Color
---@field red number (0-255)
---@field green number (0-255)
---@field blue number (0-255)
---@field alpha number (0-1)
Color = {}

---@param red number
---@param green number
---@param blue number
---@param alpha number
---@return Color
function Color:new(red, green, blue, alpha)
    local instance = {
        red = red or 0,
        green = green or 0,
        blue = blue or 0,
        alpha = alpha or 1
    }
    setmetatable(instance, { __index = self })
    return instance
end

---@param hex string
---@return Color
function Color:from_rgb(hex)
    -- Remove the hash (#) symbol if it exists
    hex = hex:gsub("#", "")

    -- If the hex code is shorthand (like #FFF), expand it to the full format (like #FFFFFF)
    if #hex == 3 then
        hex = hex:sub(1, 1) .. hex:sub(1, 1) .. hex:sub(2, 2) .. hex:sub(2, 2) .. hex:sub(3, 3) .. hex:sub(3, 3)
    end

    -- Convert the hex string into RGB values
    local r, g, b = hex:match("(..)(..)(..)")
    r, g, b = tonumber(r, 16), tonumber(g, 16), tonumber(b, 16)

    -- Return the RGB values
    return Color:new(r, g, b, 1)
end

---@class Theme
---@field theme_type ThemeType
---@field window_background Color
Theme = {}

---@param theme_type ThemeType
---@param window_background Color
---@param text_color Color
---@return Theme
function Theme:new(theme_type, window_background, text_color)
    local instance = {
        theme_type = theme_type or ThemeType.dark,
        window_background = window_background or Color:new(0, 0, 0, 1),
        text_color = text_color or Color:new(200, 200, 200, 1),
    }
    setmetatable(instance, { __index = self })
    return instance
end

-- Default theme definitions
local default_dark = Theme:new(ThemeType.dark, Color:new(0, 0, 0, 1), Color:new(200, 200, 200, 1))
local default_light = Theme:new(ThemeType.light, Color:new(255, 255, 255, 1), Color:new(0, 0, 0, 1))

---comment
---@param metadata METADATA
---@param theme Theme
---@return table
function Theme:New(metadata, theme)
    local newTheme = {}

    -- Helper function to assign values
    local function mergeDefaults(themeDefaults, userTheme)
        local mergerTheme = {}
        for k, v in pairs(themeDefaults) do
            mergerTheme[k] = userTheme[k] or v
        end
        return mergerTheme
    end

    -- Use the light or dark theme defaults based on the `theme_type`
    if theme.theme_type == ThemeType.light then
        newTheme = mergeDefaults(default_light, theme)
    else
        newTheme = mergeDefaults(default_dark, theme)
    end

    -- Merge metadata
    local newMetadata = {}
    for k, v in pairs(METADATA) do
        newMetadata[k] = metadata[k] or v
    end

    return { theme = newTheme, metadata = newMetadata }
end
