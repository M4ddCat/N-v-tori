return {
  {
    "stevearc/conform.nvim",
    -- event = 'BufWritePre', -- uncomment for format on save
    opts = require "configs.conform",
  },

  -- These are some examples, uncomment them if you want to see them work!
  {
    "neovim/nvim-lspconfig",
    config = function()
      require "configs.lspconfig"
    end,
  },

  {
    "nvim-treesitter/nvim-treesitter",
    branch = "main",
    build = ":TSUpdate",
    config = function()
        vim.api.nvim_create_autocmd("User", {
            pattern = "TSUpdate",
            callback = function()
                require("nvim-treesitter.parsers").rsl = {
                    install_info = {
                        type = "self_contained",
                        url = "https://github.com/M4ddCat/tree-sitter-rsl",
                        branch = "main",
                        files = { "src/parser.c" },
                    },
                    filetype = "mac",
                }
            end,
        })

        require("nvim-treesitter").install({ "rsl" })

        vim.filetype.add({
            extension = {
                mac = "mac",
            },
        })

        vim.api.nvim_create_autocmd("FileType", {
            pattern = "mac",
            callback = function(args)
                local buf = args.buf
                vim.treesitter.start(buf, "rsl")
                vim.bo[buf].indentexpr = "v:lua.require'nvim-treesitter'.indentexpr()"
            end,
        })
    end,
  },
}
