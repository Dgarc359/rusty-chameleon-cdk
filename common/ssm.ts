import * as AWS from 'aws-sdk';
const creds = new AWS.Config({
  region: 'us-east-1'
})
const SSM = new AWS.SSM(creds);

export const ssmCredentials = async (credentials: string[], region?: string): Promise<Record<string, string |undefined>> => {
  const response = await SSM.getParameters({
    WithDecryption: true,
    Names: credentials
  })
    .promise()
    .then((resp: any) => resp.Parameters)
    .catch((err: any) => { throw new Error(err) });

  if(response?.length === 0 || !response) throw new Error("No params received");

  credentials.sort();
  const returnCredentials: Record<string, string | undefined> = {};

  credentials.forEach((element, i) => {
    if(!response[i].Value) throw new Error('Failed to retrieve params');
    returnCredentials[element] = response[i].Value;
  });

  return returnCredentials;
}