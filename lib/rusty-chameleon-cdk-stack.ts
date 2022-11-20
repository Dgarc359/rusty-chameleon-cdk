import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { Architecture, Runtime } from 'aws-cdk-lib/aws-lambda';
import * as path from 'path';
import {config} from 'dotenv';

export class RustyChameleonCdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);
    config(); // config dotenv

    const pk = process.env.PUBLIC_KEY;
    if(!pk) {
      throw new Error("No Public Key Set in Env Vars")
    }
      // ? process.env.PUBLIC_KEY
      // : throw new Error("Public key not set");

    const chameleon = new lambda.Function(this, 'RustyChameleonlambda', {
      runtime: Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.join(__dirname, '../src/chameleon/target/lambda/chameleon/bootstrap.zip')),

      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1',
        PUBLIC_KEY: pk, // will throw invalidStringLength error if not set
        NASA_API_KEY: process.env.NASA_API_KEY || "",
      },
      architecture: Architecture.X86_64,
    });
  }
}
