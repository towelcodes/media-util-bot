# media-util-bot
> a user installable discord bot that manipultes images (and maybe other media in future) anywhere

<a href="https://discord.com/oauth2/authorize?client_id=1340081114919862344">
  <img alt="Add to account button" src="https://img.shields.io/badge/discord%20bot-add%20to%20account-%23a6d189?style=for-the-badge&logo=discord&logoColor=%238caaee&labelColor=%23414559">
</a>

## Things it can do

| Command | Function |
| --------|----------|
| /ping   | ping pong|
| /crush \<file> [bits] | crush the bit depth of any image |
| /compress \<file> [quality] | apply JPEG compression to any image (loses alpha channel) |

## Run it yourself
> Docker images are published at [teatowel/media-bot](https://hub.docker.com/r/teatowel/media-bot)

- Create an app on the [Discord developer portal](https://discord.com/developers)
- In the installation tab, enable User Instal under the installation methods.
- Pull Docker container and run. Make sure to set `DISCORD_TOKEN` in the environment to your bot token.
