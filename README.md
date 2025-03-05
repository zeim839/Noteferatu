<div align="center">
<h1>NoteFeratu</h1>
<img src="https://github.com/user-attachments/assets/480cba58-1efe-4909-a90b-8ddc3452f06e" width="300px" />
<h3>

[Install](#install) | [Build](#build) | [Releases](https://github.com/zeim839/Noteferatu/releases) | [Issues](https://github.com/zeim839/Noteferatu/issues)

</h3>

![GitHub issues](https://img.shields.io/github/issues-raw/zeim839/Noteferatu)
![GitHub Actions Workflow Status](https://img.shields.io/github/actions/workflow/status/zeim839/Noteferatu/tauri-build.yml)
![GitHub](https://img.shields.io/github/license/zeim839/Noteferatu)

NoteFeratu is a plain-text personal knowledge management system with LLM capabilities. Build your second brain, discover new information through knowledge graphs, and chat with your notes using frontier LLM models.
</div>

## Install
To install NoteFeratu, begin by installing the following dependencies:
 - [NodeJS & NPM](https://nodejs.org/en)
 - [Tauri (requires Rust)](https://v2.tauri.app/start/prerequisites/)

Next, clone the repository:
```
git clone https://github.com/zeim839/Noteferatu.git
```

Enter the repository directory:
```
cd Noteferatu
```

Install NPM dependencies:
```
npm i
```
## Usage
You can run a development version of NoteFeratu either through the NextJS dev server or directly via Tauri. Tauri is a web app bundler that exposes NoteFeratu as a native desktop application.

To start the NextJS development server (which launches NoteFeratu on the browser), run:
```
npm run dev
```

To start the Tauri app preview, run:
```
npm run tauri dev .
```

## Build
To build NoteFeratu and export it as a Desktop application, run:
```
npm run tauri build .
```

## License
[GPL-3.0-or-later](LICENSE) <br/>
Copyright (C) 2025 Michail Zeipekki, Daniel Wildsmith, Mathew Alangadan, Grayson Kornberg, Anton Salvador
