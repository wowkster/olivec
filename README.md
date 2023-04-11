# A Safe Rust Wrapper for olive.c

[![Crates.io](https://img.shields.io/crates/v/olivec)](https://crates.io/crates/olivec)
![GitHub](https://img.shields.io/github/license/wowkster/olivec?color=blue)

<p align="center">
<a href="https://tsoding.github.io/olive.c/"><img src="https://raw.githubusercontent.com/tsoding/olive.c/274eb615187415bf4603c79fb4b7458ff2a15811/assets/olivec-200.png"></a>
</p>

**IMPORTANT! THIS LIBRARY IS A WORK IN PROGRESS! ANYTHING CAN CHANGE AT ANY MOMENT WITHOUT ANY NOTICE! USE THIS LIBRARY AT YOUR OWN RISK!**

This library provides a safe Rusty wrapper over the native C functions in olive.c

The wrapper is based on the bindgen bindings from [olivec-sys](https://github.com/kaiserthe13th/olivec-sys/).

## What is olive.c?

olive.c is a simple graphics library that does not have any dependencies and renders everything into the given memory pixel by pixel.

Visit [https://tsoding.github.io/olive.c/](https://tsoding.github.io/olive.c/) to see some demos.

The library is not concerned with displaying the image. It only fills up the memory with pixels. It's up to you what to do with those pixels.

The name is pronounced as "olivets'" which is a Ukrainian word for "pencil" (["олівець"](https://translate.google.com/?sl=uk&tl=en&text=%D0%BE%D0%BB%D1%96%D0%B2%D0%B5%D1%86%D1%8C&op=translate)).
