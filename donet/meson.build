conf = configuration_data()
conf.set_quoted('BINARY', daemon_bin)
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GIT_URL', git_url)
conf.set_quoted('VCS_TAG', vcs_tag)

configure_file(
  input: 'src' / 'meson.rs.in',
  output: 'meson.rs',
  configuration: conf
)

# Copy the meson.rs source file to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'donet' / 'meson.rs',
  meson.project_source_root() / 'donet' / 'src' / 'meson.rs',
  check: true
)
