return {
  "nvim-lualine/lualine.nvim",
  opts = function(_, opts)
    local c = {
      bg = "#3b332f",
      surface = "#807772",
      overlay = "#807772",
      text = "#ccc2bd",
      subtle = "#807772",
      accent = "#3b070a",
      dark = "#3b332f",
    }

    local theme = {
      normal = {
        a = { bg = c.overlay, fg = c.dark, gui = "bold" },
        b = { bg = c.bg, fg = c.text },
        c = { bg = c.bg, fg = c.text },
      },
      insert = {
        a = { bg = c.text, fg = c.dark, gui = "bold" },
        b = { bg = c.bg, fg = c.text },
        c = { bg = c.bg, fg = c.text },
      },
      visual = {
        a = { bg = c.accent, fg = c.text, gui = "bold" },
        b = { bg = c.bg, fg = c.text },
        c = { bg = c.bg, fg = c.text },
      },
      replace = {
        a = { bg = c.accent, fg = c.text, gui = "bold" },
        b = { bg = c.bg, fg = c.text },
        c = { bg = c.bg, fg = c.text },
      },
      command = {
        a = { bg = c.subtle, fg = c.text, gui = "bold" },
        b = { bg = c.bg, fg = c.text },
        c = { bg = c.bg, fg = c.text },
      },
      inactive = {
        a = { bg = c.bg, fg = c.subtle },
        b = { bg = c.bg, fg = c.subtle },
        c = { bg = c.bg, fg = c.subtle },
      },
    }

    opts.options = {
      theme = theme,
      component_separators = { left = "", right = "" },
      section_separators = { left = "", right = "" },
    }

    opts.sections = {
      lualine_a = { "mode" },
      lualine_b = {
        {
          "filetype",
          icon_only = true,
          colored = false,
          color = { fg = c.text },
        },
        {
          "filename",
          color = { fg = c.text },
          symbols = { modified = "●", readonly = "", unnamed = "[No Name]" },
        },
      },
      lualine_c = {},
      lualine_x = {
        {
          "diagnostics",
          color = { fg = c.text },
        },
        {
          "location",
          color = { fg = c.text },
        },
      },
      lualine_y = {
        {
          "progress",
          color = { fg = c.text },
        },
      },
      lualine_z = {},
    }

    opts.inactive_sections = {
      lualine_a = {},
      lualine_b = {},
      lualine_c = { "filename" },
      lualine_x = {},
      lualine_y = {},
      lualine_z = {},
    }

    return opts
  end,
}
