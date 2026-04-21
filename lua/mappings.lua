require "nvchad.mappings"

-- add yours here

local map = vim.keymap.set

map("n", ";", ":", { desc = "CMD enter command mode" })
map("i", "jk", "<ESC>")

map("n", "]d", vim.diagnostic.goto_next, { desc = "Next diagnostic" })
map("n", "[d", vim.diagnostic.goto_prev, { desc = "Previous diagnostic" })
map("n", "<leader>ds", vim.diagnostic.open_float, { desc = "Floating diagnostic" })
map("n", "<leader>dq", vim.diagnostic.setloclist, { desc = "Diagnostic loclist" })

-- map({ "n", "i", "v" }, "<C-s>", "<cmd> w <cr>")
