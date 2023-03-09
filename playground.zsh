#!/bin/zsh
set -o errexit -o pipefail -o nounset

msg1() {
  echo '==>' $@ >&2
}

msg2() {
  echo ' ->' $@ >&2
}

cd "$(dirname "$0")"
wdir=$(pwd)

msg1 'Building the binary...'
cargo build
path+=$wdir/target/debug
export PATH

msg1 'Building the completion...'
mkdir -p completions
bug-report-clap-completion-when-help-contains-special-characters --completion=zsh > completions/_bug-report-clap-completion-when-help-contains-special-characters
fpath+=$wdir/completions
export FPATH

msg1 'Rebuilding completion cache...'
ls -A $ZDOTDIR | while read -r filename; do
  if [[ $filename = .zcompdump* ]]; then
    fullpath=$ZDOTDIR/$filename
    msg2 "Deleting $fullpath..."
    rm $fullpath
  fi
done

msg1 'Entering a new ZSH context...'
msg2 'hint: Run `compinit` to initialize the completions'
msg2 'hint: Try writing `bug-report-clap-completion-when-help-contains-special-characters --website=` then press <TAB> to see what happens'
zsh -i
