--[[
    This is a theme file declaration mod file.
    This file will be included by default and is expected to be overridden by the user.
    If the values are not overridden, then the default values will be used.

    DO NOT INCLUDE THIS FILE, IT WILL BE INCLUDED BY DEFAULT.
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

---@return Color
local function color_grey()
    return Color:from_hex("#2a2a2a")
end


---@class Radius
---@field top_left number
---@field top_right number
---@field bottom_left number
---@field bottom_right number
Radius = {}

---@param top_left number
---@param top_right number
---@param bottom_left number
---@param bottom_right number
---@return Radius
function Radius:new(top_left, top_right, bottom_left, bottom_right)
    local instance = {
        top_left = top_left or 0,
        top_right = top_right or 0,
        bottom_left = bottom_left or 0,
        bottom_right = bottom_right or 0
    }
    setmetatable(instance, { __index = self })
    return instance
end

---@param radius number
---@return Radius
function Radius:new_equal(radius)
    return Radius:new(radius, radius, radius, radius)
end

---@return Radius
local function zero_radius()
    return Radius:new(0, 0, 0, 0)
end

---@class Border
---@field color Color
---@field width number
---@field radius Radius
Border = {}

---@param color Color
---@param width number
---@param radius Radius
---@return Border
function Border:new(color, width, radius)
    local instance = {
        color = color or color_grey(),
        width = width or 0,
        radius = radius or zero_radius()
    }
    setmetatable(instance, { __index = self })
    return instance
end

---@return Border
local function default_border()
    return Border:new(color_grey(), 0, zero_radius())
end

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
function Color:from_hex(hex)
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
---@field background Color
---@field color Color
---@field primary_color Color
---@field success_color Color
---@field warning_color Color
---@field error_color Color
---@field border Border
Theme = {}

---@param theme_type ThemeType
---@param background Color
---@param color Color
---@param primary_color Color
---@param success_color Color
---@param warning_color Color
---@param error_color Color
---@param border Border
---@return Theme
function Theme:new(theme_type, background, color, primary_color, success_color, warning_color, error_color, border)
    local instance = {
        theme_type = theme_type or ThemeType.dark,
        background = background or color_grey(),
        color = color or Color:new(200, 200, 200, 1),
        primary_color = primary_color or color_grey(),
        success_color = success_color or color_grey(),
        warning_color = warning_color or color_grey(),
        error_color = error_color or color_grey(),
        border = border or default_border()
    }
    setmetatable(instance, { __index = self })
    return instance
end

-- Default theme definitions
local default_dark = Theme:new(ThemeType.dark, color_grey(), Color:new(200, 200, 200, 1), color_grey(),
    color_grey(), color_grey(), color_grey(), default_border())
local default_light = Theme:new(ThemeType.light, Color:new(255, 255, 255, 1), color_grey(),
    color_grey(), color_grey(), color_grey(), color_grey(), default_border())

---comment
---@param metadata METADATA
---@param theme Theme
---@return table
function Theme:New(metadata, theme)
    local newTheme = {}

    -- Helper function to assign values
    local function mergeDefaults(themeDefaults, userTheme)
        local mergedTheme = {}
        for k, v in pairs(themeDefaults) do
            mergedTheme[k] = (userTheme[k] ~= nil) and userTheme[k] or v
        end
        return mergedTheme
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
