FROM centos:7

ARG CARGO_NET_GIT_FETCH_WITH_CLI=true

ARG VARNISH_VERSION
ARG VMOD_REVISION

RUN yum -y update && yum -y install \
      epel-release \
      yum-plugin-priorities \
      apt-transport-https \
      curl \
      centos-release-scl \
      rpm-build \
      git \
      python \
      python-docutils \
      openssl-devel \
      clang \
      python3 \
      && yum -y install llvm-toolset-7 \
      && yum clean all

RUN curl -s https://packagecloud.io/install/repositories/varnishcache/varnish73/script.rpm.sh | bash

RUN yum -y install \
      varnish-devel

RUN curl https://sh.rustup.rs -sSf | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"

RUN mkdir -p /root/rpmbuild/SPECS && \
    mkdir -p /root/rpmbuild/SOURCES/libvmod-unleash

COPY ./pkg/rpm/vmod-unleash.spec /root/rpmbuild/SPECS

COPY . /root/rpmbuild/SOURCES/libvmod-unleash

RUN cd /root/rpmbuild/SOURCES/ && \
    tar -zcvf vmod-unleash-$VARNISH_VERSION.tar.gz libvmod-unleash

RUN LIBCLANG_PATH=/opt/rh/llvm-toolset-7/root/usr/lib64 \
    rpmbuild -ba /root/rpmbuild/SPECS/vmod-unleash.spec
