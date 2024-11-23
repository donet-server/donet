conf = configuration_data()
conf.set('GIT_HOST', git_host)
conf.set('GIT_HOST_USER', git_host_user)
conf.set('GIT_HOST_REPO', git_host_repo)
conf.set('GIT_URL', git_url)
conf.set('DONET_URL', donet_website_url)
conf.set('LIBDONET_URL', libdonet_website_url)
conf.set('SPHINX_URL', sphinx_website_url)

# The Git branch name is the version name for sphinx docs in the live site
git_branch = get_option('ci_branch')

if ci_branch = ''
  git_branch = run_command(git_bin, 'branch', '--show-current', check: true).stdout().strip()
endif

message('Building Sphinx docs with version "' + git_branch + '"')

configure_file(
  input: 'conf.py.in',
  output: 'conf.py',
  configuration: conf
)

configure_file(
  input: '_templates' / 'layout.html.in',
  output: 'layout.html',
  configuration: conf
)

# Copy the configured conf.py file to the Sphinx source directory.
run_command(
  'cp',
  meson.project_build_root() / 'docs' / 'conf.py',
  meson.project_source_root() / 'docs' / 'conf.py',
  check: true
)

# Copy the configured layout.html file to the _templates directory.
run_command(
  'cp',
  meson.project_build_root() / 'docs' / 'layout.html',
  meson.project_source_root() / 'docs' / '_templates' / 'layout.html',
  check: true
)

if sphinx_bin.found()
  sphinx_docs = custom_target(
    'sphinx',
    build_by_default: false,
    build_always_stale: true,
    output: 'sphinx',
    command: [
      sphinx_bin, '--jobs', 'auto', '--write-all', '--fresh-env',
      meson.project_source_root() / 'docs',
      meson.project_build_root() / 'sphinx' / git_branch
    ]
  )
endif
