$ErrorActionPreference = "Stop"
Set-StrictMode -Version Latest

$workspace = ridk exec bash -lc "cygpath -au '$env:GITHUB_WORKSPACE'"
$gemdir = "$workspace/packages/ruby"
ridk exec bash -lc "cd $gemdir && bundle exec rake clean && bundle exec rake build"
