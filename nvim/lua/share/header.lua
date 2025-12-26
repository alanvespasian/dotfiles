return {

  { "nvimdev/dashboard-nvim", enabled = false },
  { "echasnovski/mini.starter", enabled = false },
  -- Dashboard. This runs when neovim starts, and is what displays
  -- the "LAZYVIM" banner.
  {
    "goolord/alpha-nvim",
    event = "VimEnter",
    enabled = true,
    init = false,
    opts = function()
      local dashboard = require("alpha.themes.dashboard")
      local logo = [[
      ████████╗███████╗██╗  ██╗████████╗
╚══██╔══╝██╔════╝╚██╗██╔╝╚══██╔══╝
   ██║   █████╗   ╚███╔╝    ██║   
   ██║   ██╔══╝   ██╔██╗    ██║   
   ██║   ███████╗██╔╝ ██╗   ██║   
   ╚═╝   ╚══════╝╚═╝  ╚═╝   ╚═╝   

      ]]

      dashboard.section.header.val = vim.split(logo, "\n")
      -- stylua: ignore
      dashboard.section.buttons.val = {
        dashboard.button("f", " " .. " Find file",       "<cmd> Telescope find_files <cr>"),
        dashboard.button("n", " " .. " New file",        "<cmd> ene <BAR> startinsert <cr>"),
        dashboard.button("r", " " .. " Recent files",    "<cmd> Telescope oldfiles <cr>"),
        dashboard.button("g", " " .. " Find text",       "<cmd> Telescope live_grep <cr>"),
        dashboard.button("c", " " .. " Config", "<cmd>e ~/.config/nvim/init.lua<cr>"),
        dashboard.button("q", "  Quit", ":qa")
}
for _, button in ipairs(dashboard.section.buttons.val) do
        button.opts.hl = "AlphaButtons"
        button.opts.hl_shortcut = "AlphaShortcut"
      end
      dashboard.section.header.opts.hl = "AlphaHeader"
      dashboard.section.buttons.opts.hl = "AlphaButtons"
      dashboard.section.footer.opts.hl = "AlphaFooter"
      dashboard.opts.layout[1].val = 8
      return dashboard
    end,
    config = function(_, dashboard)
      -- close Lazy and re-open when the dashboard is ready
      if vim.o.filetype == "lazy" then
        vim.cmd.close()
        vim.api.nvim_create_autocmd("User", {
          once = true,
          pattern = "AlphaReady",
          callback = function()
            require("lazy").show()
          end,
        })
      end

require("alpha").setup(dashboard.opts)

vim.api.nvim_create_autocmd("User", {
  once = true,
  pattern = "LazyVimStarted",
  callback = function()
    local stats = require("lazy").stats()
    local ms = (math.floor(stats.startuptime * 100 + 0.5) / 100)

    -- List of possible phrases
    local phrases = {
            [[     "Todo lo puedo en Cristo que me fortalece" - Filipenses 4:13  ]],
[[     "El Señor es mi pastor; nada me faltará" - Salmos 23:1  ]],
[[     "Porque yo sé los planes que tengo para vosotros" - Jeremías 29:11  ]],
[[     "La fe es la certeza de lo que se espera" - Hebreos 11:1  ]],
[[     "Bienaventurados los que tienen hambre y sed de justicia" - Mateo 5:6  ]],
[[     "El amor todo lo sufre, todo lo cree, todo lo espera" - 1 Corintios 13:7  ]],
[[     "El Señor es mi luz y mi salvación; ¿de quién temeré?" - Salmos 27:1  ]],
[[     "No temas, porque yo estoy contigo" - Isaías 41:10  ]],
[[     "Clama a mí, y yo te responderé" - Jeremías 33:3  ]],
[[     "La paz os dejo, mi paz os doy" - Juan 14:27  ]],
[[     "Esforzaos y cobrad ánimo; no temáis" - Deuteronomio 31:6  ]],
[[     "El gozo del Señor es vuestra fuerza" - Nehemías 8:10  ]],
[[     "Confía en el Señor con todo tu corazón" - Proverbios 3:5  ]],
[[     "El que habita al abrigo del Altísimo morará bajo la sombra del Omnipotente" - Salmos 91:1  ]],
[[     "Mira que te mando que te esfuerces y seas valiente" - Josué 1:9  ]],

    }

    -- Select a random phrase
    math.randomseed(os.time()) -- Ensure randomness
    local random_phrase = phrases[math.random(1, #phrases)]

    dashboard.section.footer.val = random_phrase
    pcall(vim.cmd.AlphaRedraw)
  end,
})
    end,
  },
}


