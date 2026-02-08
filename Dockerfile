FROM ubuntu:20.04

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
    PYTHON_ARCHIVE="Python-3.11.14"

RUN apt-get -y update && apt-get -y upgrade && \
    apt-get -y install ${APT_REQUIREMENTS} && \
    curl --output ${PYTHON_ARCHIVE}.tgz https://www.python.org/ftp/python/3.11.14/${PYTHON_ARCHIVE}.tgz && \
    tar -xf ${PYTHON_ARCHIVE}.tgz && \
    cd ${PYTHON_ARCHIVE} && \
    ./configure --enable-optimizations && \
    make -j $(nproc) && \
    make altinstall && \
    python3.11 -m ensurepip --default-pip && \
    python3.11 -m pip install --upgrade pip setuptools wheel && \
    curl -o- https://raw.githubusercontent.com/nvm-sh/nvm/v0.40.3/install.sh | bash && \
    \. "$HOME/.nvm/nvm.sh" && \
    nvm install 20 && \
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y

WORKDIR /app

ENV PATH="/root/.cargo/bin:${PATH}"

COPY . .

RUN \. "$HOME/.nvm/nvm.sh" && \
    nvm use 20 && \
    ./bootstrap.sh && \
    cargo cache --autoclean && \
    npm cache clean --force
