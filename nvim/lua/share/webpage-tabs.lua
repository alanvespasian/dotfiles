return {
  {
    "romgrk/barbar.nvim",
    dependencies = {
      -- "lewis6991/gitsigns.nvim", -- OPTIONAL: for git status
      -- "nvim-tree/nvim-web-devicons", -- OPTIONAL: for file icons
    },
    init = function()
      vim.g.barbar_auto_setup = false
    end,
    opts = {
      -- animation = true,
      -- insert_at_start = true,
      -- ...etc
    },
    version = "^1.0.0",
  },
}

