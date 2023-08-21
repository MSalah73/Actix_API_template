# Compute recipe file 
FROM clux/muslrust:stable as chef

WORKDIR /app

RUN apt-get update && apt-get install -y \
	lld \
	clang

# Compute lcok-like files for caching
FROM chef as planner

COPY . . 

RUN cargo install cargo-chef
# Compute a lock-like file for the project
RUN cargo chef prepare --recipe-path recipe.json

# Build UPX Stage
FROM chef as upx
#
RUN apt-get update && apt-get install -y build-essential curl cmake \
	&& mkdir -p /upx \
	&& curl -# -L https://github.com/upx/upx/releases/download/v4.0.1/upx-4.0.1-src.tar.xz | tar xJ --strip 1 -C /upx \
	&& make -C /upx build/release-gcc -j$(nproc) \
	&& cp -v /upx/build/release-gcc/upx /usr/bin/upx

# Caching and Building Stage 
FROM chef as builder

RUN cargo install cargo-chef

COPY --from=planner /app/recipe.json recipe.json

# Build our project dependencies, not out application 
RUN cargo chef cook --release --recipe-path recipe.json 
# Up to this point, if our dependency tree stays the same,
# all layer should be cached.
COPY . . 

# Build the application
RUN cargo build --bin app --release --target x86_64-unknown-linux-musl

# Optimazize the applization binary for lower size
COPY --from=upx /usr/bin/upx /usr/bin/upx

# Best option takes a long time for large binaries. Check UPX docs - https://github.com/upx/upx/blob/devel/doc/upx-doc.txt for more info
RUN upx --best --lzma /app/target/x86_64-unknown-linux-musl/release/app

# Runtime Stage 
FROM gcr.io/distroless/static:nonroot as runtime

WORKDIR /app
# Copy the compiled binary from the builder environment
# to the runtime 
COPY --from=builder --chown=nonroot:nonroot /app/target/x86_64-unknown-linux-musl/release/app app

# We need the configuration file at runtime 
COPY configuration configuration

ENV APP_ENVIRONMENT production

ENTRYPOINT ["./app"]
