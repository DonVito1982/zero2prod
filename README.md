# Comandos para trabajar

* `find src/ tests/ -path *.rs | entr -c cargo fmt --all -- --check` revisa el formato del archivo a medida que se edita
* `find src/ -path *.rs | entr -c cargo build` ejecuta el _build_ del proyecto para mostrar errores/advertencias de compilación
