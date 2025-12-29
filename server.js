import "dotenv/config"
import {BskyAgent} from "@atproto/api";
import express from "express";


const app = express();
app.use(express.json());

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

app.use((req, res, next) => {
    if(req.headers["x-api-key"]!== process.env.SECRET) {
        return res.status(401).send("nuh uh");
    }

    next()
})

app.post("/post", async(req,res)=>{
    const {text} = req.body;
    if(!text||typeof text !== "string"){
        return res.status(401).send("get real");
    }

    await agent.post({text});
    res.send("posted");
})

app.listen(6967, ()=>{
    console.log("Server started on port 6967");
});



