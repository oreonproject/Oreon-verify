# Oreon-verify
A CLI tool used to check the integrity of Oreon ISOs
## 📦 Installation
Clone the repository and build it with Cargo:
```bash
git clone https://github.com/oreonproject/Oreon-verify
cd Oreon-verify
cargo build --release
```
## 🧪 Usage
```bash
oreon-verify -f <ISO_FILE> -H <HASH>
```
## Arguments

| Flag              | Description                                   |
|-------------------|-----------------------------------------------|
| `-f`, `--file`    | **Required** – Path to the ISO file to verify |
| `-H`, `--hash`    | **Required** – Expected hash to compare against |
## 💡 Example
```bash
oreon-verify -f oreon.iso -H 0d4a1183c7c19d1ef9e7d5e9cc70f5dc
```
Output (valid ISO):
```bash
ISO is fine!
```
Output (corrupted ISO):
```bash
ISO is corrupted!
```
## ⚠️ Errors
* Missing arguments will trigger a panic with a message.
* File or hash issues will print an error and exit.