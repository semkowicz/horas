Horas
=====

Horas is a tool for generating booklets for the Liturgia Horarum.

## Getting started

This project is in an early development state. We do not provide stable binaries available for easy
installation at this point. In order to use the Horas application, you need to build it
from sources.

### Dependencies

The Horas project is written mostly in Rust. The Rust compiler and Cargo build tool are required
for building. Please follow https://www.rust-lang.org/ to install these tools on your system.

### Building the project

Fetch this repository:

```shell
git clone https://github.com/semkowicz/horas.git
cd horas
```

Build the debug version:

```shell
cargo build
```

Install the release version:

```shell
cargo install --path .
```

### External assets

At this stage, most of the interfaces to external assets are not implemented. Some assets
need to be manually downloaded to the project data directory.

```shell
HORAS_PROJECT_DIR=$(pwd)
HORAS_DATA_DIR=${HOME}/.local/share/horas
mkdir ${HORAS_DATA_DIR} 

cd ${HORAS_DATA_DIR}
git clone https://github.com/DivinumOfficium/divinum-officium.git

mkdir psalm-tone-tool
cd psalm-tone-tool
wget https://www.semkowicz.pl/horas/psalm-tone-tool/{canticles-gabc,canticles,psalms-gabc,psalms}.zip
for f in *.zip; do unzip ${f} -d ${f%.zip}; done
cd ..

ln -s ${HORAS_PROJECT_DIR}/templates templates
```
