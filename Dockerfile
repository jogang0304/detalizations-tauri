FROM ubuntu:20.04 as builder

WORKDIR /tmp

ENV APT_REQUIREMENTS="git \
    curl \
    wget \
    file \
    libxdo-dev \
    libayatana-appindicator3-dev \
    libgtk-3-dev \
    libwebkit2gtk-4.0-dev \
    libappindicator3-dev \
    librsvg2-dev \
    patchelf \
    software-properties-common \
    ca-certificates \
    build-essential \
    libssl-dev \
    zlib1g-dev \
    libncurses5-dev \
    libncursesw5-dev \
    libreadline-dev \
    libsqlite3-dev \
    libgdbm-dev \
    libdb5.3-dev \
    libbz2-dev \
    libexpat1-dev \
    liblzma-dev \
    libffi-dev \
    tar \
    xz-utils" \
    TZ=Europe/Zurich \
    DEBIAN_FRONTEND=noninteractive \
    PYTHON_ARCHIVE="Python-3.11.14" \
    LD_LIBRARY_PATH="/usr/local/lib:$LD_LIBRARY_PATH" \
    PATH="/root/.cargo/bin:${PATH}"

RUN apt-get -y update && apt-get -y upgrade && \
    apt-get -y install ${APT_REQUIREMENTS} && \
    curl --output ${PYTHON_ARCHIVE}.tgz https://www.python.org/ftp/python/3.11.14/${PYTHON_ARCHIVE}.tgz && \
    tar -xf ${PYTHON_ARCHIVE}.tgz && \
    cd ${PYTHON_ARCHIVE} && \
    ./configure --enable-optimizations --enable-shared && \
    make -j $(nproc) && \
    make altinstall && \
    python3.11 -m ensurepip --default-pip && \
    python3.11 -m pip install --upgrade pip setuptools wheel && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash && \
    \. "$HOME/.nvm/nvm.sh" && \
    nvm install 20 && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

ENTRYPOINT cd /app && \
    python3.11 -m pip install -r requirements.txt && \
    \. "$HOME/.nvm/nvm.sh" && \
    nvm use 20 && \
    npm i && \
    ./src-tauri/sidecars/bootstrap.sh && \
    rustup update && \
    npm run tauri build -- --verbose --ci --bundles appimage
