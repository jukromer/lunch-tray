<div align="center">
  <img src="data/icons/hicolor/scalable/apps/de.jukromer.LunchTray.svg" width="128" alt="Lunch Tray icon">
  <h1>Lunch Tray</h1>
  <p>See what's on the menu at your canteen.</p>
</div>

Lunch Tray is a native GNOME app for the menus of all canteens listed on [OpenMensa](https://openmensa.org). It is written in Rust with GTK 4, libadwaita and relm4.

<p align="center">
  <img src="data/screenshots/main-window.png" width="380" alt="Menu of the day with prices for students">
  <img src="data/screenshots/canteen-picker.png" width="380" alt="Choosing a canteen">
</p>

## Features

- Menus of more than 1300 canteens, searchable by name or city
- Browse the days of the week
- Prices for students, employees and guests
- Remembers your canteen and price group
- Shows the last saved menu when you are offline

## Building

Lunch Tray needs GTK 4.22 and libadwaita 1.9 (GNOME 50). On Ubuntu 26.04:

```bash
sudo apt install libgtk-4-dev libadwaita-1-dev
cargo run
```

## Installing for your user

```bash
./install-local.sh
```

This builds a release binary and installs it together with its desktop file and icons to `~/.local`, so Lunch Tray shows up in the GNOME overview. Run it again after pulling changes. To uninstall, delete `~/.local/bin/lunch-tray`, `~/.local/share/applications/de.jukromer.LunchTray.desktop` and the two `de.jukromer.LunchTray` icons under `~/.local/share/icons/hicolor/`.

## Status

Early development, not packaged yet. Planned next: allergen and dietary notes, a nicer layout, and a Flatpak.

## Data

Menus come from the [OpenMensa API](https://docs.openmensa.org/api/v2/). Lunch Tray is not affiliated with OpenMensa.

## License

Lunch Tray is licensed under the GPL-3.0-or-later, see [LICENSE](LICENSE). The symbolic icon is `cutlery-symbolic` from the GNOME Icon Development Kit (CC0).
