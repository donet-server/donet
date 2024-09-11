conf = configuration_data()
conf.set('GIT_URL', git_url)
conf.set('DONET_URL', donet_website_url)
conf.set('LIBDONET_URL', libdonet_website_url)
conf.set('SPHINX_URL', sphinx_website_url)

configure_file(
  input: '_templates' / 'layout.html.in',
  output: 'layout.html',
  configuration: conf
)

# Copy the configured layout.html file to the _templates directory.
run_command(
  'cp',
  meson.project_build_root() / 'docs' / 'layout.html',
  meson.project_source_root() / 'docs' / '_templates' / 'layout.html',
  check: true
)

sphinx_docs = custom_target(
  'sphinx',
  build_by_default: false,
  build_always_stale: true,
  output: 'sphinx',
  command: [
    sphinx_bin, meson.project_source_root() / 'docs',
    meson.project_build_root() / 'sphinx'
  ]
)
