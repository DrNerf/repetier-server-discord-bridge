# Repetier Server Discord bridge

### The problem:
[Repetier Server](https://www.repetier-server.com/) has built in push messages, however they don't work with [Discord webhooks](https://discord.com/developers/docs/resources/webhook).

----------
### The solution:
This lightweight bridge server can listen for a Repetier Server push messages and route them to a Discord webhook of your choice.

----------
## Getting started:
### 1. Get a Discord bridge
You need bridge server executable either grab one from the releases or build it from source like this:
```
# Produces a release build targeting the host OS architecture
cargo build --release
```

### 2. Setup your webhook
Create a webhook integration in the desired Discord text channel. Check [this](https://hookdeck.com/webhooks/platforms/how-to-get-started-with-discord-webhooks#how-do-i-add-a-webhook-to-discord) for more information on how to do that.

Once you have your webhook URL you need to set the `DISCORD_BRIDGE_HOOK_URL` environment variable. It should look something like this:
```console
DISCORD_BRIDGE_HOOK_URL=https://discord.com/api/webhooks/XXXXXXXX/XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
```

### 2. Setup Repetier Server
You need to configure repetier server to send push notifications to our Discord bridge. By default the Discord bridge will run on port `3030`.
   1. Open the Repetier Server UI
   2. Click on the ⚙ in the top right
   3. Select `Global Settings`
   4. On the left menu select `Push Messages`
   5. Don't forget to select what type of messages you want in the first segment on this page
   6. **Flip on** the last segment `Third-Party` and in the `URL` field put `http://localhost:3030/push?message={{message}}`

### 3. Run the Discord bridge
The Discord bridge should be started as any other executable. It can even be setup to run as a Windows Service.

### 4. Sweet. All done
Give it whirl. In the same `Push Messages`>`Third-Party` page there is a **🔔 Send Test Message** button. Click it and you should receive a message in Discord with a similar content:
```console
Test message from Repetier-Server 1.4.6
```