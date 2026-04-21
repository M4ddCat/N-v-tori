require("nvchad.configs.lspconfig").defaults()

local servers = { "html", "cssls" }
vim.lsp.enable(servers)

local install_path = vim.fn.stdpath("data") .. "/rsl-language-server"
local server_js = install_path .. "/out/server.js"

local function ensure_rsl_lsp()
    if vim.loop.fs_stat(server_js) then
        return true
    end

    vim.notify("📦 Установка RSL Language Server...", vim.log.levels.INFO)

    vim.fn.mkdir(install_path, "p")

    local clone_cmd = string.format('git clone https://github.com/yohanson/rsl-language-server "%s"', install_path)
    vim.fn.system(clone_cmd)

    if vim.v.shell_error ~= 0 then
        vim.notify("❌ Ошибка клонирования RSL LSP", vim.log.levels.ERROR)
        return false
    end

    vim.fn.system('cd "' .. install_path .. '" && npm install && npm run build')

    if vim.v.shell_error ~= 0 then
        vim.notify("❌ Ошибка сборки RSL LSP", vim.log.levels.ERROR)
        return false
    end

    vim.notify("✅ RSL Language Server установлен!", vim.log.levels.INFO)
    return true
end

if ensure_rsl_lsp() then
    vim.lsp.config.rsl = {
        cmd = { "node", server_js, "--stdio" },
        filetypes = { "rsl", "mac", "rslv", "rsi" },
        root_markers = { ".git", "package.json" },
        single_file_support = true,
        settings = {
            RSLanguageServer = {
                import = true
            }
        },
    }
    vim.lsp.enable("rsl")
end

vim.filetype.add({
    extension = {
        mac = "rsl",
        rsl = "rsl",
        rslv = "rsl",
        rsi = "rsl",
    },
})