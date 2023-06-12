FROM starcoder-server-server AS build

COPY ./ /usr/src/server/
RUN cd /usr/src/server \
    && cargo build --release

FROM debian:stable-slim

COPY --from=build /usr/src/server/target/release/server /usr/src/server
COPY --from=build /usr/src/starcoder.cpp/main /usr/src/inference
COPY --from=build /usr/src/starcoder.cpp/models/bigcode/gpt_bigcode-santacoder-ggml-q4_1.bin /usr/src/model.bin

ENTRYPOINT [ "/usr/src/server", "/usr/src/inference", "/usr/src/model.bin" ]
