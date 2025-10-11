# media-util-bot
> a user installable discord bot that manipultes images (and maybe other media in future) anywhere

<a href="https://discord.com/oauth2/authorize?client_id=1340081114919862344">
  <img alt="Add to account button" src="https://img.shields.io/badge/discord%20bot-add%20to%20account-%23a6d189?style=for-the-badge&logo=discord&logoColor=%238caaee&labelColor=%23414559">
</a>

## Things it can do

| Command | Function                                                  |
| --------|-----------------------------------------------------------|
| /ping   | ping pong                                                 |
| /crush \<file> [bits] | crush the bit depth of any image                          |
| /compress \<file> [quality] | apply JPEG compression to any image (loses alpha channel) |
| /mask \<image> \<mask> | applies the luma of the mask to each pixel of the image   |
| /interact <action> [user] | finds a gif for an interaction from nekos.best |

## Run it yourself
> Docker images are published at [teatowel/media-bot](https://hub.docker.com/r/teatowel/media-bot)

- Create an app on the [Discord developer portal](https://discord.com/developers)
- In the installation tab, enable User Instal under the installation methods.
- Pull Docker container and run. Make sure to set `DISCORD_TOKEN` in the environment to your bot token.
- Note that commands can take some time to appear on Discord. You can set `GUILD_ID` to immediately add commands to a server for testing.
- Check .env.example for more optional environment variables.
