import { Stack, StackProps } from 'aws-cdk-lib';
import { Construct } from 'constructs';
import * as lambda from 'aws-cdk-lib/aws-lambda';
import { Architecture, Runtime } from 'aws-cdk-lib/aws-lambda';
import * as path from 'path';
import { NodejsFunction } from 'aws-cdk-lib/aws-lambda-nodejs';
import { Parallel, StateMachine, Pass} from 'aws-cdk-lib/aws-stepfunctions';
import * as sfnTask from 'aws-cdk-lib/aws-stepfunctions-tasks';

export class RustyChameleonCdkStack extends Stack {
  constructor(scope: Construct, id: string, props?: StackProps) {
    super(scope, id, props);

    const chameleon = new lambda.Function(this, 'RustyChameleonlambda', {
      runtime: Runtime.PROVIDED_AL2,
      code: lambda.Code.fromAsset(path.join(__dirname, '../src/chameleon/target/lambda/chameleon/bootstrap.zip')),

      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1',
        PUBLIC_KEY: "TEST",
      },
      architecture: Architecture.X86_64,
      // allowedOrigins: ['https://discord.com'],
    });


    //const testLambda = new NodejsFunction(this, 'test-lambda', {
    //  entry: path.join(__dirname, '../src/test-lambda/test.ts')
    //})
    
    //const parallel = new Parallel(this, 'start');
    //const testOurLambda = new sfnTask.LambdaInvoke(this, 'test-our-lambda', {
    //  lambdaFunction: testLambda,

    //  outputPath: '$.Payload'
    //});

    //const testRustLambda = new sfnTask.LambdaInvoke(this, 'test-rust-lambda', {
    //  lambdaFunction: chameleon,
    //  outputPath: '$.Payload'
    //})

    //parallel.branch(testOurLambda);
    //parallel.branch(testRustLambda);

    //const stepFnStateMachine = new StateMachine(this, 'state-machine', {
    //  definition: parallel,
      
    //}) 
  }
}
