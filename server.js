import "dotenv/config"
import {BskyAgent} from "@atproto/api";

const agent = new BskyAgent({
    service: "https://bsky.social"
})

await agent.login({

})