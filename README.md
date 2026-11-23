Purpose: To have a single place to safely hold all your passwords
First release:
    init    - Prompt the user to choose a master password and create an empty encrypted vault. If a vault already exists, return an error and preserve it.
    add     - Unlock the vault and save a label, username, and password. If the label already exists, return an error and preserve the original entry.
    list    - Unlock the vault and display only the saved labels.
    get     - Unlock the vault and retrieve the username and password for one label.
Threat Model: An attacker may obtain a copy of the locked vault, for example through a backup. The goal is to keep labels, usernames, and passwords confidential even when the attacker has that file. The attacker can try master-password guesses offline. Someone who has both the vault and the correct master password can read its contents. This design assumes the user’s computer is trustworthy while the vault is open.
Design decision: I chose one local vault file so the user can manage logins offline and back them up by copying a single file. This keeps the first release small enough to build and test. It also means that losing the only copy loses every saved login.