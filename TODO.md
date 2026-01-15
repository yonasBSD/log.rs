# TODO

## Features
- OpenTelemetry
  - opentelemetry-otlp
  - logs (Loki), metrics (Prometheus), and traces (Jaeger)
- Remote storage
  - Standard Storage Protocols 	     ftp http sftp webdav
  - Object Storage Services 	     azblob cos gcs obs oss s3 b2 openstack_swift upyun vercel-blob
  - File Storage Services            fs alluxio azdls azfile compfs dbfs gridfs hdfs hdfs-native ipfs webhdfs **zfs**
  - Consumer Cloud Storage Service   aliyun-drive gdrive onedrive dropbox koofr pcloud seafile yandex-disk
  - Key-Value Storage Services 	     cacache cloudflare-kv dashmap memory etcd foundationdb persy redis rocksdb sled redb tikv
  - Database Storage Services 	     d1 mongodb mysql postgresql sqlite surrealdb
  - Cache Storage Services           ghac memcached mini-moka moka vercel-artifacts
- Append-only
  - File-based
  - Distributed blockchain
- Format and Schema
  - fingerprint
    - hash: BLAKE-3
    - signature: minisign
    - append-only blockchain: ...
  - RFC3339 nanosecond precision
  - pub/sub via #hashtags
- Encryption
  - quantum-safe encryption
- Security
  - automatic secret scanner
- Config
  - log.toml and ~/.config/log/config.toml
  - credentials
  - pipelines (sources and sinks like vector)
  - names, aliases, categories, tags
- Benchmarks
  - took() should return how long it took since calling init()

## Deployments

### Deployment Stack
- OpenStack: bare metal provisioning
- Terraform: virtual machine cloud provisioning
- Orchestration: Kubernetes pods
- Services: Docker containers
- Binaries: per OS packages and repositories
- Source code: Gitea / Forgejo
- PKI

### Docker

Add Dockerfile and docker-compose.yaml.

### Helm / Kubernetes

Add helm chart.

### Packages and Binaries
- Flatpack
- AppImage
- Ubuntu Snap
- Nix

### Architecture
- OS
  - Linux
    - Ubuntu / Debian 
    - Alpine
    - OpenSUSE
    - Fedora / Rocky
    - Arch
    - Slackware
    - Gentoo
  - BSD
    - FreeBSD
    - OpenBSD
    - NetBSD
  - Mac OS X
  - Windows
- Arch
  - amd64
  - arm v6, v7, v8
  - risc v

### Encryption / PKI
- Sign all assets
  - sigstore
  - minisign [ ed25519 ]
  - GPG [ ed25519 ]
- Checksums
  - BLAKE-3

