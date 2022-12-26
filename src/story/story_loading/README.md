
<div align="center">

# 🌷🌷 Story Loading 🌸🌸

</div>

Hello, this is the documentation for how do we load stories into Project Flara.
The stories in Project Flara are stored as an early form of plugins.

They're stored as different crates that are compiled into dynamic module form.
In the GitHub repository, they're stored [here](https://github.com/project-flara/project-flara/tree/main/story). The data structs are located in a shared crate called `framework` that is used by both the host `project-flara` crate and the story plugin crate.

The API should be straight forward to use. Just look at the examples. ^^

# Writing the Story

`mainline` is for the main storyline for Project Flara.
While `events` is for the dialogs, that in big JRPG gacha games would be the post-launch content. As Project Flara is not a guaranteed-to-have-a-big-community game. Events should something that the user starts themselves, perhaps as a sort of progress or something. But that'll require discussion and RFCs.

The dependencies that you'll need should be:
```toml
[dependencies]
bevy = { version = "0.9.1", features = ["dynamic"] }
bevy_rpg = { version = "0.1.0", path = "../../../../bevy-rpg" }
framework = { version = "0.1.0", path = "../../../framework" }
```