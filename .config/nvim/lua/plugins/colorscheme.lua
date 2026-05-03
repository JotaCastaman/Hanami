return {
  { "sainnhe/gruvbox-material" },

  {
    "LazyVim/LazyVim",
    opts = {
      colorscheme = "gruvbox-material",
    },
  },

  {
    "sainnhe/gruvbox-material",
    config = function()
      vim.g.gruvbox_material_background = "soft"
      vim.g.gruvbox_material_foreground = "material"
      vim.g.gruvbox_material_transparent_background = 1

      vim.api.nvim_create_autocmd("ColorScheme", {
        pattern = "gruvbox-material",
        callback = function()
          vim.api.nvim_set_hl(0, "SnacksDashboardHeader", { fg = "#ccc2bd" })
          vim.api.nvim_set_hl(0, "SnacksDashboardDesc", { fg = "#ccc2bd" })
          vim.api.nvim_set_hl(0, "SnacksDashboardKey", { fg = "#ccc2bd" })
          vim.api.nvim_set_hl(0, "SnacksDashboardFooter", { fg = "#ccc2bd" })
          vim.api.nvim_set_hl(0, "SnacksDashboardIcon", { fg = "#ccc2bd" })
        end,
      })
    end,
  },
}
