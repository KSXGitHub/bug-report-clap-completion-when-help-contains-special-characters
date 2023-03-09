use clap::{CommandFactory, Parser, ValueEnum};
use clap_complete::{generate, Shell};

const BIN_NAME: &str = "bug-report-clap-completion-when-help-contains-special-characters";

#[derive(Debug, Clone, ValueEnum)]
enum Website {
    /// This documentation is contains a link to [GitHub](https://github.com).
    /// This link would not play well with shell completion.
    ///
    /// It was a mistake to not also specify `help` attribute for `clap` as it messes up the completion script,
    /// but there was no error message during compilation.
    #[clap(name = "github")]
    GitHub,

    /// This documentation is contains a link to [GitLab](https://gitlab.com).
    /// This link would not play well with shell completion.
    ///
    /// It was a mistake to not also specify `help` attribute for `clap` as it messes up the completion script,
    /// but there was no error message during compilation.
    #[clap(name = "gitlab")]
    GitLab,
}

#[derive(Debug, Parser)]
struct Args {
    /// Press TAB on this flag to see the bug.
    #[clap(long, short, value_enum)]
    website: Option<Website>,

    /// Generate completion for this program.
    #[clap(long, short, value_enum)]
    completion: Option<Shell>,
}

fn run_generator(shell: Shell) {
    generate(
        shell,
        &mut Args::command(),
        BIN_NAME,
        &mut std::io::stdout(),
    );
}

fn main() {
    let args = Args::parse();
    if let Some(shell) = args.completion {
        return run_generator(shell);
    }
    println!("{args:?}");
}
