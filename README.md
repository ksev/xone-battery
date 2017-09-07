# xone-battery

xone-battery is a super simple windows application that adds the battery status of your XInput controller in the system tray.
I tried to itegrate the application as cleanly as possible into windows 10 and it's look and feel.

#### Screenshots
This is all the app does. Displays the current battery state.

| State                   | Image                     |
| ----------------------- |:-------------------------:|
| No controller connected | ![No controller][none-ex] |
| Battery is empty        | ![Empty][empty-ex]        |
| Battery low             | ![Low][low-ex]            |
| Battery half full       | ![Medium][medium-ex]      |
| Battery full            | ![Full][full-ex]          |

[none-ex]: https://raw.githubusercontent.com/ksev/xone-battery/master/res/none_ex.png
[empty-ex]: https://raw.githubusercontent.com/ksev/xone-battery/master/res/empty_ex.png
[low-ex]: https://raw.githubusercontent.com/ksev/xone-battery/master/res/low_ex.png
[medium-ex]: https://raw.githubusercontent.com/ksev/xone-battery/master/res/medium_ex.png
[full-ex]: https://raw.githubusercontent.com/ksev/xone-battery/master/res/full_ex.png

#### Building
As of the time of writing xone-battery requires nightly rustc to compile since it uses the `impl Trait` feature that is not yet stable. 
You should also execute the `build.ps1` powershell script to build since it needs to pass a few parameters to rustc for the program to link correctly.