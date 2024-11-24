message('Building Sphinx docs with version "' + git_branch + '"')

conf = configuration_data()
conf.set_quoted('SPHINX_VERSION', sphinx_version)
conf.set_quoted('SPHINX_RELEASE', sphinx_release)
conf.set('DOC_VERSIONS', doc_versions)
conf.set_quoted('GIT_BRANCH', git_branch)
conf.set_quoted('GIT_HOST', git_host)
conf.set_quoted('GIT_HOST_USER', git_host_user)
conf.set_quoted('GIT_HOST_REPO', git_host_repo)
conf.set_quoted('GIT_URL', git_url)
conf.set_quoted('DONET_URL', donet_website_url)
conf.set_quoted('LIBDONET_URL', libdonet_website_url)
conf.set_quoted('SPHINX_URL', sphinx_website_url)

configure_file(
  input: 'conf.py.in',
  output: 'conf.py',
  configuration: conf
)

# Copy the configured conf.py file to the Sphinx source directory.
run_command(
  'cp',
  meson.project_build_root() / 'docs' / 'conf.py',
  meson.project_source_root() / 'docs' / 'conf.py',
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
