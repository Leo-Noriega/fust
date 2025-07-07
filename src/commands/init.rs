pub async fn execute(shell: &str) -> Result<(), Box<dyn std::error::Error>> {
    match shell.to_lowercase().as_str() {
        "bash" => print_bash_integration(),
        "zsh" => print_zsh_integration(),
        "fish" => print_fish_integration(),
        _ => {
            eprintln!("Unsupported shell: {shell}. Supported shells: bash, zsh, fish");
            std::process::exit(1);
        }
    }
    Ok(())
}

fn print_bash_integration() {
    println!(
        r#"# Fust integration for Bash
# Add this to your ~/.bashrc

__fust_fud() {{
    local result
    result=$(fust fud 2>/dev/null)
    if [[ -n "$result" ]]; then
        cd "$result" || return 1
        echo "Changed to: $result"
    fi
}}

__fust_fuf() {{
    fust fuf
}}

# Main commands
alias fud='__fust_fud'
alias fuf='__fust_fuf'

# Short aliases
alias fd='__fust_fud'
alias ff='__fust_fuf'

# Optional: Add to PROMPT_COMMAND for better integration
if [[ -z "${{PROMPT_COMMAND}}" ]]; then
    PROMPT_COMMAND="__fust_update_pwd"
else
    PROMPT_COMMAND="${{PROMPT_COMMAND}};__fust_update_pwd"
fi

__fust_update_pwd() {{
    export FUST_PWD="$PWD"
}}
"#
    );
}

fn print_zsh_integration() {
    println!(
        r#"# Fust integration for Zsh
# Add this to your ~/.zshrc

__fust_fud() {{
    local result
    result=$(fust fud 2>/dev/null)
    if [[ -n "$result" ]]; then
        cd "$result" || return 1
        echo "Changed to: $result"
    fi
}}

__fust_fuf() {{
    fust fuf
}}

# Main commands
alias fud='__fust_fud'
alias fuf='__fust_fuf'

# Short aliases
alias fd='__fust_fud'
alias ff='__fust_fuf'

# Zsh hooks for better integration
autoload -Uz add-zsh-hook

__fust_update_pwd() {{
    export FUST_PWD="$PWD"
}}

add-zsh-hook chpwd __fust_update_pwd

# Initialize
__fust_update_pwd
"#
    );
}

fn print_fish_integration() {
    println!(
        r#"# Fust integration for Fish
# Add this to your ~/.config/fish/config.fish

function __fust_fud
    set result (fust fud 2>/dev/null)
    if test -n "$result"
        cd "$result"
        echo "Changed to: $result"
    end
end

function __fust_fuf
    fust fuf
end

# Main commands
alias fud='__fust_fud'
alias fuf='__fust_fuf'

# Short aliases
alias fd='__fust_fud'
alias ff='__fust_fuf'

# Fish variable for PWD tracking
function __fust_update_pwd --on-variable PWD
    set -gx FUST_PWD $PWD
end

# Initialize
__fust_update_pwd
"#
    );
}

pub fn print_installation_instructions() {
    println!(
        r#"
🚀 Fust Installation Instructions

1. Add fust to your PATH:
   sudo cp ./target/release/fust /usr/local/bin/
   # or add to ~/.bashrc: export PATH="$PATH:/path/to/fust/target/release"

2. Generate shell integration:
   fust init bash   # for Bash
   fust init zsh    # for Zsh  
   fust init fish   # for Fish

3. Add the output to your shell config:
   # For Bash: ~/.bashrc
   # For Zsh: ~/.zshrc
   # For Fish: ~/.config/fish/config.fish

4. Reload your shell:
   source ~/.bashrc  # or ~/.zshrc

5. Use the commands:
   fud  # Fuzzy directory navigation
   fuf  # Fuzzy file navigation
   fd   # Short alias for fud
   ff   # Short alias for fuf

Example installation for Zsh:
   echo '# Fust integration' >> ~/.zshrc
   fust init zsh >> ~/.zshrc
   source ~/.zshrc
"#
    );
}
