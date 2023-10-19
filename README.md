# This project is on halt!
This attempt at bringing Linux applications to Android devices, called the "Waylovely Compositor" has been on halt for... around more than a year and some months.
(different from ["Waylovely Stories"](instagram.com/waylovely.stories) which is an indie multimedia doujinshi circle that has me, Astriya, as one of its members derived from the same name. You might see a big banner advertising it soon. Also please follow and share!)

At first, the code lives in [a fork of the Smithay compositor](https://github.com/waylovely-project/smithay). This codebase at first was intended to be submitted to upstream, but then it became intended only for prototyping. Perhaps the core reason is that upstream added the Smallvil compositor that is much simpler because it only has support for Winit, which is how the "Waylovely Compositor" prototype was started the first time, because the initial driving force for me was, "Hey, Winit works on Android you know? So theoritically Smithay too."

If you are looking to try the experiment that has at least been run by someone, you can try this codebase. I created the APK using the [`cargo-apk`](https://github.com/rust-mobile/cargo-apk) tooling which is now deprecated. You also have to build the dependencies, which are built using an earlier version of Waylovely Compositor's [`kawaii`](https://github.com/waylovely-project/kawaii) build system using [this repository](https://github.com/waylovely-project/waylovely-build) as the package repository details (it... works a bit like Meson's repository or perhaps Gentoo?). Do something and something (I have forgotten the details, but just try compiling with cargo apk and you should face the issues to solve?). Do remember to insert the dependencies that you have built into the APK, as if I remember correctly this had to be done manually by me.

Overall, that experiment works, but the experimental compositor version has not been tested with an actual app. More questions related on how to get the apps loaded was raised. Newer Android versions disable the execute permissions for files that can be modified by the application and the binaries that come with the APK cannot be modified at all. This means that running any programs downloaded from the Internet or any other sources is impossible, except perhaps for interpreted and JIT compilers (hey how do browsers work then).

The plan was to use QEMU (though performance will suffer), compile the apps into WASI format, or depend on KVM. But then... this project is dropped altogether.

You also should look into this repository, which has a barebone XDG Portals implementation for Android and also the overall foundation to properly build Waylovely Compositor. I believe the Kawaii buildsystem files have been setup. If you like to fork this project, please fork this repository rather than the previous experiment.

___

![Waylovely](https://user-images.githubusercontent.com/66000635/193557047-993b33ed-58cd-4d6a-a8e2-890cde6bcf4a.png)

<div align="center"><h2>Run all your Linux graphical apps on Android 💖✨✨✨</h2></div>

The Android windowing system is different than those in desktop operating systems. In the desktop, windowing systems allow applications to open as much windows as the hardware able to run. However this is not the case in Android!! Android is intended to be oriented towards mobile devices, which has less powerful hardware than desktops, so it will do things like only allowing an app to get one EGLSurface!!

This makes porting windowing toolkits (and UI toolkits too!) to Android a bit difficult, most windowing toolkits like GLFW or GDK are built around the idea of having multiple. Not only that, but for GDK in particular, the developers are quite too short-handed to work on things that aren't prioritized, so obviously maintaining yet another GDK backend would be troublesome!!

Waylovely attempts to bridge the gap between Android and Linux applications by bringing interfaces found in desktop Linux systems, packaged as an Android app!!

## Development
The development is splitted up in different repositories!!

- [Smithay](https://github.com/Smithay/smithay) - The Waylovely Project contributes a backend in Smithay for use in Android applications!! Here is our [PR](https://github.com/Smithay/smithay/pull/711)!
- [xdg-desktop-portal-android](https://github.com/waylovely-project/xdg-desktop-portal-android) - Xdg Desktop Portal implementation for Android. This is also useful for SDL-based applications!! SDL actually can render without the need of a Wayland compositor but the app might depend on Portals.
- [simple-and-kawaii](https://github.com/waylovely-project/simple-and-kawaii) - A custom build system for boostrapping a large chunk of dependencies!! Not a package manager.
- [waylovely-build](https://github.com/waylovely-project/waylovely-buid) - A repository of libraries needed by Waylovely and friends!!

<!--## Step 1: Getting the compositor -->

### Related Things

#### AOSP backend for Weston - Pekka Paalanen

Back in 2012-2013, Collabora's Pekka Paalanen was working to have a fork of AOSP that uses Wayland as its display server!! However since there aren't any work anymore on that, it was removed a few months later.

I wonder if I should explore the mythical lands of getting Wayland inside of today's AOSP, but I think that's not the MVP requirement! 

- [First Signs Of Wayland Running On Android - Phoronix](https://www.phoronix.com/news/MTA5MzA)
- [Wayland's Weston Running On Android - Phoronix](https://www.phoronix.com/news/MTEwNjQ)
- [The Android Back-End For Wayland's Weston - Phoronix](https://www.phoronix.com/news/MTExMDU)
- [Android Support Removed From Wayland's Weston - Phoronix](https://www.phoronix.com/news/MTI4Mjk)

### Why Wayland on Android is a hard problem - Jason Ekstrand

Jason Ekstrand is a developer for libhydris, a library used by "Linux on mobile phones" projects to have a great support for Android OEMs!!
In this article, he lays out the technical differences between the windowing systems and EGL support in Android and Wayland, and why it's quite hard to make them talk to each other nicely, I feel in particular, actually making Wayland clients talk to Android with some compatibility layers (akin to DXVK or MetalVK)!!

[Why Wayland on Android is a hard problem - Jason Ekstrand](https://www.jlekstrand.net/jason/projects/wayland/wayland-android/)
