## Firefox Web App Launcher

This is a simple helper to launch a new instance of Firefox, with a separate profile, for a web application or website of your liking. It can help you generate a desktop file, so you can see it in the menu, and show on taskbars, pin it on docks, as if it is a separate application from your main Firefox profile. Supports any OS that uses desktop file, as specified by [Free Desktop](freedesktop.org).

There's a myriad of programs on Linux and similar OSes, that can help you "install" a website to your desktop, as if it were a native app. There's no magic in this installation process: it's just a desktop file that specifies an icon, the name, the description, and of course, how to run the program. 

Such installers usually comes with a simplified browser to render the website. Sometimes, they come bundled with an outdated runtime, and would fail to render some websites.

So, instead of writing a browser, I just run Firefox---Firefox with a separate profile for each application, with a special class (this corresponds to an entry in the desktop file, so it can show up with a different icon, rather than the orb-hugging fox).

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


