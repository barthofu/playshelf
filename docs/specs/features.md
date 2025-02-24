# Features

### Adopted

- Game indexing based on the filesystem
    - This permit mass indexing of potentially already existing collections
    - It relies on naming convention ([see this for examples](https://gamevau.lt/docs/server-docs/structure))
- Game archive upload from the client to populate the collection, with direct metadata edition while uploading
- Metadata
    - There are two kind of metadata: client and server ones
    - Server metadata are retrieved from https://rawg.io/
        - Automatic mapping
        - Can also remap it manually from the client in the admin panel
    - Client metadata are things like:
        - Status (playing, finished, etc)
        - Notes
        - Reviews
        - Tags
        - Playtime
        - etc
    - Metadata (client and server) must be synced between the client and the server
- Custom server collections (e.g: *My best games of 2023*, *Games to play when chilling*, etc)
- Integrations to import third-parties libraries (Steam, GOG, Epic Games, Origins, etc)
    - This is bound to the client. In fact, the hosted DRM-free game library will go alongside the third-parties libraries
- “Cloud” backups for locale game saves
    - For each game the user can set the game save folder
    - We could provide default paths suggestions for popular games
- In-app user submition system
- Users
- Groups (like on Paperless)
- Granular permissions for users, groups and tags
- Statistics per PlayShelf library
- Admin console on the client
- User settings
- Download queue
- Offline mode
    - When going offline, you should still be able to play your games
    - You can still edit local 
- Community
    - Communities are scoped to a single PlayShelf library
    - See the playtime, statuses, notes and reviews of other users
    - Chat with other users
        - Share screenshots
- Real time users status from the admin panel (or for all users?)
    - See who is online
    - See what they are playing
    - This is done with websockets

### To be evaluated

- Emulation support
    - With embedded emulators files?