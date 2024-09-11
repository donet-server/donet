project('donet', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

git_bin = find_program('git', required: true)
cargo_bin  = find_program('cargo', required: true)
find_program('ninja', required: true)
sphinx_bin = find_program('sphinx-build', required: true)

daemon_bin = meson.project_name() + 'd'
git_url = 'https://gitlab.com/donet-server/donet'
donet_website_url = 'https://www.donet-server.org'
libdonet_website_url = 'https://docs.donet-server.org/libdonet'
sphinx_website_url = 'https://docs.donet-server.org'
vcs_tag = run_command(git_bin, 'rev-parse', 'HEAD', check: false).stdout().strip()

cargo_opts = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opts += [ '--target-dir', meson.project_build_root()  / 'target' ]

cargo_env = [ 'CARGO_HOME=' + meson.project_build_root()  / 'cargo-home' ]

if get_option('profile') == 'release'
  cargo_opts += [ '--release' ]
  rust_target = 'release'
  message('Building Donet for Release.')
else
  rust_target = 'debug'
  message('Building Donet for Debug.')
endif

feature_flags = []

if get_option('build_client_agent')
  feature_flags += [ 'client-agent' ]
  message('Building the Client Agent.')
endif

if get_option('build_message_director')
  feature_flags += [ 'message-director' ]
  message('Building the Message Director.')
endif

if get_option('build_state_server')
  feature_flags += [ 'state-server' ]
  message('Building the State Server.')
endif

if get_option('build_database_server')
  feature_flags += [ 'database-server' ]
  message('Building the Database Server.')
endif

if get_option('build_dbss')
  feature_flags += [ 'dbss' ]
  message('Building the DBSS.')
endif

if get_option('build_event_logger')
  feature_flags += [ 'event-logger' ]
  message('Building the Event Logger.')
endif

if feature_flags.length() > 0
  cargo_opts += [ '--no-default-features', '--features', feature_flags ]
else
  message('Building all default Donet services.')
endif

subdir('donet')
subdir('docs')

run_target(
  'update',
  env: cargo_env,
  command: [ cargo_bin, 'update', '--verbose' ]
)

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  command: [
    'env', cargo_env, cargo_bin, 'build',
    cargo_opts,
    # Replace below with `--out-dir` once stable.
    # Issue: https://github.com/rust-lang/cargo/issues/6790#issuecomment-655728327=
    '&&', 'cp', 'target' / rust_target / daemon_bin, '@OUTPUT@',
  ]
)

cargo_doc = custom_target(
  'cargo-doc',
  build_by_default: false,
  build_always_stale: true,
  output: 'doc',
  console: true,
  command: [
    'env', cargo_env, cargo_bin, 'doc',
    cargo_opts, '--lib', '--no-deps', '--document-private-items',
    '--config', 'build.rustdocflags="--default-theme=ayu"'
  ],
)

run_target(
  'linting',
  env: cargo_env,
  command: [
    cargo_bin, 'clippy', cargo_opts, '&&',
    cargo_bin, 'fmt', '--all', '--', '--color=always', '--check',
  ],
)

run_target(
  'deny',
  env: cargo_env,
  command: [
    cargo_bin, 'install', 'cargo-deny', '--locked',
    '&&', 'env', cargo_env, cargo_bin, 'deny',
    '--manifest-path', meson.project_source_root() / 'Cargo.toml', 'check'
  ],
)

# Runs cargo-tarpaulin to create code coverage JSON and XML files,
# which is used by the GitLab CI/CD job to upload to codecov.io.
run_target(
  'code-coverage',
  env: cargo_env,
  command: [
    cargo_bin, 'install', 'cargo-tarpaulin',
    '&&', 'env', cargo_env, cargo_bin, 'tarpaulin',
    '--force-clean',
    '--exclude-files', 'build/*',
    '--exclude-files', 'builddebug/*',
    '--out', 'xml',
    '--output-dir', meson.project_build_root() / 'target' / 'tarpaulin',
    cargo_opts
  ],
)

# Build unit tests without running. This outputs the binary
# for the unit tests in the target directory, which can then
# be debugged using GDB or an equivalent.
run_target(
  'build-tests',
  env: cargo_env,
  command: [
    cargo_bin, 'test', '--no-run', cargo_opts,
  ]
)

# Wrapper for `meson test`, but adds the `--verbose`
# flag so it actually outputs `cargo test` stdout/stderr.
run_target(
  'tests',
  command: [ 'meson', 'test', '--verbose' ],
)

test(
  'cargo-test',
  cargo_bin,
  env: cargo_env,
  args: [
    'test', '--color=always',
    cargo_opts,
  ],
  is_parallel: false,
  timeout: 1800,
)

message('Setting up Git pre-commit hook..')
run_command('cp', '-f', 'build-aux/git/pre-commit.hook', '.git/hooks/pre-commit', check: false)

meson.add_dist_script(
  'build-aux/meson/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.project_source_root()
)
