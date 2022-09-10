# Twitch MagicBot

### TODO:
- [ ] use database to store commands
    - Fields of a command.
    **Note: each user should have your commands:**
        - id: unique command id.
        - name: name of the command. Ex: !ping
        - response: what the command should respond. Ex: Pong!
        - global_timeout: global timeout in seconds.
        - user_timeout: per user timeout in seconds.
        - permissions: which type of permission user should have to use the command. Should I use numbers for represent permissions? Each permission includes all below.
            - broadcaster: broadcaster permissions.
            - moderator: moderator or higher permissions.
            - subscriber: subscriber or higher permissions.
            - vip: vip or higher permissions.
            - user: user or higher permissions.
- [ ] basic commands
    - [ ] !ping - Ping command
    - [ ] !commands - Show all commands
        - [ ] add <args> - Add a command
        - [ ] del <args> - Delete a command
        - [ ] edit <args> - Edit a command

Twitch Version of My [Magic Bot](https://github.com/fadiinho/MagicBot)
