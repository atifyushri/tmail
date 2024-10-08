# TMail
A temporary email address (although, the address is peristent).

![TMail preview](https://github.com/user-attachments/assets/61d9e1e0-a9cd-4c71-80fd-eb758f838c65)

## Installation
```bash
cargo install tmail
```

## Usage
### Generate
Creates a new address and copies it to clipboard.
```
tmail generate
> user@email.com copied to clipboard!
```

### Me
Copies a **generated** address to clipboard.
```
tmail me
> user@email.com copied to clipboard!
```

### Delete
Deletes a **generated** address.
```
tmail delete
> Account deleted
```

### Fetch
Retrieve messages from inbox and tries to open selected message in browser,
```
tmail fetch
? Select a message
> Email Subject #1 - noreply@example.com
> Email Subject #2 - fake@email.com
[↑↓ to move, enter to select, type to filter]
```

## Credits
This tool uses [mail.tm](https://mail.tm/) under the hood.
Copyright of libraries goes to its respective owners and developers.
