return {
  -- add rose-pine
  { "rose-pine/neovim", name = "rose-pine" },

  -- Configure LazyVim to load rose-pine
  {
    "LazyVim/LazyVim",
    opts = {
      colorscheme = "rose-pine",
    },
  },
  {
    "rose-pine/neovim",
    opts = {
      styles = { transparency = true },
      palette = {
        moon = {
          base = "#191724",
        },
      },
      highlight_groups = {
        SnacksDashboardHeader = { fg = "#ccc2bd" },
        SnacksDashboardDesc = { fg = "#ccc2bd" },
        SnacksDashboardKey = { fg = "#ccc2bd" },
        SnacksDashboardFooter = { fg = "#ccc2bd" },
        SnacksDashboardIcon = { fg = "#ccc2bd" },
      },
    },
  },
}
