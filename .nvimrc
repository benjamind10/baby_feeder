
local lspconfig = require("lspconfig")

lspconfig.rust_analyzer.setup({
  cmd = { os.getenv("RUST_ANALYZER_WRAPPED") },
  settings = {
    ["rust-analyzer"] = {
      cargo = { allFeatures = true },
      procMacro = { enable = true },
      checkOnSave = { command = "clippy" },
    },
  },
})

