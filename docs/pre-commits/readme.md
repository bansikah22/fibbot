Pre-commit
Making sure before any commit best practices are applied you can read more about it here [Pre-commit docs](https://pre-commit.com/)
```bash
## Installation
pip install --user pre-commit

echo 'export PATH=$PATH:/home/adorsys/.local/bin' >> ~/.zshrc

source ~/.zshrc

pre-commit --version


pre-commit run --all-file Pre-commit

pre-commit run check-yaml --all-files

pre-commit run --all-files
```
To install pre-commit, run:

```bash
pip install pre-commit## if you installed locally
```
