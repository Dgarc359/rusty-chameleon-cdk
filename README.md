# 🦀 Rusty Chameleon 🦎

Discord Bot written in rust, powered by AWS Lambda. Packaged and Deployed with AWS CDK.

## Setup
If you encounter any issues with setup (during any prescribed install / setup script) or after setup (namely during build) please open an issue. Most of the time these seem to be one off issues with missing packages on your dev station. There is an open issue for resolving this issue that will take some time to arrive, but any PRs to expand the setup / install scripts are welcome.

* Run install script for your OS (ex: `yarn/npm install:arch`)
* Run `yarn/npm setup:dev:env`
* tbd...

## Deploy

* Set up AWS CDK (Bootstrap CDK/ AWS CLI Credentials / `npm / yarn i aws-cdk` )
* `yarn deploy`

## Scripts

* `setup:dev:env`   installs necessary dependencies such as cargo-lambda for developing

* `gen:lambda:rust`   generates a rust lambda directory in src, pass in a name for your directory, ex: `yarn gen:lambda:rust example-project`

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `cdk deploy`      deploy this stack to your default AWS account/region
* `cdk diff`        compare deployed stack with current state
* `cdk synth`       emits the synthesized CloudFormation template
