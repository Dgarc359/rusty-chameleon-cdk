# 🦀 Rusty Chameleon 🦎

Discord Bot leveraging AWS Lambda

## Setup

* Run `yarn/npm setup:dev:env`
* tbd...

## Scripts

`setup:dev:env`   installs necessary dependencies such as cargo-lambda for developing
`gen:lambda:rust`   generates a rust lambda directory in src, pass in a name for your directory, ex: `yarn gen:lambda:rust example-project`

## Useful commands

* `npm run build`   compile typescript to js
* `npm run watch`   watch for changes and compile
* `npm run test`    perform the jest unit tests
* `cdk deploy`      deploy this stack to your default AWS account/region
* `cdk diff`        compare deployed stack with current state
* `cdk synth`       emits the synthesized CloudFormation template
