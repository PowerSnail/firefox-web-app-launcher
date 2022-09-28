## Firefox Web App Launcher

This is a simple helper to launch a new instance of Firefox, with a separate profile, for a web application or website of your liking. It can help you generate a desktop file, so you can see it in the menu, and show on taskbars, pin it on docks, as if it is a separate application from your main Firefox profile. Supports any OS that uses desktop file, as specified by [Free Desktop](freedesktop.org).

## Usage

`firefox-web-app-launcher <NAME> <URL> <COMMAND>` where COMMAND can be: `run` or `generate-desktop-file`.

For example, to run :

```sh
firefox-web-app-launcher ScoreInURL https://powersnail.com/ScoreInUrl run
```

To generate a desktop file:

```sh
firefox-web-app-launcher StackEdit https://stackedit.io/ generate-desktop-file
```

For extra entries in the desktop file, for example to add icon to your desktop entry, use the `--extra` flag:

```sh
firefox-web-app-launcher AlmostPong https://www.lessmilk.com/almost-pong/ generate-desktop-file --extra "Icon=almostpong.png" --extra "Categories=Game"
```

To save the file into your applications directories:

```sh
firefox-web-app-launcher Instapaper https://www.instapaper.com/u generate-desktop-file -o $HOME/.local/share/applications/instapaper.desktop
```


