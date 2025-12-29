import "dotenv/config"
import {BskyAgent} from "@atproto/api";
import express from "express";


const app = express();
app.use(express.json());

const agent = new BskyAgent({
    service: "https://bsky.social"
})

function splitTheCunt(input){
    const segmenter = new Intl.Segmenter("en",{granularity:"grapheme"});
    const graphemes = [...segmenter.segment(input)].map(s=>s.segment);

    const chunks = [];
    for(let i =0; i<graphemes.length; i+=300){
        chunks.push(graphemes.slice(i,i+300).join(""));
    }

    return chunks;
}

async function threadEmoji(agent, text){
    const chunks = splitTheCunt(text);

    let root = null;
    let parent = null;

    for(let i = 0; i < chunks.length; i++){
        const res = await agent.post({
            text: chunks[i],
            ...(parent && {reply:{root,parent}})
        });

        const ref = {
            uri: res.uri,
            cid: res.cid
        }

        if(!root) root = ref;
        parent = ref;
    }
}

await agent.login({
    identifier: process.env.USERNAME,
    password: process.env.APP_PASSWORD,
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

    try{
        await threadEmoji(agent, text);
        res.send("posted as thread");
    }catch(e){
        console.error(e);
        res.status(500).send("implodes and dies");
    }
})

app.listen(6967, ()=>{
    console.log("Server started on port 6967");
});



