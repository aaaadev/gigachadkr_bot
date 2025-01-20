FROM oraclelinux:9-slim

COPY . /opt/gigachad-bot

RUN microdnf upgrade -y && \
    microdnf install gcc git curl -y

RUN mkdir -p /opt/rust /app

WORKDIR /opt/rust
RUN curl https://sh.rustup.rs -s >> rustup.sh
RUN chmod 755 /opt/rust/rustup.sh
RUN ./rustup.sh -y

ENV PATH=/root/.cargo/bin:$PATH

WORKDIR /opt/gigachad-bot
RUN cargo install --path .

WORKDIR /app
RUN rm -rf /opt/gigachad-bot /opt/rust

ENTRYPOINT ["gigachadbot"]