import * as commands from './commands';
import axios from 'axios';
// import { config } from './config';
import { ssmCredentials } from './ssm';

console.debug(commands);

const sendRequest = async (commands: any, parameters: {id: string, token: string}) => {

    // const { LAMBDA_BOT_APPLICATION_ID, LAMBDA_BOT_TOKEN } = await config();
    // const
    const config: any = await ssmCredentials([""]);
    const LAMBDA_BOT_APPLICATION_ID: String = config[parameters.id];
    const url = `https://discord.com/api/v10/applications/${LAMBDA_BOT_APPLICATION_ID}/commands`;

    const LAMBDA_BOT_TOKEN = config[parameters.token]
    const headers = {
        Authorization: `Bot ${LAMBDA_BOT_TOKEN}`,
        'Content-Type': 'application/json',
    };
    Object.keys(commands).map((e) => {
        axios
            //@ts-ignore
            .post(url, JSON.stringify(commands[e]), {
                headers: headers,
            })
            .then((e: { status: unknown, data: unknown }) => {
                console.log(e.status, e.data);
            })
            .catch((err: any) => { throw new Error(err) });
    });
}

await sendRequest(commands, {id: "CHAMELEON_BOT_APP_ID", token: "CHAMELEON_BOT_TOKEN"});