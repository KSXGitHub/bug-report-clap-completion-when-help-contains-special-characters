# Bug Report

Markdown links in the help message of a value enum causes ZSH completion to error.

## Steps to reproduce

1. Clone or download this repo.
2. Execute `./playground.zsh` (which will build the binary, the ZSH completion, and open an interactive ZSH shell with said completion enabled).
3. Type `bug-report-clap-completion-when-help-contains-special-characters --website=` and then press <kbd>TAB</kbd>.

## Expected behavior

Completion works normally.

## Actual behavior

The following error messages appear instead:

```
(eval):1: unmatched "
_arguments:465: command not found: (
(eval):1: unmatched "
_arguments:465: command not found: (
(eval):1: unmatched "
_arguments:465: command not found: (
```

## Suggestion

Leaving markdown code in a help message is likely a mistake. I suggest fixing it by doing one of the following:
1. Emit a compiler error when there's markdown code in the help message.
2. Transform the markdown hyperlinks into actual [ASCII hyperlinks](https://gist.github.com/egmontkob/eb114294efbcd5adb1944c9f3cb5feda).

I personally prefer option 2, but if it turns out to be too complicated to implemented, option 1 is fine as well.
