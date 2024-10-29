
-- Require necessary plugins
local lspconfig = require("lspconfig")
local cmp = require("cmp")

-- Setup nvim-cmp for autocompletion
cmp.setup({
  snippet = {
    expand = function(args)
      vim.fn["vsnip#anonymous"](args.body)  -- Adjust if using a different snippet engine
    end,
  },
  mapping = cmp.mapping.preset.insert({
    ["<C-Space>"] = cmp.mapping.complete(),       -- Manually trigger completion
    ["<CR>"] = cmp.mapping.confirm({ select = true }), -- Confirm selection with Enter
    ["<C-e>"] = cmp.mapping.abort(),              -- Abort completion
  }),
  sources = cmp.config.sources({
    { name = "nvim_lsp" },
    { name = "buffer" },
  }),
})

-- Setup rust-analyzer with nvim-lspconfig
local capabilities = require("cmp_nvim_lsp").default_capabilities()

lspconfig.rust_analyzer.setup({
  cmd = { "rust-analyzer" },  -- Assuming rust-analyzer is available in PATH
  trace = "verbose",
  capabilities = capabilities,
  settings = {
    ["rust-analyzer"] = {
      cargo = { allFeatures = true },
      procMacro = { enable = true },
      checkOnSave = {
        command = "clippy",
      },
    },
  },
})

