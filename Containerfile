FROM alpine:3.23 AS models

RUN --mount=type=cache,target=/var/cache/apk \
  apk add \
    curl \
    tar
RUN mkdir -p /app/models/stt
RUN mkdir -p /app/models/tts

# Download STT model files
RUN curl --location \
  --output /app/models/stt/model.onnx \
  https://huggingface.co/onnx-community/parakeet-ctc-0.6b-ONNX/resolve/main/onnx/model.onnx
RUN curl --location \
  --output /app/models/stt/model.onnx_data \
  https://huggingface.co/onnx-community/parakeet-ctc-0.6b-ONNX/resolve/main/onnx/model.onnx_data
RUN curl --location \
  --output /app/models/stt/tokenizer.json \
  https://huggingface.co/onnx-community/parakeet-ctc-0.6b-ONNX/resolve/main/tokenizer.json

# Download TTS model files
RUN curl --location \
  https://github.com/k2-fsa/sherpa-onnx/releases/download/tts-models/vits-piper-en_US-amy-medium-fp16.tar.bz2 \
  | tar xjf - -C /app/models/tts


FROM rust:1.92-alpine3.23 AS builder

ARG BUILD_PROFILE=release

WORKDIR /app

RUN --mount=type=cache,target=/var/cache/apk \
  apk add \
    alpine-sdk \
    alsa-lib-dev \
    clang21-dev \
    cmake \
    espeak-ng-dev \
    llvm21-dev \
    onnxruntime-dev \
    pcaudiolib-dev \
    rustfmt

COPY . .

# fix audiopus_sys crate build
ENV CMAKE_POLICY_VERSION_MINIMUM=3.5

# fix espeak-rs-sys and sherpa-rs-sys crate build and final linking
ENV CXXFLAGS="-include cstdint"
ENV RUSTFLAGS="-C target-feature=-crt-static -lonnxruntime -lpcaudio"

RUN \
  --mount=type=cache,target=/usr/local/cargo/git/ \
  --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release --no-default-features --features stt,tts


FROM alpine:3.23

COPY --from=models /app/models /app/models

WORKDIR /app

# fix espeak and ort
ENV ESPEAK_DATA_PATH=/usr/share/espeak-ng-data
ENV ORT_DYLIB_PATH=/usr/lib/libonnxruntime.so.1

RUN --mount=type=cache,target=/var/cache/apk \
  apk add \
    alsa-lib \
    espeak-ng \
    libgcc \
    libstdc++ \
    onnxruntime \
    pcaudiolib

COPY --from=builder /app/target/release/wis-rs /app

RUN adduser -D -h /app -s /sbin/nologin -u 1000 willow
USER willow

ENTRYPOINT ["/app/wis-rs"]
