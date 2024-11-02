FROM donet/base:0.1.0

RUN meson setup build -Dprofile=release -Dbuild_message_director=true;
RUN meson compile -C build;

CMD ["./build/donetd", "./data/daemon.toml"]
