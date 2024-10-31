display_build_options = ''
display_feature_flags = ''

display_build_options = meson.build_options()

if display_build_options == ''
  display_build_options = 'None (defaults)'
endif

index = 0 # can't keep index any other way
foreach m : feature_flags
  display_feature_flags += m

  if index != feature_flags.length() - 1
    display_feature_flags += ', '
  endif
  index += 1
endforeach

if display_feature_flags == ''
  display_feature_flags = 'None (defaults)'
endif

conf = configuration_data()
conf.set_quoted('BINARY', daemon_bin)
conf.set_quoted('VERSION', meson.project_version())
conf.set_quoted('GIT_URL', git_url)
conf.set_quoted('VCS_TAG', vcs_tag)
conf.set_quoted('COMPILE_TIME', build_timestamp)
conf.set_quoted('BUILD_OPTIONS', display_build_options)
conf.set_quoted('FEATURE_FLAGS', display_feature_flags)

configure_file(
  input: 'src' / 'meson.rs.in',
  output: 'meson.rs',
  configuration: conf
)

# Copy the meson.rs source file to the source directory.
run_command(
  'cp',
  meson.project_build_root() / 'donet-daemon' / 'meson.rs',
  meson.project_source_root() / 'donet-daemon' / 'src' / 'meson.rs',
  check: true
)
