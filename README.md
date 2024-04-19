# cr023
> A portfolio website dedicated to showing my work in a more creative environment.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![WASM](https://img.shields.io/badge/WebAssembly-8541fa?style=for-the-badge&logo=WebAssembly&logoColor=white)
![Sass](https://img.shields.io/badge/Sass-CC6699?style=for-the-badge&logo=sass&logoColor=white)

<br />

#### Overview
This project specifically is different as I used it as an introduction to the Rust programming language with its web assembly compilation feature. This was created utilizing the <b><a href="https://yew.rs/">Yew</a></b> framework which is very similar to **React** for creating front end web services.

<br />

#### Notes
I am aware this could have been easily created with traditional **React** or even **JQuery** to be honest. However, as mentioned above, this project is mainly for learning Rust and it was by no means practical in comparison. :cry:

<br />

#### Development Environment
The setup below follows **Yew** setup instrucions <a href="https://yew.rs/docs/getting-started/introduction">here</a>.

Add **WebAssembly** as a compilation target:
```bash
rustup target add wasm32-unknown-unknown
```

Install trunk which is used for bundling **WASM** appliactions:
```bash
cargo install --locked trunk wasm-bindgen-cli
```

In the root directory, run:
```bash
trunk serve
```
to serve the **WASM** application to `http://localhost:8080`.

<br />
<br />

<div align="center">
  Developed by <b>Connell Reffo</b> in <b>2024</b>
</div>
