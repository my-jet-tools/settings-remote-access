FROM ubuntu:22.04
COPY ./target/release/settings-remote-access ./target/release/settings-remote-access
ENTRYPOINT ["./target/release/settings-remote-access"]
