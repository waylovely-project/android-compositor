# Waylovely
Run all your Linux graphical apps on Android 💖✨✨✨

The Android windowing system is different than those in desktop operating systems. In the desktop, windowing systems allow applications to open as much windows as the hardware able to run. However this is not the case in Android!! Android is intended to be oriented towards mobile devices, which has less powerful hardware than desktops, so it will do things like only allowing an app to get one EGLSurface!!

This makes porting windowing toolkits (and UI toolkits too!) to Android a bit difficult, most windowing toolkits like GLFW or GDK are built around the idea of having multiple. Not only that, but for GDK in particular, the developers are quite too short-handed to work on things that aren't prioritized, so obviously maintaining yet another GDK backend would be troublesome!!

Waylovely attempts to bridge the gap between Android and Linux applications by bringing interfaces found in desktop Linux systems, packaged as an Android app!!

## Development
The development is splitted up in different repositories!!

- [Smithay](https://github.com/Smithay/smithay) - The Waylovely Project contributes a backend in Smithay for use in Android applications!! Here is our [PR](https://github.com/Smithay/smithay/pull/711)!
- [xdg-desktop-portal-android](https://github.com/waylovely-project/xdg-desktop-portal-android) - In order for desktop Linux apps to use relevant Android APIs, we need . So we have this Xdg Desktop Portal implementation for Android. This is also useful for SDL-based applications!! SDL actually can render without the need of a Wayland compositor but the app might depend on Portals.

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
