# Excel to Microsoft SQL Server

I need a CLI that can efficiently move data that was previously exported from SQL Server (like through the VSCode SQL Server extension) into Excel back into the same table. I believe the SQL Server Management Studio can do this but I am not on Windows and the VSCode extension's functionality looks half-baked as of 2026-06-19. Even if the Excel was not originally exported from SQL Server, the ability to move data from a hand-written Excel file into a SQL Server database table may be valuable to other people.

Depends on the following crates:

- [`anyhow`](https://github.com/dtolnay/anyhow) for easier error handling
- [`calamine`](https://github.com/tafia/calamine) for reading Excel
- [`tiberius`](https://github.com/prisma/tiberius) for connecting to SQL Server
- [`tokio`](https://github.com/tokio-rs/tokio) for running async operations
- [`tokio-util`](https://github.com/tokio-rs/tokio) acts as an adapter between a `tokio` stream and `tiberius`
- [`clap`](https://github.com/clap-rs/clap) for handling CLI arguments
