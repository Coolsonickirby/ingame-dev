# ingame-dev

This is a plugin showcasing the power of ImGUI, as well as allowing users to have more fun in Photo Mode.

## Usage

Install the [host imgui-smash plugin](https://github.com/Coolsonickirby/imgui-smash?tab=readme-ov-file#installing-plugin-user), then install this plugin. To use the features this plugin offers, go into a normal smash match, then go to Photo Mode. Make sure you have a mouse and keyboard plugged in to your switch.


## Planned Features

- Percise Camera Properties Editor
- Stage Elements Editor (Modifying the animations, positions, scales, etc... of stages)
- Timeline Editor (having a main animation that contains keyframes, and interporlates the properties for the Camera, Fighters, etc....)
- Timeline Save and Load
- Etc... (Ideas floating but none concrete)

## Notes

- This works with added characters/modded skins
- It's very prone to crashing, so it's recommended to use it with a minimal setup (just all the mods you need for the picture/scene)
- Although the trailer did show me putting a wireless mouse and keyboard on the table before using the plugin, wireless m&k does NOT play nicely with the switch. It's recommended to use a wired mouse and keyboard with this plugin (and imgui-smash in general.)

## Credits
- [Coolsonickirby](https://github.com/Coolsonickirby) - [imgui-smash](https://github.com/Coolsonickirby/imgui-smash) and this plugin
- [Raytwo](https://github.com/Raytwo) & [itsmeft24](https://github.com/itsmeft24) - Resource and offsets code used to get the `motion_list.bin` for the motion options
- [ThatNintendoNerd](https://github.com/ThatNintendoNerd) - [CameraMeleePhotoController struct](https://github.com/ThatNintendoNerd/camera_free/blob/main/src/app/camera/camera_melee_photo_controller.rs#L9-L76)
- [Zarek Syed](https://github.com/zrksyd) - Helped with setting a fighter's animation
- [PhazoGanon](https://github.com/Armasher5872) - Helped with giving Final Smashe
- [WuBoy](https://github.com/WuBoyTH) - Helped with moving the characters along the Y-Axis
- [Joey de Vries](https://joeydevries.com/#home) - [OpenGL tutorial about Camera (used for freecam)](https://learnopengl.com/Getting-started/Camera)
