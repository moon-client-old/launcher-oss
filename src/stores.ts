import {writable} from "svelte/store";

export class UserContext {
    username: string;
    rank: UserRank;
    channels: Array<Channel>;

    serialize(json: object) {
        this.username = json.username;
        this.rank = UserRank[json.rank];
        // Parse all channels
        this.channels = new Array<Channel>
        for (let i = 0; i < json.available_channels.length; i++) {
            let availableChannel = json.available_channels[i];
            let channel = new Channel();
            channel.serialize(availableChannel);
            this.channels[i] = channel;
        }
    }
}

export class Channel {
    name: string;
    description: string;
    latestVersion: string;
    lastUpdated: bigint;
    versions: Array<Version>;

    serialize(json: Object) {
        this.name = json.name;
        this.description = json.description;
        this.latestVersion = json.latestVersion;
        this.lastUpdated = json.lastUpdated;
        this.versions = new Array<Version>;
        for (let i = 0; i < json.availableVersions.length; i++) {
            let availableVersion = json.availableVersions[i];
            let version = new Version();
            version.serialize(availableVersion);
            this.versions[i] = version;
        }
    }
}

export class Version {
    id: string;
    name: string;
    changelog: string;
    releasedAt: bigint;

    serialize(json: Object) {
        this.id = json._id;
        this.name = json.name;
        this.changelog = json.changelog;
        this.releasedAt = json.releasedAt;
    }
}

export enum UserRank {
    ADMIN = "Admin",
    STAFF = "Staff",
    BETA = "Beta",
    USER = "User"
}

export const userContext = writable(new UserContext())