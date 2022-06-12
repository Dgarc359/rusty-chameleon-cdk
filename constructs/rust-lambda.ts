import { Architecture, Code, Function, FunctionOptions, Runtime, } from "aws-cdk-lib/aws-lambda";
import { exec, execSync } from "child_process";
import { Construct } from "constructs";
import path = require("path");

interface RustFunctionProps extends FunctionOptions {
  /** Path to Rust Project Directory
   * Generate your lambda with cargo lambda new
   */
  entry: string;
  packageName: string;
}

export class RustLambda extends Function {
  constructor(scope: Construct, id: string, props: RustFunctionProps) {

    // go to path of rust project and cargo build --release --arm64 --output-format zip
    const {entry, packageName} = props;

    (async () => {
      await exec("echo 'hello world'");
      console.log('entry rust lambda',entry);
    })()
    const pathToPackageZip = path.join(entry, `target/lambda/${packageName}/bootstrap.zip`)

    super(scope, id, {
      code: Code.fromAsset(pathToPackageZip),
      handler: 'not.required',
      environment: {
        RUST_BACKTRACE: '1'
      },
      architecture: Architecture.ARM_64,
      runtime: Runtime.PROVIDED_AL2
    });
  }
}