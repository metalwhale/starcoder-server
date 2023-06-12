FROM rust:1.70.0

RUN rustup component add rustfmt

RUN apt update -y

COPY requirements.txt /usr/src/requirements.txt
RUN apt install -y python3-pip \
    && python3 -m pip install -r /usr/src/requirements.txt

RUN cd /usr/src/ \
    && git clone https://github.com/bigcode-project/starcoder.cpp.git \
    && cd starcoder.cpp \
    && git checkout 8d4b5845 \
    && make \
    && python3 convert-hf-to-ggml.py bigcode/gpt_bigcode-santacoder \
    && ./quantize models/bigcode/gpt_bigcode-santacoder-ggml.bin models/bigcode/gpt_bigcode-santacoder-ggml-q4_1.bin 3
