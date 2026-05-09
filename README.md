# PC Toolkit gerado por IA
Toolkit de diagnóstico e reparo para Windows feito em Rust.

## Features

- Informações do sistema
- Benchmark básico
- Verificação de disco
- Reparo do Windows
- Exportação de relatório

## Compilar

```bash
cargo build --release
```

## Windows Build

```bash
cargo build --release --target x86_64-pc-windows-gnu
```

## Roadmap

- SMART real
- Temperaturas
- Interface TUI
- Relatórios HTML
- Benchmark avançado

## Requisitos

Para as cores funcionarem corretamente no CMD do Windows, execute o seguinte comando como administrador:

```cmd
reg add HKCU\Console /v VirtualTerminalLevel /t REG_DWORD /d 1 /f
```

Após executar, feche e abra o CMD novamente.

> Não é necessário no Windows Terminal, que já suporta cores nativamente.
