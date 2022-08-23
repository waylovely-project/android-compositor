# Waylovely
Run all your Linux graphical apps on Android 💖✨✨✨

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
