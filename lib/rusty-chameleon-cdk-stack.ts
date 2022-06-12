import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { Architecture, Runtime } from 'aws-cdk-lib/aws-lambda';
import * as path from 'path';
import { RustLambda } from '../constructs/rust-lambda';
// import * as sqs from 'aws-cdk-lib/aws-sqs';

export class RustyChameleonCdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const chameleon = new lambda.Function(this, 'RustyChameleonlambda', {
      runtime: Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.join(__dirname, '../src/chameleon/target/lambda/chameleon/bootstrap.zip')),

      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1',
      },
      architecture: Architecture.ARM_64,
    });

    // const test = new RustLambda(this, 'test', {
    //   entry: path.join(__dirname, '../src/chameleon'),
    //   packageName: 'chameleon'
    // })
  }
}
