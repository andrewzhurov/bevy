((nil . (
         ;; 1. Only load this example's manifest
         ;; (lsp-rust-analyzer-linked-projects . ["${proj-root}/examples/some-example/Cargo.toml"])

         ;; 2a. Only check this example on save
         (lsp-rust-analyzer-cargo-watch-args . ["--example" "borders"])

         ;; 2b. Only run this example via CodeLens/runnables
         (lsp-rust-analyzer-cargo-extra-args . ["--example" "borders"])

         ;; 3. Speed up reuse of build artifacts
         (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t)
         )))
