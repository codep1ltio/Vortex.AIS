## Welcome!
We won't make this too long for you to read.

### Introduction to Vortex AIS
- Community -> https://discord.gg/E9y6WfEdPW
- > Join our community and help us come up with ideas, or contribute, or just hangout and keep up with new updates!

> Vortex AIS is a launcher for Vortex, our main purpose is to add mod compatibility and make modding Vortex alot easier.
> Users can make their own mods and publish it to our Addon Library but of course we will have to verify and check the mod's code first.
### How to Setup AIS?
> Once you've installed the latest version of AIS binary for Linux/Windows, it's recommended that you create a folder containing the AIS binary.
> After starting the program, it will automatically create required files e.g. mods folder, data.json
> - It's recommended that you don't delete anything in the mods folder and data.json as it can break stuff.
>
> If you need any help, we are free to help in our discord server!

#### About mods (Addons)
> Our addons' language will be in lua which would be very familiar for you guys and also it's very easy for us
> developers to expand on it instead of using other languages like C#, Rust, or python.

Example Code
```lua
on("self_join", function(player)
    player:send_chat("hi guys")
end)
```
^ This chunk of code is supposed to send "hi guys" message when you connect into a game.

##### How to publish mods?
> If you have made a mod(Addon) and you want to publish it for others to use in the Addons Library,
> contact p1lt(Code), @cod.io in discord or join our discord server.
> After that, you will have to send us the code then we will publish it for you if it's safe and works.
> - How to be a trusted publisher?
> - > A trusted publisher is one that can update/change their addon anytime they want without contacting the developers of AIS,
>   > (more info coming soon.. ..we havent decided stuff that much yet..)
