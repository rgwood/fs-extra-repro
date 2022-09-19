A minimal repro for a bug in [`fs_extra`](https://github.com/webdesus/fs_extra).

Attempting to rename a directory with the same name in a different case ends up deleting the directory silently. I believe this is only an issue on Windows (or other OSs with case-insensitive filesystems?).

Nushell issue: https://github.com/nushell/nushell/issues/6583