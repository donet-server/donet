FROM donet/base:0.1.0

RUN meson setup build -Dprofile=release -Ddockerized=true -Dbuild_event_logger=true;
RUN meson compile -C build;

CMD ["./build/donetd", "./data/daemon.toml"]
