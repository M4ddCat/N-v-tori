return {
  {
    "stevearc/conform.nvim",
    -- event = 'BufWritePre', -- uncomment for format on save
    opts = require "configs.conform",
  },

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
                mac = "rsl",
                rsl = "rsl",
            },
        })

        vim.api.nvim_create_autocmd("FileType", {
            pattern = "rsl",
            callback = function(args)
                local buf = args.buf
                
                -- Функция для безопасного запуска подсветки
                local function start_treesitter()
                    if pcall(vim.treesitter.start, buf, "rsl") then
                        vim.bo[buf].indentexpr = "v:lua.require'nvim-treesitter'.indentexpr()"
                        return true
                    end
                    return false
                end
                
                -- Пробуем запустить сразу
                if not start_treesitter() then
                    -- Если не получилось, ждём установки парсера
                    vim.notify("Installing RSL parser...", vim.log.levels.INFO)
                    vim.cmd("TSInstall rsl")
                    start_treesitter()
                end
            end,
        })
    end,
  },

  {
    'stevearc/aerial.nvim',
    opts = {
        backends = { 'treesitter', 'lsp' },
        nerd_font = true,
        layout = {
            max_width = { 40, 0.3 },
            min_width = 25,
        },
        show_guides = true,
        close_automatic_events = { 'unfocus', 'switch_buffer' },
    },
    keys = {
        { "<leader>o", "<cmd>AerialToggle!<CR>", desc = "Toggle Outline" }
    },
    dependencies = {
        "nvim-treesitter/nvim-treesitter",
        "nvim-tree/nvim-web-devicons"
    },
    config = function(_, opts)
        require("aerial").setup(opts)
    end
  },

  {
    "mfussenegger/nvim-lint",
    event = { "BufReadPre", "BufNewFile" },
    config = function()
        local lint = require("lint")
        local config_dir = vim.fn.stdpath("config")
        local linter_dir = config_dir .. "/tools/rsl-linter"
        local bin_path = linter_dir .. "/target/release/rsl-lint.exe"
        local build_script = linter_dir .. "/build.ps1"

        if vim.fn.executable(bin_path) == 0 then
            vim.notify("🚀 Линтер не найден. Запуск сборки...", vim.log.levels.INFO)
    
            local build_cmd = string.format(
                "powershell.exe -NoProfile -ExecutionPolicy Bypass -Command \"cd '%s'; & './build.ps1'\"",
                linter_dir:gsub("/", "\\")
            )

            vim.fn.jobstart(build_cmd, {
                on_exit = function(_, code)
                    if code == 0 then
                        vim.notify("✅ Линтер успешно собран!", vim.log.levels.INFO)
                        require("lint").try_lint("my_rsl_linter")
                    else
                        vim.notify("❌ Ошибка сборки. Проверьте build.ps1 вручную.", vim.log.levels.ERROR)
                    end
                end,
                stdout_buffered = true,
                stderr_buffered = true,
            })
        end

        lint.linters.my_rsl_linter = {
            cmd = config_dir .. "/tools/rsl-linter/target/release/rsl-lint.exe",
            stdin = false,
            append_fname = true,
            args = {},
            stream = "stdout",
            ignore_exitcode = true,
            parser = require("lint.parser").from_errorformat("%f:%l:%c: %trror: %m,%f:%l:%c: %tarning: %m", {
                source = "RSL-Validator",
            }),
        }

        lint.linters_by_ft = {
            mac = { "my_rsl_linter" },
        }

        vim.api.nvim_create_autocmd({ "BufWritePost", "BufReadPost" }, {
            callback = function()
                lint.try_lint("my_rsl_linter")
            end,
        })
    end,
  },
}
