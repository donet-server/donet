project('donet', 'rust',
          version: '0.1.0',
    meson_version: '>= 0.62.0',
  default_options: [ 'warning_level=2', 'werror=false', ],
)

git_bin = find_program('git', required: true)
cargo_bin  = find_program('cargo', required: true)
find_program('ninja', required: true)

daemon_bin = meson.project_name() + 'd'
git_url = 'https://gitlab.com/donet-server/donet'
vcs_tag = run_command(git_bin, 'rev-parse', 'HEAD', check: false).stdout().strip()

cargo_opts = [ '--manifest-path', meson.project_source_root() / 'Cargo.toml' ]
cargo_opts += [ '--target-dir', meson.project_build_root()  / 'target' ]

cargo_doc_opts = ['--no-deps', '--document-private-items']
cargo_doc_opts += ['--config', 'build.rustdocflags="--default-theme=ayu"']

if get_option('profile') == 'release'
  cargo_opts += [ '--release' ]
  rust_target = 'release'
  message('Building Donet for Release.')
else
  rust_target = 'debug'
  message('Building Donet for Debug.')
endif

subdir('libdonet')
subdir('donet')

cargo_build = custom_target(
  'cargo-build',
  build_by_default: true,
  build_always_stale: true,
  output: meson.project_name(),
  console: true,
  command: [
    cargo_bin, 'build',
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
    cargo_bin, 'doc',
    cargo_opts + cargo_doc_opts,
  ],
)

run_target(
  'linting',
  command: [
    cargo_bin, 'clippy', cargo_opts, '&&',
    cargo_bin, 'fmt', '--all', '--', '--color=always', '--check',
  ],
)

# Wrapper for `meson test`, but adds the `--verbose`
# flag so it actually outputs `cargo test` stdout/stderr.
run_target(
  'tests',
  command: ['meson', 'test', '--verbose'],
)

test(
  'cargo-test',
  cargo_bin,
  args: [
    'test', '--color=always',
    cargo_opts,
  ],
  is_parallel: false,
  timeout: 1800,
)

if get_option('profile') == 'debug'
  message('Setting up Git pre-commit hook..')
  run_command('cp', '-f', 'hooks/pre-commit.hook', '.git/hooks/pre-commit', check: false)
endif

meson.add_dist_script(
  'build-aux/meson/dist-vendor.sh',
  meson.project_build_root() / 'meson-dist' / meson.project_name() + '-' + meson.project_version(),
  meson.project_source_root()
)