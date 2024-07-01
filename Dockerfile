FROM lukemathwalker/cargo-chef:latest-rust-1.75.0 AS chef
WORKDIR /app

FROM chef AS planner
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS builder 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json
COPY . .
RUN cargo build --release --bin telegram-goal-bot

FROM debian:stable
WORKDIR /app
RUN apt-get update && apt-get install -y xvfb chromium
RUN rm -rf /var/lib/apt/lists/*
RUN apt-get install -y xvfb
RUN Xvfb :0 -ac -screen 0 1920x1080x24 -nolisten tcp &
ENV DISPLAY=:0
COPY --from=builder /app/target/release/telegram-goal-bot /usr/local/bin
RUN adduser appuser
RUN chmod 777 /usr/local/bin/telegram-goal-bot
USER appuser
CMD Xvfb :0 -ac -screen 0 1920x1080x24 -nolisten tcp & telegram-goal-bot
