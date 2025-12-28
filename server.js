import "dotenv/config"
import {BskyAgent} from "@atproto/api";

const agent = new BskyAgent({
    service: "https://bsky.social"
})

await agent.login({
    identifier: process.env.USERNAME,
    password: process.env.APP_PASSWORD,
})

await agent.post({
    text: "Node.js Hello world"
})