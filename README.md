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
> user@email.com copied to clipboard!
```

### Fetch
Retrieve messages from inbox and tries to open selected message in browser,
```
tmail fetch
? Select a message
> Subject of Email
[↑↓ to move, enter to select, type to filter]
```
