FROM debian:stretch-slim
RUN apt update \
  && apt install -y gpg curl \
  && echo 'deb https://deb.torproject.org/torproject.org stretch main' > /etc/apt/sources.list.d/tor \
  && echo 'deb-src https://deb.torproject.org/torproject.org stretch main' >> /etc/apt/sources.list.d/tor \
  && curl https://deb.torproject.org/torproject.org/A3C4F0F979CAA22CDBA8F512EE8CBC9E886DDD89.asc | gpg --import \
  && gpg --export A3C4F0F979CAA22CDBA8F512EE8CBC9E886DDD89 | apt-key add - \
  && apt update \
  # && apt install -y tor deb.torproject.org-keyring \
  && rm -rf /var/cache/apk/* \
  && mkdir -p /run/tor/service