import * as config from './config';
import * as axios from 'axios';
import * as commands from './command';


const {DISCORD_APP_ID, BOT_TOKEN} = config.config;

const url = `https://discord.com/api/v10/applications/${DISCORD_APP_ID}/commands`

const headers = {
  Authorization: `Bot ${BOT_TOKEN}`,
  'Content-Type': 'application/json',
};

Object.keys(commands).map((e) => {
  axios
    //@ts-ignore
    .post(url, JSON.stringify(commands[e]), {
      headers: headers,
    })
    .then((e: any) => {
      console.log(e.status, e.data);
    });
});