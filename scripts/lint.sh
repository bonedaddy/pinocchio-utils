#! /bin/bash

cargo fmt

# Then check if any changes were made (and show them)
if ! cargo fmt -- --check; then
    echo "${YELLOW}Some files were not formatted properly. The changes have been applied.${RESET}"
    # Show what changed
    cargo fmt -- --check -v
    echo "${GREEN}✓ Formatting has been fixed${RESET}"
else
    echo "${GREEN}✓ Formatting looks good!${RESET}"
fi
# Run comprehensive clippy checks
print_header "Running clippy with extended checks..."
CLIPPY_FLAGS=(
    "-D" "warnings"                # Make all warnings deny
    "-W" "clippy::all"            # All default lints
    "-W" "clippy::cargo"          # Cargo-related lints
    "-D" "clippy::perf"           # Performance-related lints
    "-D" "clippy::complexity"     # Code complexity lints
    "-D" "clippy::style"          # Style lints
    # Exclude some overly strict lints
    "-A" "clippy::must_use_candidate"
    "-A" "clippy::missing_errors_doc"
    "-A" "clippy::module_name_repetitions"
    "-A" "clippy::multiple_crate_versions"
    "-A" "clippy::cargo_common_metadata"
    "-A" "clippy::negative_feature_names"
)

# Run clippy on all targets (including tests and examples)
if cargo clippy --all-targets --fix -- "${CLIPPY_FLAGS[@]}"; then
    echo "${GREEN}✓ No clippy warnings!${RESET}"
else
    echo "${RED}✗ Clippy found some issues${RESET}"
    echo "Please fix the issues above and try again"
    exit 1
fi


